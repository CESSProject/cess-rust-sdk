use std::str::FromStr;

use crate::core::ApiProvider;
use crate::polkadot::storage_handler::calls::types::territory_rename;
use crate::polkadot::{
    self,
    runtime_types::bounded_collections::bounded_vec::BoundedVec,
    runtime_types::pallet_storage_handler::types::{ConsignmentInfo, OrderInfo, TerritoryInfo},
    storage_handler::{calls::TransactionApi, storage::StorageApi},
};

use crate::utils::account_from_slice;
use crate::utils::get_ss58_address;
use crate::{impl_api_provider, query_storage, H256};

use subxt::ext::codec::Encode;
use subxt::utils::AccountId32;

// impl ApiProvider for StorageApiProvider
impl_api_provider!(
    StorageApiProvider,
    StorageApi,
    polkadot::storage().storage_handler()
);

// impl ApiProvider for TransactionApiProvider
impl_api_provider!(
    TransactionApiProvider,
    TransactionApi,
    polkadot::tx().storage_handler()
);

pub struct StorageQuery;

impl StorageQuery {
    fn get_api() -> StorageApi {
        crate::core::get_api::<StorageApiProvider>()
    }

    pub async fn territory_key(
        token: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let query = api.territory_key(token);

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => {
                    let account = get_ss58_address(&value.0.to_string())?;
                    let territory: String = String::from_utf8(value.1 .0).unwrap();
                    Ok(Some((account, territory)))
                }
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
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

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    pub async fn consignment(
        token: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<ConsignmentInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let query = api.consignment(token);

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    pub async fn territory_frozen(
        block_number: u32,
        token: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<bool>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let query = api.territory_frozen(block_number, token);

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    pub async fn territory_frozen_counter(
        block_number: u32,
        block_hash: Option<H256>,
    ) -> Result<Option<u32>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.territory_frozen_counter(block_number);

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    pub async fn territory_expired(
        block_number: u32,
        token: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<bool>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let query = api.territory_expired(block_number, token);

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    pub async fn unit_price(
        block_hash: Option<H256>,
    ) -> Result<Option<u128>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.unit_price();

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    pub async fn total_power(
        block_hash: Option<H256>,
    ) -> Result<Option<u128>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.total_idle_space();

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    pub async fn total_space(
        block_hash: Option<H256>,
    ) -> Result<Option<u128>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.total_service_space();

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    pub async fn purchased_space(
        block_hash: Option<H256>,
    ) -> Result<Option<u128>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let query = api.purchased_space();

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }

    pub async fn pay_order(
        order_hash: &str,
        block_hash: Option<H256>,
    ) -> Result<Option<OrderInfo>, Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let order_hash = order_hash.as_bytes().to_vec();
        let query = api.pay_order(BoundedVec(order_hash));

        match query_storage(&query, block_hash).await {
            Ok(result) => match result {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }
}
