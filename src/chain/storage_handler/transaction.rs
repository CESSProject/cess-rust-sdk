use crate::chain::{Call, Chain};
use crate::core::{ApiProvider, Error};
use crate::impl_api_provider;
use crate::polkadot::storage_handler::calls::types::exec_order::OrderId;
use crate::polkadot::storage_handler::events::PaidOrder;
use crate::polkadot::{
    self,
    runtime_types::bounded_collections::bounded_vec::BoundedVec,
    runtime_types::pallet_storage_handler::types::OrderType,
    storage_handler::calls::TransactionApi,
    storage_handler::events::{
        BuyConsignment, CancelPurchaseAction, CancleConsignment, Consignment, CreatePayOrder,
        ExpansionTerritory, MintTerritory, ReactivateTerritory, RenewalTerritory,
    },
};
use crate::H256;
use std::str::FromStr;
use subxt::ext::sp_core::{sr25519::Pair as PairS, Pair};
use subxt::ext::subxt_core::utils::AccountId32;
use subxt::tx::PairSigner;
use subxt::PolkadotConfig;

// impl ApiProvider for TransactionApiProvider
impl_api_provider!(
    TransactionApiProvider,
    TransactionApi,
    polkadot::tx().storage_handler()
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

    pub async fn mint_territory(
        &self,
        gib_count: u32,
        territory_name: &str,
        days: u32,
    ) -> Result<(TxHash, MintTerritory), Error> {
        let api = Self::get_api();
        let territory_name = territory_name.as_bytes().to_vec();

        if days < 30 {
            return Err("Invalid input: The number of days must be 30 or more.".into());
        }

        let tx = api.mint_territory(gib_count, BoundedVec(territory_name), days);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<MintTerritory>(event)
    }

    pub async fn expand_territory(
        &self,
        territory_name: &str,
        gib_count: u32,
    ) -> Result<(TxHash, ExpansionTerritory), Error> {
        let api = Self::get_api();
        let territory_name = territory_name.as_bytes().to_vec();
        let tx = api.expanding_territory(BoundedVec(territory_name), gib_count);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<ExpansionTerritory>(event)
    }

    pub async fn renew_territory(
        &self,
        territory_name: &str,
        days: u32,
    ) -> Result<(TxHash, RenewalTerritory), Error> {
        let api = Self::get_api();
        let territory_name = territory_name.as_bytes().to_vec();
        let tx = api.renewal_territory(BoundedVec(territory_name), days);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<RenewalTerritory>(event)
    }

    pub async fn reactivate_territory(
        &self,
        territory_name: &str,
        days: u32,
    ) -> Result<(TxHash, ReactivateTerritory), Error> {
        let api = Self::get_api();
        let territory_name = territory_name.as_bytes().to_vec();
        let tx = api.reactivate_territory(BoundedVec(territory_name), days);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<ReactivateTerritory>(event)
    }

    pub async fn territory_consignment(
        &self,
        territory_name: &str,
        price: u128,
    ) -> Result<(TxHash, Consignment), Error> {
        let api = Self::get_api();
        let territory_name = territory_name.as_bytes().to_vec();
        let tx = api.territory_consignment(BoundedVec(territory_name), price);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<Consignment>(event)
    }

    pub async fn buy_consignment(
        &self,
        token: &str,
        rename: &str,
    ) -> Result<(TxHash, BuyConsignment), Error> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let rename = rename.as_bytes().to_vec();
        let tx = api.buy_consignment(token, BoundedVec(rename));
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<BuyConsignment>(event)
    }

    pub async fn cancel_consignment(
        &self,
        territory_name: &str,
    ) -> Result<(TxHash, CancleConsignment), Error> {
        let api = Self::get_api();
        let territory_name = territory_name.as_bytes().to_vec();
        let tx = api.cancel_consignment(BoundedVec(territory_name));
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<CancleConsignment>(event)
    }

    pub async fn cancel_purchase_action(
        &self,
        token: &str,
    ) -> Result<(TxHash, CancelPurchaseAction), Error> {
        let api = Self::get_api();
        let token = H256::from_str(token).unwrap();
        let tx = api.cancel_purchase_action(token);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<CancelPurchaseAction>(event)
    }

    pub async fn territory_grants(
        &self,
        territory_name: &str,
        receiver: &str,
    ) -> Result<TxHash, Error> {
        let api = Self::get_api();
        let territory_name = territory_name.as_bytes().to_vec();
        let receiver = AccountId32::from_str(receiver).map_err(|e| Error::Custom(e.to_string()))?;
        let tx = api.territory_grants(BoundedVec(territory_name), receiver);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;
        let hash = event.extrinsic_hash();
        Ok(format!("0x{}", hex::encode(hash.0)))
    }

    pub async fn territory_rename(
        &self,
        old_territory_name: &str,
        new_territory_name: &str,
    ) -> Result<TxHash, Error> {
        let api = Self::get_api();

        let old_territory_name = if old_territory_name.starts_with("0x") {
            hex::decode(
                old_territory_name
                    .strip_prefix("0x")
                    .unwrap_or(old_territory_name),
            )?
        } else {
            old_territory_name.as_bytes().to_vec()
        };
        let new_territory_name = new_territory_name.as_bytes().to_vec();

        // Warning! Currently if user sends hex string this will throw error,
        // This happens when the user territory name is a hex string.
        // This is handeled above for now.
        // but the core code should not set the hex string as
        // default territory name during territory_grants
        let tx = api.territory_rename(
            BoundedVec(old_territory_name),
            BoundedVec(new_territory_name),
        );
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;
        let hash = event.extrinsic_hash();
        Ok(format!("0x{}", hex::encode(hash.0)))
    }

    pub async fn create_order(
        &self,
        target_acc: &str,
        territory_name: &str,
        order_type: OrderType,
        gib_count: u32,
        days: u32,
        expired: u32,
    ) -> Result<(TxHash, CreatePayOrder), Error> {
        let api = Self::get_api();
        let target_acc =
            AccountId32::from_str(target_acc).map_err(|e| Error::Custom(e.to_string()))?;
        let territory_name = territory_name.as_bytes().to_vec();
        let tx = api.create_order(
            target_acc,
            BoundedVec(territory_name),
            order_type,
            gib_count,
            days,
            expired,
        );
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<CreatePayOrder>(event)
    }

    pub async fn exec_order(&self, order_id: OrderId) -> Result<(TxHash, PaidOrder), Error> {
        let api = Self::get_api();
        let tx = api.exec_order(order_id);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<PaidOrder>(event)
    }
}
