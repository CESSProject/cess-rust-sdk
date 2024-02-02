use super::ChainSdk;
use crate::polkadot;
use async_trait::async_trait;
use polkadot::tee_worker::{calls::TransactionApi, storage::StorageApi};

fn tee_worker_storage() -> StorageApi {
    polkadot::storage().tee_worker()
}

fn tee_worker_tx() -> TransactionApi {
    polkadot::tx().tee_worker()
}

#[async_trait]
pub trait TeeWorker {
}

#[async_trait]
impl TeeWorker for ChainSdk {
    /* Query functions */
}
