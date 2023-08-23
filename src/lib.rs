extern crate reed_solomon_erasure;

pub mod chain;
pub mod config;
pub mod core;
pub mod utils;
use config::{get_custom_url, URL};
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod polkadot {}

async fn init_api() -> OnlineClient<PolkadotConfig> {
    let url = if let Some(url) = get_custom_url() {
        url
    } else {
        URL.to_string()
    };

    match OnlineClient::<PolkadotConfig>::from_url(url).await {
        Ok(api) => api,
        Err(e) => panic!("Failed to initialize API: {}", e),
    }
}
