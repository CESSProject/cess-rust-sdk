use crate::core::pattern::{SegmentDataInfo, SEGMENT_SIZE};
use crate::core::utils;
use crate::core::utils::hash::calc_sha256;

use super::Sdk;
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use anyhow::{anyhow, bail, Result};
use sha2::{Sha256, Digest};

impl Sdk {
    pub async fn processing_data(file: &str) -> Result<(SegmentDataInfo, String)> {
        let segment_path = match cut_file(file) {
            Ok(segment_path) => segment_path,
            Err(err) => {
                bail!("[cutfile]: {}", err)
            }
        };

        // Todo: Reed Solomon algo
        
        Ok((SegmentDataInfo { ..Default::default() }, "".to_string()))
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
