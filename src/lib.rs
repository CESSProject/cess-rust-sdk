extern crate reed_solomon_erasure;

pub mod chain;
pub mod config;
pub mod core;
pub mod utils;
pub use subxt;

use config::get_url;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod polkadot {}

async fn init_api() -> OnlineClient<PolkadotConfig> {
    let url = get_url();

    match OnlineClient::<PolkadotConfig>::from_url(url).await {
        Ok(api) => api,
        Err(e) => panic!("Failed to initialize API: {}", e),
    }
}
