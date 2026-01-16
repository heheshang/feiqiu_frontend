# Change: Enable Peer Heartbeat and Automatic Offline Detection

## Why

The current peer management system relies entirely on passive offline detection - peers are only marked offline when they explicitly send an `IPMSG_BR_EXIT` message. This creates a critical gap where peers that crash, lose network connectivity, or experience silent failures remain marked as "online" indefinitely. Users see an inaccurate peer list, try to send messages to offline peers, and experience poor UX.

## What Changes

- Implement active heartbeat mechanism with periodic `IPMSG_BR_ENTRY` broadcasts
- Add background task to detect and mark offline peers based on timeout
- Create configurable heartbeat interval and peer timeout settings
- Add peer cleanup task to remove stale offline peers from memory
- Emit real-time events when peers transition from online to offline
- Update frontend to properly handle offline peer events
- Add metrics and monitoring for heartbeat/timeout detection

## Impact

- **User-facing**: Accurate online/offline peer status, reliable messaging
- **Technical**: Background tasks for periodic operations, improved peer management
- **Breaking**: None (purely additive)
- **Dependencies**: None (uses existing config constants)

---

## Summary

**Change ID:** `enable-peer-heartbeat`
**Status:** Draft
**Created:** 2026-01-16
**Author:** AI Assistant

## Problem Statement

### Current State

The peer management system (`feiqiu/src-tauri/src/modules/peer/`) has the following characteristics:

**What Works:**
- Peer discovery via UDP broadcast (`IPMSG_BR_ENTRY`)
- Online detection when peers announce presence
- Offline detection when peers send exit message (`IPMSG_BR_EXIT`)
- Manual peer status queries via IPC commands
- Event emission for status changes (when detected)

**What's Missing:**
- **NO heartbeat mechanism** - No periodic "I'm alive" messages
- **NO timeout-based offline detection** - Silent disconnections never detected
- **NO background monitoring** - No periodic status checks
- **NO peer cleanup** - Stale peers accumulate in memory
- **NO retry logic** - No re-announcement on network recovery

### The Gap

**Critical Issue:** Peers that experience silent failures remain "online" forever

**Scenarios Not Handled:**
1. Application crashes (force quit, segfault, power failure)
2. Network cable unplugged
3. WiFi disconnection
4. System suspend/hibernate
5. IP address change
6. Firewall rule changes

**User Impact:**
- Inaccurate peer list shows peers as online when they're not
- Messages sent to "ghost" peers appear to succeed but fail silently
- No visibility into actual peer availability
- Confusion about who is actually reachable

**Evidence from Code:**

`feiqiu/src-tauri/src/modules/peer/manager.rs:285-314` - Online messages are handled:
```rust
fn handle_online_msg(peers: &Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
                    msg: &ProtocolMessage, sender: SocketAddr) -> Result<()> {
    // Sets status to Online
    peer.status = PeerStatus::Online;
    peer.last_seen = std::time::SystemTime::now();
}
```

`feiqiu/src-tauri/src/modules/peer/manager.rs:316-330` - Offline messages are handled:
```rust
fn handle_offline_msg(peers: &Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
                     ip: IpAddr) -> Result<()> {
    if let Some(peer) = peers.get_mut(&ip) {
        peer.mark_offline();  // Only called when BR_EXIT received
    }
}
```

**No periodic checking loop exists.**

### Existing Infrastructure (Underutilized)

**Config Constants Already Defined** (`feiqiu/src-tauri/src/config/app.rs:30-34`):
```rust
pub const DEFAULT_HEARTBEAT_INTERVAL: u64 = 60;    // 60 seconds
pub const DEFAULT_PEER_TIMEOUT: u64 = 180;         // 3 minutes
```

These constants exist but are **never used** in the peer management code.

**Database Methods Available** (`feiqiu/src-tauri/src/storage/peer_repo.rs`):
- `find_online()` - Query peers active within last X seconds
- `find_offline()` - Query peers inactive for X seconds
- `cleanup_offline()` - Delete peers inactive for X seconds

These methods exist but are **never called**.

## Proposed Solution

### Phase 1: Heartbeat Mechanism

Implement periodic heartbeat broadcasts to maintain peer presence:

1. **Heartbeat Task** (`modules/peer/heartbeat.rs`)
   - Spawn background task in PeerManager::start()
   - Every `heartbeat_interval` seconds, broadcast `IPMSG_BR_ENTRY`
   - Use existing `discovery.announce_online()` method
   - Log heartbeat emissions for debugging

