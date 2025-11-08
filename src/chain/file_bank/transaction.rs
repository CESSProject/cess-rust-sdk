//! # File Bank Transaction Module
//!
//! This module defines functions for performing extrinsics related to the
//! `pallet_file_bank` runtime pallet.  
//!
//! It provides methods for uploading files, reporting transfers,
//! certifying idle space, and handling file restoration orders.
//!
//! Each call signs and submits a transaction to the blockchain, then
//! listens for success events emitted by the chain.

use crate::chain::{AnySigner, Call, Chain, DynSigner};
use crate::core::{ApiProvider, Error};
use crate::impl_api_provider;
use crate::polkadot::{
    self,
    file_bank::calls::{
        types::replace_idle_space::{IdleSigInfo, TeePuk, TeeSig, TeeSigNeedVerify},
        TransactionApi,
    },
    file_bank::events::{
        CalculateReport, ClaimRestoralOrder, DeleteFile, GenerateRestoralOrder, IdleSpaceCert,
        RecoveryCompleted, ReplaceIdleSpace, TerritoryFileDelivery, TransferReport,
        UploadDeclaration,
    },
    runtime_types::bounded_collections::bounded_vec::BoundedVec,
    runtime_types::pallet_file_bank::types::{DigestInfo, SegmentList, TagSigInfo, UserBrief},
};
use crate::utils::hash_from_string;
use std::str::FromStr;
use subxt::ext::sp_core::{sr25519::Pair as PairS, Pair};
use subxt::ext::subxt_core::utils::AccountId32;
use subxt::tx::PairSigner;
use subxt::PolkadotConfig;

// Implements the API provider for the `pallet_file_bank` transaction module.
impl_api_provider!(
    TransactionApiProvider,
    TransactionApi,
    polkadot::tx().file_bank()
);

pub type TxHash = String;
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

    /// Submits an `upload_declaration` transaction.
    ///
    /// Declares a new file upload with its hash, segment list, owner info,
    /// and file size.
    pub async fn upload_declaration(
        &self,
        file_hash: &str,
        segment_list: BoundedVec<SegmentList>,
        user_brief: UserBrief,
        file_size: u128,
    ) -> Result<(TxHash, UploadDeclaration), Error> {
        let api = Self::get_api();
        let file_hash = hash_from_string(file_hash)?;
        let tx = api.upload_declaration(file_hash, segment_list, user_brief, file_size);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<UploadDeclaration>(event)
    }

    /// Initiates delivery of a file to a specific target territory.
    pub async fn territory_file_delivery(
        &self,
        account: &str,
        file_hash: &str,
        target_territory: &str,
    ) -> Result<(TxHash, TerritoryFileDelivery), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let file_hash = hash_from_string(file_hash)?;
        let target_territory = target_territory.as_bytes().to_vec();
        let tx = api.territory_file_delivery(account, file_hash, BoundedVec(target_territory));
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<TerritoryFileDelivery>(event)
    }

    /// Submits a report for a completed file transfer.
    pub async fn transfer_report(
        &self,
        index: u8,
        deal_hash: &str,
    ) -> Result<(TxHash, TransferReport), Error> {
        let api = Self::get_api();
        let deal_hash = hash_from_string(deal_hash)?;
        let tx = api.transfer_report(index, deal_hash);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<TransferReport>(event)
    }

    /// Submits a report calculation proof for a given file hash.
    pub async fn calculate_report(
        &self,
        tee_sig: &str,
        account: &str,
        digest: BoundedVec<DigestInfo>,
        file_hash: &str,
    ) -> Result<(TxHash, CalculateReport), Error> {
        let api = Self::get_api();
        let tee_sig = tee_sig.as_bytes().to_vec();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let file_hash = hash_from_string(file_hash)?;
        let tag_sig_info = TagSigInfo {
            miner: account,
            digest,
            file_hash,
        };
        let tx = api.calculate_report(BoundedVec(tee_sig), tag_sig_info);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<CalculateReport>(event)
    }

    /// Replaces idle storage space after verification.
    pub async fn replace_idle_space(
        &self,
        idle_sig_info: IdleSigInfo,
        tee_sig_need_verify: TeeSigNeedVerify,
        tee_sig: TeeSig,
        tee_puk: TeePuk,
    ) -> Result<(TxHash, ReplaceIdleSpace), Error> {
        let api = Self::get_api();
        let tx = api.replace_idle_space(idle_sig_info, tee_sig_need_verify, tee_sig, tee_puk);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<ReplaceIdleSpace>(event)
    }

    /// Deletes a file from the blockchain’s storage index.
    pub async fn delete_file(
        &self,
        account: &str,
        file_hash: &str,
    ) -> Result<(TxHash, DeleteFile), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let file_hash = hash_from_string(file_hash)?;
        let tx = api.delete_file(account, file_hash);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<DeleteFile>(event)
    }

    /// Certifies the idle space of a storage node using TEE verification.
    pub async fn cert_idle_space(
        &self,
        idle_sig_info: IdleSigInfo,
        tee_sig_need_verify: TeeSigNeedVerify,
        tee_sig: TeeSig,
        tee_puk: TeePuk,
    ) -> Result<(TxHash, IdleSpaceCert), Error> {
        let api = Self::get_api();
        let tx = api.cert_idle_space(idle_sig_info, tee_sig_need_verify, tee_sig, tee_puk);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<IdleSpaceCert>(event)
    }

    /// Generates a new restoral order for a specific file fragment.
    pub async fn generate_restoral_order(
        &self,
        file_hash: &str,
        restoral_fragment: &str,
    ) -> Result<(TxHash, GenerateRestoralOrder), Error> {
        let api = Self::get_api();
        let file_hash = hash_from_string(file_hash)?;
        let restoral_fragment = hash_from_string(restoral_fragment)?;
        let tx = api.generate_restoral_order(file_hash, restoral_fragment);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<GenerateRestoralOrder>(event)
    }

    /// Claims an existing restoral order for a fragment.
    pub async fn claim_restoral_order(
        &self,
        restoral_fragment: &str,
    ) -> Result<(TxHash, ClaimRestoralOrder), Error> {
        let api = Self::get_api();
        let restoral_fragment = hash_from_string(restoral_fragment)?;
        let tx = api.claim_restoral_order(restoral_fragment);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<ClaimRestoralOrder>(event)
    }

    /// Claims a restoral order that does not yet exist on-chain.
    pub async fn claim_restoral_noexist_order(
        &self,
        account: &str,
        file_hash: &str,
        restoral_fragment: &str,
    ) -> Result<(TxHash, ClaimRestoralOrder), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let file_hash = hash_from_string(file_hash)?;
        let restoral_fragment = hash_from_string(restoral_fragment)?;
        let tx = api.claim_restoral_noexist_order(account, file_hash, restoral_fragment);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<ClaimRestoralOrder>(event)
    }

    /// Marks a restoral order as completed.
    pub async fn restoral_order_complete(
        &self,
        fragment_hash: &str,
    ) -> Result<(TxHash, RecoveryCompleted), Error> {
        let api = Self::get_api();
        let fragment_hash = hash_from_string(fragment_hash)?;
        let tx = api.restoral_order_complete(fragment_hash);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<RecoveryCompleted>(event)
    }
}
