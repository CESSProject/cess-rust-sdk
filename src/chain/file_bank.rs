use super::Sdk;
use crate::utils::{account_from_slice, hex_string_to_bytes, query_storage, sign_and_submit_tx};
use crate::{init_api, polkadot};
use anyhow::{anyhow, bail, Result};
use log::info;
use polkadot::{
    file_bank::events::{CreateBucket, DeleteBucket},
    runtime_types::{
        cp_cess_common::Hash as CPHash,
        pallet_file_bank::types::{
            BucketInfo, DealInfo, FileInfo, RestoralOrderInfo, RestoralTargetInfo,
            UserFileSliceInfo,
        },
        sp_core::bounded::bounded_vec::BoundedVec,
    },
};
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

impl Sdk {
    /* Query functions */
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
        let account = account_from_slice(pk);

        let query = polkadot::storage()
            .file_bank()
            .user_hold_file_list(&account);

        let result = query_storage(&query).await;

        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_pending_replacements
    pub async fn query_pending_replacements(&self, pk: &[u8]) -> Result<u128> {
        let account = account_from_slice(pk);

        let query = polkadot::storage()
            .file_bank()
            .pending_replacements(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_invalid_file
    pub async fn query_invalid_file(&self, pk: &[u8]) -> Result<BoundedVec<CPHash>> {
        let account = account_from_slice(pk);

        let query = polkadot::storage().file_bank().invalid_file(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_miner_lock
    pub async fn query_miner_lock(&self, pk: &[u8]) -> Result<u32> {
        let account = account_from_slice(pk);

        let query = polkadot::storage().file_bank().miner_lock(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_bucket_info
    pub async fn query_bucket_info(&self, pk: &[u8], bucket_name: &str) -> Result<BucketInfo> {
        let account = account_from_slice(pk);

        let name_bytes: BoundedVec<u8> = BoundedVec(bucket_name.as_bytes().to_vec());

        let query = polkadot::storage().file_bank().bucket(&account, name_bytes);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_user_bucket_list
    pub async fn query_user_bucket_list(&self, pk: &[u8]) -> Result<BoundedVec<BoundedVec<u8>>> {
        let account = account_from_slice(pk);

        let query = polkadot::storage().file_bank().user_bucket_list(&account);

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

    // query_restoral_target
    pub async fn query_restoral_target(
        &self,
        pk: &[u8],
    ) -> Result<RestoralTargetInfo<AccountId32, u32>> {
        let account = account_from_slice(pk);

        let query = polkadot::storage().file_bank().restoral_target(&account);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_restoral_order
    pub async fn query_restoral_order(&self, hash: &str) -> Result<RestoralOrderInfo> {
        let hash_bytes = hex_string_to_bytes(hash);
        let hash = CPHash(hash_bytes);

        let query = polkadot::storage().file_bank().restoral_order(&hash);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_clear_user_list
    pub async fn query_clear_user_list(&self) -> Result<BoundedVec<AccountId32>> {
        let query = polkadot::storage().file_bank().clear_user_list();

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /* Transactional functions */
    // // upload_declaration
    // pub async fn upload_declaration(&self, file_hash: &str, deal_info: SegmentList, user: UserBrief, file_size: u64) -> Result<String> {
    //     Ok("".to_string())
    // }

    // create_bucket
    pub async fn create_bucket(&self, pk: &[u8], name: &str) -> Result<String> {
        let account = account_from_slice(pk);

        let name_bytes: BoundedVec<u8> = BoundedVec(name.as_bytes().to_vec());

        let tx = polkadot::tx()
            .file_bank()
            .create_bucket(account, name_bytes);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx(&tx, &from).await?;

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
        let account = account_from_slice(pk);

        let name_bytes: BoundedVec<u8> = BoundedVec(name.as_bytes().to_vec());

        let tx = polkadot::tx()
            .file_bank()
            .delete_bucket(account, name_bytes);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(delete_bucket_event) = events.find_first::<DeleteBucket>()? {
            info!("Bucket seleted: {:?}", delete_bucket_event);
            return Ok(tx_hash);
        } else {
            bail!("Unable to delete bucket");
        }
    }

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
    const MNEMONIC: &str =
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    #[tokio::test]
    async fn test_query_bucket_info() {
        let sdk = Sdk::new(MNEMONIC, "service_name");
        let pk = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let pk_bytes = hex::decode(pk).unwrap();
        let name = "MyFirstBucket";
        let result = sdk.query_bucket_info(&pk_bytes, name).await;
        match result {
            Ok(v) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_query_all_bucket_name() {
        let sdk = Sdk::new(MNEMONIC, "service_name");
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
    async fn test_query_user_bucket_list() {
        let sdk = Sdk::new(MNEMONIC, "service_name");
        let pk = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let pk_bytes = hex::decode(pk).unwrap();
        let result = sdk.query_user_bucket_list(&pk_bytes).await;
        match result {
            Ok(v) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_create_bucket() {
        let sdk = Sdk::new(MNEMONIC, "service_name");
        let pk = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let pk_bytes = hex::decode(pk).unwrap();
        let name = "MyFirstBucket";
        let result = sdk.create_bucket(&pk_bytes, name).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_delete_bucket() {
        let sdk = Sdk::new(MNEMONIC, "service_name");
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
