use super::ChainSdk;
use super::{deoss::DeOss, file_bank::FileBank};
use crate::config::{get_deoss_account, get_deoss_url};
use crate::core::{
    erasure::read_solomon_restore,
    pattern::SegmentDataInfo,
    utils::{
        account::parsing_public_key,
        bucket::check_bucket_name,
        str::get_random_code,
    },
};
use crate::polkadot;
use crate::utils::account_from_slice;
use anyhow::{bail, Result};
use async_trait::async_trait;
use base58::ToBase58;
use polkadot::{
    file_bank::events::UploadDeclaration,
    runtime_types::{
        bounded_collections::bounded_vec::BoundedVec,
        cp_cess_common::Hash,
        pallet_file_bank::types::{SegmentList, UserBrief},
    },
};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{multipart, Client, RequestBuilder};
use std::fs::{metadata, File as FFile};
use std::io::{Read, Write};
use std::path::PathBuf;
use subxt::ext::sp_core::Pair;

#[async_trait]
pub trait File {
    async fn generate_storage_order(
        &self,
        root_hash: &str,
        segment: Vec<SegmentDataInfo>,
        owner: &[u8],
        file_name: &str,
        buck_name: &str,
        file_size: u64,
    ) -> Result<(String, UploadDeclaration)>;
    async fn redundancy_recovery(&self, out_path: &str, shards_path: Vec<String>) -> Result<()>;
    async fn store_file(&self, file: &str, bucket: &str) -> Result<String>;
    async fn retrieve_file(&self, root_hash: &str, save_path: &str) -> Result<()>;
    async fn upload_to_gateway(
        &self,
        url: &str,
        upload_file: &str,
        bucket_name: &str,
    ) -> Result<String>;
    async fn download_from_gateway(
        &self,
        url: &str,
        root_hash: &str,
        save_path: &str,
    ) -> Result<()>;
}

#[async_trait]
impl File for ChainSdk {
    async fn generate_storage_order(
        &self,
        root_hash: &str,
        segment: Vec<SegmentDataInfo>,
        owner: &[u8],
        file_name: &str,
        buck_name: &str,
        file_size: u64,
    ) -> Result<(String, UploadDeclaration)> {
        let mut segment_list = Vec::new();
        for seg_info in &segment {
            let mut segment_hash = [0u8; 64];
            let mut fragment_hashes = Vec::new();
            for (i, &byte) in PathBuf::from(&seg_info.segment_hash)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .as_bytes()
                .iter()
                .enumerate()
            {
                segment_hash[i] = byte;
            }

            for frag_hash in &seg_info.fragment_hash {
                let mut fragment_hash = [0u8; 64];
                for (i, &byte) in PathBuf::from(frag_hash)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .as_bytes()
                    .iter()
                    .enumerate()
                {
                    fragment_hash[i] = byte;
                }
                fragment_hashes.push(Hash(fragment_hash));
            }

            segment_list.push(SegmentList {
                hash: Hash(segment_hash),
                fragment_list: BoundedVec(fragment_hashes),
            });
        }
        let acc = account_from_slice(owner);
        let user = UserBrief {
            user: acc,
            file_name: BoundedVec(file_name.as_bytes().to_vec()),
            bucket_name: BoundedVec(buck_name.as_bytes().to_vec()),
        };
        self.upload_declaration(root_hash, BoundedVec(segment_list), user, file_size.into())
            .await
    }

    async fn redundancy_recovery(&self, out_path: &str, shards_path: Vec<String>) -> Result<()> {
        read_solomon_restore(out_path, shards_path)
    }

    async fn store_file(&self, file: &str, bucket: &str) -> Result<String> {
        let deoss_account = get_deoss_account();

        let pk = parsing_public_key(&deoss_account).unwrap();
        if let Err(err) = self.authorize(&pk).await {
            bail!(err)
        };

        let deoss_url = get_deoss_url();

        self.upload_to_gateway(&deoss_url, file, bucket).await
    }

