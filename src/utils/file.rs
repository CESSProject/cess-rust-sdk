use crate::core::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

pub fn write_buf_to_file(buf: &[u8], file: &str) -> Result<(), Error> {
    let base_dir = match Path::new(file).parent() {
        Some(path) => path,
        None => return Err("Invalid file path".into()),
    };

    let id = Uuid::new_v4();
    let temp_file_path = base_dir.join(id.to_string());
    let mut f = File::create(&temp_file_path)?;
    f.write_all(buf)?;

    // Ensure data is written to disk before renaming.
    f.sync_all()?;

    fs::rename(&temp_file_path, file)?;
    Ok(())
}
