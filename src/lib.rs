#![recursion_limit = "1024"]

pub mod chain;
pub mod constants;
pub mod core;
pub mod utils;
pub use subxt;

use core::Error;
use dotenv::dotenv;
use futures::future;
use log::info;
use once_cell::sync::Lazy;
use std::env;
use std::sync::Arc;
use subxt::utils::Yes;
use subxt::{
    config::substrate::H256, storage::Address as StorageAddress, OnlineClient, PolkadotConfig,
};
use tokio::sync::Mutex;
use tokio::task;

static CHAIN_API: Lazy<Arc<Mutex<Option<OnlineClient<PolkadotConfig>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod polkadot {}

async fn try_connect(url: Option<&str>) -> Result<OnlineClient<PolkadotConfig>, Error> {
    let api = match url {
        Some(url) => OnlineClient::<PolkadotConfig>::from_url(url).await?,
        None => OnlineClient::<PolkadotConfig>::new().await?,
    };

    Ok(api)
}

async fn try_default_connect() -> Result<OnlineClient<PolkadotConfig>, Error> {
    dotenv().ok();

    let mut urls = [
        "wss://testnet-rpc.cess.cloud:443/ws/",
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
            urls = [
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

pub async fn init_api() -> Result<OnlineClient<PolkadotConfig>, Error> {
    dotenv().ok();

    let url = env::var("RPC_URL").ok();

    let mut chain_api = CHAIN_API.lock().await;

    if let Some(ref api) = *chain_api {
        Ok(api.clone())
    } else {
        let api = if let Some(url) = url {
            match try_connect(Some(&url)).await {
                Ok(api) => {
                    info!(target: "SDK", "Connected to: {}", url);
                    api
                }
                Err(_) => match try_default_connect().await {
                    Ok(api) => {
                        info!(target: "SDK", "Connected to official RPC server");
                        api
                    }
                    Err(_) => return Err("All connections failed.".into()),
                },
            }
        } else {
            try_connect(None)
                .await
                .map_err(|_| Error::Custom("All connections failed.".into()))?
        };
        *chain_api = Some(api.clone());
        Ok(api)
    }
}
