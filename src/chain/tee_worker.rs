use super::ChainSdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use async_trait::async_trait;
use polkadot::{
    runtime_types::{
        bounded_collections::bounded_vec::BoundedVec,
        pallet_tee_worker::types::{SgxAttestationReport, TeeWorkerInfo},
    },
    tee_worker::{calls::TransactionApi, events::Exit, storage::StorageApi},
};
use subxt::ext::sp_core::H256;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn tee_worker_storage() -> StorageApi {
    polkadot::storage().tee_worker()
}

fn tee_worker_tx() -> TransactionApi {
    polkadot::tx().tee_worker()
}

#[async_trait]
pub trait TeeWorker {
    async fn query_tee_worker_map(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<TeeWorkerInfo>>;
    async fn query_bond_acc(
        &self,
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<AccountId32>>>;
    async fn query_tee_podr2_pk(&self, block_hash: Option<H256>) -> Result<Option<[u8; 270]>>;
    async fn query_mr_enclave_whitelist(
        &self,
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<[u8; 64]>>>;
    async fn register_tee_worker(
        &self,
        stash_account: &[u8],
        peer_id: [u8; 38],
        podr2_pbk: [u8; 270],
        sgx_attestation_report: SgxAttestationReport,
        end_point: BoundedVec<u8>,
    ) -> Result<String>;
    async fn update_whitelist(&self, mr_enclave: [u8; 64]) -> Result<String>;
    async fn exit(&self) -> Result<(String, Exit)>;
}

#[async_trait]
impl TeeWorker for ChainSdk {
    /* Query functions */

    // query_tee_worker_map
    async fn query_tee_worker_map(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<TeeWorkerInfo>> {
        let account = account_from_slice(pk);
        let query = tee_worker_storage().tee_worker_map(&account);

        query_storage(&query, block_hash).await
    }

    // query_bond_acc
    async fn query_bond_acc(
        &self,
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<AccountId32>>> {
        let query = tee_worker_storage().bond_acc();

        query_storage(&query, block_hash).await
    }

    // query_tee_podr2_pk
    async fn query_tee_podr2_pk(&self, block_hash: Option<H256>) -> Result<Option<[u8; 270]>> {
        let query = tee_worker_storage().tee_podr2_pk();

        query_storage(&query, block_hash).await
    }

    // query_mr_enclave_whitelist
    async fn query_mr_enclave_whitelist(
        &self,
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<[u8; 64]>>> {
        let query = tee_worker_storage().mr_enclave_whitelist();

        query_storage(&query, block_hash).await
    }
    /* Transactional functions */

    // register
    async fn register_tee_worker(
        &self,
        stash_account: &[u8],
        peer_id: [u8; 38],
        podr2_pbk: [u8; 270],
        sgx_attestation_report: SgxAttestationReport,
        end_point: BoundedVec<u8>,
    ) -> Result<String> {
        let account = account_from_slice(stash_account);
        let tx = tee_worker_tx().register(
            account,
            peer_id,
            podr2_pbk,
            sgx_attestation_report,
            end_point,
        );
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // update_whitelist
    async fn update_whitelist(&self, mr_enclave: [u8; 64]) -> Result<String> {
        let tx = tee_worker_tx().update_whitelist(mr_enclave);
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
    async fn exit(&self) -> Result<(String, Exit)> {
        let tx = tee_worker_tx().exit();
        let from = PairSigner::new(self.pair.clone());
        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(exit) = events.find_first::<Exit>()? {
            Ok((tx_hash, exit))
        } else {
            bail!("Unable to exit");
        }
    }
}
