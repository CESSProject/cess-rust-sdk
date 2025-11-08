//! # OSS Storage Query Module
//!
//! This module defines query functions for accessing on-chain data related to
//! the `pallet_oss` runtime pallet.  
//!
//! It provides read-only access to the storage of OSS authority lists and
//! individual OSS information records.  
//!
//! All queries are asynchronous and can optionally be executed at a specific
//! block hash for historical lookups.

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

// Implements the API provider for the `pallet_oss` storage queries.
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
    /// Fetches the list of authorities associated with a given OSS account.
    ///
    /// # Arguments
    /// * `account` - The account ID in string form (e.g. SS58 address).
    /// * `block_hash` - Optional block hash for querying a past state.
    ///
    /// # Returns
    /// * `Ok(Some(BoundedVec<AccountId32>))` if data exists.
    /// * `Ok(None)` if no authority list exists for the account.
    /// * `Err(Error)` if parsing or query execution fails.
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

    /// Fetches the `OssInfo` record associated with a specific account.
    ///
    /// # Arguments
    /// * `account` - The OSS account ID to look up.
    /// * `block_hash` - Optional block hash for historical queries.
    ///
    /// # Returns
    /// * `Ok(Some(OssInfo))` if an OSS record exists.
    /// * `Ok(None)` if the account has no associated OSS record.
    /// * `Err(Error)` if the account format or query fails.
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
