//! # Chain Abstraction Layer
//!
//! This module defines generic asynchronous traits and types that abstract over
//! Substrate-based chain interactions using [`subxt`].
//!
//! It provides a unified interface for:
//! - Querying on-chain storage ([`Query`])
//! - Submitting and watching extrinsics ([`Call`])
//! - Retrieving chain metadata ([`Chain`])
//! - Managing dynamic signers ([`DynSigner`])
//!
//! ## Traits Overview
//! - [`Chain`]: Base trait providing chain-level utility methods (like block queries).
//! - [`Query`]: Extends [`Chain`] to read on-chain storage (single or iterable).
//! - [`Call`]: Extends [`Chain`] to build, sign, and submit extrinsics.
//!
//! ## Type Overview
//! - [`AnySigner`]: A trait object for any `SubxtSigner` implementation (e.g., Ledger, keypair).
//! - [`DynSigner`]: Wrapper around `AnySigner` with reference-counted ownership.
//!
//! ## Example
//! ```ignore
//! use crate::chain::{Call, Query, DynSigner};
//!
//! // Example: querying on-chain balance
//! let balance = MyQueryStruct::execute_query(&account_storage_key, None).await?;
//!
//! // Example: signing and submitting a transaction
//! let signer = DynSigner::new(Box::new(LedgerSigner::new("m/44'/354'/0'/0'/0'")?));
//! let tx_result = MyCallStruct::sign_and_submit_tx_then_watch_default(&tx, &signer).await?;
//! ```
//!
//! ## Notes
//! - All network access is done through `init_api()`.
//! - Errors are wrapped into a unified [`Error`] type for simplicity.

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

/// Type alias for a boxed Subxt signer object that is thread-safe.
pub type AnySigner = Box<dyn SubxtSignerTrait<PolkadotConfig> + Send + Sync>;

/// A dynamic signer that can hold any Subxt-compatible signer (e.g. Ledger, keypair).
///
/// Wraps the underlying signer in an [`Arc`] for shared ownership across async contexts.
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

/// Implements the [`Signer`] trait for [`DynSigner`], delegating all calls to the inner signer.
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

/// Provides core chain utility functions such as fetching block metadata.
///
/// This trait is meant to be extended by other higher-level traits like [`Query`] and [`Call`].
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

/// Provides methods for reading on-chain storage via [`subxt`].
///
/// This includes fetching single values or iterating through iterable maps.
#[async_trait]
pub trait Query: Chain {
    type Api;

    fn get_api() -> Self::Api;

    /// Executes a one-time read query from chain storage.
    ///
    /// Optionally specify a `block_hash` to query at a specific block.
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

    /// Internal helper to perform the actual storage read.
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

    /// Executes a query that returns an iterable stream of results.
    ///
    /// Useful for large maps or datasets stored on-chain.
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

    /// Internal helper to iterate through storage entries.
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

/// Provides methods for signing and submitting extrinsics.
///
/// This trait extends [`Chain`] and assumes a signer capable of producing signatures.
#[async_trait]
pub trait Call: Chain {
    type Api;

    fn get_api() -> Self::Api;
    fn get_signer(&self) -> &DynSigner;

    /// Extracts the first occurrence of a specific event type from extrinsic events.
    ///
    /// Returns both the extrinsic hash (as a hex string) and the event data.
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

    /// Signs and submits a transaction, then waits for it to be finalized successfully.
    ///
    /// Returns the resulting [`ExtrinsicEvents`] for further inspection.
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
