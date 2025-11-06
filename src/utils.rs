//! # Core Utilities
//!
//! This module provides fundamental helper functions and submodules
//! for account formatting, hashing, IP handling, and block/extrinsic access.
//!
//! It acts as a foundation layer for higher-level SDK features, simplifying
//! operations such as address encoding, hash conversions, and fetching
//! extrinsics from specific blocks.
//!
//! ## Included Submodules
//! - `account`: Account-related utilities and types
//! - `file`: File management utilities
//! - `ip`: IP utilities for node/network operations
//! - `str`: String-related helpers

pub mod account;
pub mod file;
pub mod ip;
pub mod str;

use crate::polkadot::runtime_types::cp_cess_common::Hash;
use crate::{core::Error, init_api};
use subxt::{
    blocks::Extrinsics,
    ext::sp_core::crypto::{AccountId32, Ss58AddressFormat, Ss58AddressFormatRegistry, Ss58Codec},
    utils::H256,
    OnlineClient, PolkadotConfig,
};

/// Converts an account public key string into a CESS-compatible SS58 address.
///
/// # Arguments
/// * `account_str` - Raw account string (public key or address)
///
/// # Returns
/// A CESS Testnet-formatted SS58 address string.
///
/// # Errors
/// Returns an [`Error`] if the provided string cannot be parsed into a valid `AccountId32`.
pub fn get_ss58_address(account_str: &str) -> Result<String, Error> {
    let ss58_address = AccountId32::from_string(account_str)?;
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}

/// Converts a string into a `cp_cess_common::Hash` ([u8; 64]).
///
/// This function expects the input to be a hex-like 64-byte string and handles
/// optional `"0x"` prefixes.
///
/// # Arguments
/// * `v` - Hex string or byte-like representation
///
/// # Errors
/// Returns an [`Error`] if the conversion to a fixed 64-byte array fails.
///
/// # Panics
/// None - handled safely unless `try_into()` fails, which is converted to `Error`.
pub fn hash_from_string(v: &str) -> Result<Hash, Error> {
    // Check if the hash starts with "0x"
    let v = if v.starts_with("0x") {
        v.strip_prefix("0x").unwrap_or(v)
    } else {
        v
    };

    // Convert to bytes and try to convert into a fixed-size array
    let bytes = v.as_bytes();
    let hash_array: [u8; 64] = bytes.try_into()?;

    Ok(Hash(hash_array))
}

/// Converts a 32-byte block hash in hex format into an [`H256`] type.
///
/// # Arguments
/// * `hex` - Block hash in hex string form (with or without `"0x"` prefix)
///
/// # Panics
/// Panics if the decoded value is not exactly 32 bytes long or hex decoding fails.
///
/// # Example
/// ```
/// let h = block_hex_string_to_h256("0x1234abcd...");
/// ```
pub fn block_hex_string_to_h256(hex: &str) -> H256 {
    let hex_without_prefix = if let Some(hex_without_prefix) = hex.strip_prefix("0x") {
        hex_without_prefix
    } else {
        hex
    };

    let decoded = hex::decode(hex_without_prefix).expect("Failed to decode hex string");

    // Ensure the decoded bytes are exactly 32 bytes
    if decoded.len() != 32 {
        panic!("Hex string does not have the expected length");
    }

    let mut hash_array = [0u8; 32];
    hash_array.copy_from_slice(&decoded);

    H256(hash_array)
}

/// Fetches all extrinsics for a given block hash.
///
/// # Arguments
/// * `block_hash` - Target block hash as [`H256`]
///
/// # Returns
/// A collection of [`Extrinsics`] corresponding to the block.
///
/// # Errors
/// Returns an [`Error`] if the block or extrinsics cannot be fetched (e.g., RPC failure).
///
/// # Example
/// ```ignore
/// let block_hash = block_hex_string_to_h256("0xabc..."); 
/// let extrinsics = get_extrinsics_at(block_hash).await?;
/// ```
pub async fn get_extrinsics_at(
    block_hash: H256,
) -> Result<Extrinsics<PolkadotConfig, OnlineClient<PolkadotConfig>>, Error> {
    let api = init_api().await?;

    let block = api.blocks().at(block_hash).await?;
    let extrinsics = block.extrinsics().await?;

    Ok(extrinsics)
}
