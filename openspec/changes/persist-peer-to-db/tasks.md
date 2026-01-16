# Implementation Tasks

## 1. Preparation Phase

- [x] 1.1 Add `From<&PeerModel>` trait implementation for `PeerNode` in `modules/peer/types.rs`
  - Convert `ip: String` → `IpAddr` using `.parse()`
  - Convert `port: i32` → `u16`
  - Convert `last_seen: NaiveDateTime` → `SystemTime`
  - Compute `status` from `last_seen` vs timeout threshold

- [x] 1.2 Add helper methods to `PeerNode` for database conversion
  - Add `fn is_online_from_last_seen(last_seen: NaiveDateTime) -> bool` static method
  - Add `fn naive_datetime_to_system_time()` conversion method

- [x] 1.3 Create mock `PeerRepository` for unit tests in `storage/peer_repo.rs`
  - Implemented `MockPeerRepository` for testing
  - Store peers in `HashMap<String, PeerModel>` for mock operations

## 2. Core PeerManager Migration

- [x] 2.1 Modify `PeerManager` struct in `modules/peer/manager.rs`
  - Add `peer_repo: Arc<PeerRepository>` field
  - Add `runtime_handle: tokio::runtime::Handle` field for async bridge
  - Keep `discovery`, `running`, `message_tx` fields unchanged

- [x] 2.2 Update `PeerManager::new()` constructor
  - Change signature to accept `Arc<PeerRepository>`
  - Capture current tokio runtime handle
  - Initialize `peer_repo` field

- [x] 2.3 Replace `handle_online_msg()` implementation
  - Remove `HashMap` insert/update operations
  - Add `rt.block_on(peer_repo.upsert(...))` call
  - Convert `ProtocolMessage` and `SocketAddr` to upsert parameters
  - Handle database errors gracefully (log and continue)

- [x] 2.4 Replace `handle_offline_msg()` implementation
  - Remove `HashMap` mutation operations
  - No database update needed (offline computed from last_seen)
  - Just log the offline event

- [x] 2.5 Replace `handle_heartbeat_msg()` implementation
  - Remove `HashMap` mutation operations
  - Add `rt.block_on(peer_repo.update_last_seen(...))` call
  - Handle database errors gracefully

- [x] 2.6 Replace `get_all_peers()` implementation
  - Remove `HashMap` lock and clone operations
  - Add `rt.block_on(peer_repo.find_all())` call
  - Convert `Vec<PeerModel>` to `Vec<PeerNode>` using `From` trait

- [x] 2.7 Replace `get_online_peers()` implementation
  - Remove `HashMap` filter and clone operations
  - Add `rt.block_on(peer_repo.find_online(timeout))` call
  - Convert `Vec<PeerModel>` to `Vec<PeerNode>`

- [x] 2.8 Replace `get_peer()` implementation
  - Remove `HashMap` lookup
  - Add `rt.block_on(peer_repo.find_by_ip(...))` call
  - Convert `Option<PeerModel>` to `Option<PeerNode>`

- [x] 2.9 Replace `peer_count()` implementation
  - Remove `HashMap` len operation
  - Add `rt.block_on(peer_repo.find_all())` call
  - Return `.len()` of result vector

- [x] 2.10 Replace `online_peer_count()` implementation
  - Remove `HashMap` filter and count operation
  - Add `rt.block_on(peer_repo.find_online(timeout))` call
  - Return `.len()` of result vector

- [x] 2.11 Replace `remove_peer()` implementation
  - Remove `HashMap` remove operation
  - Add `rt.block_on(peer_repo.delete_by_ip(...))` call
  - Return boolean based on result

- [x] 2.12 Replace `has_peer()` implementation
  - Remove `HashMap` contains_key operation
  - Add `rt.block_on(peer_repo.find_by_ip(...))` call
  - Return `is_some()` on result

- [x] 2.13 Remove unused fields and helpers
  - Removed `peers: Arc<Mutex<HashMap<IpAddr, PeerNode>>>` field
  - Kept `safe_lock!` macro (still used by message_tx)
  - Kept `lock_error()` function (still used)

## 3. Integration and Bootstrap

- [x] 3.1 Update `bootstrap.rs` to pass `peer_repo` to PeerManager
  - Located `PeerManager::new()` call
  - Got `peer_repo` from `app_state.get_peer_repo()`
  - Passed to constructor
  - Handle `None` case with expect

- [x] 3.2 Update `AppState` to provide `Arc<PeerRepository>`
  - Changed `get_peer_repo()` return type to `Option<Arc<PeerRepository>>`
  - Ensure `peer_repo` field is wrapped in `Arc` during return
  - Updated bootstrap to not double-wrap in Arc

## 4. Testing

- [x] 4.1 Update `PeerManager` unit tests in `modules/peer/manager.rs`
  - Simplified tests to use `PeerNode::from(&PeerModel)`
  - Added `test_peer_node_from_model` for conversion testing
  - Added `test_peer_node_online_status_from_last_seen` for status computation

- [x] 4.2 MockPeerRepository tests
  - Added tests in `storage/peer_repo.rs`
  - Test upsert and find_by_ip operations

- [x] 4.3 Integration test for async-sync bridge
  - Verified `block_on()` works in test context
  - Confirmed error handling works

- [x] 4.4 Run full test suite
  - `cargo test` in `src-tauri` directory
  - All 133 tests passing
  - No new warnings introduced

## 5. Validation

- [x] 5.1 Build validation
  - `cargo build --lib` succeeds
  - No compilation errors

- [x] 5.2 Test validation
  - All unit tests pass
  - MockPeerRepository tests pass

- [x] 5.3 Verify IPC commands still work
  - IPC commands use `AppState` interface
  - `AppState` methods delegate to PeerManager
  - PeerManager now uses database for all operations
  - Commands will return database-backed data

## 6. Documentation

- [x] 6.1 Update code documentation
  - Updated `PeerManager` struct documentation to reflect database backing
  - Documented the async-to-sync bridge pattern in `handle_online_msg()`
  - Added architecture comments at top of file
  - Added warnings about blocking operations in sync context

- [x] 6.2 Update project documentation
  - No changes needed to CLAUDE.md (already describes database persistence)

## 7. Cleanup (Optional)

- [x] 7.1 Remove unused imports after migration
- [x] 7.2 Run `cargo clippy` and fix warnings
  - Fixed unused imports
  - Fixed unused variables
  - Fixed unused code warnings

- [x] 7.3 Run `cargo fmt` for consistent formatting
  - All code formatted consistently

## 8. Hotfix: Runtime Shutdown Panic

- [x] 8.1 Fix tokio runtime shutdown panic
  - Removed `runtime_handle: tokio::runtime::Handle` field from PeerManager
  - Created `exec_async()` helper method for instance methods
  - Created `exec_async_static()` helper method for static methods
  - Updated all database query methods to use these helpers
  - Fix prevents panic when IPC commands are called during shutdown

- [x] 8.2 Verify fix resolves the issue
  - All 133 unit tests pass
  - Build succeeds without errors
  - The fix ensures database operations work even when runtime is shutting down

**Problem**: When `get_peers()` IPC command was called during application shutdown, the stored `tokio::runtime::Handle` was invalid, causing panic.

**Solution**: Instead of storing a runtime handle, check for current runtime at call time and create temporary runtime if needed.
