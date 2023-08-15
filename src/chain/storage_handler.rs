use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use polkadot::{
    runtime_types::{
        pallet_storage_handler::types::OwnedSpaceDetails, sp_core::bounded::bounded_vec::BoundedVec,
    },
    storage_handler::{
        calls::TransactionApi,
        events::{BuySpace, ExpansionSpace, RenewalSpace},
        storage::StorageApi,
    },
};
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn storage_handler_storage() -> StorageApi {
    polkadot::storage().storage_handler()
}

fn storage_handler_tx() -> TransactionApi {
    polkadot::tx().storage_handler()
}

impl Sdk {
    /* Query functions */
    // query_user_owned_space
    pub async fn query_user_owned_space(&self, pk: &[u8]) -> Result<OwnedSpaceDetails> {
        let account = account_from_slice(pk);
        let query = storage_handler_storage().user_owned_space(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_unit_price
    pub async fn query_unit_price(&self) -> Result<u128> {
        let query = storage_handler_storage().unit_price();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_total_power
    pub async fn query_total_power(&self) -> Result<u128> {
        let query = storage_handler_storage().total_idle_space();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_total_space
    pub async fn query_total_space(&self) -> Result<u128> {
        let query = storage_handler_storage().total_service_space();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_purchased_space
    pub async fn query_purchased_space(&self) -> Result<u128> {
        let query = storage_handler_storage().purchased_space();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /* Transactional functions */

    pub async fn buy_space(&self, gib_count: u32) -> Result<(String, BuySpace)> {
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

    pub async fn expansion_space(&self, gib_count: u32) -> Result<(String, ExpansionSpace)> {
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

    pub async fn renewal_space(&self, days: u32) -> Result<(String, RenewalSpace)> {
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

    pub async fn update_price(&self) -> Result<String> {
        let tx = storage_handler_tx().update_price();
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
}
