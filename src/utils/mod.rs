use anyhow::{anyhow, bail, Result};

use subxt::{
    blocks::ExtrinsicEvents,
    config::ExtrinsicParams,
    runtime_api::RuntimeApiPayload,
    storage::{address::Yes, StorageAddress},
    tx::{Signer as SignerT, TxPayload},
    utils::{AccountId32, H256},
    Config, PolkadotConfig,
};

use crate::{core::pattern::URL, init_api, polkadot};
use polkadot::runtime_types::cp_cess_common::Hash;

pub fn hex_string_to_bytes(hex: &str) -> [u8; 64] {
    let hex_without_prefix = if let Some(hex_without_prefix) = hex.strip_prefix("0x") {
        hex_without_prefix
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
) -> Result<Option<<Address as StorageAddress>::Target>>
where
    Address: StorageAddress<IsFetchable = Yes> + 'address,
{
    let api = init_api(URL).await;

    match api.storage().at_latest().await {
        Ok(mid_result) => match mid_result.fetch(query).await {
            Ok(value) => Ok(value),
            Err(e) => {
                bail!("Failed to retrieve data from storage: {}", e);
            }
        },
        Err(e) => {
            bail!("Failed to fetch data from storage: {}", e);
        }
    }
}

pub(crate) async fn runtime_api_call<Call: RuntimeApiPayload>(
    payload: Call,
) -> Result<Call::ReturnType> {
    let api = init_api(URL).await;

    match api.runtime_api().at_latest().await?.call(payload).await {
        Ok(result) => Ok(result),
        Err(err) => {
            bail!("Error: {}", err)
        }
    }
}

pub(crate) async fn sign_and_sbmit_tx_default<Call, Signer, T>(
    tx: &Call,
    from: &Signer,
) -> Result<H256>
where
    Call: TxPayload,
    Signer: SignerT<T> + subxt::tx::Signer<subxt::PolkadotConfig>,
    T: Config,
    <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams: Default,
{
    let api = init_api(URL).await;

    match api.tx().sign_and_submit_default(tx, from).await {
        Ok(hash) => Ok(hash),
        Err(err) => bail!("Error submitting transaction: {}", err),
    }
}

pub(crate) async fn sign_and_submit_tx_then_watch_default<Call, Signer, T>(
    tx: &Call,
    from: &Signer,
) -> Result<ExtrinsicEvents<PolkadotConfig>>
where
    Call: TxPayload,
    Signer: SignerT<T> + subxt::tx::Signer<subxt::PolkadotConfig>,
    T: Config,
{
    let api = init_api(URL).await;

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

pub(crate) fn hash_from_string(hash_str: &str) -> Hash {
    let hash_bytes = hex_string_to_bytes(hash_str);
    Hash(hash_bytes)
}
