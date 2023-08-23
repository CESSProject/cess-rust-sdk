extern crate reed_solomon_erasure;

pub mod chain;
pub mod config;
pub mod core;
pub mod utils;
pub use subxt::subxt;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod polkadot {}

async fn init_api(url: &str) -> OnlineClient<PolkadotConfig> {
    match OnlineClient::<PolkadotConfig>::from_url(url).await {
        Ok(api) => api,
        Err(e) => panic!("Failed to initialize API: {}", e),
    }
}
