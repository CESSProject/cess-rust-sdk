use anyhow::{bail, Result};
use subxt::storage::{address::Yes, StorageAddress};


use crate::init_api;

pub fn hex_string_to_bytes(hex: &str) -> [u8; 64] {
    let hex_without_prefix = if hex.starts_with("0x") {
        &hex[2..]
    } else {
        hex
    };

    let bytes = hex::decode(hex_without_prefix).expect("Failed to decode hex string");
    let mut result = [0u8; 64];

    if bytes.len() != result.len() {
        panic!("Hex string does not have the expected length");
    }

    result.copy_from_slice(&bytes);
    result
}

pub(crate) async fn query_storage<'address, Address>(
    query: &'address Address,
) -> Result<<Address as StorageAddress>::Target>
where
    Address: StorageAddress<IsFetchable = Yes> + 'address,
{
    let api = match init_api().await {
        Ok(api) => api,
        Err(e) => bail!("Failed to initialize API: {}", e),
    };

    let result = match api.storage().at_latest().await {
        Ok(mid_result) => match mid_result.fetch(query).await {
            Ok(Some(result)) => Ok(result),
            Ok(None) => {
                bail!("Value not found in storage for query");
            }
            Err(e) => {
                bail!("Failed to retrieve data from storage: {}", e);
            }
        },
        Err(e) => {
            bail!("Failed to fetch data from storage: {}", e);
        }
    };

    result
}