2. **Outgoing Heartbeat**
   - Purpose: Tell other peers "I'm still here"
   - Message: `IPMSG_BR_ENTRY` (same as initial online announcement)
   - Interval: 60 seconds (configurable)

### Phase 2: Timeout Detection

Implement background task to detect silent offline peers:

1. **Timeout Checker Task** (`modules/peer/timeout_checker.rs`)
   - Spawn background task in PeerManager::start()
   - Every 30 seconds, check all peers' `last_seen` timestamps
   - Mark peers offline if `now - last_seen > peer_timeout`
   - Emit `PeerOffline` event for each transition

2. **Detection Logic**
   - Iterate through in-memory peer map
   - Calculate elapsed time since `last_seen`
   - If exceeds `peer_timeout` (180 seconds):
     - Change status to `Offline`
     - Emit `TauriEvent::PeerOffline`
     - Log the timeout detection

### Phase 3: Peer Cleanup

Implement periodic cleanup of stale offline peers:

1. **Cleanup Task** (`modules/peer/cleanup.rs`)
   - Spawn background task in PeerManager::start()
   - Run every 5 minutes
   - Remove peers offline for >24 hours from in-memory map
   - Keep database records for history

2. **Cleanup Policy**
   - In-memory: Remove if offline >24 hours
   - Database: Keep permanently for history
   - Exception: Keep manually added contacts (if implemented)

### Phase 4: Frontend Integration

Update frontend to properly handle offline events:

1. **Event Handling**
   - Ensure `peer-offline` events update contact list
   - Show toast notification when peer goes offline
   - Update online count in statistics

2. **UI Improvements**
   - Show "last seen" time for offline contacts
   - Visual distinction between "recently offline" vs "long gone"
   - Option to filter out long-offline contacts

### Phase 5: Configuration and Monitoring

Make heartbeat/timeout configurable and observable:

1. **IPC Commands**
   - `get_heartbeat_config()` - Return current interval/timeout settings
   - `update_heartbeat_interval()` - Change heartbeat frequency
   - `update_peer_timeout()` - Change offline detection threshold
   - `get_peer_health_stats()` - Return metrics (heartbeat sent, timeouts detected, etc.)

2. **Metrics**
   - Count heartbeats sent
   - Count timeouts detected
   - Track peer transition frequency
   - Monitor background task health

## Affected Components

### Backend Files to Create

| Path | Purpose |
|------|---------|
| `src-tauri/src/modules/peer/heartbeat.rs` | Heartbeat broadcasting logic |
| `src-tauri/src/modules/peer/timeout_checker.rs` | Timeout detection logic |
| `src-tauri/src/modules/peer/cleanup.rs` | Peer cleanup logic |

### Backend Files to Modify

| Path | Changes |
|------|---------|
| `src-tauri/src/modules/peer/manager.rs` | Spawn background tasks in `start()`, add getters for config |
| `src-tauri/src/modules/peer/mod.rs` | Export new modules |
| `src-tauri/src/commands/peer.rs` | Add config/metrics commands |
| `src-tauri/src/state/events.rs` | Ensure offline events emitted |
| `src-tauri/src/state/app_state.rs` | Add heartbeat metrics to state |
| `src-tauri/src/lib.rs` | Register new IPC commands |

### Frontend Files to Modify

| Path | Changes |
|------|---------|
| `src/components/contacts/Contacts.tsx` | Ensure offline events handled |
| `src/components/contacts/ContactItem.tsx` | Show last seen time prominently |
| `src/lib/events/types.ts` | Add event types if missing |
| `src/lib/api/peers.ts` | Add config/metrics API calls |

### Configuration Files

| Path | Changes |
|------|---------|
| `src-tauri/src/config/app.rs` | Use existing constants (already defined) |

## Dependencies

### Required (Already Installed)
- `tokio` - Async runtime for background tasks (already in use)
- `tracing` - Logging for heartbeat/debug (already in use)
- `chrono` - Timestamp calculations (already in use)

### No New Dependencies Required

All necessary crates are already in use for other features.

## Success Criteria

### Functional Requirements

- [ ] Heartbeat broadcasts sent every 60 seconds (configurable)
- [ ] Peers marked offline after 180 seconds of inactivity (configurable)
- [ ] PeerOffline events emitted when timeout detected
- [ ] Offline peers removed from memory after 24 hours
- [ ] Frontend updates UI within 1 second of offline event
- [ ] Configuration changes take effect immediately

### Technical Requirements

- [ ] Background tasks use Tokio `spawn` with proper error handling
- [ ] No race conditions between heartbeat and timeout checking
- [ ] Peer map access properly synchronized via `Arc<Mutex<>>`
- [ ] Database updates for status changes atomic
- [ ] Event emission non-blocking (use try_send or channels)
- [ ] Tasks stop gracefully on app shutdown

