extern crate reed_solomon_erasure;

pub mod chain;
pub mod config;
pub mod core;
pub mod utils;
use anyhow::{Result, bail};
pub use subxt;

use config::get_url;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod polkadot {}

async fn init_api() -> Result<OnlineClient<PolkadotConfig>> {
    let url = get_url();
    let alternate_urls: Vec<&str> = vec!["wss://testnet-rpc1.cess.cloud:443/ws/", "wss://testnet-rpc2.cess.cloud:443/ws/"];


    if let Ok(api) = OnlineClient::<PolkadotConfig>::from_url(url).await {
        return Ok(api);
    } else {
        for alternate_url in alternate_urls {
            tokio::select! {
                // Concurrently try an alternate URL
                result = OnlineClient::<PolkadotConfig>::from_url(alternate_url) => {
                    if let Ok(api) = result {
                        return Ok(api);
                    }
                },
            }
        }
    }

    bail!("Failed to initialize API")
}
