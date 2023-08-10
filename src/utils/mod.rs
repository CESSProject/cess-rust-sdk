use anyhow::{anyhow, bail, Result};
use subxt::{
    blocks::ExtrinsicEvents,
    config::ExtrinsicParams,
    storage::{address::Yes, StorageAddress},
    tx::{Signer as SignerT, TxPayload},
    utils::AccountId32,
    Config, PolkadotConfig,
};

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

pub(crate) async fn sign_and_submit_tx<Call, Signer, T>(
    tx: &Call,
    from: &Signer,
) -> Result<ExtrinsicEvents<PolkadotConfig>>
where
    Call: TxPayload,
    Signer: SignerT<T> + subxt::tx::Signer<subxt::PolkadotConfig>,
    T: Config,
{
    let api = match init_api().await {
        Ok(api) => api,
        Err(e) => bail!("Failed to initialize API: {}", e),
    };

    match api.tx().sign_and_submit_then_watch_default(tx, from).await {
        Ok(result) => match result.wait_for_finalized_success().await {
            Ok(r) => Ok(r),
            Err(e) => {
                let err = anyhow!("Error waiting for finalized success: {}", e);
                bail!("{}", err);
            }
        },
        Err(e) => {
            let err = anyhow!("Error signing and submitting transaction: {}", e);
            bail!("{}", err);
        }
    }
}

pub(crate) fn account_from_slice(pk: &[u8]) -> AccountId32 {
    let mut pk_array = [0u8; 32];
    pk_array.copy_from_slice(&pk[..32]); // Ensure the slice is exactly 32 bytes

    AccountId32::from(pk_array)
}
