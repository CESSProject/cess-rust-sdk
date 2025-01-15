pub mod account;
pub mod bucket;
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

pub fn get_ss58_address(account_str: &str) -> Result<String, Error> {
    let ss58_address = AccountId32::from_string(account_str)?;
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}

// returns cp_cess_common::Hash([u8; 64])
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

pub async fn get_extrinsics_at(
    block_hash: H256,
) -> Result<Extrinsics<PolkadotConfig, OnlineClient<PolkadotConfig>>, Error> {
    let api = init_api().await?;

    let block = api.blocks().at(block_hash).await?;
    let extrinsics = block.extrinsics().await?;

    Ok(extrinsics)
}
