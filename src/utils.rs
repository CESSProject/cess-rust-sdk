pub mod account;
pub mod bucket;
pub mod file;
pub mod ip;
pub mod str;

use subxt::ext::sp_core::crypto::{
    AccountId32, Ss58AddressFormat, Ss58AddressFormatRegistry, Ss58Codec,
};

use crate::polkadot::runtime_types::cp_cess_common::Hash;

pub fn get_ss58_address(account_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ss58_address = AccountId32::from_string(account_str)?;
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}

// returns cp_cess_common::Hash([u8; 64])
pub fn hash_from_string(v: &str) -> Result<Hash, Box<dyn std::error::Error>> {
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
