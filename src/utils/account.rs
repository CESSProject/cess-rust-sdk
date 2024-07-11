use blake2::{Blake2b512, Digest};
use sp_keyring::sr25519::sr25519::Pair;
use subxt::{
    ext::sp_core::{
        crypto::{AccountId32, Ss58AddressFormat, Ss58AddressFormatRegistry, Ss58Codec},
        ByteArray, Pair as sp_core_pair,
    },
    utils::AccountId32 as SubxtUtilsAccountId32,
};

const SS_PREFIX: [u8; 7] = [0x53, 0x53, 0x35, 0x38, 0x50, 0x52, 0x45];
const SUBSTRATE_PREFIX: [u8; 1] = [0x2a];
const CESS_PREFIX: [u8; 2] = [0x50, 0xac];

fn append_bytes(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data1.len() + data2.len());
    result.extend_from_slice(data1);
    result.extend_from_slice(data2);
    result
}

pub fn verify_address(address: &str, prefix: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn parsing_public_key(address: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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

pub fn encode_public_key_as_substrate_account(
    public_key: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
    encode_public_key_as_account(public_key, &SUBSTRATE_PREFIX)
}

pub fn encode_public_key_as_cess_account(
    public_key: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
    encode_public_key_as_account(public_key, &CESS_PREFIX)
}

fn encode_public_key_as_account(
    public_key: &[u8],
    prefix: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
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

pub fn get_ss58_address(account_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ss58_address = AccountId32::from_string(account_str)?;
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}

pub fn get_ss58_address_from_subxt_accountid32(
    account: SubxtUtilsAccountId32,
) -> Result<String, Box<dyn std::error::Error>> {
    let ss58_address = match AccountId32::from_slice(&account.0) {
        Ok(ss58_address) => ss58_address,
        Err(_) => return Err("Error: Unable to parse AccountId32".into()),
    };
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address =
        ss58_address.to_ss58check_with_version(Ss58AddressFormat::custom(address_type));

    Ok(ss58_cess_address)
}

pub fn get_pair_address_as_ss58_address(pair: Pair) -> Result<String, Box<dyn std::error::Error>> {
    let address_type = Ss58AddressFormatRegistry::CessTestnetAccount as u16;
    let ss58_cess_address = pair
        .public()
        .to_ss58check_with_version(Ss58AddressFormat::custom(address_type));
    Ok(ss58_cess_address)
}

pub fn account_from_slice(pk: &[u8]) -> SubxtUtilsAccountId32 {
    let mut pk_array = [0u8; 32];
    pk_array.copy_from_slice(&pk[..32]); // Ensure the slice is exactly 32 bytes

    SubxtUtilsAccountId32::from(pk_array)
}
