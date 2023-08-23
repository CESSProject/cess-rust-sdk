use super::ChainSdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use async_trait::async_trait;
use polkadot::{
    file_bank::{
        calls::TransactionApi,
        events::{
            CalculateEnd, ClaimRestoralOrder, CreateBucket, DeleteBucket, DeleteFile,
            GenerateRestoralOrder, TransferReport, UploadDeclaration,
        },
        storage::StorageApi,
    },
    runtime_types::{
        cp_cess_common::{Hash as CPHash, SpaceProofInfo},
        pallet_file_bank::types::{
            BucketInfo, DealInfo, FileInfo, RestoralOrderInfo, SegmentList, UserBrief,
            UserFileSliceInfo,
        },
        sp_core::bounded::bounded_vec::BoundedVec,
    },
};
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;

fn file_bank_storage() -> StorageApi {
    polkadot::storage().file_bank()
}

fn file_bank_tx() -> TransactionApi {
    polkadot::tx().file_bank()
}
#[async_trait]
pub trait FileBank {
    async fn query_storage_order(&self, root_hash: &str) -> Result<Option<DealInfo>>;
    async fn query_file_metadata(&self, root_hash: &str) -> Result<Option<FileInfo>>;
    async fn query_user_hold_file_list(
        &self,
        pk: &[u8],
    ) -> Result<Option<BoundedVec<UserFileSliceInfo>>>;
    async fn query_pending_replacements(&self, pk: &[u8]) -> Result<Option<u128>>;
    async fn query_bucket_info(&self, pk: &[u8], bucket_name: &str) -> Result<Option<BucketInfo>>;
    async fn query_user_bucket_list(&self, pk: &[u8])
        -> Result<Option<BoundedVec<BoundedVec<u8>>>>;
    async fn query_all_bucket_name(&self, pk: &[u8]) -> Result<Vec<String>>;
    async fn query_restoral_order(&self, hash: &str) -> Result<Option<RestoralOrderInfo>>;
    async fn query_clear_user_list(&self) -> Result<Option<BoundedVec<AccountId32>>>;
    async fn upload_declaration(
        &self,
        file_hash: &str,
        deal_info: BoundedVec<SegmentList>,
        user: UserBrief,
        file_size: u128,
    ) -> Result<(String, UploadDeclaration)>;
    async fn deal_reassign_miner(&self, deal_hash: &str, count: u8, life: u32) -> Result<String>;
    async fn ownership_transfer(&self, target_brief: UserBrief, file_hash: &str) -> Result<String>;
    async fn transfer_report(&self, deal_hash: Vec<&str>) -> Result<(String, AccountId32)>;
    async fn calculate_end(&self, deal_hash: &str) -> Result<(String, CPHash)>;
    async fn replace_idle_space(
        &self,
        idle_sig_info: SpaceProofInfo<AccountId32>,
        sign: &[u8; 256],
    ) -> Result<String>;
    async fn delete_file(&self, pk: &[u8], file_hash: Vec<String>)
        -> Result<(String, Vec<CPHash>)>;
    async fn cert_idle_space(
        &self,
        idle_sig_info: SpaceProofInfo<AccountId32>,
        sign: &[u8; 256],
    ) -> Result<String>;
    async fn create_bucket(&self, pk: &[u8], name: &str) -> Result<(String, CreateBucket)>;
    async fn delete_bucket(&self, pk: &[u8], name: &str) -> Result<(String, DeleteBucket)>;
    async fn generate_restoral_order(
        &self,
        file_hash: &str,
        restoral_fragment: &str,
    ) -> Result<(String, GenerateRestoralOrder)>;
    async fn claim_restoral_order(
        &self,
        restoral_fragment: &str,
    ) -> Result<(String, ClaimRestoralOrder)>;
    async fn claim_restoral_no_exist_order(
        &self,
        miner: &[u8],
        file_hash: &str,
        restoral_fragment: &str,
    ) -> Result<String>;
    async fn restoral_order_complete(&self, fragment_hash: &str) -> Result<String>;
    async fn root_clear_failed_count(&self) -> Result<String>;
    async fn miner_clear_failed_count(&self) -> Result<String>;
}

#[async_trait]
impl FileBank for ChainSdk {
    /* Query functions */
    // query_storage_order
    async fn query_storage_order(&self, root_hash: &str) -> Result<Option<DealInfo>> {
        let hash = hash_from_string(root_hash);
        let query = file_bank_storage().deal_map(hash);

        query_storage(&query).await
    }

    // query_file_metadata
    async fn query_file_metadata(&self, root_hash: &str) -> Result<Option<FileInfo>> {
        let hash = hash_from_string(root_hash);
        let query = file_bank_storage().file(hash);

        query_storage(&query).await
    }

    // query_user_hold_file_list
    async fn query_user_hold_file_list(
        &self,
        pk: &[u8],
    ) -> Result<Option<BoundedVec<UserFileSliceInfo>>> {
        let account = account_from_slice(pk);

        let query = file_bank_storage().user_hold_file_list(&account);

        query_storage(&query).await
    }

