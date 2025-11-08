//! # File Utilities
//!
//! Provides low-level file handling utilities used by the SDK to safely
//! write data buffers to disk.  
//!
//! Ensures that writes are **atomic** - data is first written to a temporary
//! file and only moved to the target location after a successful write and sync.
//! This prevents partial or corrupted files in case of an I/O error or crash.

use crate::core::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

/// Writes a byte buffer to a file atomically.
///
/// The function first writes data to a temporary file in the same directory,
/// flushes it to disk, and then renames it to the target filename.
/// This ensures that the resulting file is either fully written or not written at all.
///
/// # Arguments
/// * `buf` - The byte slice to write.
/// * `file` - Path to the destination file.
///
/// # Returns
/// * `Ok(())` on success.
/// * [`Error`] if any I/O or path-related error occurs.
///
/// # Behavior
/// - A random temporary filename (UUID) is created in the target directory.
/// - Data is written and synced to disk before the rename.
/// - The rename operation replaces the target file atomically.
///
/// # Example
/// ```ignore
/// let data = b"Hello, world!";
/// write_buf_to_file(data, "/tmp/output.txt")?;
/// ```
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
