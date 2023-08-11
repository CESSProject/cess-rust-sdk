use super::Sdk;
use crate::polkadot;
use crate::utils::{
    account_from_slice, hash_from_string, query_storage, sign_and_sbmit_tx_default,
    sign_and_submit_tx_then_watch_default,
};
use anyhow::{bail, Result};
use log::info;
use polkadot::{
    file_bank::{
        calls::types::{DealReassignMiner, OwnershipTransfer},
        events::{
            CalculateEnd, ClaimRestoralOrder, CreateBucket, DeleteBucket, DeleteFile,
            GenerateRestoralOrder, TransferReport, UploadDeclaration, MinerExitPrep
        },
    },
    runtime_types::{
        cp_cess_common::{Hash as CPHash, SpaceProofInfo},
        pallet_file_bank::types::{
            BucketInfo, DealInfo, FileInfo, RestoralOrderInfo, RestoralTargetInfo, SegmentList,
            UserBrief, UserFileSliceInfo,
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
        let hash = hash_from_string(root_hash);
        let query = polkadot::storage().file_bank().deal_map(hash);

        let result = query_storage(&query).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    // query_file_metadata
    pub async fn query_file_metadata(&self, root_hash: &str) -> Result<FileInfo> {
        let hash = hash_from_string(root_hash);
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
        let hash = hash_from_string(hash);

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
    // upload_declaration
    pub async fn upload_declaration(
        &self,
        file_hash: &str,
        deal_info: BoundedVec<SegmentList>,
        user: UserBrief,
        file_size: u128,
    ) -> Result<String> {
        let hash = hash_from_string(file_hash);

        let tx = polkadot::tx()
            .file_bank()
            .upload_declaration(hash, deal_info, user, file_size);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(upload_declaration) = events.find_first::<UploadDeclaration>()? {
            info!("Upload declaration: {:?}", upload_declaration);
            return Ok(tx_hash);
        } else {
            bail!("Unable to upload declaration");
        }
    }

    pub async fn deal_reassign_miner(
        &self,
        deal_hash: &str,
        count: u8,
        life: u32,
    ) -> Result<String> {
        let hash = hash_from_string(deal_hash);
        let tx = polkadot::tx()
            .file_bank()
            .deal_reassign_miner(hash, count, life);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn ownership_transfer(
        &self,
        target_brief: UserBrief,
        file_hash: &str,
    ) -> Result<String> {
        let hash = hash_from_string(file_hash);

        let tx = polkadot::tx()
            .file_bank()
            .ownership_transfer(target_brief, hash);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn transfer_report(&self, deal_hash: Vec<&str>) -> Result<(String, AccountId32)> {
        let hash: Vec<CPHash> = deal_hash
            .iter()
            .map(|hash| hash_from_string(hash))
            .collect();

        let tx = polkadot::tx().file_bank().transfer_report(hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(transfer_report) = events.find_first::<TransferReport>()? {
            info!("Transfer report: {:?}", transfer_report);
            return Ok((tx_hash, transfer_report.acc));
        } else {
            bail!("Unable to transfer");
        }
    }

    pub async fn calculate_end(&self, deal_hash: &str) -> Result<(String, CPHash)> {
        let hash = hash_from_string(deal_hash);

        let tx = polkadot::tx().file_bank().calculate_end(hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(calculate_end) = events.find_first::<CalculateEnd>()? {
            info!("calculate end: {:?}", calculate_end);
            return Ok((tx_hash, calculate_end.file_hash));
        } else {
            bail!("Unable to transfer");
        }
    }

    pub async fn replace_idle_space(
        &self,
        idle_sig_info: SpaceProofInfo<AccountId32, u32>,
        sign: &[u8; 256],
    ) -> Result<String> {
        let tx = polkadot::tx()
            .file_bank()
            .replace_idle_space(idle_sig_info, *sign);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // delete_file
    pub async fn delete_file(
        &self,
        pk: &[u8],
        file_hash: Vec<String>,
    ) -> Result<(String, Vec<CPHash>)> {
        let account = account_from_slice(pk);

        let file_hash: Vec<CPHash> = file_hash
            .iter()
            .map(|hash| hash_from_string(hash))
            .collect();

        let tx = polkadot::tx().file_bank().delete_file(account, file_hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(delete_file) = events.find_first::<DeleteFile>()? {
            info!("File(s) deleted: {:?}", delete_file);
            return Ok((tx_hash, delete_file.file_hash_list));
        } else {
            bail!("Unable to delete file");
        }
    }

    pub async fn cert_idle_space(
        &self,
        idle_sig_info: SpaceProofInfo<AccountId32, u32>,
        sign: &[u8; 256],
    ) -> Result<String> {
        let tx = polkadot::tx()
            .file_bank()
            .cert_idle_space(idle_sig_info, *sign);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    // create_bucket
    pub async fn create_bucket(&self, pk: &[u8], name: &str) -> Result<String> {
        let account = account_from_slice(pk);

        let name_bytes: BoundedVec<u8> = BoundedVec(name.as_bytes().to_vec());

        let tx = polkadot::tx()
            .file_bank()
            .create_bucket(account, name_bytes);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

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

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(delete_bucket_event) = events.find_first::<DeleteBucket>()? {
            info!("Bucket seleted: {:?}", delete_bucket_event);
            return Ok(tx_hash);
        } else {
            bail!("Unable to delete bucket");
        }
    }

    pub async fn generate_restoral_order(
        &self,
        file_hash: &str,
        restoral_fragment: &str,
    ) -> Result<String> {
        let file_hash = hash_from_string(file_hash);
        let restoral_fragment_hash = hash_from_string(restoral_fragment);

        let tx = polkadot::tx()
            .file_bank()
            .generate_restoral_order(file_hash, restoral_fragment_hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(generate_restoral_order) = events.find_first::<GenerateRestoralOrder>()? {
            info!("Generate restoral order: {:?}", generate_restoral_order);
            return Ok(tx_hash);
        } else {
            bail!("Unable to claim restoral order");
        }
    }

    pub async fn claim_restoral_order(&self, restoral_fragment: &str) -> Result<String> {
        let hash = hash_from_string(restoral_fragment);

        let tx = polkadot::tx().file_bank().claim_restoral_order(hash);

        let from = PairSigner::new(self.pair.clone());

        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(claim_restoral_order) = events.find_first::<ClaimRestoralOrder>()? {
            info!("Claim restoral order: {:?}", claim_restoral_order);
            return Ok(tx_hash);
        } else {
            bail!("Unable to claim restoral order");
        }
    }

    pub async fn claim_restoral_no_exist_order(
        &self,
        miner: &[u8],
        file_hash: &str,
        restoral_fragment: &str,
    ) -> Result<String> {
        let account = account_from_slice(miner);
        let file_hash = hash_from_string(file_hash);
        let restoral_fragment_hash = hash_from_string(restoral_fragment);

        let tx = polkadot::tx().file_bank().claim_restoral_noexist_order(
            account,
            file_hash,
            restoral_fragment_hash,
        );

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn restoral_order_complete(&self, fragment_hash: &str) -> Result<String> {
        let hash = hash_from_string(fragment_hash);

        let tx = polkadot::tx()
            .file_bank()
            .restoral_order_complete(hash);

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn miner_exit_prep(&self) -> Result<String> {
        let tx = polkadot::tx()
            .file_bank()
            .miner_exit_prep();

        let from = PairSigner::new(self.pair.clone());
        
        let events = sign_and_submit_tx_then_watch_default(&tx, &from).await?;

        let tx_hash = events.extrinsic_hash().to_string();
        if let Some(exit_prep) = events.find_first::<MinerExitPrep>()? {
            info!("Miner exit prep: {:?}", exit_prep);
            return Ok(tx_hash);
        } else {
            bail!("Unable to execute miner exit prep");
        }
    }

    pub async fn miner_exit(&self, miner: &[u8]) -> Result<String> {
        let account = account_from_slice(miner);

        let tx = polkadot::tx()
            .file_bank()
            .miner_exit(account);

        let from = PairSigner::new(self.pair.clone());
        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
    }

    pub async fn miner_withdraw(&self) -> Result<String> {
        let tx = polkadot::tx()
            .file_bank()
            .miner_withdraw();

        let from = PairSigner::new(self.pair.clone());

        let hash = sign_and_sbmit_tx_default(&tx, &from).await?;

        Ok(hash.to_string())
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
