//! # File Bank Query Module
//!
//! This module defines query interfaces for accessing on-chain data
//! related to the `pallet_file_bank` runtime module.
//!
//! It allows fetching information such as file deals, file metadata,
//! user-held files, and restoration orders directly from blockchain storage.
//!
//! The module leverages the unified [`Chain`] and [`Query`] traits to
//! standardize access patterns across storage modules.

use crate::chain::{Chain, Query};
use crate::core::{ApiProvider, Error};
use crate::polkadot::{
    self,
    file_bank::storage::StorageApi,
    runtime_types::{
        bounded_collections::bounded_vec::BoundedVec,
        pallet_file_bank::types::{DealInfo, FileInfo, RestoralOrderInfo, UserFileSliceInfo},
    },
};
use crate::utils::hash_from_string;
use crate::{impl_api_provider, H256};
use std::str::FromStr;
use subxt::utils::AccountId32;

// Implements the API provider for the `pallet_file_bank` storage module.
impl_api_provider!(
    StorageApiProvider,
    StorageApi,
    polkadot::storage().file_bank()
);

/// Provides access to on-chain queries for the `pallet_file_bank` module.
pub struct StorageQuery;

impl Chain for StorageQuery {}

impl Query for StorageQuery {
    type Api = StorageApi;

    fn get_api() -> Self::Api {
        crate::core::get_api::<StorageApiProvider>()
    }
}

impl StorageQuery {
    /// Fetches the deal information associated with a specific file hash.
    ///
    /// # Arguments
    /// * `hash` - The hex string representing the file’s unique hash.
    /// * `block_hash` - Optional block hash to query at a specific block state.
    pub async fn deal_map(hash: &str, block_hash: Option<H256>) -> Result<Option<DealInfo>, Error> {
        let api = Self::get_api();
        let hash = hash_from_string(hash)?;
        let query = api.deal_map(hash);

        Self::execute_query(&query, block_hash).await
    }

    /// Retrieves file metadata (`FileInfo`) associated with a given file hash.
    pub async fn file(hash: &str, block_hash: Option<H256>) -> Result<Option<FileInfo>, Error> {
        let api = Self::get_api();
        let hash = hash_from_string(hash)?;
        let query = api.file(hash);

        Self::execute_query(&query, block_hash).await
    }

    /// Fetches the list of file slices currently held by a user.
    ///
    /// # Arguments
    /// * `account` - The user’s account address in SS58 format.
    /// * `block_hash` - Optional block hash to query historical state.
    pub async fn user_hold_file_list(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<UserFileSliceInfo>>, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let query = api.user_hold_file_list(account);

        Self::execute_query(&query, block_hash).await
    }

    /// Retrieves information about a file restoration order.
    pub async fn restoral_order(
        hash: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<RestoralOrderInfo>, Error> {
        let api = Self::get_api();
        let hash = hash_from_string(hash)?;
        let query = api.restoral_order(hash);

        Self::execute_query(&query, block_hash).await
    }

    /// Returns a list of users and their associated cleanup data.
    pub async fn clear_user_list(
        block_hash: Option<H256>,
    ) -> Result<Option<BoundedVec<(AccountId32, BoundedVec<u8>)>>, Error> {
        let api = Self::get_api();
        let query = api.clear_user_list();

        Self::execute_query(&query, block_hash).await
    }

    /// Returns the count of failed file bank tasks associated with a given user.
    pub async fn task_failed_count(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<u8>, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let query = api.task_failed_count(account);

        Self::execute_query(&query, block_hash).await
    }
}
