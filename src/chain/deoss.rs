use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use polkadot::{
    oss::{calls::TransactionApi, 
        events::{
        SubmitIdleProof,
        SubmitServiceProof,
    }
        , storage::StorageApi},
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
    pub async fn query_authority_list(&self, pk: &[u8]) -> Result<AccountId32>{
        let account = account_from_slice(pk);

        let query = oss_storage().authority_list(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn oss(&self, pk: &[u8]) -> Result<[u8; 38]> {
        let account = account_from_slice(pk);

        let query = oss_storage().oss(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    
}
