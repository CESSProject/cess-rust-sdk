use crate::core::ApiProvider;
use crate::polkadot::{
    self, runtime_types::bounded_collections::bounded_vec::BoundedVec,
    storage_handler::calls::TransactionApi,
};
use crate::{impl_api_provider, H256};

// impl ApiProvider for TransactionApiProvider
impl_api_provider!(
    TransactionApiProvider,
    TransactionApi,
    polkadot::tx().storage_handler()
);

pub struct StorageTransaction;

impl StorageTransaction {
    fn get_api() -> TransactionApi {
        crate::core::get_api::<TransactionApiProvider>()
    }

    pub async fn mint_territory(gib_count: u32, territory_name: &str) {
        let api = Self::get_api();
        let territory_name = territory_name.as_bytes().to_vec();
        let tx = api.mint_territory(gib_count, BoundedVec(territory_name));
    }
}
