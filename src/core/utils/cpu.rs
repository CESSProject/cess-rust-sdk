use anyhow::{bail, Result};
use std::path::Path;

pub fn get_dir_free_space(dir: &str) -> Result<u64> {
    let path = Path::new(dir);
    match fs4::available_space(path) {
        Ok(disk_space) => Ok(disk_space),
        Err(err) => {
            bail!("{:?}", err);
        }
    }
}

pub fn get_sys_mem_available() -> Result<u64> {
    let mem_info = sys_info::mem_info()?;
    let avail_mem = mem_info.avail;

    Ok(avail_mem)
}

#[cfg(test)]
mod test {
    use super::{get_dir_free_space, get_sys_mem_available};

    #[test]
    fn test_get_dir_free_space() {
        let path = "/";
        match get_dir_free_space(path) {
            Ok(_) => {
                assert!(true);
            }
            Err(err) => {
                debug_assert!(false, "Error: {:?}", err)
            }
        }
    }

    #[test]
    fn test_get_sys_mem_available() {
        match get_sys_mem_available() {
            Ok(_) => {
                assert!(true);
            }
            Err(err) => {
                debug_assert!(false, "Error: {:?}", err)
            }
        }
    }
}
