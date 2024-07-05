use subxt::ext::sp_core::crypto::{
    AccountId32, ByteArray, Ss58AddressFormat, Ss58AddressFormatRegistry, Ss58Codec,
};
use subxt::utils::AccountId32 as SubxtUtilsAccountId32;

pub fn account_from_slice(pk: &[u8]) -> SubxtUtilsAccountId32 {
    let mut pk_array = [0u8; 32];
    pk_array.copy_from_slice(&pk[..32]); // Ensure the slice is exactly 32 bytes

    SubxtUtilsAccountId32::from(pk_array)
}

pub fn get_ss58_address(account_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ss58_address = AccountId32::from_string(account_str)?;
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}
