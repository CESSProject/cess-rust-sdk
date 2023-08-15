use anyhow::{bail, Result};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use super::pattern::{self, DATA_SHARDS, PAR_SHARDS, SEGMENT_SIZE};

// pub fn reed_solomon(path: &str) -> Result<Vec<String>> {
//     let shards_path: Vec<String> = Vec::new();
//     let fstat = fs::metadata(path)?;
//     if fstat.is_dir() {
//         return Err("not a file".into());
//     }
//     if fstat.len() != SEGMENT_SIZE as u64 {
//         bail!("invalid size");
//     }
//     let (datashards, parshards) = (DATA_SHARDS, PAR_SHARDS);
//     let base_dir = Path::new(path).parent().unwrap();
// }
