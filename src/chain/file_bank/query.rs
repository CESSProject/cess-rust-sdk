use crate::chain::Query;
use crate::core::ApiProvider;
use crate::polkadot::{
    self,
    file_bank::storage::StorageApi,
    runtime_types::{
        bounded_collections::bounded_vec::BoundedVec,
        pallet_file_bank::types::{
            BucketInfo, DealInfo, FileInfo, RestoralOrderInfo, UserFileSliceInfo,
        },
    },
};
use crate::utils::hash_from_string;
use crate::{impl_api_provider, H256};
use std::str::FromStr;
use subxt::utils::AccountId32;

// impl ApiProvider for StorageApiProvider
impl_api_provider!(
    StorageApiProvider,
    StorageApi,
    polkadot::storage().file_bank()
);

pub struct StorageQuery;

impl Query for StorageQuery {
    type Api = StorageApi;

    fn get_api() -> Self::Api {
        crate::core::get_api::<StorageApiProvider>()
    }
}

impl StorageQuery {
    pub async fn deal_map(
        hash: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<DealInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let hash = hash_from_string(hash)?;
        let query = api.deal_map(hash);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn file(
        hash: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<FileInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let hash = hash_from_string(hash)?;
        let query = api.file(hash);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn user_hold_file_list(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<UserFileSliceInfo>>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let query = api.user_hold_file_list(account);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn bucket(
        account: &str,
        bucket_name: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<BucketInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let bucket_name = bucket_name.as_bytes().to_vec();
        let query = api.bucket(account, BoundedVec(bucket_name));

        Self::execute_query(&query, block_hash).await
    }

    pub async fn user_bucket_list(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<BoundedVec<u8>>>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let query = api.user_bucket_list(account);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn restoral_order(
        hash: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<RestoralOrderInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let hash = hash_from_string(hash)?;
        let query = api.restoral_order(hash);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn clear_user_list(
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<(AccountId32, BoundedVec<u8>)>>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.clear_user_list();

        Self::execute_query(&query, block_hash).await
    }

    pub async fn task_failed_count(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<u8>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let query = api.task_failed_count(account);

        Self::execute_query(&query, block_hash).await
    }
}
