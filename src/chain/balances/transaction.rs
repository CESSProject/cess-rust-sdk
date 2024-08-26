use crate::chain::{Call, Chain};
use crate::core::ApiProvider;
use crate::impl_api_provider;
use crate::polkadot::balances::events::Transfer;
use crate::polkadot::{self, balances::calls::TransactionApi};
// use crate::utils::hash_from_string;
use std::str::FromStr;
use subxt::ext::sp_core::{sr25519::Pair as PairS, Pair};
use subxt::ext::subxt_core::utils::AccountId32;
use subxt::tx::PairSigner;
use subxt::PolkadotConfig;

// impl ApiProvider for TransactionApiProvider
impl_api_provider!(
    TransactionApiProvider,
    TransactionApi,
    polkadot::tx().balances()
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

    pub async fn transfer(
        &self,
        account: &str,
        amount: u128,
    ) -> Result<(TxHash, Transfer), Box<dyn std::error::Error>> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account)?;
        let tx = api.transfer_allow_death(subxt::utils::MultiAddress::Id(account), amount);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<Transfer>(event)
    }
}
