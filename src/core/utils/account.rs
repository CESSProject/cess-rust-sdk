use anyhow::{bail, Context, Result};
use blake2::{Blake2b512, Digest};
use sp_keyring::sr25519::sr25519::Pair;
use subxt::ext::sp_core::crypto::{
    AccountId32, Ss58AddressFormat, Ss58AddressFormatRegistry, Ss58Codec,
};
use subxt::ext::sp_core::{ByteArray, Pair as sp_core_pair};
use subxt::utils::AccountId32 as SubxtUtilAccountId32;

const SS_PREFIX: [u8; 7] = [0x53, 0x53, 0x35, 0x38, 0x50, 0x52, 0x45];
const SUBSTRATE_PREFIX: [u8; 1] = [0x2a];
const CESS_PREFIX: [u8; 2] = [0x50, 0xac];

pub fn parsing_public_key(address: &str) -> Result<Vec<u8>> {
    match verify_address(address, &CESS_PREFIX) {
        Err(_) => {
            if verify_address(address, &SUBSTRATE_PREFIX).is_err() {
                bail!("Invalid Account");
            }
            let data = bs58::decode(address)
                .into_vec()
                .with_context(|| "Public key decoding failed")?;
            if data.len() != 32 + SUBSTRATE_PREFIX.len() {
                bail!("Public key decoding failed")
            }
            Ok(data[SUBSTRATE_PREFIX.len()..data.len() - 2].to_vec())
        }
        Ok(()) => {
            let data = bs58::decode(address)
                .into_vec()
                .with_context(|| "Public key decoding failed")?;

            if data.len() != 34 + CESS_PREFIX.len() {
                bail!("Public key decoding failed")
            }
            Ok(data[CESS_PREFIX.len()..data.len() - 2].to_vec())
        }
    }
}

pub fn encode_public_key_as_substrate_account(public_key: &[u8]) -> Result<String> {
    encode_public_key_as_account(public_key, &SUBSTRATE_PREFIX)
}

pub fn encode_public_key_as_cess_account(public_key: &[u8]) -> Result<String> {
    encode_public_key_as_account(public_key, &CESS_PREFIX)
}

fn encode_public_key_as_account(public_key: &[u8], prefix: &[u8]) -> Result<String> {
    if public_key.len() != 32 {
        bail!("Invalid public key")
    }
    let payload = append_bytes(prefix, public_key);
    let input = append_bytes(&SS_PREFIX, &payload);
    let mut hasher = Blake2b512::new();
    hasher.update(input);
    let ck = hasher.finalize();
    let checksum = &ck[..2];
    let address = bs58::encode(append_bytes(&payload, checksum)).into_string();
    if address.is_empty() {
        bail!("Public key encoding failed");
    }
    Ok(address)
}

fn append_bytes(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data1.len() + data2.len());
    result.extend_from_slice(data1);
    result.extend_from_slice(data2);
    result
}

pub fn verify_address(address: &str, prefix: &[u8]) -> Result<()> {
    let decode_bytes = bs58::decode(address)
        .into_vec()
        .with_context(|| "Public key decoding failed")?;

    if decode_bytes.len() != 34 + prefix.len() {
        bail!("Public key decoding failed");
    }

    if decode_bytes[0] != prefix[0] {
        bail!("Invalid account prefix");
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
            bail!("Invalid account");
        }
    }
    if pub_key.len() != 32 {
        bail!("Invalid account public key");
    }

    Ok(())
}

pub fn get_ss58_address(account_str: &str) -> Result<String> {
    let ss58_address = AccountId32::from_string(account_str)?;
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}

pub fn get_ss58_address_from_subxt_accountid32(account: SubxtUtilAccountId32) -> Result<String> {
    let ss58_address = match AccountId32::from_slice(&account.0) {
        Ok(ss58_address) => ss58_address,
        Err(_) => bail!("Error"),
    };
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}

pub fn get_pair_address_as_ss58_address(pair: Pair) -> Result<String> {
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address = pair
        .public()
        .to_ss58check_with_version(Ss58AddressFormat::custom(address_type));
    Ok(ss58_cess_address)
}

#[cfg(test)]
mod test {
    use crate::{config::get_deoss_account, utils::account_from_slice};

    use super::parsing_public_key;
    use super::{get_ss58_address_from_subxt_accountid32, SubxtUtilAccountId32};

    #[test]
    fn test_parsing_public_key() {
        let pk = parsing_public_key(&get_deoss_account()).unwrap();
        let account = account_from_slice(&pk);
        assert_eq!(
            account.to_string(),
            "5F2EcqaLtFps43aGFLHAkZ4RSHC6qAxZKdvg5bYH4uEo7Ufx"
        );
    }
    #[test]
    fn test_get_ss58_address_from_subxt_accountid32() {
        let account = SubxtUtilAccountId32([
            44, 237, 227, 3, 163, 58, 80, 236, 155, 150, 17, 162, 47, 85, 153, 202, 120, 76, 8,
            151, 23, 35, 43, 161, 189, 88, 201, 0, 134, 112, 249, 66,
        ]);
        let ss58_account = get_ss58_address_from_subxt_accountid32(account);
        let ss58_account = match ss58_account {
            Ok(ss58_account) => ss58_account,
            _ => "Error".to_string(),
        };
        assert_eq!(
            "cXfzZzcdn5b8gmsZ6vQvPvgcP7ZEMUsgKoxnXz4SRV6T6423B",
            &ss58_account
        );
    }
}
