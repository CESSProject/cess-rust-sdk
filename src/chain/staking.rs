use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use polkadot::{
    runtime_types::{
        pallet_cess_staking::{
            slashing::SlashingSpans, ActiveEraInfo, EraRewardPoints, Exposure, Forcing,
            Nominations, RewardDestination, StakingLedger, ValidatorPrefs,
        },
        sp_arithmetic::per_things::Perbill,
        sp_core::bounded::bounded_vec::BoundedVec,
    },
    staking::{
        calls::TransactionApi,
        // events::{IncreaseCollateral, UpdatePeerId},
        storage::StorageApi,
    },
};
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn staking_storage() -> StorageApi {
    polkadot::storage().staking()
}

fn staking_tx() -> TransactionApi {
    polkadot::tx().staking()
}

impl Sdk {
    /* Query functions */
    // query_validator_count
    pub async fn query_validator_count(&self) -> Result<u32> {
        let query = staking_storage().validator_count();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_minimum_validator_count
    pub async fn query_minimum_validator_count(&self) -> Result<u32> {
        let query = staking_storage().minimum_validator_count();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_invulnerables
    pub async fn query_invulnerables(&self) -> Result<Vec<AccountId32>> {
        let query = staking_storage().invulnerables();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_bonded
    pub async fn query_bonded(&self, pk: &[u8]) -> Result<AccountId32> {
        let account = account_from_slice(pk);

        let query = staking_storage().bonded(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_ledger
    pub async fn query_ledger(&self, pk: &[u8]) -> Result<StakingLedger> {
        let account = account_from_slice(pk);

        let query = staking_storage().ledger(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_payee
    pub async fn query_payee(&self, pk: &[u8]) -> Result<RewardDestination<AccountId32>> {
        let account = account_from_slice(pk);

        let query = staking_storage().payee(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_validators
    pub async fn query_validators(&self, pk: &[u8]) -> Result<ValidatorPrefs> {
        let account = account_from_slice(pk);

        let query = staking_storage().validators(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_nominators
    pub async fn query_nominators(&self, pk: &[u8]) -> Result<Nominations> {
        let account = account_from_slice(pk);

        let query = staking_storage().nominators(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_current_era
    pub async fn query_current_era(&self) -> Result<u32> {
        let query = staking_storage().current_era();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_active_era
    pub async fn query_active_era(&self) -> Result<ActiveEraInfo> {
        let query = staking_storage().active_era();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_eras_start_session_index
    pub async fn query_eras_start_session_index(&self, era_index: u32) -> Result<u32> {
        let query = staking_storage().eras_start_session_index(era_index);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_eras_stakers
    pub async fn query_eras_stakers(
        &self,
        era_index: u32,
        pk: &[u8],
    ) -> Result<Exposure<AccountId32, u128>> {
        let account = account_from_slice(pk);
        let query = staking_storage().eras_stakers(era_index, &account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_eras_stakers_clipped
    pub async fn query_eras_stakers_clipped(
        &self,
        era_index: u32,
        pk: &[u8],
    ) -> Result<Exposure<AccountId32, u128>> {
        let account = account_from_slice(pk);
        let query = staking_storage().eras_stakers_clipped(era_index, &account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_eras_validator_prefs
    pub async fn query_eras_validator_prefs(
        &self,
        era_index: u32,
        pk: &[u8],
    ) -> Result<ValidatorPrefs> {
        let account = account_from_slice(pk);
        let query = staking_storage().eras_validator_prefs(era_index, &account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_eras_validator_reward
    pub async fn query_eras_validator_reward(&self, era_index: u32) -> Result<u128> {
        let query = staking_storage().eras_validator_reward(era_index);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_eras_reward_points
    pub async fn query_eras_reward_points(
        &self,
        era_index: u32,
    ) -> Result<EraRewardPoints<AccountId32>> {
        let query = staking_storage().eras_reward_points(era_index);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_eras_total_stake
    pub async fn query_eras_total_stake(&self, era_index: u32) -> Result<u128> {
        let query = staking_storage().eras_total_stake(era_index);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_force_era
    pub async fn query_force_era(&self) -> Result<Forcing> {
        let query = staking_storage().force_era();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_slash_reward_fraction
    pub async fn query_slash_reward_fraction(&self) -> Result<Perbill> {
        let query = staking_storage().slash_reward_fraction();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_canceled_slash_payout
    pub async fn query_canceled_slash_payout(&self) -> Result<u128> {
        let query = staking_storage().canceled_slash_payout();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_slashing_spans
    pub async fn query_slashing_spans(&self, pk: &[u8]) -> Result<SlashingSpans> {
        let account = account_from_slice(pk);

        let query = staking_storage().slashing_spans(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_current_planned_session
    pub async fn query_current_planned_session(&self) -> Result<u32> {
        let query = staking_storage().current_planned_session();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_offending_validators
    pub async fn query_offending_validators(&self) -> Result<Vec<(u32, bool)>> {
        let query = staking_storage().offending_validators();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}
