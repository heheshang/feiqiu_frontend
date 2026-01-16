# Design: PeerManager Database Migration

## Context

The `PeerManager` currently maintains peer state in an `Arc<Mutex<HashMap<IpAddr, PeerNode>>>` structure. All peer discovery, online/offline transitions, and query operations work against this in-memory store. A separate `PeerRepository` already exists with full database operations but is only used by the `MessageHandler` for tracking peers during message exchanges.

### Current Architecture

```
┌─────────────────┐
│  PeerManager    │
│  (in-memory)    │
│  HashMap        │
└────────┬────────┘
         │
         │ get_peers()
         ▼
┌─────────────────┐
│  IPC Commands   │
│  (peer.rs)      │
└─────────────────┘

┌─────────────────┐
│ MessageHandler  │
└────┬────────────┘
     │
     │ peer_repo.upsert()
     ▼
┌─────────────────┐
│  PeerRepository │
│  (database)     │
└─────────────────┘
```

### Stakeholders

- **Users**: Want peer information to persist across restarts
- **Frontend**: Expects `get_peers()`, `get_online_peers()` to work immediately
- **MessageHandler**: Already uses `peer_repo`, needs coordination to avoid duplicate updates
- **Tests**: Existing unit tests for `PeerManager` need database mocking

## Goals / Non-Goals

### Goals
1. PeerManager uses database as source of truth for all peer operations
2. Minimal latency impact on peer discovery operations
3. Backward compatibility with existing IPC commands
4. Coordinate with MessageHandler to avoid redundant database writes
5. Maintain thread safety with concurrent access patterns

