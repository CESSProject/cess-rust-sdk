//! # OSS Transaction Module
//!
//! This module provides transaction interfaces (extrinsic calls) for interacting
//! with the `pallet_oss` runtime pallet.
//!
//! It includes functions for:
//! - Registering and updating OSS nodes
//! - Allowing users to authorize or revoke OSS permissions to act on their behalf
//! - Managing proxy and EVM-compatible authorizations
//! - Destroying OSS node records
//!
//! These transactions are signed and submitted through Subxt, and each call
//! returns the corresponding event or transaction hash upon success.

use crate::chain::{AnySigner, Call, Chain, DynSigner};
use crate::core::{ApiProvider, Error};
use crate::impl_api_provider;
use crate::polkadot::oss::calls::types::proxy_authorzie::Sig;
use crate::polkadot::runtime_types::pallet_oss::types::ProxyAuthPayload;
use crate::polkadot::{
    self,
    oss::calls::TransactionApi,
    oss::events::{Authorize, CancelAuthorize, OssDestroy, OssRegister, OssUpdate},
    runtime_types::bounded_collections::bounded_vec::BoundedVec,
};
use std::str::FromStr;
use subxt::ext::sp_core::{sr25519::Pair as PairS, Pair};
use subxt::ext::subxt_core::utils::AccountId32;
use subxt::tx::PairSigner;
use subxt::PolkadotConfig;

// impl ApiProvider for TransactionApiProvider
impl_api_provider!(TransactionApiProvider, TransactionApi, polkadot::tx().oss());

pub type TxHash = String;

// Implements the API provider for the `pallet_oss` transaction module.
pub struct StorageTransaction {
    signer: DynSigner,
}

impl Chain for StorageTransaction {}

impl Call for StorageTransaction {
    type Api = TransactionApi;

    fn get_api() -> Self::Api {
        crate::core::get_api::<TransactionApiProvider>()
    }

    fn get_signer(&self) -> &DynSigner {
        &self.signer
    }
}

impl StorageTransaction {
    pub fn from_mnemonic(mnemonic: &str) -> Self {
        let pair = PairS::from_string(mnemonic, None).unwrap();
        let boxed: AnySigner = Box::new(PairSigner::<PolkadotConfig, _>::new(pair));
        Self {
            signer: DynSigner::new(boxed),
        }
    }

    pub fn with_signer(signer: AnySigner) -> Self {
        Self {
            signer: DynSigner::new(signer),
        }
    }

    /// **Authorizes an OSS node** to perform file-related tasks on behalf of the user.
    ///
    /// This grants the specified OSS account permission to handle actions such as
    /// uploading, restoring, or maintaining files in the network for the user.
    ///
    /// # Arguments
    /// * `account` - The OSS account to authorize.
    pub async fn authorize(&self, account: &str) -> Result<(TxHash, Authorize), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.authorize(account);

        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<Authorize>(event)
    }

    /// **Revokes a previous OSS authorization**, removing its ability to perform tasks on behalf of the user.
    ///
    /// # Arguments
    /// * `account` - The OSS account whose authorization should be cancelled.
    pub async fn cancel_authorize(
        &self,
        account: &str,
    ) -> Result<(TxHash, CancelAuthorize), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.cancel_authorize(account);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<CancelAuthorize>(event)
    }

    /// Registers a new OSS node with the given endpoint and domain.
    pub async fn register(
        &self,
        endpoint: [u8; 38],
        domain: BoundedVec<u8>,
    ) -> Result<(TxHash, OssRegister), Error> {
        let api = Self::get_api();
        let tx = api.register(endpoint, domain);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<OssRegister>(event)
    }

    /// Updates an existing OSS node’s endpoint and domain.
    pub async fn update(
        &self,
        endpoint: [u8; 38],
        domain: BoundedVec<u8>,
    ) -> Result<(TxHash, OssUpdate), Error> {
        let api = Self::get_api();
        let tx = api.update(endpoint, domain);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<OssUpdate>(event)
    }

    /// Destroys the OSS record for the signer’s account.
    pub async fn destroy(&self) -> Result<(TxHash, OssDestroy), Error> {
        let api = Self::get_api();
        let tx = api.destroy();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<OssDestroy>(event)
    }

    /// Performs a **proxy authorization**, allowing a user to pre-sign a payload
    /// that gives permission for an OSS to act on their behalf.
    ///
    /// Returns the transaction hash upon success.
    pub async fn proxy_authorize(
        &self,
        account: &str,
        sig: Sig,
        payload: ProxyAuthPayload,
    ) -> Result<TxHash, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.proxy_authorzie(account.0, sig, payload);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;
        let hash = event.extrinsic_hash();
        Ok(format!("0x{}", hex::encode(hash.0)))
    }

    /// Performs an EVM-compatible proxy authorization.
    ///
    /// This variant uses a 65-byte ECDSA signature for cross-chain compatibility.
    pub async fn evm_proxy_authorzie(
        &self,
        account: &str,
        sig: [u8; 65],
        payload: ProxyAuthPayload,
    ) -> Result<TxHash, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.evm_proxy_authorzie(account.0, sig, payload);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;
        let hash = event.extrinsic_hash();
        Ok(format!("0x{}", hex::encode(hash.0)))
    }
}
