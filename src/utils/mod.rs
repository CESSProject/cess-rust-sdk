use anyhow::{anyhow, bail, Result};

use subxt::{
    blocks::{ExtrinsicEvents, Extrinsics},
    config::ExtrinsicParams,
    storage::{address::Yes, StorageAddress},
    tx::{Signer as SignerT, TxPayload},
    utils::{AccountId32, H256},
    Config, PolkadotConfig, OnlineClient,
};

use crate::{init_api, polkadot};
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

pub(crate) async fn query_storage<'address, Address>(
    query: &'address Address,
    block_hash: Option<H256>,
) -> Result<Option<<Address as StorageAddress>::Target>>
where
    Address: StorageAddress<IsFetchable = Yes> + 'address,
{
    let api = init_api().await;
    if let Some(block_hash) = block_hash {
        match api.storage().at(block_hash).fetch(query).await {
            Ok(value) => Ok(value),
            Err(e) => {
                bail!("Failed to retrieve data from storage: {}", e);
            }
        }
    } else {
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
    let api = init_api().await;

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
    let api = init_api().await;

    match api.tx().sign_and_submit_then_watch_default(tx, from).await {
        Ok(result) => match result.wait_for_finalized_success().await {
            Ok(r) => Ok(r),
            Err(_) => {
                bail!("Error waiting for finalized success");
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

pub async fn get_extrinsics_at(block_hash: H256) -> Result<Extrinsics<PolkadotConfig, OnlineClient<PolkadotConfig>>> {
    let api = init_api().await;

    let block = api.blocks().at(block_hash).await?;
    let extrinsics = block.body().await?.extrinsics();
    Ok(extrinsics)
}