    async fn retrieve_file(&self, root_hash: &str, save_path: &str) -> Result<()> {
        let deoss_url = get_deoss_url();

        self.download_from_gateway(&deoss_url, root_hash, save_path)
            .await
    }

    async fn upload_to_gateway(
        &self,
        url: &str,
        upload_file: &str,
        bucket_name: &str,
    ) -> Result<String> {
        let fstat = metadata(upload_file)?;
        if fstat.is_dir() {
            bail!("not a file")
        }

        if fstat.len() == 0 {
            bail!("empty file")
        }

        if !check_bucket_name(bucket_name) {
            bail!("invalid bucket name")
        }

        let message = get_random_code(16).unwrap();
        let sig = self.pair.sign(message.as_bytes());

        let mut headers = HeaderMap::new();
        headers.insert("BucketName", HeaderValue::from_str(bucket_name)?);
        headers.insert("Account", HeaderValue::from_str(&self.get_signature_acc())?);
        headers.insert("Message", HeaderValue::from_str(&message)?);
        headers.insert("Signature", HeaderValue::from_str(&sig.0.to_base58())?);

        // Create multipart form data
        let mut form = multipart::Form::new();

        let mut file = FFile::open(upload_file)?;

        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content)?;

        let upload_file = upload_file.to_string().clone();
        form = form.part(
            "file",
            multipart::Part::stream(file_content.clone()).file_name(upload_file),
        );

        let client = match Client::builder().build() {
            Ok(client) => client,
            Err(err) => {
                bail!("{}", err)
            }
        };
        let request_builder: RequestBuilder = client.put(url).headers(headers).multipart(form);

        let response = request_builder.send().await?;
        let status_code = response.status();
        let response_text = response.text().await?;
        if !status_code.is_success() {
            if !response_text.is_empty() {
                bail!(response_text)
            }
            bail!("Deoss service failure, please retry or contact administrator.");
        }

        Ok(response_text.trim_matches('"').to_string())
    }

    async fn download_from_gateway(
        &self,
        url: &str,
        root_hash: &str,
        save_path: &str,
    ) -> Result<()> {
        let mut save_path = String::from(save_path);
        let mut url = String::from(url);

        if let Ok(fstat) = metadata(&save_path) {
            if fstat.is_dir() {
                save_path = format!("{}/{}", save_path, root_hash);
            }

            if fstat.len() == 0 {
                return Ok(());
            }
        }

        if url.is_empty() {
            bail!("empty url")
        }

        if !url.ends_with('/') {
            url = format!("{}/", url);
        }

        let mut headers = HeaderMap::new();
        headers.insert("Operation", HeaderValue::from_static("download"));

        let client = Client::new();
        let request_builder: RequestBuilder = client
            .get(&format!("{}{}", url, root_hash))
            .headers(headers);

        let f = FFile::create(&save_path)?;
        let response = request_builder.send().await?;
        let status_code = response.status();

        if !status_code.is_success() {
            bail!("failed");
        }
        let mut writer = f;

        let mut response_body = response.bytes().await?;
        while !response_body.is_empty() {
            let bytes_written = writer.write(&response_body)?;
            response_body = response_body[bytes_written..].to_vec().into();
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        chain::{file::File, ChainSdk},
        config::get_deoss_url,
    };

    const MNEMONIC: &str =
        "total pattern kind myself about paddle thrive flash paper leader tired noise";

    fn init_chain() -> ChainSdk {
        ChainSdk::new(MNEMONIC, "service_name")
    }

    #[tokio::test]
    async fn test_store_file() {
        let sdk = init_chain();
        let result = sdk.store_file("README.md", "SampleBucket").await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_download_from_gateway() {
        let sdk = init_chain();
        let path = "/tmp";
        let root_hash = "5b476ba750c0da1ec2392a0819384b2b3348f032e118578f6e13c41a57c9ec6f";
        let result = sdk
            .download_from_gateway(&get_deoss_url(), root_hash, path)
            .await;
        println!("{:?}", result);
    }
}
