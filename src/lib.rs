extern crate reed_solomon_erasure;

pub mod chain;
pub mod core;
pub mod utils;
pub use subxt::subxt;
use anyhow::Result;
use subxt::{OnlineClient, PolkadotConfig};

const URL: &str = "wss://testnet-rpc0.cess.cloud:443/ws/";

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod polkadot {}

async fn init_api() -> Result<OnlineClient<PolkadotConfig>> {
    // let api = OnlineClient::<PolkadotConfig>::new().await?;
    let api = OnlineClient::<PolkadotConfig>::from_url(URL).await?;

    Ok(api)
}
