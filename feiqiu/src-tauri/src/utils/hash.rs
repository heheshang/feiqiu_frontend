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
/// use neolan_lib::utils::hash;
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
/// use neolan_lib::utils::hash;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::io::Write;

    #[test]
    fn test_calculate_file_md5_empty_file() {
        // Create a temporary empty file
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_md5_empty.txt");

        // Create empty file
        File::create(&test_file).unwrap();

        // MD5 of empty string is "d41d8cd98f00b204e9800998ecf8427e"
        let md5 = calculate_file_md5(&test_file).unwrap();
        assert_eq!(md5, "d41d8cd98f00b204e9800998ecf8427e");

        // Clean up
        std::fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_calculate_file_md5_simple_content() {
        // Create a temporary file with known content
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_md5_content.txt");

        // Write "Hello World" to file
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"Hello World").unwrap();

        // MD5 of "Hello World" is "b10a8db164e0754105b7a99be72e3fe5"
        let md5 = calculate_file_md5(&test_file).unwrap();
        assert_eq!(md5, "b10a8db164e0754105b7a99be72e3fe5");

        // Clean up
        std::fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_calculate_file_md5_large_file() {
        // Create a temporary file with 1MB of data
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_md5_large.bin");

        // Write 1MB of zeros
        let mut file = File::create(&test_file).unwrap();
        let data = vec![0u8; 1024 * 1024];
        file.write_all(&data).unwrap();

        // Calculate MD5 (should be fast even for large files)
        let md5 = calculate_file_md5(&test_file).unwrap();
        assert_eq!(md5.len(), 32);

        // Clean up
        std::fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_get_file_size() {
        // Create a temporary file
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_size.txt");

        // Write 100 bytes
        let mut file = File::create(&test_file).unwrap();
        let data = vec![b'A'; 100];
        file.write_all(&data).unwrap();

        // Check size
        let size = get_file_size(&test_file).unwrap();
        assert_eq!(size, 100);

        // Clean up
        std::fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_get_file_size_empty() {
        // Create an empty file
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_size_empty.txt");

        File::create(&test_file).unwrap();

        // Check size
        let size = get_file_size(&test_file).unwrap();
        assert_eq!(size, 0);

        // Clean up
        std::fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_calculate_md5_nonexistent_file() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("nonexistent_file.txt");

        let result = calculate_file_md5(&test_file);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_file_size_nonexistent_file() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("nonexistent_file.txt");

        let result = get_file_size(&test_file);
        assert!(result.is_err());
    }

    #[test]
    fn test_md5_output_format() {
        // MD5 hash should always be 32 hex characters
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_format.txt");

        File::create(&test_file).unwrap();

        let md5 = calculate_file_md5(&test_file).unwrap();
        assert_eq!(md5.len(), 32);
        assert!(md5.chars().all(|c| c.is_ascii_hexdigit()));

        // Clean up
        std::fs::remove_file(&test_file).unwrap();
    }
}
