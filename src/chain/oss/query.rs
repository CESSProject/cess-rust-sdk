use std::str::FromStr;

use crate::chain::{Chain, Query};
use crate::core::{ApiProvider, Error};
use crate::polkadot::{
    self,
    oss::storage::StorageApi,
    runtime_types::{bounded_collections::bounded_vec::BoundedVec, pallet_oss::types::OssInfo},
};
use crate::{impl_api_provider, H256};
use subxt::utils::AccountId32;

// impl ApiProvider for StorageApiProvider
impl_api_provider!(StorageApiProvider, StorageApi, polkadot::storage().oss());

pub struct StorageQuery;

impl Chain for StorageQuery {}

impl Query for StorageQuery {
    type Api = StorageApi;

    fn get_api() -> Self::Api {
        crate::core::get_api::<StorageApiProvider>()
    }
}

impl StorageQuery {
    pub async fn authority_list(
        &self,
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<AccountId32>>, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let query = api.authority_list(account);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn oss(
        &self,
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<OssInfo>, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let query = api.oss(account);

        Self::execute_query(&query, block_hash).await
    }
}
