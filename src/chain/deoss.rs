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
    runtime_types::{bounded_collections::bounded_vec::BoundedVec, pallet_oss::types::OssInfo},
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
    async fn query_authority_list(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<AccountId32>>>;
    async fn query_oss(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<OssInfo>>;
    async fn authorize(&self, pk: &[u8]) -> Result<(String, Authorize)>;
    async fn cancel_authorize(&self, pk: &[u8]) -> Result<String>;
    async fn register_deoss(&self, endpoint: [u8; 38], domain: BoundedVec<u8>) -> Result<String>;
    async fn update(&self, endpoint: [u8; 38], domain: BoundedVec<u8>) -> Result<String>;
    async fn destroy(&self) -> Result<String>;
}

#[async_trait]
impl DeOss for ChainSdk {
    /* Query functions */

    // query_authority_list
    async fn query_authority_list(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<AccountId32>>> {
        let account = account_from_slice(pk);

        let query = oss_storage().authority_list(&account);

        query_storage(&query, block_hash).await
    }

    // oss
    async fn query_oss(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<OssInfo>> {
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

    async fn register_deoss(&self, endpoint: [u8; 38], domain: BoundedVec<u8>) -> Result<String> {
        let tx = oss_tx().register(endpoint, domain);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn update(&self, endpoint: [u8; 38], domain: BoundedVec<u8>) -> Result<String> {
        let tx = oss_tx().update(endpoint, domain);

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

#[cfg(test)]
mod test {
    use crate::{chain::ChainSdk, core::utils::account::parsing_public_key};

    use super::DeOss;

    const MNEMONIC: &str =
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

    const ACCOUNT_ADDRESS: &str = "cXjmuHdBk4J3Zyt2oGodwGegNFaTFPcfC48PZ9NMmcUFzF6cc";

    fn init_chain() -> ChainSdk {
        ChainSdk::new(MNEMONIC, "service_name")
    }

    #[tokio::test]
    async fn test_authorize() {
        let sdk = init_chain();
        let pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
        let result = sdk.authorize(&pk_bytes).await;
        match result {
            Ok(r) => {
                println!("Account authorize successful: {:?}", r);
            }
            Err(e) => {
                println!("Account authorize failed: {:?}", e);
            }
        }
    }
}
