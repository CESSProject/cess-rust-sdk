//! # Balances Transaction Module
//!
//! This module defines the transaction logic for interacting with the
//! `pallet_balances` runtime module on the blockchain.  
//!
//! It provides a structured way to perform balance transfers using
//! various signer types - including mnemonic-based signers and
//! hardware wallets (Ledger).
//!
//! The module builds upon the generic [`Chain`] and [`Call`] traits
//! for unified blockchain interaction.

use crate::chain::{AnySigner, Call, Chain, DynSigner};
use crate::core::{ApiProvider, Error};
use crate::impl_api_provider;
use crate::ledger::LedgerSigner;
use crate::polkadot::balances::events::Transfer;
use crate::polkadot::{self, balances::calls::TransactionApi};
use std::str::FromStr;
use subxt::ext::sp_core::{sr25519::Pair as PairS, Pair};
use subxt::ext::subxt_core::utils::AccountId32;
use subxt::tx::PairSigner;
use subxt::PolkadotConfig;

// Implements the API provider for the `pallet_balances` transaction module.
impl_api_provider!(
    TransactionApiProvider,
    TransactionApi,
    polkadot::tx().balances()
);

pub type TxHash = String;

/// Provides a high-level abstraction for submitting balance-related
/// transactions to the blockchain.
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
    /// Creates a new transaction object from a mnemonic phrase.
    ///
    /// # Arguments
    /// * `mnemonic` - A valid 12/24-word mnemonic phrase.
    ///
    /// # Example
    /// ```
    /// let tx = StorageTransaction::from_mnemonic("your mnemonic words here");
    /// ```
    pub fn from_mnemonic(mnemonic: &str) -> Self {
        let pair = PairS::from_string(mnemonic, None).unwrap();
        let boxed: AnySigner = Box::new(PairSigner::<PolkadotConfig, _>::new(pair));
        Self {
            signer: DynSigner::new(boxed),
        }
    }

    /// Creates a transaction object from an existing signer instance.
    pub fn with_signer(signer: AnySigner) -> Self {
        Self {
            signer: DynSigner::new(signer),
        }
    }

    /// Creates a new transaction signer using a Ledger hardware wallet.
    ///
    /// # Arguments
    /// * `derivation_path` - The derivation path to use on the Ledger device.
    ///
    /// # Returns
    /// Returns an [`Error`] if the Ledger device is not connected or
    /// fails to initialize.
    pub fn from_ledger(derivation_path: &str) -> Result<Self, Error> {
        let ledger = LedgerSigner::new(derivation_path)?;
        let boxed: AnySigner = Box::new(ledger);
        Ok(Self {
            signer: DynSigner::new(boxed),
        })
    }

    /// Transfers a specified amount to another account.
    ///
    /// # Arguments
    /// * `account` - Destination account (SS58 string format).
    /// * `amount` - Amount to transfer in base units.
    ///
    /// # Returns
    /// On success, returns the transaction hash and the emitted [`Transfer`] event.
    ///
    /// # Example
    /// ```
    /// let result = tx.transfer("5Grwva...", 1_000_000_000_000).await?;
    /// ```
    pub async fn transfer(&self, account: &str, amount: u128) -> Result<(TxHash, Transfer), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.transfer_allow_death(subxt::utils::MultiAddress::Id(account), amount);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<Transfer>(event)
    }
}
