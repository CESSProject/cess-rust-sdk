#![recursion_limit = "1024"]

pub mod chain;
pub mod core;
pub mod utils;
pub use subxt;

use dotenv::dotenv;
use futures::future;
use log::info;
use std::env;
use subxt::utils::Yes;
use subxt::{
    config::substrate::H256, storage::Address as StorageAddress, OnlineClient, PolkadotConfig,
};
use tokio::task;

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod polkadot {}

async fn try_connect(
    url: Option<&str>,
) -> Result<OnlineClient<PolkadotConfig>, Box<dyn std::error::Error>> {
    let api = match url {
        Some(url) => OnlineClient::<PolkadotConfig>::from_url(url).await?,
        None => OnlineClient::<PolkadotConfig>::new().await?,
    };

    Ok(api)
}

async fn try_default_connect() -> Result<OnlineClient<PolkadotConfig>, Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut urls = vec![
        "wss://testnet-rpc0.cess.cloud:443/ws/",
        "wss://testnet-rpc1.cess.cloud:443/ws/",
        "wss://testnet-rpc2.cess.cloud:443/ws/",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect::<Vec<String>>();

    if let Ok(is_mainnet) = env::var("RPC_NETWORK").map(|val| val == "mainnet") {
        if is_mainnet {
            // TODO: Replace with mainnet URLs when mainnet launch
            urls = vec![
                "wss://devnet-rpc.cess.cloud/ws/", // This is devnet
            ]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        }
    }

    let tasks: Vec<_> = urls
        .into_iter()
        .map(|url| task::spawn(async move { try_connect(Some(&url)).await.ok() }))
        .collect();

    // Returns the first successful connection or an error
    match future::select_ok(tasks).await {
        Ok((Some(api), _)) => Ok(api),
        Ok((None, _)) => Err("No successful connection.".into()),
        Err(_) => Err("All connections failed.".into()),
    }
}

pub async fn init_api() -> Result<OnlineClient<PolkadotConfig>, String> {
    dotenv().ok();

    let url = env::var("RPC_URL").ok();

    let api = if let Some(url) = url {
        match try_connect(Some(&url)).await {
            Ok(api) => {
                info!(target: "SDK" ,"Connected to: {}", url);
                api
            }
            Err(_) => match try_default_connect().await {
                Ok(api) => {
                    info!(target: "SDK" ,"Connected to official RPC server");
                    api
                }
                Err(_) => return Err("All connections failed.".into()),
            },
        }
    } else {
        try_connect(None)
            .await
            .map_err(|_| "All connections failed.")?
    };

    Ok(api)
}

pub async fn query_storage<'address, Address>(
    query: &'address Address,
    block_hash: Option<H256>,
) -> Result<Option<<Address as StorageAddress>::Target>, String>
where
    Address: StorageAddress<IsFetchable = Yes> + 'address,
{
    let api = init_api().await?;
    if let Some(block_hash) = block_hash {
        match api.storage().at(block_hash).fetch(query).await {
            Ok(value) => Ok(value),
            Err(_) => Err("Failed to retrieve data from storage".into()),
        }
    } else {
        match api.storage().at_latest().await {
            Ok(mid_result) => match mid_result.fetch(query).await {
                Ok(value) => Ok(value),
                Err(_) => Err("Failed to retrieve data from storage".into()),
            },
            Err(_) => Err("Failed to retrieve data from storage".into()),
        }
    }
}
