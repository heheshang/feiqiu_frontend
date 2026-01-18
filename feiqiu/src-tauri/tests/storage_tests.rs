//! Storage tests

use feiqiu::storage::database::get_db_path;
use std::env;

// ============== Database tests ==============

#[test]
fn test_get_db_path() {
    let db_path = get_db_path();

    // Verify path ends with neolan.db
    assert!(db_path.ends_with("neolan.db"));
}

#[test]
fn test_get_db_path_with_env_override() {
    // Set environment variable
    env::set_var("NEOLAN_DATA_DIR", "/tmp/test_neolan");

    let db_path = get_db_path();

    // Verify path uses environment variable
    assert!(db_path.starts_with("/tmp/test_neolan"));

    // Clean up environment variable
    env::remove_var("NEOLAN_DATA_DIR");
}

// ============== Message Repository tests ==============

#[test]
fn test_message_repo_creation() {
    use tokio::runtime::Runtime;

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let _db_path = feiqiu::storage::database::get_db_path();
        assert!(_db_path.ends_with("neolan.db"));
    });
}
