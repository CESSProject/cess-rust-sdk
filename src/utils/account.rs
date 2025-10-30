//! # Account Utilities
//!
//! Provides utilities for encoding, decoding, and verifying CESS/Substrate-style
//! SS58 addresses and public keys. This includes checksum verification,
//! format conversion, and helpers for generating valid account identifiers
//! compatible with the CESS network.
//!
//! ## Features
//! - Verify CESS and Substrate SS58 addresses
//! - Parse public keys from addresses
//! - Encode public keys back to SS58 addresses
//! - Convert between `AccountId32`, `Pair`, and `Subxt` types
//! 

use blake2::{Blake2b512, Digest};
use subxt::{
    ext::sp_core::{
        crypto::{AccountId32, Ss58AddressFormat, Ss58AddressFormatRegistry, Ss58Codec},
        sr25519::Pair,
        ByteArray, Pair as sp_core_pair,
    },
    utils::AccountId32 as SubxtUtilsAccountId32,
};

use crate::core::Error;

/// Prefix constants used for SS58 encoding/decoding.
const SS_PREFIX: [u8; 7] = [0x53, 0x53, 0x35, 0x38, 0x50, 0x52, 0x45];
const SUBSTRATE_PREFIX: [u8; 1] = [0x2a];
const CESS_PREFIX: [u8; 2] = [0x50, 0xac];

/// Concatenates two byte slices into a new `Vec<u8>`.
fn append_bytes(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data1.len() + data2.len());
    result.extend_from_slice(data1);
    result.extend_from_slice(data2);
    result
}

/// Verifies whether an address is valid for a given prefix (CESS or Substrate).
///
/// Performs SS58 checksum verification and prefix validation.
///
/// # Arguments
/// * `address` - Address string to verify
/// * `prefix` - Network-specific prefix bytes (`CESS_PREFIX` or `SUBSTRATE_PREFIX`)
///
/// # Errors
/// Returns an [`Error`] if the address fails decoding, has an invalid prefix,
/// checksum, or incorrect length.
pub fn verify_address(address: &str, prefix: &[u8]) -> Result<(), Error> {
    let decode_bytes = bs58::decode(address)
        .into_vec()
        .map_err(|_| "Public key decoding failed")?;

    if decode_bytes.len() != 34 + prefix.len() {
        return Err("Public key decoding failed".into());
    }

    if decode_bytes[0] != prefix[0] {
        return Err("Invalid account prefix".into());
    }

    let pub_key = &decode_bytes[prefix.len()..decode_bytes.len() - 2];

    let data = append_bytes(prefix, pub_key);
    let input = append_bytes(&SS_PREFIX, &data);
    let mut hasher = Blake2b512::new();
    hasher.update(input);
    let ck = hasher.finalize();
    let check_sum = &ck[..2];
    for i in 0..2 {
        if check_sum[i] != decode_bytes[32 + prefix.len() + i] {
            return Err("Invalid account".into());
        }
    }
    if pub_key.len() != 32 {
        return Err("Invalid account public key".into());
    }

    Ok(())
}

/// Extracts the 32-byte public key from a valid SS58 address.
///
/// Automatically handles both CESS and Substrate address prefixes.
///
/// # Arguments
/// * `address` - The address string to parse
///
/// # Returns
/// A 32-byte vector representing the account’s public key.
///
/// # Errors
/// Returns an [`Error`] if decoding fails or the address format is invalid.
pub fn parsing_public_key(address: &str) -> Result<Vec<u8>, Error> {
    match verify_address(address, &CESS_PREFIX) {
        Err(_) => {
            if verify_address(address, &SUBSTRATE_PREFIX).is_err() {
                return Err("Invalid Account".into());
            }
            let data = bs58::decode(address)
                .into_vec()
                .map_err(|_| "Public key decoding failed")?;
            if data.len() != 32 + SUBSTRATE_PREFIX.len() {
                return Err("Public key decoding failed".into());
            }
            Ok(data[SUBSTRATE_PREFIX.len()..data.len() - 2].to_vec())
        }
        Ok(()) => {
            let data = bs58::decode(address)
                .into_vec()
                .map_err(|_| "Public key decoding failed")?;

            if data.len() != 34 + CESS_PREFIX.len() {
                return Err("Public key decoding failed".into());
            }
            Ok(data[CESS_PREFIX.len()..data.len() - 2].to_vec())
        }
    }
}

/// Encodes a raw public key as a Substrate-format SS58 address.
pub fn encode_public_key_as_substrate_account(public_key: &[u8]) -> Result<String, Error> {
    encode_public_key_as_account(public_key, &SUBSTRATE_PREFIX)
}

/// Encodes a raw public key as a CESS-format SS58 address.
pub fn encode_public_key_as_cess_account(public_key: &[u8]) -> Result<String, Error> {
    encode_public_key_as_account(public_key, &CESS_PREFIX)
}

/// Generic internal encoder for SS58 address construction.
///
/// Handles prefix attachment, checksum computation, and Base58 encoding.
fn encode_public_key_as_account(public_key: &[u8], prefix: &[u8]) -> Result<String, Error> {
    if public_key.len() != 32 {
        return Err("Invalid public key".into());
    }
    let payload = append_bytes(prefix, public_key);
    let input = append_bytes(&SS_PREFIX, &payload);
    let mut hasher = Blake2b512::new();
    hasher.update(input);
    let ck = hasher.finalize();
    let checksum = &ck[..2];
    let address = bs58::encode(append_bytes(&payload, checksum)).into_string();
    if address.is_empty() {
        return Err("Public key encoding failed".into());
    }
    Ok(address)
}

/// Converts an account string to a CESS SS58 address using the testnet format.
pub fn get_ss58_address(account_str: &str) -> Result<String, Error> {
    let ss58_address = AccountId32::from_string(account_str)?;
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}

/// Converts a `Subxt` [`AccountId32`] into a CESS-formatted SS58 address.
pub fn get_ss58_address_from_subxt_accountid32(
    account: SubxtUtilsAccountId32,
) -> Result<String, Error> {
    let ss58_address = match AccountId32::from_slice(&account.0) {
        Ok(ss58_address) => ss58_address,
        Err(_) => return Err("Error: Unable to parse AccountId32".into()),
    };
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}

/// Derives a CESS SS58 address from an `sr25519::Pair` (keypair).
///
/// Useful for key management or signing scenarios.
pub fn get_pair_address_as_ss58_address(pair: Pair) -> Result<String, Error> {
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address = pair
        .public()
        .to_ss58check_with_version(Ss58AddressFormat::custom(address_type));
    Ok(ss58_cess_address)
}

/// Converts a 32-byte slice into a `Subxt`-compatible [`AccountId32`].
///
/// # Panics
/// Panics if the provided slice is shorter than 32 bytes.
pub fn account_from_slice(pk: &[u8]) -> SubxtUtilsAccountId32 {
    let mut pk_array = [0u8; 32];
    pk_array.copy_from_slice(&pk[..32]); // Ensure the slice is exactly 32 bytes

    SubxtUtilsAccountId32::from(pk_array)
}
