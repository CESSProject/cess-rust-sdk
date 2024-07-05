pub trait ApiProvider {
    type Api;

    fn get_api() -> Self::Api;
}

pub fn get_api<P: ApiProvider>() -> P::Api {
    P::get_api()
}

/// Macro to reduce the boilerplate code when implementing the `ApiProvider` trait.
/// # Example:
///
/// ```
/// impl_api_provider!(StorageApiProvider, StorageApi, polkadot::storage().storage_handler());
/// ```
/// Expands to:
/// ```
/// pub struct StorageApiProvider;
///
/// impl ApiProvider for StorageApiProvider {
///     type Api = StorageApi;
///     fn get_api() -> Self::Api {
///         polkadot::storage().storage_handler()
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_api_provider {
    ($provider:ident, $api:ty, $path:expr) => {
        pub struct $provider;

        impl ApiProvider for $provider {
            type Api = $api;

            fn get_api() -> Self::Api {
                $path
            }
        }
    };
}
