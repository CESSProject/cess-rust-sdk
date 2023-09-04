use super::ChainSdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use async_trait::async_trait;
use polkadot::{
    runtime_types::{
        cp_cess_common::PoISKey,
        pallet_sminer::types::{MinerInfo, RestoralTargetInfo, Reward},
        sp_core::bounded::bounded_vec::BoundedVec,
    },
    sminer::{
        calls::TransactionApi,
        events::{IncreaseCollateral, MinerExitPrep},
        storage::StorageApi,
    },
};
use subxt::ext::sp_core::H256;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn sminer_storage() -> StorageApi {
    polkadot::storage().sminer()
}

fn sminer_tx() -> TransactionApi {
    polkadot::tx().sminer()
}

#[async_trait]
pub trait SMiner {
    async fn query_miner_items(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<MinerInfo>>;
    async fn query_all_miner(&self, block_hash: Option<H256>) -> Result<Option<BoundedVec<AccountId32>>>;
    async fn query_reward_map(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<Reward>>;
    async fn query_currency_reward(&self, block_hash: Option<H256>) -> Result<Option<u128>>;
    async fn query_miner_public_key(&self, slice: [u8; 32], block_hash: Option<H256>) -> Result<Option<AccountId32>>;
    async fn query_expenders(&self, block_hash: Option<H256>) -> Result<Option<(u64, u64, u64)>>;
    async fn query_miner_lock(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<u32>>;
    async fn query_restoral_target(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<RestoralTargetInfo<AccountId32, u32>>>;
    async fn regnstk(
        &self,
        beneficiary: &[u8],
        peer_id: [u8; 38],
        staking_val: u128,
        pois_key: PoISKey,
        tee_sig: [u8; 256],
    ) -> Result<String>;
    async fn increase_collateral(&self, collaterals: u128) -> Result<(String, IncreaseCollateral)>;
    async fn update_beneficiary(&self, beneficiary: &[u8]) -> Result<String>;
    async fn update_peer_id(&self, peer_id: [u8; 38]) -> Result<String>;
    async fn receive_reward(&self) -> Result<String>;
    async fn miner_exit_prep(&self) -> Result<(String, MinerExitPrep)>;
    async fn miner_exit(&self, miner: &[u8]) -> Result<String>;
    async fn miner_withdraw(&self) -> Result<String>;
}

#[async_trait]
impl SMiner for ChainSdk {
    /* Query functions */

    // query_miner_items
    async fn query_miner_items(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<MinerInfo>> {
        let account = account_from_slice(pk);

        let query = sminer_storage().miner_items(&account);

        query_storage(&query, block_hash).await
    }

    // query_all_miner
    async fn query_all_miner(&self, block_hash: Option<H256>) -> Result<Option<BoundedVec<AccountId32>>> {
        let query = sminer_storage().all_miner();

        query_storage(&query, block_hash).await
    }

    // query_reward_map
    async fn query_reward_map(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<Reward>> {
        let account = account_from_slice(pk);

        let query = sminer_storage().reward_map(&account);

        query_storage(&query, block_hash).await
    }

    // query_currency_reward
    async fn query_currency_reward(&self, block_hash: Option<H256>) -> Result<Option<u128>> {
        let query = sminer_storage().currency_reward();

        query_storage(&query, block_hash).await
    }

    // query_miner_public_key
    async fn query_miner_public_key(&self, slice: [u8; 32], block_hash: Option<H256>) -> Result<Option<AccountId32>> {
        let query = sminer_storage().miner_public_key(slice);

        query_storage(&query, block_hash).await
    }

    // query_expenders
    async fn query_expenders(&self, block_hash: Option<H256>) -> Result<Option<(u64, u64, u64)>> {
        let query = sminer_storage().expenders();

        query_storage(&query, block_hash).await
    }

    // query_miner_lock
    async fn query_miner_lock(&self, pk: &[u8], block_hash: Option<H256>) -> Result<Option<u32>> {
        let account = account_from_slice(pk);

        let query = sminer_storage().miner_lock(&account);

        query_storage(&query, block_hash).await
    }

    // query_restoral_target
    async fn query_restoral_target(
        &self,
        pk: &[u8],
        block_hash: Option<H256>,
    ) -> Result<Option<RestoralTargetInfo<AccountId32, u32>>> {
        let account = account_from_slice(pk);

        let query = sminer_storage().restoral_target(&account);

        query_storage(&query, block_hash).await
    }

    /* Transactional functions */
    // regnstk
    async fn regnstk(
        &self,
        beneficiary: &[u8],
        peer_id: [u8; 38],
        staking_val: u128,
        pois_key: PoISKey,
        tee_sig: [u8; 256],
    ) -> Result<String> {
        let account = account_from_slice(beneficiary);

        let tx = sminer_tx().regnstk(account, peer_id, staking_val, pois_key, tee_sig);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // increase_collateral
    async fn increase_collateral(&self, collaterals: u128) -> Result<(String, IncreaseCollateral)> {
        let tx = sminer_tx().increase_collateral(collaterals);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(collateral) = events.find_first::<IncreaseCollateral>()? {
            Ok((tx_hash, collateral))
        } else {
            bail!("Unable to increase collateral");
        }
    }

    // update_beneficiary
    async fn update_beneficiary(&self, beneficiary: &[u8]) -> Result<String> {
        let account = account_from_slice(beneficiary);

        let tx = sminer_tx().update_beneficiary(account);
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // update_peer_id
    async fn update_peer_id(&self, peer_id: [u8; 38]) -> Result<String> {
        let tx = sminer_tx().update_peer_id(peer_id);
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // receive_reward
    async fn receive_reward(&self) -> Result<String> {
        let tx = sminer_tx().receive_reward();
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn miner_exit_prep(&self) -> Result<(String, MinerExitPrep)> {
        let tx = sminer_tx().miner_exit_prep();

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(exit_prep) = events.find_first::<MinerExitPrep>()? {
            Ok((tx_hash, exit_prep))
        } else {
            bail!("Unable to execute miner exit prep");
        }
    }

    async fn miner_exit(&self, miner: &[u8]) -> Result<String> {
        let account = account_from_slice(miner);

        let tx = sminer_tx().miner_exit(account);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn miner_withdraw(&self) -> Result<String> {
        let tx = sminer_tx().miner_withdraw();

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
}
