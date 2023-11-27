use super::ChainSdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use async_trait::async_trait;
use polkadot::{
    audit::{
        calls::TransactionApi,
        events::{SubmitIdleProof, SubmitServiceProof},
        storage::StorageApi,
    },
    runtime_types::{
        bounded_collections::{bounded_vec::BoundedVec, weak_bounded_vec::WeakBoundedVec},
        cp_bloom_filter::BloomFilter,
        pallet_audit::{sr25519::app_sr25519::Public, types::ChallengeInfo},
    },
};
use subxt::ext::sp_core::H256;
use subxt::tx::PairSigner;

fn audit_storage() -> StorageApi {
    polkadot::storage().audit()
}

fn audit_tx() -> TransactionApi {
    polkadot::tx().audit()
}

#[async_trait]
pub trait Audit {
    async fn query_verify_duration(&self, block_hash: Option<H256>) -> Result<Option<u32>>;
    async fn query_cur_authority_index(&self, block_hash: Option<H256>) -> Result<Option<u16>>;
    async fn query_keys(&self, block_hash: Option<H256>) -> Result<Option<WeakBoundedVec<Public>>>;
    async fn query_challenge_snapshot(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<ChallengeInfo>>;
    async fn query_counted_idle_failed(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<u32>>;
    async fn query_counted_service_failed(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<u32>>;
    async fn query_counted_clear(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<u8>>;
    async fn query_challenge_era(&self, block_hash: Option<H256>) -> Result<Option<u32>>;
    async fn query_verify_result(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<(Option<bool>, Option<bool>)>>;
    async fn query_verify_reassign_count(&self, block_hash: Option<H256>) -> Result<Option<u8>>;
    async fn submit_idle_proof(
        &self,
        idle_prove: BoundedVec<u8>,
    ) -> Result<(String, SubmitIdleProof)>;
    async fn submit_service_proof(
        &self,
        service_prove: BoundedVec<u8>,
    ) -> Result<(String, SubmitServiceProof)>;
    async fn submit_verify_idle_result(
        &self,
        total_prove_hash: BoundedVec<u8>,
        front: u64,
        rear: u64,
        accumulator: &[u8; 256],
        idle_result: bool,
        signature: &[u8; 256],
        tee_acc: &[u8],
    ) -> Result<String>;
    async fn submit_verify_service_result(
        &self,
        service_result: bool,
        signature: &[u8; 256],
        service_bloom_filter: BloomFilter,
        miner: &[u8],
    ) -> Result<String>;
}

#[async_trait]
impl Audit for ChainSdk {
    /* Query functions */
    // query_verify_duration
    async fn query_verify_duration(&self, block_hash: Option<H256>) -> Result<Option<u32>> {
        let query = audit_storage().verify_duration();

        query_storage(&query, block_hash).await
    }

    // query_cur_authority_index
    async fn query_cur_authority_index(&self, block_hash: Option<H256>) -> Result<Option<u16>> {
        let query = audit_storage().cur_authority_index();

        query_storage(&query, block_hash).await
    }

    // query_keys
    async fn query_keys(&self, block_hash: Option<H256>) -> Result<Option<WeakBoundedVec<Public>>> {
        let query = audit_storage().keys();

        query_storage(&query, block_hash).await
    }

    // query_challenge_snapshot
    async fn query_challenge_snapshot(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<ChallengeInfo>> {
        let account = account_from_slice(pk);

        let query = audit_storage().challenge_snap_shot(&account);

        query_storage(&query, block_hash).await
    }

    // query_counted_idle_failed
    async fn query_counted_idle_failed(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<u32>> {
        let account = account_from_slice(pk);

        let query = audit_storage().counted_idle_failed(&account);

        query_storage(&query, block_hash).await
    }

    // query_counted_service_failed
    async fn query_counted_service_failed(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<u32>> {
        let account = account_from_slice(pk);

        let query = audit_storage().counted_service_failed(&account);

        query_storage(&query, block_hash).await
    }

    // query_counted_clear
    async fn query_counted_clear(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<u8>> {
        let account = account_from_slice(pk);

        let query = audit_storage().counted_clear(&account);

        query_storage(&query, block_hash).await
    }

    // query_challenge_era
    async fn query_challenge_era(&self, block_hash: Option<H256>) -> Result<Option<u32>> {
        let query = audit_storage().challenge_era();

        query_storage(&query, block_hash).await
    }

    // query_verify_result
    async fn query_verify_result(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<(Option<bool>, Option<bool>)>> {
        let account = account_from_slice(pk);

        let query = audit_storage().verify_result(&account);

        query_storage(&query, block_hash).await
    }

    // query_verify_reassign_count
    async fn query_verify_reassign_count(&self, block_hash: Option<H256>) -> Result<Option<u8>> {
        let query = audit_storage().verify_reassign_count();

        query_storage(&query, block_hash).await
    }

    /* Transactional functions */

    // submit_idle_proof
    async fn submit_idle_proof(
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
    async fn submit_service_proof(
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
    async fn submit_verify_idle_result(
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
    async fn submit_verify_service_result(
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

#[cfg(test)]
mod test {
    use super::Audit;
    use crate::{chain::ChainSdk, utils::block_hex_string_to_h256};

    const MNEMONIC: &str =
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

    // Testnet: Block number: 731,214
    const BLOCK_HASH: &str = "0x41bf39c7e335a5e258a0d55c8e7a95958294f617f96b02a154fcf39a0f40e7f8";

    fn init_chain() -> ChainSdk {
        ChainSdk::new(MNEMONIC, "service_name")
    }

    #[tokio::test]
    async fn test_query_verify_duration() {
        let sdk = init_chain();
        let result = sdk.query_verify_duration(None).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }

        let hash = block_hex_string_to_h256(BLOCK_HASH);
        let result = sdk.query_verify_duration(Some(hash)).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_query_cur_authority_index() {
        let sdk = init_chain();
        let result = sdk.query_cur_authority_index(None).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }

        let hash = block_hex_string_to_h256(BLOCK_HASH);
        let result = sdk.query_cur_authority_index(Some(hash)).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_query_keys() {
        let sdk = init_chain();
        let result = sdk.query_keys(None).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }

        let hash = block_hex_string_to_h256(BLOCK_HASH);
        let result = sdk.query_keys(Some(hash)).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
