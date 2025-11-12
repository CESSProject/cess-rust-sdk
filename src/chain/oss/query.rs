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

use crate::chain::oss::types::{Oss, OssData};
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

    pub async fn oss_list(block_hash: Option<H256>) -> Result<Option<Vec<Oss>>, Error> {
        let api = Self::get_api();
        let query = api.oss_iter();

        let mut results = Vec::new();
        let mut stream = Self::execute_iter(query, block_hash).await?;

        while let Some(result) = stream.next().await {
            let key_value = result?;
            let mut acc_bytes = [0u8; 32];

            let raw_key = &key_value.key_bytes;

            if raw_key.len() < 32 {
                return Err(Error::Custom("storage key too short".into()));
            }
            acc_bytes.copy_from_slice(&raw_key[raw_key.len() - 32..]);
            let account = AccountId32(acc_bytes);

            let oss_data = Oss {
                account: account.to_string(),
                data: OssData {
                    domain: String::from_utf8(key_value.value.domain.0)
                        .map_err(|_| Error::Custom("failed to decode domain string".into()))?,
                    peer_id: String::from_utf8(key_value.value.peer_id.to_vec())
                        .map_err(|_| Error::Custom("failed to decode domain string".into()))?,
                },
            };
            results.push(oss_data);
        }

        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(results))
        }
    }
}
