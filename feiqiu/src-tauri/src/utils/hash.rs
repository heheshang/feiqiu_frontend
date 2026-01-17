// File hashing utilities for MD5 calculation and file size
use crate::{NeoLanError, Result};
use md5::{Digest, Md5};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Calculate MD5 hash of a file
///
/// Reads the file in chunks to avoid loading large files into memory.
/// Uses a 8KB buffer size for efficient reading.
///
/// # Arguments
///
/// * `path` - Path to the file to hash
///
/// # Returns
///
/// MD5 hash as a hexadecimal string (32 characters)
///
/// # Errors
///
/// Returns `NeoLanError::FileTransfer` if:
/// - File cannot be opened
/// - File cannot be read
///
/// # Example
///
/// ```no_run
/// use feiqiu::utils::hash;
/// use std::path::Path;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let md5 = hash::calculate_file_md5(Path::new("test.txt"))?;
/// assert_eq!(md5.len(), 32);
/// # Ok(())
/// # }
/// ```
pub fn calculate_file_md5(path: &Path) -> Result<String> {
    // Open the file
    let file = File::open(path).map_err(|e| {
        NeoLanError::FileTransfer(format!("Failed to open file {}: {}", path.display(), e))
    })?;

    // Create buffered reader for efficient reading
    let mut reader = BufReader::new(file);
    let mut hasher = Md5::new();
    let mut buffer = [0u8; 8192]; // 8KB buffer

    // Read file in chunks and update hash
    loop {
        let n = reader.read(&mut buffer).map_err(|e| {
            NeoLanError::FileTransfer(format!("Failed to read file {}: {}", path.display(), e))
        })?;

        if n == 0 {
            break; // EOF
        }

        hasher.update(&buffer[..n]);
    }

    // Get the final hash as hex string
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Get the size of a file in bytes
///
/// # Arguments
///
/// * `path` - Path to the file
///
/// # Returns
///
/// File size in bytes
///
/// # Errors
///
/// Returns `NeoLanError::FileTransfer` if:
/// - Metadata cannot be retrieved
///
/// # Example
///
/// ```no_run
/// use feiqiu::utils::hash;
/// use std::path::Path;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let size = hash::get_file_size(Path::new("test.txt"))?;
/// assert!(size >= 0);
/// # Ok(())
/// # }
/// ```
pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = std::fs::metadata(path).map_err(|e| {
        NeoLanError::FileTransfer(format!(
            "Failed to get metadata for {}: {}",
            path.display(),
            e
        ))
    })?;

    Ok(metadata.len())
}
