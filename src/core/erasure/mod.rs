use anyhow::{bail, Result};
use std::borrow::BorrowMut;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::pattern::{DATA_SHARDS, PAR_SHARDS, SEGMENT_SIZE};
use super::utils::hash::calc_sha256;
use reed_solomon_erasure::galois_8::{ReedSolomon, self};
use reed_solomon_erasure::{ReconstructShard, Field, convert_2D_slices, Error};
use smallvec::SmallVec;

pub fn reed_solomon(path: &str) -> Result<Vec<PathBuf>> {
    let mut shards_path = Vec::new();
    let fstat = fs::metadata(path)?;
    if fstat.is_dir() {
        bail!("not a file");
    }
    if fstat.len() != SEGMENT_SIZE as u64 {
        bail!("invalid size");
    }

    let base_dir = Path::new(path).parent().unwrap();

    let enc = match ReedSolomon::new(DATA_SHARDS as usize, PAR_SHARDS as usize) {
        Ok(enc) => enc,
        Err(err) => {
            bail!("{}", err)
        }
    };

    let b = match fs::read(path) {
        Ok(b) => b, 
        Err(err) => {
            bail!("{}", err)
        }
    };

    // Split the data into equally sized shards.
    let shard_size = b.len() / (DATA_SHARDS + PAR_SHARDS) as usize;
    let shards = split_data_into_shards(&b, shard_size);

    let mut encoded_shards = shards.clone();
    enc.encode(&mut encoded_shards)?;

    // Write out the resulting files.
    for (index, shard) in encoded_shards.iter().enumerate() {
        let hash_str = calc_sha256(shard)?;
        let newpath = base_dir.join(hash_str);

        if newpath.exists() {
            continue;
        }

        fs::write(&newpath, shard)?;

        shards_path.push(newpath);
    }

    Ok(shards_path)
}

fn split_data_into_shards(data: &[u8], shard_size: usize) -> Vec<Vec<u8>> {
    let mut shards = Vec::new();
    let total_shards = DATA_SHARDS + PAR_SHARDS;

    for i in 0..total_shards as usize {
        let start = i * shard_size;
        let end = (i + 1) * shard_size;
        let shard = data[start..end].to_vec();
        shards.push(shard);
    }

    shards
}

fn read_solomon_restore(out_path: &str, shards_path: Vec<String>) -> Result<()> {
    if Path::new(out_path).exists() {
        return Ok(())
    }

    let enc = match ReedSolomon::new(DATA_SHARDS as usize, PAR_SHARDS as usize) {
        Ok(enc) => enc,
        Err(err) => {
            bail!("{}", err)
        }
    };

    let mut shards = Vec::new();

    for v in &shards_path {
        let shard = fs::read(v).unwrap_or_default();
        shards.push(shard);
    }

    let result: Vec<_> = if let Err(_) = enc.verify(&shards) {
        let mut shards: Vec<_> = shards.iter().cloned().map(Some).collect();
        enc.reconstruct(&mut shards)?;
        shards.into_iter().filter_map(|x| x).collect()
    } else {
        Vec::new()
    };

    if !result.is_empty() {
        let mut f = fs::File::create(out_path)?;
        // f.write_all(&result)?;

        f.write_all(&result.iter().flat_map(|x| x.iter()).cloned().collect::<Vec<_>>())?;
    }
    Ok(())
}
