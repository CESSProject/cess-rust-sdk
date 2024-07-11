use crate::chain::Query;
use crate::core::ApiProvider;
use crate::polkadot::{
    self,
    runtime_types::bounded_collections::bounded_vec::BoundedVec,
    runtime_types::pallet_storage_handler::types::{ConsignmentInfo, OrderInfo, TerritoryInfo},
    storage_handler::storage::StorageApi,
};
use crate::utils::get_ss58_address;
use crate::{impl_api_provider, H256};
use std::str::FromStr;
use subxt::utils::AccountId32;

// impl ApiProvider for StorageApiProvider
impl_api_provider!(
    StorageApiProvider,
    StorageApi,
    polkadot::storage().storage_handler()
);

pub struct StorageQuery;

impl Query for StorageQuery {
    type Api = StorageApi;

    fn get_api() -> Self::Api {
        crate::core::get_api::<StorageApiProvider>()
    }
}

impl StorageQuery {
    pub async fn territory_key(
        token: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let query = api.territory_key(token);

        match Self::execute_query(&query, block_hash).await? {
            Some(value) => {
                let account = get_ss58_address(&value.0.to_string())?;
                let territory: String = String::from_utf8(value.1 .0).unwrap();
                Ok(Some((account, territory)))
            }
            None => Ok(None),
        }
    }

    pub async fn territory(
        account: &str,
        territory_name: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<TerritoryInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let territory_name = territory_name.as_bytes().to_vec();
        let query = api.territory(account, BoundedVec(territory_name));

        Self::execute_query(&query, block_hash).await
    }

    pub async fn consignment(
        token: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<ConsignmentInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let query = api.consignment(token);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn territory_frozen(
        block_number: u32,
        token: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<bool>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let query = api.territory_frozen(block_number, token);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn territory_frozen_counter(
        block_number: u32,
        block_hash: Option<H256>,
    ) -> Result<Option<u32>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.territory_frozen_counter(block_number);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn territory_expired(
        block_number: u32,
        token: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<bool>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let query = api.territory_expired(block_number, token);

        Self::execute_query(&query, block_hash).await
    }

    pub async fn unit_price(
        block_hash: Option<H256>,
    ) -> Result<Option<u128>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.unit_price();

        Self::execute_query(&query, block_hash).await
    }

    pub async fn total_power(
        block_hash: Option<H256>,
    ) -> Result<Option<u128>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.total_idle_space();

        Self::execute_query(&query, block_hash).await
    }

    pub async fn total_space(
        block_hash: Option<H256>,
    ) -> Result<Option<u128>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.total_service_space();

        Self::execute_query(&query, block_hash).await
    }

    pub async fn purchased_space(
        block_hash: Option<H256>,
    ) -> Result<Option<u128>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.purchased_space();

        Self::execute_query(&query, block_hash).await
    }

    pub async fn pay_order(
        order_hash: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<OrderInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let order_hash = order_hash.as_bytes().to_vec();
        let query = api.pay_order(BoundedVec(order_hash));

        Self::execute_query(&query, block_hash).await
    }
}
