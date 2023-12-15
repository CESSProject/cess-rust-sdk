use std::{
    fs::{self, metadata, remove_file},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context, Result};
use rand::Rng;

use super::{
    crypt,
    erasure::reed_solomon,
    hashtree::{build_merkle_root_hash, build_simple_merkle_root_hash},
    pattern::{SegmentDataInfo, SEGMENT_SIZE},
    utils::{hash::calc_sha256, str::rand_str},
};

pub fn processing_data(file: &str) -> Result<(Vec<SegmentDataInfo>, String)> {
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

        remove_file(v)?;
    }
    let hash = if segment_paths.len() == 1 {
        println!("build_simple_merkle_root_hash");
        build_simple_merkle_root_hash(
            &Path::new(&segment_paths[0])
                .file_name()
                .unwrap()
                .to_string_lossy(),
        )?
    } else {
        build_merkle_root_hash(extract_segment_hash(&segment_paths))?
    };

    Ok((segment_data_info, hash))
}

pub fn sharded_encryption_processing(
    file: &str,
    cipher: &str,
) -> Result<(Vec<SegmentDataInfo>, String)> {
    let mut segment_path: Vec<PathBuf>;

    if !cipher.is_empty() {
        segment_path =
            cut_file_with_encryption(file).with_context(|| "Failed to cut file with encryption")?;
        match encrypted_segment(segment_path.clone(), cipher)
            .with_context(|| "Failed to encrypt segments")
        {
            Ok(new_segment_path) => {
                segment_path = new_segment_path;
            }
            Err(e) => {
                for path in &segment_path {
                    fs::remove_file(path)
                        .with_context(|| format!("Failed to remove file: {:?}", path))?;
                }
                return Err(e).with_context(|| "Error during encrypted_segment");
            }
        }
    } else {
        segment_path = cut_file(file).with_context(|| "Failed to cut file")?;
    }

    let mut segment_data_info = Vec::with_capacity(segment_path.len());

    for path in &segment_path {
        let fragment_hash = reed_solomon(&path.to_string_lossy())
            .with_context(|| format!("Failed to calculate Reed-Solomon hash for: {:?}", path))?;

        let segment_info = SegmentDataInfo {
            segment_hash: path.clone(),
            fragment_hash,
        };

        segment_data_info.push(segment_info);

        fs::remove_file(path).with_context(|| format!("Failed to remove file: {:?}", path))?;
    }

    let hash: String;

    if segment_path.len() == 1 {
        hash = build_simple_merkle_root_hash(
            &Path::new(&segment_path[0])
                .file_name()
                .unwrap()
                .to_string_lossy(),
        )
        .with_context(|| "Failed to build simple Merkle root hash")?;
    } else {
        hash = build_merkle_root_hash(extract_segment_hash(&segment_path))
            .with_context(|| "Failed to build Merkle root hash")?;
    }

    Ok((segment_data_info, hash))
}

fn cut_file(file: &str) -> Result<Vec<PathBuf>> {
    let fstat = metadata(file)?;
    if fstat.is_dir() {
        bail!("not a file");
    }
    if fstat.len() == 0 {
        bail!("empty file");
    }

    let base_dir = Path::new(file)
        .parent()
        .ok_or_else(|| anyhow!("Invalid parent directory"))?;
    let mut segment_count = fstat.len() / SEGMENT_SIZE as u64;
    if fstat.len() % SEGMENT_SIZE as u64 != 0 {
        segment_count += 1;
    }
    let mut segments = Vec::with_capacity(segment_count as usize);
    let mut buf = vec![0u8; SEGMENT_SIZE as usize];
    let mut f = fs::File::open(file)?;

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
            buf[num..].copy_from_slice(&vec![0; SEGMENT_SIZE as usize - num]);
        }

        let hash = calc_sha256(&buf)?;
        let segment_path = base_dir.join(hash);
        let mut segment_file = fs::File::create(&segment_path)?;
        segment_file.write_all(&buf)?;

        segments.push(segment_path);
    }

    Ok(segments)
}

fn cut_file_with_encryption(file: &str) -> Result<Vec<PathBuf>> {
    let fstat = fs::metadata(file)?;
    if fstat.is_dir() {
        bail!("not a file");
    }
    if fstat.len() == 0 {
        bail!("empty file");
    }
    let base_dir = Path::new(file)
        .parent()
        .ok_or_else(|| anyhow!("failed to determine parent directory"))?;
    let segment_count = (fstat.len() + SEGMENT_SIZE as u64 - 1) / SEGMENT_SIZE as u64;

    let mut segments: Vec<PathBuf> = Vec::with_capacity(segment_count as usize);
    let mut buf = vec![0u8; SEGMENT_SIZE as usize];
    let mut f = fs::File::open(file)?;

    for i in 0..segment_count {
        let offset = i * SEGMENT_SIZE as u64;
        f.seek(SeekFrom::Start(offset))?;
        let num = f.read(&mut buf)?;

        if num == 0 {
            bail!("read file is empty");
        }
        if num < SEGMENT_SIZE as usize {
            if i + 1 != segment_count {
                bail!("read file err");
            }
            let mut rng = rand::thread_rng();
            for j in num..SEGMENT_SIZE as usize {
                buf[j] = rng.gen::<u8>();
            }
        }

        let hash = calc_sha256(&buf)?;
        let segment_path = base_dir.join(&hash);

        let mut segment_file = fs::File::create(&segment_path)?;
        segment_file.write_all(&buf)?;

        segments.push(segment_path);
    }

    Ok(segments)
}

fn encrypted_segment(segments: Vec<PathBuf>, cipher: &str) -> Result<Vec<PathBuf>> {
    let mut encrypted_segments: Vec<PathBuf> = Vec::with_capacity(segments.len());

    for segment_path in &segments {
        let mut buf = fs::read(segment_path)?;

        buf = crypt::aes_cbc_encrypt(&buf, cipher.as_bytes())?;

        let hash = calc_sha256(&buf)?;
        let encrypted_segment_path = segment_path
            .parent()
            .ok_or_else(|| anyhow!("failed to determine parent directory"))?
            .join(&hash);

        fs::write(&encrypted_segment_path, &buf)?;

        encrypted_segments.push(encrypted_segment_path);
    }

    for segment_path in &segments {
        fs::remove_file(segment_path)?;
    }

    Ok(encrypted_segments)
}

fn extract_segment_hash(segment: &[PathBuf]) -> Vec<String> {
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