    // query_pending_replacements
    async fn query_pending_replacements(&self, pk: &[u8]) -> Result<Option<u128>> {
        let account = account_from_slice(pk);

        let query = file_bank_storage().pending_replacements(&account);

        query_storage(&query).await
    }

    // query_bucket_info
    async fn query_bucket_info(&self, pk: &[u8], bucket_name: &str) -> Result<Option<BucketInfo>> {
        let account = account_from_slice(pk);

        let name_bytes: BoundedVec<u8> = BoundedVec(bucket_name.as_bytes().to_vec());

        let query = file_bank_storage().bucket(&account, name_bytes);

        query_storage(&query).await
    }

    // query_user_bucket_list
    async fn query_user_bucket_list(
        &self,
        pk: &[u8],
    ) -> Result<Option<BoundedVec<BoundedVec<u8>>>> {
        let account = account_from_slice(pk);

        let query = file_bank_storage().user_bucket_list(&account);

        query_storage(&query).await
    }

    // query_all_bucket_name
    async fn query_all_bucket_name(&self, pk: &[u8]) -> Result<Vec<String>> {
        match self.query_user_bucket_list(pk).await {
            Ok(bucketlist) => {
                if let Some(bucketlist) = bucketlist {
                    let buckets: Vec<String> = bucketlist
                        .0
                        .iter()
                        .map(|v| String::from_utf8_lossy(&v.0).to_string())
                        .collect();
                    Ok(buckets)
                } else {
                    Ok(vec![])
                }
            }
            Err(e) => {
                bail!("{}", e);
            }
        }
    }

    // query_restoral_order
    async fn query_restoral_order(&self, hash: &str) -> Result<Option<RestoralOrderInfo>> {
        let hash = hash_from_string(hash);

        let query = file_bank_storage().restoral_order(&hash);

        query_storage(&query).await
    }

    // query_clear_user_list
    async fn query_clear_user_list(&self) -> Result<Option<BoundedVec<AccountId32>>> {
        let query = file_bank_storage().clear_user_list();

        query_storage(&query).await
    }

    /* Transactional functions */
    // upload_declaration
    async fn upload_declaration(
        &self,
        file_hash: &str,
        deal_info: BoundedVec<SegmentList>,
        user: UserBrief,
        file_size: u128,
    ) -> Result<(String, UploadDeclaration)> {
        let hash = hash_from_string(file_hash);

        let tx = file_bank_tx().upload_declaration(hash, deal_info, user, file_size);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(upload_declaration) = events.find_first::<UploadDeclaration>()? {
            Ok((tx_hash, upload_declaration))
        } else {
            bail!("Unable to upload declaration");
        }
    }

