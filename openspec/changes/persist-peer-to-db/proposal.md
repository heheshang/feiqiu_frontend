# Change: Persist PeerManager to Database

## Why

The current `PeerManager` stores all peer information in an in-memory `HashMap<IpAddr, PeerNode>`. This approach has several critical limitations:

1. **Data loss on restart**: All peer information (username, hostname, groups, nickname, avatar) is lost when the application restarts, requiring peers to be rediscovered from scratch
2. **No historical tracking**: Cannot query previously seen peers or track peer activity over time
3. **Inconsistent with other features**: The contacts feature already uses database persistence (via `PeerRepository`), creating an architectural inconsistency
4. **Limited query capabilities**: Cannot perform complex queries (e.g., "find all peers seen in the last 24 hours")
5. **Poor offline experience**: Users cannot see peer history when offline

A `PeerRepository` already exists in `storage/peer_repo.rs` with full CRUD operations, but `PeerManager` does not use it. This change will migrate all peer operations in `PeerManager` to use the database while maintaining backward compatibility with the IPC command layer.

## What Changes

- **MODIFIED**: `PeerManager` in `modules/peer/manager.rs` will use `PeerRepository` for all peer storage operations instead of in-memory `HashMap`
- **MODIFIED**: Peer state transitions (online/offline) will persist to database immediately
- **MODIFIED**: Peer discovery messages will update database records instead of memory-only structures
- **MODIFIED**: All `PeerManager` query methods (`get_all_peers`, `get_online_peers`, `get_peer`, etc.) will read from database
- **REMOVED**: In-memory `peers: Arc<Mutex<HashMap<IpAddr, PeerNode>>>` field from `PeerManager`
- **ADDED**: Database-backed cache layer for performance optimization (optional read cache)
- **BREAKING**: `PeerManager` constructor will require `PeerRepository` instead of just `PeerDiscovery`

## Impact

- **Affected specs**: New `peer-discovery` capability will be created
- **Affected code**:
  - `feiqiu/src-tauri/src/modules/peer/manager.rs` - Core peer management logic
  - `feiqiu/src-tauri/src/bootstrap.rs` - Peer manager initialization
  - `feiqiu/src-tauri/src/state/app_state.rs` - May need adjustments for repository access
  - `feiqiu/src-tauri/src/modules/message/handler.rs` - Already uses `peer_repo`, may need coordination
  - `feiqiu/src-tauri/src/commands/peer.rs` - IPC commands continue to work via `AppState` interface

- **Benefits**:
  - Peer information persists across application restarts
  - Historical peer data available for querying
  - Consistent architecture with contacts feature
  - Better offline user experience
  - Foundation for future features (peer statistics, activity timeline)

- **Risks**:
  - Database I/O latency may affect peer discovery responsiveness (mitigated by optional cache)
  - Existing unit tests for `PeerManager` will need updates
  - Need to ensure database operations are non-blocking in async context
