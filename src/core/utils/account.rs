use anyhow::{bail, Context, Result};
use blake2::{Blake2b512, Digest};

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

#[cfg(test)]
mod test {
    use crate::{core::pattern::PUBLIC_DEOSS_ACCOUNT, utils::account_from_slice};

    use super::parsing_public_key;

    #[test]
    fn test_parsing_public_key() {
        let pk = parsing_public_key(PUBLIC_DEOSS_ACCOUNT).unwrap();
        let account = account_from_slice(&pk);
        assert_eq!(
            account.to_string(),
            "5F2EcqaLtFps43aGFLHAkZ4RSHC6qAxZKdvg5bYH4uEo7Ufx"
        );
    }
}