### Performance Requirements

- [ ] Heartbeat broadcasts <1% CPU usage
- [ ] Timeout checking <100ms for 1000 peers
- [ ] Memory usage stable (no leaks from background tasks)
- [ ] No UI freezing during background operations

### Observability Requirements

- [ ] All heartbeat emissions logged at DEBUG level
- [ ] All timeout detections logged at INFO level
- [ ] All peer status transitions logged at INFO level
- [ ] Metrics available via IPC command

## Alternatives Considered

### Alternative 1: Use IPMsg heartbeat message type

Some IPMsg implementations use a specific heartbeat message type instead of re-broadcasting BR_ENTRY.

**Pros:**
- More explicit about intent
- Distinguishes initial join from heartbeat

**Cons:**
- Not standard in IPMsg protocol
- Legacy clients may not recognize it
- Adds complexity to message handling

**Decision:** Rejected - Use standard BR_ENTRY for maximum compatibility.

### Alternative 2: Request-response heartbeat (ping/pong)

Send direct unicast pings to each peer and wait for response.

**Pros:**
- More accurate detection (confirm bidirectional connectivity)
- Detect asymmetric network failures

**Cons:**
- O(n²) network traffic for n peers
- Higher latency
- More complex implementation
- May overload network with many peers

**Decision:** Rejected - Too complex for LAN broadcast-based discovery.

### Alternative 3: Reactive timeout only (no heartbeat)

Only detect timeout when we receive a message from a peer, don't actively heartbeat.

**Pros:**
- Less network traffic
- Simpler implementation

**Cons:**
- Slower detection (must wait for incoming message)
- Doesn't help other peers detect our presence
- Asymmetric behavior

**Decision:** Rejected - Heartbeat is necessary for symmetric detection.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Heartbeat traffic floods network | Medium | Make interval configurable, default 60s is conservative |
| False positives (mark active peers offline) | High | Set timeout to 3x heartbeat interval, make configurable |
| Race condition between heartbeat and timeout | Low | Use atomic operations, check last_seen after marking offline |
| Background task crashes | Medium | Use tokio::spawn with error recovery, monitor task health |
| Memory leak from accumulating events | Low | Use bounded channels, drop events if frontend not consuming |
| Wakes system from power save | Low | Use Tokio time that respects system power state |

## Testing Strategy

### Unit Tests

1. **Heartbeat Logic**
   - Verify heartbeat broadcast sent at correct interval
   - Test configurable interval changes take effect
   - Mock discovery to verify announce_online called

2. **Timeout Detection**
   - Test peer marked offline after timeout
   - Test peer not marked offline if recent activity
   - Test multiple peers checked in batch

3. **Cleanup Logic**
   - Test old offline peers removed
   - Test recent offline peers kept
   - Test manually added contacts preserved

### Integration Tests

1. **End-to-End Scenario**
   - Start two peers
   - Kill one peer (simulate crash)
   - Verify other peer detects offline within timeout + 1 interval

2. **Network Recovery**
   - Start peer, disconnect network, reconnect
   - Verify peer re-announces and comes back online

3. **Configuration Changes**
   - Change heartbeat interval at runtime
   - Verify new interval used

### Manual Testing

1. **Real LAN Test**
   - Run on 3+ physical machines
   - Pull network cable on one machine
   - Verify others detect offline

2. **Stress Test**
   - Run with 100+ peers (mocked)
   - Monitor CPU/memory usage

## Related Specs

This change creates the following new spec:

- **NEW:** `peer-heartbeat` - Heartbeat and timeout detection requirements

This change modifies the following specs:

- **MODIFY:** `contacts-list` - Add last seen time handling for timeout-detected offline
- **MODIFY:** `basic-info-config` - Add heartbeat/timeout configuration options

## Open Questions

1. Should heartbeat interval be user-configurable in the UI, or developer-configurable only?
   - **Recommendation:** Developer-configurable only (advanced settings), not exposed in main UI

2. Should we emit a notification/toast when a peer goes offline due to timeout?
   - **Recommendation:** No, too noisy. Just update the status indicator silently.

3. What should happen to messages queued for a peer that times out?
   - **Recommendation:** Messages remain queued, error returned when delivery attempted.

4. Should the cleanup interval be configurable?
   - **Recommendation:** Yes, add to AppConfig as `offline_peer_cleanup_interval`.

5. Should we persist heartbeat/timeout metrics to database?
   - **Recommendation:** No, keep in-memory only. Stats reset on restart.
