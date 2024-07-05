pub mod storage_handler;

use crate::polkadot::storage_handler::storage::StorageApi;

use crate::{query_storage, StorageAddress, Yes, H256};

trait Query {
    fn get_api() -> StorageApi;

    async fn execute_query<'address, Address>(
        query: &'address Address,
        block_hash: Option<H256>,
    ) -> Result<Option<<Address as StorageAddress>::Target>, Box<dyn std::error::Error>>
    where
        Address: StorageAddress<IsFetchable = Yes> + 'address,
    {
        match query_storage(query, block_hash).await {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("Query failed: {}", err).into()),
        }
    }
}