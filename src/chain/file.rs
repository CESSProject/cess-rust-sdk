use crate::core::erasure::reed_solomon;
use crate::core::pattern::{SegmentDataInfo, SEGMENT_SIZE};
use crate::core::utils;
use crate::core::utils::hash::calc_sha256;
use crate::core::hashtree::{build_tree, build_simple_merkle_root_hash, build_merkle_root_hash};
use super::Sdk;
use std::fs::{File, self};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use anyhow::{anyhow, bail, Result};
use sha2::{Sha256, Digest};

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

            segment_data_info.push(SegmentDataInfo{
                segment_hash,
                fragment_hash
            });

            fs::remove_file(v)?;
        }

        let hash = if segment_paths.len() == 1 {
            build_simple_merkle_root_hash(&Path::new(&segment_paths[0]).file_name().unwrap().to_string_lossy())?
        } else {
            build_merkle_root_hash(&segment_paths)?
        };
        

        Ok((segment_data_info, hash))
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

    let base_dir = Path::new(file).parent().ok_or_else(|| anyhow!("Invalid parent directory"))?;
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
