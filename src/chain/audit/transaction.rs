use crate::chain::Call;
use crate::core::ApiProvider;
use crate::impl_api_provider;
use crate::polkadot::audit::calls::types::submit_verify_idle_result::Accumulator;
use crate::polkadot::{
    self,
    audit::calls::TransactionApi,
    audit::events::{
        SubmitIdleProof, SubmitIdleVerifyResult, SubmitServiceProof, SubmitServiceVerifyResult,
    },
    runtime_types::bounded_collections::bounded_vec::BoundedVec,
    runtime_types::cp_bloom_filter::BloomFilter,
};
use subxt::ext::sp_core::{sr25519::Pair as PairS, Pair};
use subxt::tx::PairSigner;
use subxt::PolkadotConfig;

// impl ApiProvider for TransactionApiProvider
impl_api_provider!(
    TransactionApiProvider,
    TransactionApi,
    polkadot::tx().audit()
);

pub type TxHash = String;
pub struct StorageTransaction {
    pair: PairS,
}

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

    pub async fn submit_idle_proof(
        &self,
        idle_prove: BoundedVec<u8>,
    ) -> Result<(TxHash, SubmitIdleProof), Box<dyn std::error::Error>> {
        let api = Self::get_api();

        let tx = api.submit_idle_proof(idle_prove);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<SubmitIdleProof>(event)
    }

    pub async fn submit_service_proof(
        &self,
        service_prove: BoundedVec<u8>,
    ) -> Result<(TxHash, SubmitServiceProof), Box<dyn std::error::Error>> {
        let api = Self::get_api();

        let tx = api.submit_service_proof(service_prove);
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<SubmitServiceProof>(event)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn submit_verify_idle_result(
        &self,
        total_prove_hash: BoundedVec<u8>,
        front: u64,
        rear: u64,
        accumulator: Accumulator,
        idle_result: bool,
        signature: BoundedVec<u8>,
        tee_puk: [u8; 32],
    ) -> Result<(TxHash, SubmitIdleVerifyResult), Box<dyn std::error::Error>> {
        let api = Self::get_api();

        let tx = api.submit_verify_idle_result(
            total_prove_hash,
            front,
            rear,
            accumulator,
            idle_result,
            signature,
            tee_puk,
        );
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<SubmitIdleVerifyResult>(event)
    }

    pub async fn submit_verify_service_result(
        &self,
        service_result: bool,
        signature: BoundedVec<u8>,
        service_bloom_filter: BloomFilter,
        tee_puk: [u8; 32],
    ) -> Result<(TxHash, SubmitServiceVerifyResult), Box<dyn std::error::Error>> {
        let api = Self::get_api();

        let tx = api.submit_verify_service_result(
            service_result,
            signature,
            service_bloom_filter,
            tee_puk,
        );
        let from = self.get_pair_signer();
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        Self::find_first::<SubmitServiceVerifyResult>(event)
    }
}
