//! # cess-rust-sdk Library
//!
//! This crate provides core functionalities and utilities to interact with the
//! CESS blockchain network. It offers modules for chain operations, ledger
//! access, RPC connectivity, and data retrieval utilities.  
//!
//! The library uses [`subxt`] to connect to Substrate-based chains and
//! simplifies setup and environment-based configuration for CESS RPC endpoints.
//!
//! ## Overview
//! - Auto-initializes a connection to a CESS node (via `init_api`)
//! - Supports both environment-configured and default RPC URLs
//! - Provides chain access modules (`chain`, `ledger`, `retriever`, etc.)
//!
//! ## Deprecated
//! The `gateway` module is deprecated; use `retriever` instead.

#![recursion_limit = "1024"]

pub mod chain;
pub mod constants;
pub mod core;

#[deprecated(
    since = "0.8.0-premainnet",
    note = "The `gateway` module is deprecated. Please use the `retriever` module instead."
)]
pub mod gateway;
pub mod ledger;
pub mod retriever;
pub mod utils;

use core::Error;
use dotenv::dotenv;
use futures::future;
use log::info;
use once_cell::sync::Lazy;
use std::env;
use std::sync::Arc;
use std::time::Duration;
pub use subxt;
use subxt::backend::rpc::reconnecting_rpc_client::{ExponentialBackoff, RpcClient};
use subxt::utils::Yes;
use subxt::{
    config::substrate::H256, storage::Address as StorageAddress, OnlineClient, PolkadotConfig,
};
use tokio::sync::Mutex;
use tokio::task;

/// Global shared instance of [`OnlineClient`].
/// Ensures that the SDK maintains a single initialized RPC client across async contexts.
static CHAIN_API: Lazy<Arc<Mutex<Option<OnlineClient<PolkadotConfig>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

/// Auto-generated Polkadot runtime metadata.
/// Used by `subxt` to encode/decode calls and storage access.
#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod polkadot {}

/// Builds an RPC client with retry and exponential backoff configuration.
///
/// # Arguments
/// * `url` - WebSocket endpoint for the CESS node
///
/// # Errors
/// Returns an [`Error::Custom`] if the client cannot be constructed or connected.
async fn prepare_rpc_client(url: &str) -> Result<RpcClient, Error> {
    let client = RpcClient::builder()
        .retry_policy(
            ExponentialBackoff::from_millis(100)
                .max_delay(Duration::from_secs(10))
                .take(3),
        )
        .build(url.to_string())
        .await
        .map_err(|e| Error::Custom(e.to_string()))?;

    Ok(client)
}

/// Attempts to establish a connection to the provided URL or falls back to a local node.
///
/// Used as a lower-level connector by `init_api_with_force`.
async fn try_connect(url: Option<&str>) -> Result<OnlineClient<PolkadotConfig>, Error> {
    let rpc = match url {
        Some(url) => prepare_rpc_client(url).await?,
        None => prepare_rpc_client("ws://127.0.0.1:9944").await?,
    };
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc.clone()).await?;
    Ok(api)
}

/// Tries connecting to one of the official CESS RPC endpoints.
///
/// Connection is attempted concurrently across candidate URLs; the first successful
/// connection is used. Mainnet vs. testnet selection is controlled by the `RPC_NETWORK`
/// environment variable.
async fn try_default_connect() -> Result<OnlineClient<PolkadotConfig>, Error> {
    let mut urls = [
        "wss://t2-rpc.cess.network/",
        "wss://testnet-rpc.cess.cloud:443/ws/",
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

/// Initializes the global chain API, forcing a new connection.
///
/// Equivalent to `init_api_with_force(true)`.
pub async fn init_api() -> Result<OnlineClient<PolkadotConfig>, Error> {
    init_api_with_force(true).await
}

/// Initializes the chain API if not already connected.
///
/// Equivalent to `init_api_with_force(false)`.
pub async fn init_api_no_force() -> Result<OnlineClient<PolkadotConfig>, Error> {
    init_api_with_force(false).await
}

/// Core API initializer.
///
/// Handles `.env` loading, environment-based configuration (`RPC_URL`, `RPC_NETWORK`),
/// and manages shared access to the global [`CHAIN_API`] instance.
///
/// # Arguments
/// * `force_new` - If `true`, always creates a fresh connection, even if one already exists.
///
/// # Behavior
/// - Uses `RPC_URL` from environment if provided.
/// - Falls back to official or local nodes if connection fails.
/// - Logs the active endpoint through the `SDK` log target.
///
/// # Returns
/// A ready-to-use [`OnlineClient`] instance connected to the CESS network.
pub async fn init_api_with_force(force_new: bool) -> Result<OnlineClient<PolkadotConfig>, Error> {
    dotenv().ok();

    let url = env::var("RPC_URL").ok();

    let mut chain_api = CHAIN_API.lock().await;

    if !force_new {
        if let Some(ref api) = *chain_api {
            return Ok(api.clone());
        }
    }

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
