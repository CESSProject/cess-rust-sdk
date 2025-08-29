pub mod audit;
pub mod balances;
pub mod file_bank;
pub mod oss;
pub mod storage_handler;

use crate::core::Error;
use crate::{init_api, StorageAddress, Yes, H256};
use async_trait::async_trait;
use std::marker::Sync;
use std::sync::Arc;
use subxt::backend::StreamOfResults;
use subxt::storage::StorageKeyValuePair;
use subxt::{
    blocks::ExtrinsicEvents,
    tx::{Payload, Signer as SubxtSignerTrait},
    Config, PolkadotConfig,
};

pub type AnySigner = Box<dyn SubxtSignerTrait<PolkadotConfig> + Send + Sync>;

#[derive(Clone)]
pub struct DynSigner {
    inner: Arc<AnySigner>,
}

impl DynSigner {
    pub fn new(inner: AnySigner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl SubxtSignerTrait<PolkadotConfig> for DynSigner {
    fn account_id(&self) -> <PolkadotConfig as Config>::AccountId {
        self.inner.account_id()
    }

    fn address(&self) -> <PolkadotConfig as Config>::Address {
        self.inner.address()
    }

    fn sign(&self, payload: &[u8]) -> <PolkadotConfig as Config>::Signature {
        self.inner.sign(payload)
    }
}

#[async_trait]
pub trait Chain {
    async fn get_latest_block() -> Result<u64, Error> {
        let api = init_api()
            .await
            .map_err(|_| Error::Custom("All connections failed.".into()))?;

        let block = api.blocks().at_latest().await?;
        Ok(block.number().into())
    }
}

#[async_trait]
pub trait Query: Chain {
    type Api;

    fn get_api() -> Self::Api;

    async fn execute_query<'address, Address>(
        query: &'address Address,
        block_hash: Option<H256>,
    ) -> Result<Option<<Address as StorageAddress>::Target>, Error>
    where
        Address: StorageAddress<IsFetchable = Yes> + Sync + 'address,
    {
        match Self::query_storage(query, block_hash).await {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    async fn query_storage<'address, Address>(
        query: &'address Address,
        block_hash: Option<H256>,
    ) -> Result<Option<<Address as StorageAddress>::Target>, Error>
    where
        Address: StorageAddress<IsFetchable = Yes> + Sync + 'address,
    {
        let api = init_api()
            .await
            .map_err(|_| Error::Custom("All connections failed.".into()))?;
        if let Some(block_hash) = block_hash {
            match api.storage().at(block_hash).fetch(query).await {
                Ok(value) => Ok(value),
                Err(_) => Err("Failed to retrieve data from storage".into()),
            }
        } else {
            match api.storage().at_latest().await {
                Ok(mid_result) => match mid_result.fetch(query).await {
                    Ok(value) => Ok(value),
                    Err(_) => Err("Failed to retrieve data from storage".into()),
                },
                Err(_) => Err("Failed to retrieve data from storage".into()),
            }
        }
    }

    async fn execute_iter<Address>(
        query: Address,
        block_hash: Option<H256>,
    ) -> Result<StreamOfResults<StorageKeyValuePair<Address>>, Error>
    where
        Address: StorageAddress<IsIterable = Yes> + 'static + Send,
        Address::Keys: 'static + Sized,
    {
        match Self::query_iter_storage(query, block_hash).await {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    async fn query_iter_storage<Address>(
        query: Address,
        block_hash: Option<H256>,
    ) -> Result<StreamOfResults<StorageKeyValuePair<Address>>, Error>
    where
        Address: StorageAddress<IsIterable = Yes> + 'static + Send,
        Address::Keys: 'static + Sized,
    {
        let api = init_api()
            .await
            .map_err(|_| Error::Custom("All connections failed.".into()))?;
        if let Some(block_hash) = block_hash {
            match api.storage().at(block_hash).iter(query).await {
                Ok(value) => Ok(value),
                Err(_) => Err("Failed to retrieve data from storage".into()),
            }
        } else {
            match api.storage().at_latest().await {
                Ok(mid_result) => match mid_result.iter(query).await {
                    Ok(value) => Ok(value),
                    Err(_) => Err("Failed to retrieve data from storage".into()),
                },
                Err(_) => Err("Failed to retrieve data from storage".into()),
            }
        }
    }
}

#[async_trait]
pub trait Call: Chain {
    type Api;

    fn get_api() -> Self::Api;
    fn get_signer(&self) -> &DynSigner;

    fn find_first<E: subxt::events::StaticEvent>(
        event: ExtrinsicEvents<PolkadotConfig>,
    ) -> Result<(String, E), Error> {
        let hash = event.extrinsic_hash();
        match event.find_first::<E>() {
            Ok(data) => {
                if let Some(event_data) = data {
                    Ok((format!("0x{}", hex::encode(hash.0)), event_data))
                } else {
                    Err("Error: Unable to fetch event".into())
                }
            }
            Err(e) => Err(format!("{}", e).into()),
        }
    }

    async fn sign_and_submit_tx_then_watch_default<Call>(
        tx: &Call,
        from: &DynSigner,
    ) -> Result<ExtrinsicEvents<PolkadotConfig>, Error>
    where
        Call: Payload + Sync,
    {
        let api = init_api().await?;

        match api.tx().sign_and_submit_then_watch_default(tx, from).await {
            Ok(result) => match result.wait_for_finalized_success().await {
                Ok(r) => Ok(r),
                Err(e) => Err(format!("{}", e).into()),
            },
            Err(e) => Err(format!("{}", e).into()),
        }
    }
}
