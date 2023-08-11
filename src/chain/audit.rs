use super::Sdk;
use crate::core::utils::account;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use log::info;

use polkadot::{
    audit::storage::StorageApi,
    runtime_types::{
        cp_cess_common::{Hash as CPHash, SpaceProofInfo, ServiceProveInfo},
        pallet_audit::{
            sr25519::app_sr25519::Public,
            types::{ChallengeInfo, IdleProveInfo},
        },
        sp_core::bounded::{
            bounded_vec::BoundedVec,
            weak_bounded_vec::WeakBoundedVec,

        },
    },
};
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn storage_audit() -> StorageApi {
    polkadot::storage().audit()
}

impl Sdk {
    /* Query functions */
    pub async fn query_challenge_duration(&self) -> Result<u32> {
        let query = storage_audit().challenge_duration();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_verify_duration(&self) -> Result<u32> {
        let query = storage_audit().verify_duration();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_cur_authority_index(&self) -> Result<u16> {
        let query = storage_audit().cur_authority_index();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_keys(&self) -> Result<WeakBoundedVec<Public>> {
        let query = storage_audit().keys();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_challenge_proposal(&self, slice: &[u8; 32]) -> Result<(u32, ChallengeInfo)> {
        let query = storage_audit().challenge_proposal(slice);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_challenge_snap_shot(&self) -> Result<ChallengeInfo> {
        let query = storage_audit().challenge_snap_shot();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_counted_idle_failed(&self, pk: &[u8]) -> Result<u32> {
        let account = account_from_slice(pk);

        let query = storage_audit().counted_idle_failed(&account);
        
        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_counted_service_failed(&self, pk: &[u8]) -> Result<u32> {
        let account = account_from_slice(pk);

        let query = storage_audit().counted_service_failed(&account);
        
        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_counted_clear(&self, pk: &[u8]) -> Result<u8> {
        let account = account_from_slice(pk);

        let query = storage_audit().counted_clear(&account);
        
        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_challenge_era(&self) -> Result<u32> {
        let query = storage_audit().challenge_era();
        
        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
    
    pub async fn query_unverify_idle_proof(&self, pk: &[u8]) -> Result<BoundedVec<IdleProveInfo>> {
        let account = account_from_slice(pk);

        let query = storage_audit().unverify_idle_proof(&account);
        
        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_unverify_service_proof(&self, pk: &[u8]) -> Result<BoundedVec<ServiceProveInfo>>{
        let account = account_from_slice(pk);

        let query = storage_audit().unverify_service_proof(&account);
        
        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
    
    pub async fn query_verify_result(&self, pk: &[u8]) -> Result<(Option<bool>, Option<bool>)>{
        let account = account_from_slice(pk);

        let query = storage_audit().verify_result(&account);
        
        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_lock(&self) -> Result<bool> {
        let query = storage_audit().lock();
        
        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
    
    pub async fn query_verify_reassign_count(&self) -> Result<u8> {
        let query = storage_audit().verify_reassign_count();
        
        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /* Transactional functions */

    
}