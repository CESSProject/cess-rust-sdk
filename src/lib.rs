pub mod chain;
pub mod core;
pub mod utils;

use anyhow::Result;
use subxt::{OnlineClient, PolkadotConfig};

const URL: &str = "ws://127.0.0.1:9944";

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod polkadot {}

async fn init_api() -> Result<OnlineClient<PolkadotConfig>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    Ok(api)
}
