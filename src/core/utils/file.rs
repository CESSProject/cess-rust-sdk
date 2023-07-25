use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

pub fn write_buf_to_file(buf: &[u8], file: &str) -> Result<()> {
    let base_dir = Path::new(file)
        .parent()
        .with_context(|| "Invalid file path")?;

    let id = Uuid::new_v4();
    let temp_file_path = base_dir.join(id.to_string());
    let mut f = File::create(&temp_file_path)?;
    f.write_all(buf)?;

    // Ensure data is written to disk before renaming.
    f.sync_all()?;

    fs::rename(&temp_file_path, file)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{fs, io::Read, path::Path};

    use crate::core::utils::file::write_buf_to_file;

    #[test]
    fn test_write_buf_to_file() {
        let test_data = b"Hello, world!";
        let dir = Path::new("/tmp");

        let temp_file = dir.join("temp_file.txt");

        write_buf_to_file(test_data, &temp_file.to_string_lossy())
            .expect("Failed to write buffer to file");

        let mut file_content = Vec::new();
        let mut file = fs::File::open(&temp_file).expect("Failed to open temp file");
        file.read_to_end(&mut file_content)
            .expect("Failed to read file");

        assert_eq!(file_content, test_data);
    }
}
