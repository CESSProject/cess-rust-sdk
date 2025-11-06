use crate::chain::{Chain, Query};
use crate::core::{ApiProvider, Error};
use crate::polkadot::{
    self, audit::storage::StorageApi, runtime_types::pallet_audit::types::ChallengeInfo,
};
use crate::{impl_api_provider, H256};
use std::str::FromStr;
use subxt::utils::AccountId32;

// Implements the API provider for accessing the `pallet_audit` storage module
// through the Substrate-based Polkadot runtime.
impl_api_provider!(StorageApiProvider, StorageApi, polkadot::storage().audit());

/// Query interface for the `pallet_audit` module.
/// 
/// This struct provides asynchronous methods to retrieve audit-related on-chain
/// storage data, such as challenge snapshots and failure counts.
pub struct StorageQuery;

impl Chain for StorageQuery {}

impl Query for StorageQuery {
    type Api = StorageApi;

    fn get_api() -> Self::Api {
        crate::core::get_api::<StorageApiProvider>()
    }
}

impl StorageQuery {
    /// Queries how many times a given account’s service has failed.
    ///
    /// # Arguments
    /// * `account` - The SS58-encoded account string.
    /// * `block_hash` - Optional block hash for querying a specific block state.
    ///
    /// # Returns
    /// * `Ok(Some(u32))` if data is found.
    /// * `Ok(None)` if no data exists.
    /// * `Err(Error)` on failure.
    pub async fn counted_service_failed(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<u32>, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let query = api.counted_service_failed(account);

        Self::execute_query(&query, block_hash).await
    }

    /// Retrieves the number of cleared challenge attempts for a given account.
    pub async fn counted_clear(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<u8>, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let query = api.counted_clear(account);

        Self::execute_query(&query, block_hash).await
    }

    /// Retrieves the current challenge snapshot for the specified account.
    ///
    /// The returned [`ChallengeInfo`] structure contains metadata about
    /// the ongoing or latest challenge for that account.
    pub async fn challenge_snapshot(
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<ChallengeInfo>, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let query = api.challenge_snap_shot(account);

        Self::execute_query(&query, block_hash).await
    }

    /// Checks if a challenge slip exists for the given account at a specific block.
    pub async fn challenge_slip(
        block_number: u32,
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<bool>, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let query = api.challenge_slip(block_number, account);

        Self::execute_query(&query, block_hash).await
    }

    /// Verifies whether a given challenge slip has been validated for a block and account.
    pub async fn verify_slip(
        block_number: u32,
        account: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<bool>, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let query = api.verify_slip(block_number, account);

        Self::execute_query(&query, block_hash).await
    }
}
