use crate::chain::{Chain, Query};
use crate::core::ApiProvider;
use crate::polkadot::{
    self, audit::storage::StorageApi, runtime_types::pallet_audit::types::ChallengeInfo,
};
use crate::{impl_api_provider, H256};
use std::str::FromStr;
use subxt::utils::AccountId32;

// impl ApiProvider for StorageApiProvider
impl_api_provider!(StorageApiProvider, StorageApi, polkadot::storage().audit());

pub struct StorageQuery;

impl Chain for StorageQuery{}

impl Query for StorageQuery {
    type Api = StorageApi;

    fn get_api() -> Self::Api {
        crate::core::get_api::<StorageApiProvider>()
    }
}

impl StorageQuery {
    pub async fn counted_service_failed(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<u32>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let query = api.counted_service_failed(account);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn counted_clear(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<u8>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let query = api.counted_clear(account);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn challenge_snapshot(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<ChallengeInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let query = api.challenge_snap_shot(account);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn challenge_slip(
        block_number: u32,
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<bool>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let query = api.challenge_slip(block_number, account);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn verify_slip(
        block_number: u32,
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<bool>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let query = api.verify_slip(block_number, account);

        Self::execute_query(&query, block_hash).await
    }
}
