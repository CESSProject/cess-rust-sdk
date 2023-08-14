use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use polkadot::{
    runtime_types::{
        pallet_storage_handler::types::{OwnedSpaceDetails},
        sp_core::bounded::bounded_vec::BoundedVec,
    },
    storage_handler::{
        calls::TransactionApi, 
        // events::IncreaseCollateral, 
        storage::StorageApi
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
}