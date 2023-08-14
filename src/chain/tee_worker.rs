use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use polkadot::{
    runtime_types::{
        pallet_tee_worker::types::{TeeWorkerInfo},
        sp_core::bounded::bounded_vec::BoundedVec,
    },
    tee_worker::{
        calls::TransactionApi, 
        // events::{BuySpace, ExpansionSpace, RenewalSpace}, 
        storage::StorageApi
    },
};
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn tee_worker_storage() -> StorageApi {
    polkadot::storage().tee_worker()
}

fn tee_worker_tx() -> TransactionApi {
    polkadot::tx().tee_worker()
}


impl Sdk {
    /* Query functions */

    // query_tee_worker_map
    pub async fn query_tee_worker_map(&self, pk: &[u8]) -> Result<TeeWorkerInfo> {
        let account = account_from_slice(pk);
        let query = tee_worker_storage().tee_worker_map(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_bond_acc
    pub async fn query_bond_acc(&self) -> Result<BoundedVec<AccountId32>> {
        let query = tee_worker_storage().bond_acc();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_tee_podr2_pk
    pub async fn query_tee_podr2_pk(&self) -> Result<[u8; 270]> {
        let query = tee_worker_storage().tee_podr2_pk();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_mr_enclave_whitelist
    pub async fn query_mr_enclave_whitelist(&self) -> Result<[u8; 64]> {
        let query = tee_worker_storage().mr_enclave_whitelist();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
    /* Transactional functions */

}