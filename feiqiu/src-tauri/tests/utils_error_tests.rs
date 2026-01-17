//! Utils and Error tests

use feiqiu::utils::hash::{calculate_file_md5, get_file_size};
use feiqiu::utils::logger::init_logger;
use feiqiu::NeoLanError;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// ============== Hash tests ==============

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
fn test_get_file_size_nonexistent_file() {
    let nonexistent = PathBuf::from("/tmp/nonexistent_file_12345.txt");
    let result = get_file_size(&nonexistent);
    assert!(result.is_err());
}

// ============== Logger tests ==============

#[test]
fn test_logger_init() {
    // This test ensures logger can be initialized without panicking
    init_logger();
    tracing::info!("Logger initialized successfully");
}

// ============== Error tests ==============

#[test]
fn test_error_display() {
    let err = NeoLanError::Protocol("invalid message format".to_string());
    assert_eq!(format!("{}", err), "Protocol error: invalid message format");
}

#[test]
fn test_peer_not_found() {
    let err = NeoLanError::PeerNotFound("192.168.1.100".to_string());
    assert!(err.to_string().contains("192.168.1.100"));
}

#[test]
fn test_network_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "connection refused");
    let err: NeoLanError = io_err.into();
    assert!(matches!(err, NeoLanError::Network(_)));
}

#[test]
fn test_json_error_from() {
    let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let err: NeoLanError = json_err.into();
    assert!(matches!(err, NeoLanError::Json(_)));
}

#[test]
fn test_result_type_alias() {
    fn returns_ok() -> feiqiu::Result<String> {
        Ok("success".to_string())
    }

    fn returns_err() -> feiqiu::Result<String> {
        Err(NeoLanError::Config("invalid config".to_string()))
    }

    assert!(returns_ok().is_ok());
    assert!(returns_err().is_err());
}

#[test]
fn test_with_context_storage() {
    let err = NeoLanError::Storage("database locked".to_string());
    let enriched = err.with_context("while saving message");
    assert_eq!(
        format!("{}", enriched),
        "Storage error: while saving message: database locked"
    );
}

#[test]
fn test_with_context_protocol() {
    let err = NeoLanError::Protocol("invalid packet".to_string());
    let enriched = err.with_context("parsing broadcast");
    assert_eq!(
        format!("{}", enriched),
        "Protocol error: parsing broadcast: invalid packet"
    );
}

#[test]
fn test_with_context_network() {
    let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "connection refused");
    let err: NeoLanError = io_err.into();
    let enriched = err.with_context("connecting to peer 192.168.1.100");
    // Network errors get converted to Storage with context
    assert!(format!("{}", enriched).contains("connecting to peer 192.168.1.100"));
}

#[test]
fn test_storage_context() {
    let err = NeoLanError::storage_context("database connection failed");
    assert_eq!(
        format!("{}", err),
        "Storage error: database connection failed"
    );
}

#[test]
fn test_network_context() {
    let err = NeoLanError::network_context("UDP send failed");
    assert_eq!(
        format!("{}", err),
        "Other error: Network error: UDP send failed"
    );
}

#[test]
fn test_protocol_context() {
    let err = NeoLanError::protocol_context("malformed message");
    assert_eq!(format!("{}", err), "Protocol error: malformed message");
}
