use crate::chain::{Call, Chain};
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

// impl ApiProvider for TransactionApiProvider
impl_api_provider!(
    TransactionApiProvider,
    TransactionApi,
    polkadot::tx().file_bank()
);

pub type TxHash = String;
pub struct StorageTransaction {
    pair: PairS,
}

impl Chain for StorageTransaction {}

impl Call for StorageTransaction {
    type Api = TransactionApi;

    fn get_api() -> Self::Api {
        crate::core::get_api::<TransactionApiProvider>()
    }

    fn get_pair_signer(&self) -> PairSigner<PolkadotConfig, PairS> {
        PairSigner::new(self.pair.clone())
    }
}

impl StorageTransaction {
    pub fn new(mnemonic: &str) -> Self {
        let pair = PairS::from_string(mnemonic, None).unwrap();
        Self { pair }
    }

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
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<UploadDeclaration>(event)
    }

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
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<TerritoryFileDelivery>(event)
    }

    pub async fn transfer_report(
        &self,
        index: u8,
        deal_hash: &str,
    ) -> Result<(TxHash, TransferReport), Error> {
        let api = Self::get_api();
        let deal_hash = hash_from_string(deal_hash)?;
        let tx = api.transfer_report(index, deal_hash);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<TransferReport>(event)
    }

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
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<CalculateReport>(event)
    }

    pub async fn replace_idle_space(
        &self,
        idle_sig_info: IdleSigInfo,
        tee_sig_need_verify: TeeSigNeedVerify,
        tee_sig: TeeSig,
        tee_puk: TeePuk,
    ) -> Result<(TxHash, ReplaceIdleSpace), Error> {
        let api = Self::get_api();
        let tx = api.replace_idle_space(idle_sig_info, tee_sig_need_verify, tee_sig, tee_puk);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<ReplaceIdleSpace>(event)
    }

    pub async fn delete_file(
        &self,
        account: &str,
        file_hash: &str,
    ) -> Result<(TxHash, DeleteFile), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let file_hash = hash_from_string(file_hash)?;
        let tx = api.delete_file(account, file_hash);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<DeleteFile>(event)
    }

    pub async fn cert_idle_space(
        &self,
        idle_sig_info: IdleSigInfo,
        tee_sig_need_verify: TeeSigNeedVerify,
        tee_sig: TeeSig,
        tee_puk: TeePuk,
    ) -> Result<(TxHash, IdleSpaceCert), Error> {
        let api = Self::get_api();
        let tx = api.cert_idle_space(idle_sig_info, tee_sig_need_verify, tee_sig, tee_puk);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<IdleSpaceCert>(event)
    }

    pub async fn generate_restoral_order(
        &self,
        file_hash: &str,
        restoral_fragment: &str,
    ) -> Result<(TxHash, GenerateRestoralOrder), Error> {
        let api = Self::get_api();
        let file_hash = hash_from_string(file_hash)?;
        let restoral_fragment = hash_from_string(restoral_fragment)?;
        let tx = api.generate_restoral_order(file_hash, restoral_fragment);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<GenerateRestoralOrder>(event)
    }

    pub async fn claim_restoral_order(
        &self,
        restoral_fragment: &str,
    ) -> Result<(TxHash, ClaimRestoralOrder), Error> {
        let api = Self::get_api();
        let restoral_fragment = hash_from_string(restoral_fragment)?;
        let tx = api.claim_restoral_order(restoral_fragment);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<ClaimRestoralOrder>(event)
    }

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
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<ClaimRestoralOrder>(event)
    }

    pub async fn restoral_order_complete(
        &self,
        fragment_hash: &str,
    ) -> Result<(TxHash, RecoveryCompleted), Error> {
        let api = Self::get_api();
        let fragment_hash = hash_from_string(fragment_hash)?;
        let tx = api.restoral_order_complete(fragment_hash);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<RecoveryCompleted>(event)
    }
}
