use crate::chain::{AnySigner, Call, Chain, DynSigner};
use crate::core::{ApiProvider, Error};
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

    pub async fn transfer(&self, account: &str, amount: u128) -> Result<(TxHash, Transfer), Error> {
        let api = Self::get_api();
        let account = AccountId32::from_str(account).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.transfer_allow_death(subxt::utils::MultiAddress::Id(account), amount);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<Transfer>(event)
    }
}
