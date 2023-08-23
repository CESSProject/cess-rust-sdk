use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use polkadot::{
    runtime_types::{
        pallet_tee_worker::types::{SgxAttestationReport, TeeWorkerInfo},
        sp_core::{bounded::bounded_vec::BoundedVec, ed25519::Public as ed25519Public},
    },
    tee_worker::{calls::TransactionApi, events::Exit, storage::StorageApi},
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
    pub async fn query_tee_worker_map(&self, pk: &[u8]) -> Result<Option<TeeWorkerInfo>> {
        let account = account_from_slice(pk);
        let query = tee_worker_storage().tee_worker_map(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_bond_acc
    pub async fn query_bond_acc(&self) -> Result<Option<BoundedVec<AccountId32>>> {
        let query = tee_worker_storage().bond_acc();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_tee_podr2_pk
    pub async fn query_tee_podr2_pk(&self) -> Result<Option<[u8; 270]>> {
        let query = tee_worker_storage().tee_podr2_pk();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_mr_enclave_whitelist
    pub async fn query_mr_enclave_whitelist(&self) -> Result<Option<BoundedVec<[u8; 64]>>> {
        let query = tee_worker_storage().mr_enclave_whitelist();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
    /* Transactional functions */

    // register
    pub async fn register_tee_worker(
        &self,
        stash_account: &[u8],
        node_key: ed25519Public,
        peer_id: [u8; 38],
        podr2_pbk: [u8; 270],
        sgx_attestation_report: SgxAttestationReport,
    ) -> Result<String> {
        let account = account_from_slice(stash_account);
        let tx = tee_worker_tx().register(
            account,
            node_key,
            peer_id,
            podr2_pbk,
            sgx_attestation_report,
        );
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // update_whitelist
    pub async fn update_whitelist(&self, mr_enclave: [u8; 64]) -> Result<String> {
        let tx = tee_worker_tx().update_whitelist(mr_enclave);
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
    pub async fn exit(&self) -> Result<(String, Exit)> {
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
