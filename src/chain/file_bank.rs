use std::arch::x86_64::CpuidResult;

use super::Sdk;
use crate::utils::{hex_string_to_bytes, query_storage};
use crate::{init_api, polkadot};
use anyhow::{anyhow, bail, Error, Result};
use log::{info, warn};
use polkadot::{
    file_bank::events::{CreateBucket, DeleteBucket},
    runtime_types::{
        cp_cess_common::Hash as CPHash,
        pallet_file_bank::types::{
            BucketInfo, DealInfo, FileInfo, RestoralTargetInfo, RestoralOrderInfo, UserFileSliceInfo,
        },
        sp_core::bounded::bounded_vec::BoundedVec,
    },
};
use sp_keyring::AccountKeyring;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

impl Sdk {
    // query_storage_order
    pub async fn query_storage_order(&self, root_hash: &str) -> Result<DealInfo> {
        let hash_bytes = hex_string_to_bytes(root_hash);
        let hash = CPHash(hash_bytes);
        let query = polkadot::storage().file_bank().deal_map(hash);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_file_metadata
    pub async fn query_file_metadata(&self, root_hash: &str) -> Result<FileInfo> {
        let hash_bytes = hex_string_to_bytes(root_hash);
        let hash = CPHash(hash_bytes);
        let query = polkadot::storage().file_bank().file(hash);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_user_hold_file_list
    pub async fn query_user_hold_file_list(
        &self,
        pk: &[u8],
    ) -> Result<BoundedVec<UserFileSliceInfo>> {
        let account =
            AccountKeyring::from_raw_public(pk.try_into().expect("Invalid slice length")).unwrap();

        let query = polkadot::storage()
            .file_bank()
            .user_hold_file_list(&account.to_account_id().into());

        let result = query_storage(&query).await;

        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_pending_replacements
    pub async fn query_pending_replacements(&self, pk: &[u8]) -> Result<u128> {
        let account =
            AccountKeyring::from_raw_public(pk.try_into().expect("Invalid slice length")).unwrap();

        let query = polkadot::storage()
            .file_bank()
            .pending_replacements(&account.to_account_id().into());

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_invalid_file
    pub async fn query_invalid_file(&self, pk: &[u8]) -> Result<BoundedVec<CPHash>> {
        let account =
            AccountKeyring::from_raw_public(pk.try_into().expect("Invalid slice length")).unwrap();

        let query = polkadot::storage()
            .file_bank()
            .invalid_file(&account.to_account_id().into());

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_miner_lock
    pub async fn query_miner_lock(&self, pk: &[u8]) -> Result<u32> {
        let account =
            AccountKeyring::from_raw_public(pk.try_into().expect("Invalid slice length")).unwrap();

        let query = polkadot::storage()
            .file_bank()
            .miner_lock(&account.to_account_id().into());

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_bucket_info
    pub async fn query_bucket_info(&self, pk: &[u8], bucket_name: &str) -> Result<BucketInfo> {
        let account =
            AccountKeyring::from_raw_public(pk.try_into().expect("Invalid slice length")).unwrap();

        let name_bytes: BoundedVec<u8> = BoundedVec(bucket_name.as_bytes().to_vec());

        let query = polkadot::storage()
            .file_bank()
            .bucket(&account.to_account_id().into(), name_bytes);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_user_bucket_list
    pub async fn query_user_bucket_list(&self, pk: &[u8]) -> Result<BoundedVec<BoundedVec<u8>>> {
        let account =
            AccountKeyring::from_raw_public(pk.try_into().expect("Invalid slice length")).unwrap();

        let query = polkadot::storage()
            .file_bank()
            .user_bucket_list(&account.to_account_id().into());

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_all_bucket_name
    pub async fn query_all_bucket_name(&self, pk: &[u8]) -> Result<Vec<String>> {
        match self.query_user_bucket_list(pk).await {
            Ok(bucketlist) => {
                let buckets: Vec<String> = bucketlist
                    .0
                    .iter()
                    .map(|v| String::from_utf8_lossy(&v.0).to_string())
                    .collect();
                Ok(buckets)
            }
            Err(e) => {
                bail!("{}", e);
            }
        }
    }

    pub async fn query_restoral_target(
        &self,
        pk: &[u8],
    ) -> Result<RestoralTargetInfo<AccountId32, u32>> {
        let account =
            AccountKeyring::from_raw_public(pk.try_into().expect("Invalid slice length")).unwrap();

        let query = polkadot::storage()
            .file_bank()
            .restoral_target(&account.to_account_id().into());

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_restoral_target_list(&self, hash: &str) -> Result<RestoralOrderInfo> {
 
        let hash_bytes = hex_string_to_bytes(hash);
        let hash = CPHash(hash_bytes);

        let query = polkadot::storage()
            .file_bank()
            .restoral_order(&hash);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub async fn query_restoral_order(&self) -> Result<()> {
        Ok(())
    }

    pub async fn query_restoral_order_list(&self) -> Result<()> {
        Ok(())
    }

    // create_bucket
    pub async fn create_bucket(&self, pk: &[u8], name: &str) -> Result<String> {
        let api = init_api().await?;
        let account =
            AccountKeyring::from_raw_public(pk.try_into().expect("Invalid slice length")).unwrap();

        let name_bytes: BoundedVec<u8> = BoundedVec(name.as_bytes().to_vec());

        let tx = polkadot::tx()
            .file_bank()
            .create_bucket(account.to_account_id().into(), name_bytes);
        let from = PairSigner::new(account.pair());

        let events = match api
            .tx()
            .sign_and_submit_then_watch_default(&tx, &from)
            .await
        {
            Ok(result) => match result.wait_for_finalized_success().await {
                Ok(r) => r,
                Err(e) => {
                    let err = anyhow!("Error waiting for finalized success: {:?}", e);
                    bail!("{}", err);
                }
            },
            Err(e) => {
                let err = anyhow!("Error signing and submitting transaction: {:?}", e);
                bail!("{}", err);
            }
        };

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(create_bucket_event) = events.find_first::<CreateBucket>()? {
            info!("Bucket created: {:?}", create_bucket_event);
            return Ok(tx_hash);
        } else {
            bail!("Unable to create bucket");
        }
    }

    // delete_bucket
    pub async fn delete_bucket(&self, pk: &[u8], name: &str) -> Result<String> {
        let api = init_api().await?;
        let account =
            AccountKeyring::from_raw_public(pk.try_into().expect("Invalid slice length")).unwrap();

        let name_bytes: BoundedVec<u8> = BoundedVec(name.as_bytes().to_vec());

        let delete_bucket_tx = polkadot::tx()
            .file_bank()
            .delete_bucket(account.to_account_id().into(), name_bytes);
        let from = PairSigner::new(account.pair());

        let events = match api
            .tx()
            .sign_and_submit_then_watch_default(&delete_bucket_tx, &from)
            .await
        {
            Ok(result) => match result.wait_for_finalized_success().await {
                Ok(r) => r,
                Err(e) => {
                    let err = anyhow!("Error waiting for finalized success: {:?}", e);
                    bail!("{}", err);
                }
            },
            Err(e) => {
                let err = anyhow!("Error signing and submitting transaction: {:?}", e);
                bail!("{}", err);
            }
        };

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(delete_bucket_event) = events.find_first::<DeleteBucket>()? {
            info!("Bucket seleted: {:?}", delete_bucket_event);
            return Ok(tx_hash);
        } else {
            bail!("Unable to delete bucket");
        }
    }

    // // upload_declaration
    // pub async fn upload_declaration(&self, file_hash: &str, deal_info: SegmentList, user: UserBrief, file_size: u64) -> Result<String> {
    //     Ok("".to_string())
    // }

    // delete_file
    pub async fn delete_file(&self, pk: &[u8], file_hash: Vec<String>) -> Result<String> {
        // Return type need to have two values: (string, FileHash)
        Ok("".to_string())
    }

    pub async fn delete_filler(&self, file_hash: &str) -> Result<String> {
        Ok("".to_string())
    }

    pub async fn submit_file_report(&self) -> Result<()> {
        Ok(())
    }
    pub async fn report_files(&self) -> Result<()> {
        Ok(())
    }
    pub async fn replace_idle_files(&self) -> Result<()> {
        Ok(())
    }
    pub async fn replace_file(&self) -> Result<()> {
        Ok(())
    }

    pub async fn generate_restoral_order(&self) -> Result<()> {
        Ok(())
    }
    pub async fn claim_restoral_order(&self) -> Result<()> {
        Ok(())
    }
    pub async fn claim_restoral_no_exist_order(&self) -> Result<()> {
        Ok(())
    }

    pub async fn restoral_complete(&self) -> Result<()> {
        Ok(())
    }
    pub async fn cert_idle_space(&self) -> Result<()> {
        Ok(())
    }
    pub async fn replace_idle_space(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::chain::Sdk;

    #[tokio::test]
    async fn test_query_bucket_info() {
        let sdk = Sdk::new("service_name");
        let pk = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let pk_bytes = hex::decode(pk).unwrap();
        let name = "MyFirstBucket";
        let result = sdk.query_bucket_info(&pk_bytes, name).await;
        match result {
            Ok(v) => {
                println!("{:?}", v);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_query_all_bucket_name() {
        let sdk = Sdk::new("service_name");
        let pk = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let pk_bytes = hex::decode(pk).unwrap();
        let result = sdk.query_all_bucket_name(&pk_bytes).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_query_bucket_list() {
        let sdk = Sdk::new("service_name");
        let pk = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let pk_bytes = hex::decode(pk).unwrap();
        let result = sdk.query_bucket_list(&pk_bytes).await;
        match result {
            Ok(v) => {
                println!("{:?}", v);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_create_bucket() {
        let sdk = Sdk::new("service_name");
        let pk = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let pk_bytes = hex::decode(pk).unwrap();
        let name = "MyFirstBucket";
        let result = sdk.create_bucket(&pk_bytes, name).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_delete_bucket() {
        let sdk = Sdk::new("service_name");
        let pk = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let pk_bytes = hex::decode(pk).unwrap();
        let name = "MyFirstBucket";
        let result = sdk.delete_bucket(&pk_bytes, name).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}