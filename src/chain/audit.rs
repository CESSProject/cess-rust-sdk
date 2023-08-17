use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use polkadot::{
    audit::{
        calls::TransactionApi,
        events::{SubmitIdleProof, SubmitServiceProof},
        storage::StorageApi,
    },
    runtime_types::{
        cp_bloom_filter::BloomFilter,
        cp_cess_common::{Hash as CPHash, SpaceProofInfo},
        pallet_audit::{
            sr25519::app_sr25519::{Public, Signature},
            types::{ChallengeInfo, IdleProveInfo, SegDigest, ServiceProveInfo},
        },
        sp_core::bounded::{bounded_vec::BoundedVec, weak_bounded_vec::WeakBoundedVec},
    },
};
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn audit_storage() -> StorageApi {
    polkadot::storage().audit()
}

fn audit_tx() -> TransactionApi {
    polkadot::tx().audit()
}

impl Sdk {
    /* Query functions */
    // query_challenge_duration
    pub async fn query_challenge_duration(&self) -> Result<u32> {
        let query = audit_storage().challenge_duration();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_verify_duration
    pub async fn query_verify_duration(&self) -> Result<u32> {
        let query = audit_storage().verify_duration();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_cur_authority_index
    pub async fn query_cur_authority_index(&self) -> Result<u16> {
        let query = audit_storage().cur_authority_index();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_keys
    pub async fn query_keys(&self) -> Result<WeakBoundedVec<Public>> {
        let query = audit_storage().keys();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_challenge_proposal
    pub async fn query_challenge_proposal(&self, slice: &[u8; 32]) -> Result<(u32, ChallengeInfo)> {
        let query = audit_storage().challenge_proposal(slice);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_challenge_snapshot
    pub async fn query_challenge_snapshot(&self) -> Result<ChallengeInfo> {
        let query = audit_storage().challenge_snap_shot();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_counted_idle_failed
    pub async fn query_counted_idle_failed(&self, pk: &[u8]) -> Result<u32> {
        let account = account_from_slice(pk);

        let query = audit_storage().counted_idle_failed(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_counted_service_failed
    pub async fn query_counted_service_failed(&self, pk: &[u8]) -> Result<u32> {
        let account = account_from_slice(pk);

        let query = audit_storage().counted_service_failed(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_counted_clear
    pub async fn query_counted_clear(&self, pk: &[u8]) -> Result<u8> {
        let account = account_from_slice(pk);

        let query = audit_storage().counted_clear(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_challenge_era
    pub async fn query_challenge_era(&self) -> Result<u32> {
        let query = audit_storage().challenge_era();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_unverify_idle_proof
    pub async fn query_unverify_idle_proof(&self, pk: &[u8]) -> Result<BoundedVec<IdleProveInfo>> {
        let account = account_from_slice(pk);

        let query = audit_storage().unverify_idle_proof(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_unverify_service_proof
    pub async fn query_unverify_service_proof(
        &self,
        pk: &[u8],
    ) -> Result<BoundedVec<ServiceProveInfo>> {
        let account = account_from_slice(pk);

        let query = audit_storage().unverify_service_proof(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_verify_result
    pub async fn query_verify_result(&self, pk: &[u8]) -> Result<(Option<bool>, Option<bool>)> {
        let account = account_from_slice(pk);

        let query = audit_storage().verify_result(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_lock
    pub async fn query_lock(&self) -> Result<bool> {
        let query = audit_storage().lock();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_verify_reassign_count
    pub async fn query_verify_reassign_count(&self) -> Result<u8> {
        let query = audit_storage().verify_reassign_count();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /* Transactional functions */

    // save_challenge_info
    pub async fn save_challenge_info(
        &self,
        challenge_info: ChallengeInfo,
        key: Public,
        seg_digest: SegDigest<u32>,
        signature: Signature,
    ) -> Result<String> {
        let tx = audit_tx().save_challenge_info(challenge_info, key, seg_digest, signature);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // submit_idle_proof
    pub async fn submit_idle_proof(
        &self,
        idle_prove: BoundedVec<u8>,
    ) -> Result<(String, SubmitIdleProof)> {
        let tx = audit_tx().submit_idle_proof(idle_prove);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(idle_proof) = events.find_first::<SubmitIdleProof>()? {
            Ok((tx_hash, idle_proof))
        } else {
            bail!("Unable to submit idle proof");
        }
    }

    // submit_service_proof
    pub async fn submit_service_proof(
        &self,
        service_prove: BoundedVec<u8>,
    ) -> Result<(String, SubmitServiceProof)> {
        let tx = audit_tx().submit_service_proof(service_prove);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(service_proof) = events.find_first::<SubmitServiceProof>()? {
            Ok((tx_hash, service_proof))
        } else {
            bail!("Unable to submit service proof");
        }
    }

    // submit_verify_idle_result
    pub async fn submit_verify_idle_result(
        &self,
        total_prove_hash: BoundedVec<u8>,
        front: u64,
        rear: u64,
        accumulator: &[u8; 256],
        idle_result: bool,
        signature: &[u8; 256],
        tee_acc: &[u8],
    ) -> Result<String> {
        let account = account_from_slice(tee_acc);
        let tx = audit_tx().submit_verify_idle_result(
            total_prove_hash,
            front,
            rear,
            *accumulator,
            idle_result,
            *signature,
            account,
        );

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // submit_verify_service_result
    pub async fn submit_verify_service_result(
        &self,
        service_result: bool,
        signature: &[u8; 256],
        service_bloom_filter: BloomFilter,
        miner: &[u8],
    ) -> Result<String> {
        let account = account_from_slice(miner);
        let tx = audit_tx().submit_verify_service_result(
            service_result,
            *signature,
            service_bloom_filter,
            account,
        );

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
}