    async fn deal_reassign_miner(&self, deal_hash: &str, count: u8, life: u32) -> Result<String> {
        let hash = hash_from_string(deal_hash);
        let tx = file_bank_tx().deal_reassign_miner(hash, count, life);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn ownership_transfer(&self, target_brief: UserBrief, file_hash: &str) -> Result<String> {
        let hash = hash_from_string(file_hash);

        let tx = file_bank_tx().ownership_transfer(target_brief, hash);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn transfer_report(&self, deal_hash: Vec<&str>) -> Result<(String, AccountId32)> {
        let hash: Vec<CPHash> = deal_hash
            .iter()
            .map(|hash| hash_from_string(hash))
            .collect();

        let tx = file_bank_tx().transfer_report(hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(transfer_report) = events.find_first::<TransferReport>()? {
            Ok((tx_hash, transfer_report.acc))
        } else {
            bail!("Unable to transfer");
        }
    }

    async fn calculate_end(&self, deal_hash: &str) -> Result<(String, CPHash)> {
        let hash = hash_from_string(deal_hash);

        let tx = file_bank_tx().calculate_end(hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(calculate_end) = events.find_first::<CalculateEnd>()? {
            Ok((tx_hash, calculate_end.file_hash))
        } else {
            bail!("Unable to transfer");
        }
    }

    async fn replace_idle_space(
        &self,
        idle_sig_info: SpaceProofInfo<AccountId32>,
        sign: &[u8; 256],
    ) -> Result<String> {
        let tx = file_bank_tx().replace_idle_space(idle_sig_info, *sign);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // delete_file
    async fn delete_file(
        &self,
        pk: &[u8],
        file_hash: Vec<String>,
    ) -> Result<(String, Vec<CPHash>)> {
        let account = account_from_slice(pk);

        let file_hash: Vec<CPHash> = file_hash
            .iter()
            .map(|hash| hash_from_string(hash))
            .collect();

        let tx = file_bank_tx().delete_file(account, file_hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(delete_file) = events.find_first::<DeleteFile>()? {
            Ok((tx_hash, delete_file.file_hash_list))
        } else {
            bail!("Unable to delete file");
        }
    }

    async fn cert_idle_space(
        &self,
        idle_sig_info: SpaceProofInfo<AccountId32>,
        sign: &[u8; 256],
    ) -> Result<String> {
        let tx = file_bank_tx().cert_idle_space(idle_sig_info, *sign);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // create_bucket
    async fn create_bucket(&self, pk: &[u8], name: &str) -> Result<(String, CreateBucket)> {
        let account = account_from_slice(pk);

        let name_bytes: BoundedVec<u8> = BoundedVec(name.as_bytes().to_vec());

        let tx = file_bank_tx().create_bucket(account, name_bytes);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(create_bucket_event) = events.find_first::<CreateBucket>()? {
            Ok((tx_hash, create_bucket_event))
        } else {
            bail!("Unable to create bucket");
        }
    }

    // delete_bucket
    async fn delete_bucket(&self, pk: &[u8], name: &str) -> Result<(String, DeleteBucket)> {
        let account = account_from_slice(pk);

        let name_bytes: BoundedVec<u8> = BoundedVec(name.as_bytes().to_vec());

        let tx = file_bank_tx().delete_bucket(account, name_bytes);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(delete_bucket_event) = events.find_first::<DeleteBucket>()? {
            Ok((tx_hash, delete_bucket_event))
        } else {
            bail!("Unable to delete bucket");
        }
    }

    async fn generate_restoral_order(
        &self,
        file_hash: &str,
        restoral_fragment: &str,
    ) -> Result<(String, GenerateRestoralOrder)> {
        let file_hash = hash_from_string(file_hash);
        let restoral_fragment_hash = hash_from_string(restoral_fragment);

        let tx = file_bank_tx().generate_restoral_order(file_hash, restoral_fragment_hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(generate_restoral_order) = events.find_first::<GenerateRestoralOrder>()? {
            Ok((tx_hash, generate_restoral_order))
        } else {
            bail!("Unable to claim restoral order");
        }
    }

    async fn claim_restoral_order(
        &self,
        restoral_fragment: &str,
    ) -> Result<(String, ClaimRestoralOrder)> {
        let hash = hash_from_string(restoral_fragment);

        let tx = file_bank_tx().claim_restoral_order(hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(claim_restoral_order) = events.find_first::<ClaimRestoralOrder>()? {
            Ok((tx_hash, claim_restoral_order))
        } else {
            bail!("Unable to claim restoral order");
        }
    }

    async fn claim_restoral_no_exist_order(
        &self,
        miner: &[u8],
        file_hash: &str,
        restoral_fragment: &str,
    ) -> Result<String> {
        let account = account_from_slice(miner);
        let file_hash = hash_from_string(file_hash);
        let restoral_fragment_hash = hash_from_string(restoral_fragment);

        let tx =
            file_bank_tx().claim_restoral_noexist_order(account, file_hash, restoral_fragment_hash);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn restoral_order_complete(&self, fragment_hash: &str) -> Result<String> {
        let hash = hash_from_string(fragment_hash);

        let tx = file_bank_tx().restoral_order_complete(hash);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn root_clear_failed_count(&self) -> Result<String> {
        let tx = file_bank_tx().root_clear_failed_count();

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    async fn miner_clear_failed_count(&self) -> Result<String> {
        let tx = file_bank_tx().miner_clear_failed_count();

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::FileBank;
    use crate::{chain::ChainSdk, core::utils::account::parsing_public_key};

    const MNEMONIC: &str =
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

    const ACCOUNT_ADDRESS: &str = "cXjmuHdBk4J3Zyt2oGodwGegNFaTFPcfC48PZ9NMmcUFzF6cc";

    const BUCKET_NAME: &str = "SampleBucket";

    fn init_chain() -> ChainSdk {
        ChainSdk::new(MNEMONIC, "service_name")
    }

    #[tokio::test]
    async fn test_query_storage_order() {
        let sdk = init_chain();
        let root_hash = "";
        let result = sdk.query_storage_order(root_hash).await;
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
    async fn test_query_file_metadata() {
        let sdk = init_chain();
        let root_hash = "";
        let result = sdk.query_file_metadata(root_hash).await;
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
    async fn test_query_user_hold_file_list() {
        let sdk = init_chain();
        let pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
        let result = sdk.query_user_hold_file_list(&pk_bytes).await;
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
    async fn test_query_pending_replacements() {
        let sdk = init_chain();
        let pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
        let result = sdk.query_pending_replacements(&pk_bytes).await;
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
    async fn test_query_bucket_info() {
        let sdk = init_chain();
        let pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
        let result = sdk.query_bucket_info(&pk_bytes, BUCKET_NAME).await;
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
        let sdk = init_chain();
        let pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
        let result = sdk.query_user_bucket_list(&pk_bytes).await;
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
    async fn test_query_all_bucket_name() {
        let sdk = init_chain();
        let pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
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
    async fn test_query_restoral_order() {
        let sdk = init_chain();
        let root_hash = "";
        let result = sdk.query_restoral_order(root_hash).await;
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
    async fn test_query_clear_user_list() {
        let sdk = init_chain();
        let result = sdk.query_clear_user_list().await;
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
    async fn test_create_bucket() {
        let sdk = init_chain();
        let pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
        let result = sdk.create_bucket(&pk_bytes, BUCKET_NAME).await;
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
        let sdk = init_chain();
        let pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
        let result = sdk.delete_bucket(&pk_bytes, BUCKET_NAME).await;
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
