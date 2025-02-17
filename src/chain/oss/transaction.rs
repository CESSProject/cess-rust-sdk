use crate::chain::{Call, Chain};
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

    pub async fn authorize(&self, account: &str) -> Result<(TxHash, Authorize), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.authorize(account);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<Authorize>(event)
    }

    pub async fn cancel_authorize(
        &self,
        account: &str,
    ) -> Result<(TxHash, CancelAuthorize), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.cancel_authorize(account);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<CancelAuthorize>(event)
    }

    pub async fn register(
        &self,
        endpoint: [u8; 38],
        domain: BoundedVec<u8>,
    ) -> Result<(TxHash, OssRegister), Error> {
        let api = Self::get_api();
        let tx = api.register(endpoint, domain);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<OssRegister>(event)
    }

    pub async fn update(
        &self,
        endpoint: [u8; 38],
        domain: BoundedVec<u8>,
    ) -> Result<(TxHash, OssUpdate), Error> {
        let api = Self::get_api();
        let tx = api.update(endpoint, domain);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<OssUpdate>(event)
    }

    pub async fn destroy(&self) -> Result<(TxHash, OssDestroy), Error> {
        let api = Self::get_api();
        let tx = api.destroy();
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<OssDestroy>(event)
    }

    pub async fn proxy_authorize(
        &self,
        account: &str,
        sig: Sig,
        payload: ProxyAuthPayload,
    ) -> Result<TxHash, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.proxy_authorzie(account.0, sig, payload);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;
        let hash = event.extrinsic_hash();
        Ok(format!("0x{}", hex::encode(hash.0)))
    }

    pub async fn evm_proxy_authorzie(
        &self,
        account: &str,
        sig: [u8; 65],
        payload: ProxyAuthPayload,
    ) -> Result<TxHash, Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.evm_proxy_authorzie(account.0, sig, payload);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;
        let hash = event.extrinsic_hash();
        Ok(format!("0x{}", hex::encode(hash.0)))
    }
}