### Non-Goals
1. Implementing a full cache invalidation system (use simple TTL or no cache)
2. Changing the PeerRepository API (it's already well-designed)
3. Implementing peer statistics/analytics (future work)
4. Real-time database change notifications (use existing event system)

## Decisions

### Decision 1: Direct Database Access vs Cached Access

**Choice**: Direct database access with optional read cache

**Rationale**:
- **Direct access**: Simpler implementation, always consistent, SQLite is fast for read-heavy workloads
- **Cache complexity**: Cache invalidation in distributed peer discovery scenario is complex
- **SQLite performance**: For expected scale (<1000 peers), database queries are sub-millisecond
- **Future flexibility**: Can add cache layer later if profiling shows need

**Alternatives considered**:
1. **In-memory cache with TTL**: More complex, cache invalidation on peer updates
2. **Full in-memory with write-through**: Reintroduces data loss on crash
3. **Hybrid (memory for online, DB for all)**: Inconsistent state between two sources

**Implementation**:
```rust
pub struct PeerManager {
    discovery: PeerDiscovery,
    peer_repo: Arc<PeerRepository>,  // NEW: Database repository
    running: Arc<Mutex<bool>>,
    message_tx: Arc<Mutex<Option<Sender<MessageRouteRequest>>>>,
    // REMOVED: peers: Arc<Mutex<HashMap<IpAddr, PeerNode>>>
}
```

### Decision 2: Async vs Sync Database Operations

**Choice**: Use `tokio::runtime::Handle::block_on()` for async DB calls in sync context

**Rationale**:
- `PeerRepository` methods are `async` (Sea-ORM requirement)
- `PeerManager` message handling is synchronous (callback from UDP socket)
- Cannot change entire message path to async without major refactoring
- Blocking on async is acceptable for fast DB operations (<1ms)

**Alternatives considered**:
1. **Make entire PeerManager async**: Requires changing UDP transport to tokio::net::UdpSocket
2. **Spawn tasks for all DB operations**: Loses result, no error handling
3. **Channel-based async worker**: Complex, additional thread

**Implementation**:
```rust
fn handle_online_msg(msg: &ProtocolMessage, sender: SocketAddr) -> Result<()> {
    // ... existing logic ...
    let rt = tokio::runtime::Handle::try_current()
        .unwrap_or_else(|_| self.runtime_handle.clone());
    rt.block_on(async {
        peer_repo.upsert(
            ip.to_string(),
            sender.port(),
            Some(msg.sender_name.clone()),
            Some(msg.sender_host.clone()),
            chrono::Utc::now().naive_utc()
        ).await
    })?;
}
```

### Decision 3: Coordination with MessageHandler

**Choice**: MessageHandler continues to call `peer_repo.upsert()`, PeerManager does the same

**Rationale**:
- Both operate on same database, upsert is idempotent
- MessageHandler updates last_seen during messages
- PeerManager updates during discovery
- No conflict, last write wins (acceptable for this use case)

**Future improvement**: Could add a "source" field to track which system last updated peer

### Decision 4: Type Conversion Between PeerNode and PeerModel

**Choice**: Add conversion methods between `PeerNode` (in-memory type) and `PeerModel` (database type)

**Rationale**:
- `PeerNode` uses `IpAddr`, `SystemTime`, `PeerStatus` (convenient for Rust code)
- `PeerModel` uses `String`, `NaiveDateTime`, no status field (database schema)
- Need bidirectional conversion for database operations

**Implementation**:
```rust
impl From<&PeerModel> for PeerNode {
    fn from(model: &PeerModel) -> Self {
        let status = if is_online(model.last_seen) {
            PeerStatus::Online
        } else {
            PeerStatus::Offline
        };
        Self {
            ip: model.ip.parse().unwrap(),
            port: model.port as u16,
            username: model.username.clone(),
            hostname: model.hostname.clone(),
            // ... other fields
        }
    }
}
```

## Data Model

### Existing Schema (peers table)
```sql
CREATE TABLE peers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip TEXT NOT NULL UNIQUE,
    port INTEGER NOT NULL,
    username TEXT,
    hostname TEXT,
    nickname TEXT,
    avatar TEXT,
    groups TEXT,  -- JSON array
    last_seen BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT
);
```

### Status Determination
Online/offline status is **computed** from `last_seen`:
```rust
const PEER_TIMEOUT_SECONDS: i64 = 180;  // 3 minutes

fn is_online(last_seen: NaiveDateTime) -> bool {
    let timeout = chrono::Duration::seconds(PEER_TIMEOUT_SECONDS);
    let cutoff = chrono::Utc::now().naive_utc() - timeout;
    last_seen > cutoff
}
```

## Migration Plan

### Phase 1: Preparation
1. Add conversion methods between `PeerNode` and `PeerModel`
2. Add `tokio::runtime::Handle` to `PeerManager` for async DB calls
3. Update unit tests to use mock `PeerRepository`

### Phase 2: Core Migration
1. Modify `PeerManager::new()` to accept `Arc<PeerRepository>`
2. Replace `HashMap` operations with `peer_repo` calls in:
   - `handle_online_msg()` → `peer_repo.upsert()`
   - `handle_offline_msg()` → `peer_repo.update_last_seen()` (or no-op, offline is computed)
   - `handle_heartbeat_msg()` → `peer_repo.update_last_seen()`
3. Update query methods to use database:
   - `get_all_peers()` → `peer_repo.find_all()`
   - `get_online_peers()` → `peer_repo.find_online(timeout)`
   - `get_peer()` → `peer_repo.find_by_ip()`
   - `peer_count()` → `peer_repo.find_all().len()`
   - `remove_peer()` → `peer_repo.delete_by_ip()`

### Phase 3: Integration
1. Update `bootstrap.rs` to pass `peer_repo` to `PeerManager`
2. Remove in-memory `peers` field from `PeerManager`
3. Update all tests
4. Verify IPC commands still work

### Phase 4: Cleanup
1. Remove unused `safe_lock!` macro (if no other consumers)
2. Update documentation
3. Add integration tests

### Rollback Plan
If issues arise:
1. Revert `PeerManager` to use `HashMap`
2. Keep database writes for persistence (dual-write)
3. Phase out database reads in next release

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|------------|
| Database latency slows discovery | High | Profile operations; SQLite <1ms for simple queries |
| Async/sync mismatch causes blocking | Medium | Use `block_on()` carefully; monitor runtime |
| Concurrent write conflicts | Low | SQLite serializes writes; upsert is safe |
| Test complexity increases | Medium | Provide mock `PeerRepository` for unit tests |
| MessageHandler race conditions | Low | Both use same repo; upsert is idempotent |

## Open Questions

1. **Should we add an optional read cache?**
   - Decision: Start without cache, add only if profiling shows need
   - Trade-off: Complexity vs performance

2. **How to handle peer timeout configuration?**
   - Current: Hardcoded 180 seconds
   - Future: Make configurable via `AppConfig`

3. **Should we add peer activity logging?**
   - Current: No history of state changes
   - Future: Could add peer_events table for audit trail

## References

- Current implementation: `feiqiu/src-tauri/src/modules/peer/manager.rs`
- Database layer: `feiqiu/src-tauri/src/storage/peer_repo.rs`
- Schema: `feiqiu/src-tauri/src/storage/entities/peers.rs`
- MessageHandler usage: `feiqiu/src-tauri/src/modules/message/handler.rs:512-575`
