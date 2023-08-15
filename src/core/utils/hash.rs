use std::{fs::File, io::Read};

use anyhow::{bail, Result};
use sha2::{Digest, Sha256};

// calc_path_sha256 is used to calculate the sha256 value
// of a file with a given path.
pub fn calc_path_sha256(fpath: &str) -> Result<String> {
    let mut f = File::open(fpath)?;
    calc_file_sha256(&mut f)
}

// calc_file_sha256 is used to calculate the sha256 value
// of the file type.
pub fn calc_file_sha256(f: &mut File) -> Result<String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];

    loop {
        let bytes_read = f.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

// calc_sha256 is used to calculate the sha256 value
// of the data.
pub fn calc_sha256(data: &[u8]) -> Result<String> {
    if data.is_empty() {
        bail!("data is empty");
    }
    let mut hasher = Sha256::new();
    hasher.update(data);

    let result = hasher.finalize();

    Ok(hex::encode(result))
}

// calc_md5 is used to calculate the md5 value
// of the data.
pub fn calc_md5(data: &[u8]) -> Result<String> {
    if data.is_empty() {
        bail!("data is empty");
    }
    let digest = md5::compute(data);
    Ok(format!("{:x}", digest))
}

pub fn calc_path_sha256_bytes(fpath: &str) -> Result<Vec<u8>> {
    let mut f = File::open(fpath)?;
    calc_file_sha256_bytes(&mut f)
}

pub fn calc_file_sha256_bytes(f: &mut File) -> Result<Vec<u8>> {
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];

    loop {
        let bytes_read = f.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(result.to_vec())
}

#[cfg(test)]
mod test {
    use std::{fs, io::Write};

    use crate::core::utils::hash::calc_path_sha256;

    use super::{calc_md5, calc_path_sha256_bytes, calc_sha256};

    #[test]
    fn test_calc_path_sha256() {
        let path = "/tmp/temp_file1.txt";
        let expected_hash = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        match fs::File::create(path) {
            Ok(mut file) => {
                let data = b"hello world";
                if let Err(err) = file.write_all(data) {
                    debug_assert!(false, "failed to write to file: {:?}", err);
                }
            }
            Err(err) => {
                debug_assert!(false, "failed to create a file: {:?}", err);
            }
        }

        match calc_path_sha256(path) {
            Ok(hash) => {
                assert_eq!(hash, expected_hash);
            }
            Err(err) => {
                debug_assert!(false, "Error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_calc_sha256() {
        let data = b"hello world";
        let expected_hash = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";

        match calc_sha256(data) {
            Ok(hash) => {
                assert_eq!(hash, expected_hash);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_calc_md5() {
        // Test case 1: Non-empty data
        let input_data = b"abcdefghijklmnopqrstuvwxyz";
        let expected_hash = "c3fcd3d76192e4007dfb496cca67e13b";
        let result = calc_md5(input_data).unwrap();
        assert_eq!(result, expected_hash);

        // Test case 2: Empty data
        let empty_data = b"";
        let expected_err_msg = "data is empty";
        let result = calc_md5(empty_data);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), expected_err_msg);
    }

    #[test]
    fn test_calc_path_sha256_bytes() {
        let path = "/tmp/temp_file1.txt";
        let expected_hash = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        match calc_path_sha256_bytes(path) {
            Ok(hash) => {
                assert_eq!(hex::encode(hash), expected_hash);
            }
            Err(err) => {
                debug_assert!(false, "Error: {:?}", err);
            }
        }
    }
}
