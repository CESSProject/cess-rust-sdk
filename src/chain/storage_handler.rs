use super::ChainSdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use async_trait::async_trait;
use polkadot::{
    runtime_types::pallet_storage_handler::types::OwnedSpaceDetails,
    storage_handler::{
        calls::TransactionApi,
        events::{BuySpace, ExpansionSpace, RenewalSpace},
        storage::StorageApi,
    },
};
use subxt::ext::sp_core::H256;
use subxt::tx::PairSigner;

fn storage_handler_storage() -> StorageApi {
    polkadot::storage().storage_handler()
}

fn storage_handler_tx() -> TransactionApi {
    polkadot::tx().storage_handler()
}

#[async_trait]
pub trait StorageHandler {
    async fn query_user_owned_space(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<OwnedSpaceDetails>>;
    async fn query_unit_price(&self, block_hash: Option<H256>) -> Result<Option<u128>>;
    async fn query_total_power(&self, block_hash: Option<H256>) -> Result<Option<u128>>;
    async fn query_total_space(&self, block_hash: Option<H256>) -> Result<Option<u128>>;
    async fn query_purchased_space(&self, block_hash: Option<H256>) -> Result<Option<u128>>;
    async fn buy_space(&self, gib_count: u32) -> Result<(String, BuySpace)>;
    async fn expansion_space(&self, gib_count: u32) -> Result<(String, ExpansionSpace)>;
    async fn renewal_space(&self, days: u32) -> Result<(String, RenewalSpace)>;
    async fn update_price(&self) -> Result<String>;
}

#[async_trait]
impl StorageHandler for ChainSdk {
    /* Query functions */
    // query_user_owned_space
    async fn query_user_owned_space(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<OwnedSpaceDetails>> {
        let account = account_from_slice(pk);
        let query = storage_handler_storage().user_owned_space(&account);

        query_storage(&query, block_hash).await
    }

    // query_unit_price
    async fn query_unit_price(&self, block_hash: Option<H256>) -> Result<Option<u128>> {
        let query = storage_handler_storage().unit_price();

        query_storage(&query, block_hash).await
    }

    // query_total_power
    async fn query_total_power(&self, block_hash: Option<H256>) -> Result<Option<u128>> {
        let query = storage_handler_storage().total_idle_space();

        query_storage(&query, block_hash).await
    }

    // query_total_space
    async fn query_total_space(&self, block_hash: Option<H256>) -> Result<Option<u128>> {
        let query = storage_handler_storage().total_service_space();

        query_storage(&query, block_hash).await
    }

    // query_purchased_space
    async fn query_purchased_space(&self, block_hash: Option<H256>) -> Result<Option<u128>> {
        let query = storage_handler_storage().purchased_space();

        query_storage(&query, block_hash).await
    }

    /* Transactional functions */

    async fn buy_space(&self, gib_count: u32) -> Result<(String, BuySpace)> {
        let tx = storage_handler_tx().buy_space(gib_count);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(space) = events.find_first::<BuySpace>()? {
            Ok((tx_hash, space))
        } else {
            bail!("Unable to buy space");
        }
    }

    async fn expansion_space(&self, gib_count: u32) -> Result<(String, ExpansionSpace)> {
        let tx = storage_handler_tx().expansion_space(gib_count);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(space) = events.find_first::<ExpansionSpace>()? {
            Ok((tx_hash, space))
        } else {
            bail!("Unable to expand space");
        }
    }

    async fn renewal_space(&self, days: u32) -> Result<(String, RenewalSpace)> {
        let tx = storage_handler_tx().renewal_space(days);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(space) = events.find_first::<RenewalSpace>()? {
            Ok((tx_hash, space))
        } else {
            bail!("Unable to renew space");
        }
    }

    async fn update_price(&self) -> Result<String> {
        let tx = storage_handler_tx().update_price();
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::StorageHandler;
    use crate::chain::ChainSdk;

    const MNEMONIC: &str =
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

    fn init_chain() -> ChainSdk {
        ChainSdk::new(MNEMONIC, "service_name")
    }

    #[tokio::test]
    async fn test_buy_space() {
        let sdk = init_chain();
        let result = sdk.buy_space(1).await;
        if let Err(e) = result {
            println!("Error: {:?}", e);
            assert!(false);
        } else {
            assert!(true);
        }
    }
}