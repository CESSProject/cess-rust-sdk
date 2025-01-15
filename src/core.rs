use subxt::Error as SubxtError;

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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error: {0}")]
    Custom(String),

    #[error(transparent)]
    Subxt(#[from] SubxtError),

    #[error(transparent)]
    Application(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    TryFromSlice(#[from] std::array::TryFromSliceError),

    #[error(transparent)]
    PublicError(#[from] subxt::ext::sp_core::crypto::PublicError),

    #[error(transparent)]
    SecretStringError(#[from] subxt::ext::sp_core::crypto::SecretStringError),

    #[error(transparent)]
    ResolveError(#[from] trust_dns_resolver::error::ResolveError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ReqwestInvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Custom(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Custom(s)
    }
}
