use super::Sdk;
use crate::core::erasure::{read_solomon_restore, reed_solomon};
use crate::core::hashtree::{build_merkle_root_hash, build_simple_merkle_root_hash, build_tree};
use crate::core::pattern::{SegmentDataInfo, PUBLIC_DEOSS, PUBLIC_DEOSS_ACCOUNT, SEGMENT_SIZE};
use crate::core::utils::account::parsing_public_key;
use crate::core::utils::bucket::check_bucket_name;
use crate::core::utils::hash::calc_sha256;
use crate::core::utils::str::get_random_code;
use crate::core::utils::{self, file};
use crate::polkadot;
use crate::utils::account_from_slice;
use anyhow::{anyhow, bail, Result};
use base58::ToBase58;
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use subxt::ext::sp_core::Pair;

use polkadot::{
    file_bank::events::UploadDeclaration,
    runtime_types::{
        cp_cess_common::Hash,
        pallet_file_bank::types::{SegmentList, UserBrief},
        sp_core::bounded::bounded_vec::BoundedVec,
    },
};

impl Sdk {
    pub async fn processing_data(file: &str) -> Result<(Vec<SegmentDataInfo>, String)> {
        let segment_paths = match cut_file(file) {
            Ok(segment_paths) => segment_paths,
            Err(err) => {
                bail!("[cutfile]: {}", err)
            }
        };

        let mut segment_data_info = Vec::new();

        for v in &segment_paths {
            let segment_hash = v.clone();
            let fragment_hash = match reed_solomon(v.to_str().unwrap()) {
                Ok(fragment_hash) => fragment_hash,
                Err(err) => {
                    bail!("[ReedSolomon]: {}", err)
                }
            };

            segment_data_info.push(SegmentDataInfo {
                segment_hash,
                fragment_hash,
            });

            fs::remove_file(v)?;
        }

        let hash = if segment_paths.len() == 1 {
            build_simple_merkle_root_hash(
                &Path::new(&segment_paths[0])
                    .file_name()
                    .unwrap()
                    .to_string_lossy(),
            )?
        } else {
            build_merkle_root_hash(extract_segmenthash(&segment_paths))?
        };

        Ok((segment_data_info, hash))
    }

    pub async fn generate_storage_order(
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
                fragment_hashes.push(Hash(fragment_hash.clone()));
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

    pub async fn redundancy_recovery(
        &self,
        out_path: &str,
        shards_path: Vec<String>,
    ) -> Result<()> {
        read_solomon_restore(out_path, shards_path)
    }

    pub async fn store_file(&self, file: &str, bucket: &str) -> Result<String> {
        let pk = parsing_public_key(PUBLIC_DEOSS_ACCOUNT).unwrap();
        self.authorize(&pk).await;

        self.upload_to_gateway(PUBLIC_DEOSS, file, bucket).await
    }

    pub async fn retrieve_file(&self, root_hash: &str, save_path: &str) -> Result<()> {
        self.download_from_gateway(PUBLIC_DEOSS, root_hash, save_path)
            .await
    }

    pub async fn upload_to_gateway(
        &self,
        url: &str,
        upload_file: &str,
        bucket_name: &str,
    ) -> Result<String> {
        let fstat = fs::metadata(upload_file)?;
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
        headers.insert("Account", HeaderValue::from_str(&self.get_signature_acc())?); // Assuming self.name is the signature account
        headers.insert("Message", HeaderValue::from_str(&message)?);
        headers.insert("Signature", HeaderValue::from_str(&sig.0.to_base58())?);

        let client = Client::builder().build()?;
        let request_builder: RequestBuilder = client.put(url).headers(headers);

        // let mut body = Multipart::from_request(req);
        // let form_file = body.add_file("file", upload_file)?;

        let mut file = File::open(upload_file)?;
        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content)?;

        let response = request_builder
            .header(CONTENT_TYPE, "multipart/form-data")
            .body(file_content)
            .send()?;
        let status_code = response.status();

        let response_text = response.text()?;
        if !status_code.is_success() {
            if !response_text.is_empty() {
                bail!(response_text)
            }
            bail!("Deoss service failure, please retry or contact administrator.");
        }

        Ok(response_text.trim_matches('"').to_string())
    }

    pub async fn download_from_gateway(
        &self,
        url: &str,
        root_hash: &str,
        save_path: &str,
    ) -> Result<()> {
        let mut save_path = String::from(save_path);
        let mut url = String::from(url);

        if let Ok(fstat) = fs::metadata(&save_path) {
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

        if !url.ends_with("/") {
            url = format!("{}/", url);
        }

        let mut headers = HeaderMap::new();
        headers.insert("Operation", HeaderValue::from_static("download"));

        let client = Client::new();
        let request_builder: RequestBuilder = client
            .get(&format!("{}{}", url, root_hash))
            .headers(headers);

        let f = File::create(&save_path)?;
        let response = request_builder.send()?;
        let status_code = response.status();

        if !status_code.is_success() {
            bail!("failed");
        }
        let mut writer = f;

        let mut response_body = response.bytes()?;
        while !response_body.is_empty() {
            let bytes_written = writer.write(&response_body)?;
            response_body = response_body[bytes_written..].to_vec().into();
        }

        Ok(())
    }
}

fn cut_file(file: &str) -> Result<Vec<PathBuf>> {
    let fstat = std::fs::metadata(file)?;
    if fstat.is_dir() {
        bail!("not a file");
    }
    if fstat.len() == 0 {
        bail!("empty file");
    }

    let base_dir = Path::new(file)
        .parent()
        .ok_or_else(|| anyhow!("Invalid parent directory"))?;
    let segment_count = (fstat.len() + SEGMENT_SIZE as u64 - 1) / SEGMENT_SIZE as u64;

    let mut segments = Vec::with_capacity(segment_count as usize);
    let mut buf = vec![0u8; SEGMENT_SIZE as usize];
    let mut f = File::open(file)?;

    for i in 0..segment_count {
        f.seek(SeekFrom::Start(SEGMENT_SIZE as u64 * i))?;
        let num = f.read(&mut buf)?;
        if num == 0 {
            return Err(anyhow!("read file is empty"));
        }

        if num < SEGMENT_SIZE as usize {
            if i + 1 != segment_count {
                return Err(anyhow!("read file err"));
            }
            let rand_str = utils::str::rand_str(SEGMENT_SIZE as usize - num);
            buf[num..].copy_from_slice(rand_str.as_bytes());
        }

        let hash = calc_sha256(&buf)?;
        let segment_path = base_dir.join(hash);
        let mut segment_file = File::create(&segment_path)?;
        segment_file.write_all(&buf)?;

        segments.push(segment_path);
    }

    Ok(segments)
}

fn extract_segmenthash(segment: &[PathBuf]) -> Vec<String> {
    let mut segmenthash = Vec::with_capacity(segment.len());
    for seg in segment.iter() {
        let base = Path::new(seg)
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();
        segmenthash.push(base);
    }
    segmenthash
}
