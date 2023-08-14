use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use polkadot::{
    oss::{calls::TransactionApi, events::Authorize, storage::StorageApi},
    runtime_types::{
        cp_cess_common::{Hash as CPHash, SpaceProofInfo},
        // cp_bloom_filter::BloomFilter,
        // pallet_oss::{
        //     // sr25519::app_sr25519::{Public, Signature},
        //     // types::{},
        // },
        sp_core::bounded::{bounded_vec::BoundedVec, weak_bounded_vec::WeakBoundedVec},
    },
};
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn oss_storage() -> StorageApi {
    polkadot::storage().oss()
}

fn oss_tx() -> TransactionApi {
    polkadot::tx().oss()
}

impl Sdk {
    /* Query functions */

    // query_authority_list
    pub async fn query_authority_list(&self, pk: &[u8]) -> Result<AccountId32> {
        let account = account_from_slice(pk);

        let query = oss_storage().authority_list(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // oss
    pub async fn oss(&self, pk: &[u8]) -> Result<[u8; 38]> {
        let account = account_from_slice(pk);

        let query = oss_storage().oss(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /* Transactional functions */

    // authorize
    pub async fn authorize(&self, pk: &[u8]) -> Result<(String, Authorize)> {
        let account = account_from_slice(pk);

        let tx = oss_tx().authorize(account);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(authorize) = events.find_first::<Authorize>()? {
            return Ok((tx_hash, authorize));
        } else {
            bail!("Unable to authorize");
        }
    }

    pub async fn cancel_authorize(&self) -> Result<String> {
        let tx = oss_tx().cancel_authorize();

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn register_deoss(&self, endpoint: [u8; 38]) -> Result<String> {
        let tx = oss_tx().register(endpoint);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn update(&self, endpoint: [u8; 38]) -> Result<String> {
        let tx = oss_tx().update(endpoint);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn destroy(&self) -> Result<String> {
        let tx = oss_tx().destroy();

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
}
