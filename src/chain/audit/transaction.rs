use crate::chain::{AnySigner, Call, Chain, DynSigner};
use crate::core::{ApiProvider, Error};
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

// Implements the API provider for the `pallet_audit` transaction module.
impl_api_provider!(
    TransactionApiProvider,
    TransactionApi,
    polkadot::tx().audit()
);

pub type TxHash = String;

/// Provides a transaction interface for the `pallet_audit` module.
///
/// This struct enables sending audit-related extrinsics (transactions) to the blockchain,
/// including proof submissions and verification results.
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
    /// Creates a new transaction client using a mnemonic phrase.
    ///
    /// # Arguments
    /// * `mnemonic` - The secret phrase used to derive a local signing keypair.
    ///
    /// # Returns
    /// A new [`StorageTransaction`] instance ready to send signed transactions.
    pub fn from_mnemonic(mnemonic: &str) -> Self {
        let pair = PairS::from_string(mnemonic, None).unwrap();
        let boxed: AnySigner = Box::new(PairSigner::<PolkadotConfig, _>::new(pair));
        Self {
            signer: DynSigner::new(boxed),
        }
    }

    /// Creates a new transaction client using a provided dynamic signer.
    pub fn with_signer(signer: AnySigner) -> Self {
        Self {
            signer: DynSigner::new(signer),
        }
    }

    /// Submits an idle proof to the blockchain.
    ///
    /// # Arguments
    /// * `idle_prove` - The encoded idle proof data.
    ///
    /// # Returns
    /// A tuple containing the transaction hash and the [`SubmitIdleProof`] event.
    pub async fn submit_idle_proof(
        &self,
        idle_prove: BoundedVec<u8>,
    ) -> Result<(TxHash, SubmitIdleProof), Error> {
        let api = Self::get_api();

        let tx = api.submit_idle_proof(idle_prove);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<SubmitIdleProof>(event)
    }

    /// Submits a service proof for verification.
    ///
    /// # Arguments
    /// * `service_prove` - The encoded service proof data.
    ///
    /// # Returns
    /// A tuple containing the transaction hash and the [`SubmitServiceProof`] event.
    pub async fn submit_service_proof(
        &self,
        service_prove: BoundedVec<u8>,
    ) -> Result<(TxHash, SubmitServiceProof), Error> {
        let api = Self::get_api();

        let tx = api.submit_service_proof(service_prove);
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<SubmitServiceProof>(event)
    }

    /// Submits a verification result for an idle proof.
    ///
    /// # Arguments
    /// * `total_prove_hash` - The hash of the total proof data.
    /// * `front`, `rear` - Numerical bounds related to the proof range.
    /// * `accumulator` - A Merkle accumulator structure for verification.
    /// * `idle_result` - The result of the idle proof check.
    /// * `signature` - The TEE-signed proof data.
    /// * `tee_puk` - The TEE public key.
    ///
    /// # Returns
    /// A tuple containing the transaction hash and the [`SubmitIdleVerifyResult`] event.
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
    ) -> Result<(TxHash, SubmitIdleVerifyResult), Error> {
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
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<SubmitIdleVerifyResult>(event)
    }

    /// Submits a verification result for a service proof.
    ///
    /// # Arguments
    /// * `service_result` - The result of the service proof check.
    /// * `signature` - The signed verification data.
    /// * `service_bloom_filter` - A Bloom filter structure containing proof data.
    /// * `tee_puk` - The TEE public key.
    ///
    /// # Returns
    /// A tuple containing the transaction hash and the [`SubmitServiceVerifyResult`] event.
    pub async fn submit_verify_service_result(
        &self,
        service_result: bool,
        signature: BoundedVec<u8>,
        service_bloom_filter: BloomFilter,
        tee_puk: [u8; 32],
    ) -> Result<(TxHash, SubmitServiceVerifyResult), Error> {
        let api = Self::get_api();

        let tx = api.submit_verify_service_result(
            service_result,
            signature,
            service_bloom_filter,
            tee_puk,
        );
        let event = Self::sign_and_submit_tx_then_watch_default(&tx, self.get_signer()).await?;

        Self::find_first::<SubmitServiceVerifyResult>(event)
    }
}
