use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
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
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn sminer_storage() -> StorageApi {
    polkadot::storage().sminer()
}

fn sminer_tx() -> TransactionApi {
    polkadot::tx().sminer()
}

impl Sdk {
    /* Query functions */
    // query_miner_lock_in
    pub async fn query_miner_lock_in(&self, pk: &[u8]) -> Result<u32> {
        let account = account_from_slice(pk);

        let query = sminer_storage().miner_lock_in(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_miner_items
    pub async fn query_miner_items(&self, pk: &[u8]) -> Result<MinerInfo> {
        let account = account_from_slice(pk);

        let query = sminer_storage().miner_items(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_all_miner
    pub async fn query_all_miner(&self) -> Result<BoundedVec<AccountId32>> {
        let query = sminer_storage().all_miner();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_reward_map
    pub async fn query_reward_map(&self, pk: &[u8]) -> Result<Reward> {
        let account = account_from_slice(pk);

        let query = sminer_storage().reward_map(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_currency_reward
    pub async fn query_currency_reward(&self) -> Result<u128> {
        let query = sminer_storage().currency_reward();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_miner_public_key
    pub async fn query_miner_public_key(&self, slice: [u8; 32]) -> Result<AccountId32> {
        let query = sminer_storage().miner_public_key(slice);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_expenders
    pub async fn query_expenders(&self) -> Result<(u64, u64, u64)> {
        let query = sminer_storage().expenders();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_miner_lock
    pub async fn query_miner_lock(&self, pk: &[u8]) -> Result<u32> {
        let account = account_from_slice(pk);

        let query = sminer_storage().miner_lock(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_restoral_target
    pub async fn query_restoral_target(
        &self,
        pk: &[u8],
    ) -> Result<RestoralTargetInfo<AccountId32, u32>> {
        let account = account_from_slice(pk);

        let query = sminer_storage().restoral_target(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /* Transactional functions */
    // regnstk
    pub async fn regnstk(
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
    pub async fn increase_collateral(
        &self,
        collaterals: u128,
    ) -> Result<(String, IncreaseCollateral)> {
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
    pub async fn update_beneficiary(&self, beneficiary: &[u8]) -> Result<String> {
        let account = account_from_slice(beneficiary);

        let tx = sminer_tx().update_beneficiary(account);
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // update_peer_id
    pub async fn update_peer_id(&self, peer_id: [u8; 38]) -> Result<String> {
        let tx = sminer_tx().update_peer_id(peer_id);
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // receive_reward
    pub async fn receive_reward(&self) -> Result<String> {
        let tx = sminer_tx().receive_reward();
        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn miner_exit_prep(&self) -> Result<(String, MinerExitPrep)> {
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

    pub async fn miner_exit(&self, miner: &[u8]) -> Result<String> {
        let account = account_from_slice(miner);

        let tx = sminer_tx().miner_exit(account);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn miner_withdraw(&self) -> Result<String> {
        let tx = sminer_tx().miner_withdraw();

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn update_expender(&self, k: u64, n: u64, d: u64) -> Result<String> {
        let tx = sminer_tx().update_expender(k, n, d);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
}
