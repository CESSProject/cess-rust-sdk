use super::ChainSdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use async_trait::async_trait;
use polkadot::{
    oss::{calls::TransactionApi, events::Authorize, storage::StorageApi},
    runtime_types::sp_core::bounded::bounded_vec::BoundedVec,
};
use subxt::ext::sp_core::H256;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn oss_storage() -> StorageApi {
    polkadot::storage().oss()
}

fn oss_tx() -> TransactionApi {
    polkadot::tx().oss()
}

#[async_trait]
pub trait DeOss {
    async fn query_authority_list(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<BoundedVec<AccountId32>>>;
    async fn query_oss(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<[u8; 38]>>;
    async fn authorize(&self, pk: &[u8]) -> Result<(String, Authorize)>;
    async fn cancel_authorize(&self, pk: &[u8]) -> Result<String>;
    async fn register_deoss(&self, endpoint: [u8; 38]) -> Result<String>;
    async fn update(&self, endpoint: [u8; 38]) -> Result<String>;
    async fn destroy(&self) -> Result<String>;
}

#[async_trait]
impl DeOss for ChainSdk {
    /* Query functions */

    // query_authority_list
    async fn query_authority_list(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<BoundedVec<AccountId32>>> {
        let account = account_from_slice(pk);

        let query = oss_storage().authority_list(&account);

        query_storage(&query, block_hash).await
    }

    // oss
    async fn query_oss(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<[u8; 38]>> {
        let account = account_from_slice(pk);

        let query = oss_storage().oss(&account);

        query_storage(&query, block_hash).await
    }

    /* Transactional functions */

    // authorize
    async fn authorize(&self, pk: &[u8]) -> Result<(String, Authorize)> {
        let account = account_from_slice(pk);

        let tx = oss_tx().authorize(account);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(authorize) = events.find_first::<Authorize>()? {
            Ok((tx_hash, authorize))
        } else {
            bail!("Unable to authorize");
        }
    }

    async fn cancel_authorize(&self, pk: &[u8]) -> Result<String> {
        let account = account_from_slice(pk);
        let tx = oss_tx().cancel_authorize(account);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn register_deoss(&self, endpoint: [u8; 38]) -> Result<String> {
        let tx = oss_tx().register(endpoint);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn update(&self, endpoint: [u8; 38]) -> Result<String> {
        let tx = oss_tx().update(endpoint);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn destroy(&self) -> Result<String> {
        let tx = oss_tx().destroy();

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
}
