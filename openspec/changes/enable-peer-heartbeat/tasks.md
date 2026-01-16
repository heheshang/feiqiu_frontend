# Implementation Tasks: Enable Peer Heartbeat

This document breaks down the implementation of peer heartbeat and automatic offline detection into ordered, verifiable tasks.

## Task Overview

**Total Tasks:** 24
**Estimated Complexity:** Medium
**Dependencies:** None (purely additive feature)

---

## Phase 1: Heartbeat Mechanism

### Task 1.1: Create heartbeat module structure

**File:** `src-tauri/src/modules/peer/heartbeat.rs` (NEW)

**Description:** Create the heartbeat module with core data structures.

**Implementation:**
- Create `HeartbeatTask` struct with fields: interval, discovery, peer_map, metrics
- Create `HeartbeatMetrics` struct with: heartbeats_sent (AtomicU64), last_heartbeat_at
- Implement `new()` constructor function
- Add module declaration to `src-tauri/src/modules/peer/mod.rs`

**Validation:**
- Module compiles without errors
- Structs have appropriate visibility (pub where needed)

**Dependencies:** None

---

### Task 1.2: Implement heartbeat broadcast logic

**File:** `src-tauri/src/modules/peer/heartbeat.rs`

**Description:** Implement the core heartbeat sending functionality.

**Implementation:**
- Implement `send_heartbeat()` method that calls `discovery.announce_online()`
- Update metrics: increment `heartbeats_sent`, update `last_heartbeat_at`
- Add DEBUG logging: "Heartbeat sent (interval: Xs, total: Y)"
- Handle errors: log at WARN level, don't propagate

**Validation:**
- Unit test verifies `announce_online()` called
- Unit test verifies metrics updated
- Unit test verifies error handling

**Dependencies:** Task 1.1

---

### Task 1.3: Implement heartbeat task loop

**File:** `src-tauri/src/modules/peer/heartbeat.rs`

**Description:** Implement the async task that runs heartbeat at intervals.

**Implementation:**
- Implement `run(shutdown: Receiver<()>) -> Result<()>` method
- Use `tokio::time::sleep()` for interval timing
- Use `select!` macro to handle both sleep and shutdown signal
- Loop until shutdown received
- Call `send_heartbeat()` on each interval

**Validation:**
- Unit test with mocked sleep verifies heartbeat called at correct intervals
- Integration test verifies task stops on shutdown signal

**Dependencies:** Task 1.2

---

### Task 1.4: Integrate heartbeat task into PeerManager

**File:** `src-tauri/src/modules/peer/manager.rs`

**Description:** Spawn heartbeat task when PeerManager starts.

**Implementation:**
- Add `shutdown_tx: Option<broadcast::Sender<()>>` field to PeerManager
- Create shutdown channel in `start()` method
- Create HeartbeatTask instance with config interval
- Subscribe to shutdown channel
- Spawn task with `tokio::spawn()`
- Add error handling in task: log error but don't crash

**Validation:**
- Manual test: verify heartbeat broadcasts appear in logs
- Manual test: verify task stops cleanly when PeerManager stops

**Dependencies:** Task 1.3

---

## Phase 2: Timeout Detection

### Task 2.1: Create timeout checker module structure

**File:** `src-tauri/src/modules/peer/timeout_checker.rs` (NEW)

**Description:** Create the timeout checker module with core data structures.

**Implementation:**
- Create `TimeoutChecker` struct with fields: check_interval, peer_timeout, peer_map, event_tx, metrics
- Create `TimeoutMetrics` struct with: timeouts_detected, last_check_at
- Implement `new()` constructor function
- Add module declaration to `src-tauri/src/modules/peer/mod.rs`

**Validation:**
- Module compiles without errors
- Structs have appropriate visibility

**Dependencies:** None

---

### Task 2.2: Implement timeout detection logic

**File:** `src-tauri/src/modules/peer/timeout_checker.rs`

**Description:** Implement the core timeout checking functionality.

**Implementation:**
- Implement `check_timeouts() -> Vec<IpAddr>` method
- Lock peer_map with try_lock_for(1 second) to avoid blocking
- Iterate through all peers
- For each Online peer: calculate `now - last_seen`
- If elapsed > peer_timeout: change status to Offline, collect IP
- Emit `PeerOffline` event for each timed-out peer
- Update metrics
- Add INFO logging for each timeout

**Validation:**
- Unit test with peer map containing old peer verifies status change
- Unit test with recent peer verifies no status change
- Unit test verifies event emission

**Dependencies:** Task 2.1

---

### Task 2.3: Implement timeout checker task loop

**File:** `src-tauri/src/modules/peer/timeout_checker.rs`

**Description:** Implement the async task that runs timeout checks.

**Implementation:**
- Implement `run(shutdown: Receiver<()>) -> Result<()>` method
- Use `tokio::time::sleep()` for interval timing (30 seconds)
- Use `select!` macro to handle both sleep and shutdown
- Loop until shutdown received
- Call `check_timeouts()` on each interval

**Validation:**
- Unit test verifies check called at correct intervals
- Integration test verifies task stops on shutdown

**Dependencies:** Task 2.2

---

### Task 2.4: Integrate timeout checker into PeerManager

**File:** `src-tauri/src/modules/peer/manager.rs`

**Description:** Spawn timeout checker task when PeerManager starts.

**Implementation:**
- In `start()` method, after heartbeat task spawn
- Create TimeoutChecker instance
- Subscribe to shutdown channel
- Spawn task with `tokio::spawn()`
- Add error handling

**Validation:**
- Manual test: verify timeout detection in logs
- Integration test: start peer, kill it, verify offline detected

**Dependencies:** Task 2.3, Task 1.4

---

## Phase 3: Peer Cleanup

### Task 3.1: Create cleanup module structure

**File:** `src-tauri/src/modules/peer/cleanup.rs` (NEW)

**Description:** Create the cleanup module with core data structures.

**Implementation:**
- Create `CleanupTask` struct with fields: cleanup_interval, offline_retention, peer_map, metrics
- Create `CleanupMetrics` struct with: peers_cleaned_up, last_cleanup_at
- Implement `new()` constructor function
- Add module declaration to `src-tauri/src/modules/peer/mod.rs`

**Validation:**
- Module compiles without errors

**Dependencies:** None

---

### Task 3.2: Implement cleanup logic

**File:** `src-tauri/src/modules/peer/cleanup.rs`

**Description:** Implement the core cleanup functionality.

**Implementation:**
- Implement `cleanup_stale_peers() -> usize` method
- Lock peer_map
- Iterate through all peers
- For each Offline peer: calculate `now - last_seen`
- If elapsed > offline_retention: remove from map, increment counter
- Never remove localhost peer
- Update metrics
- Add INFO logging: "Cleanup completed: removed X peers"

**Validation:**
- Unit test with stale offline peers verifies removal
- Unit test with recent offline peer verifies retention
- Unit test verifies localhost never removed

**Dependencies:** Task 3.1

---

### Task 3.3: Implement cleanup task loop

**File:** `src-tauri/src/modules/peer/cleanup.rs`

**Description:** Implement the async task that runs periodic cleanup.

**Implementation:**
- Implement `run(shutdown: Receiver<()>) -> Result<()>` method
- Use `tokio::time::sleep()` for interval timing (5 minutes)
- Use `select!` macro to handle both sleep and shutdown
- Loop until shutdown received
- Call `cleanup_stale_peers()` on each interval

**Validation:**
- Unit test verifies cleanup called at correct intervals

**Dependencies:** Task 3.2

---

### Task 3.4: Integrate cleanup task into PeerManager

**File:** `src-tauri/src/modules/peer/manager.rs`

**Description:** Spawn cleanup task when PeerManager starts.

**Implementation:**
- In `start()` method, after timeout checker spawn
- Create CleanupTask instance
- Subscribe to shutdown channel
- Spawn task with `tokio::spawn()`
- Add error handling

**Validation:**
- Manual test: verify cleanup in logs after 5+ minutes
- Or use short interval for testing

**Dependencies:** Task 3.3, Task 2.4

---

## Phase 4: IPC Commands

### Task 4.1: Create DTOs for config and metrics

**File:** `src-tauri/src/commands/peer.rs`

**Description:** Define DTO structures for heartbeat configuration and metrics.

**Implementation:**
- Create `HeartbeatConfigDto` struct with: heartbeat_interval, peer_timeout
- Create `PeerHealthStatsDto` struct with: heartbeats_sent, timeouts_detected, peers_cleaned_up, last_heartbeat_at, last_timeout_check
- Add `#[serde(rename_all = "camelCase")]` for frontend compatibility
- Add `#[derive(Clone, Debug, Serialize, Deserialize)]`

**Validation:**
- Code compiles
- Serialization test produces correct JSON

**Dependencies:** None

---

### Task 4.2: Implement get_heartbeat_config command

**File:** `src-tauri/src/commands/peer.rs`

**Description:** Create IPC command to retrieve current heartbeat configuration.

**Implementation:**
- Create `#[tauri::command]` function `get_heartbeat_config`
- Accept `AppState` as parameter
- Read config from state
- Return `HeartbeatConfigDto`
- Add doc comment with frontend usage example

**Validation:**
- Frontend can invoke command and receives config
- Values match AppConfig

**Dependencies:** Task 4.1

---

### Task 4.3: Implement update_heartbeat_interval command

**File:** `src-tauri/src/commands/peer.rs`

**Description:** Create IPC command to update heartbeat interval.

**Implementation:**
- Create `#[tauri::command]` async function `update_heartbeat_interval`
- Accept `AppState` and `interval_secs: u64`
- Validate interval: 10 <= interval <= 600
- Update AppConfig.heartbeat_interval
- Persist to database via ConfigRepository
- Restart heartbeat task with new interval
- Return Ok or Error

**Validation:**
- Valid interval accepted and persisted
- Invalid interval rejected with error
- Heartbeat task uses new interval

**Dependencies:** Task 4.2

---

### Task 4.4: Implement update_peer_timeout command

**File:** `src-tauri/src/commands/peer.rs`

**Description:** Create IPC command to update peer timeout.

**Implementation:**
- Create `#[tauri::command]` async function `update_peer_timeout`
- Accept `AppState` and `timeout_secs: u64`
- Validate timeout: timeout > heartbeat_interval
- Update AppConfig.peer_timeout
- Persist to database
- Restart timeout checker with new timeout
- Return Ok or Error

**Validation:**
- Valid timeout accepted and persisted
- Timeout less than heartbeat rejected
- Timeout checker uses new timeout

**Dependencies:** Task 4.3

---

### Task 4.5: Implement get_peer_health_stats command

**File:** `src-tauri/src/commands/peer.rs`

**Description:** Create IPC command to retrieve peer health statistics.

**Implementation:**
- Create `#[tauri::command]` function `get_peer_health_stats`
- Accept `AppState` as parameter
- Collect metrics from all three tasks
- Return `PeerHealthStatsDto`
- Convert Option<SystemTime> to i64 milliseconds

**Validation:**
- Returns correct metrics
- Timestamps properly converted

**Dependencies:** Task 4.1

---

### Task 4.6: Register new IPC commands

**File:** `src-tauri/src/lib.rs`

**Description:** Register new commands in Tauri invoke handler.

**Implementation:**
- Add new commands to `invoke_handler!` macro:
  - `get_heartbeat_config`
  - `update_heartbeat_interval`
  - `update_peer_timeout`
  - `get_peer_health_stats`

**Validation:**
- Application compiles
- Commands can be invoked from frontend

**Dependencies:** Tasks 4.2, 4.3, 4.4, 4.5

---

## Phase 5: Frontend Integration

### Task 5.1: Add peer health stats API wrappers

**File:** `src/lib/api/peers.ts` (MODIFY)

**Description:** Add TypeScript wrappers for new IPC commands.

**Implementation:**
- Export `getHeartbeatConfig()` function
- Export `updateHeartbeatInterval()` function
- Export `updatePeerTimeout()` function
- Export `getPeerHealthStats()` function
- Add proper TypeScript types
- Add error handling

**Validation:**
- Functions compile
- Return types match DTOs

**Dependencies:** Task 4.6

---

### Task 5.2: Ensure event types are defined

**File:** `src/lib/events/types.ts` (MODIFY if needed)

**Description:** Verify or add event types for peer offline/online.

**Implementation:**
- Verify `PeerOfflineEvent` type exists with: type, peerIp, username, reason, timestamp
- Verify `PeerOnlineEvent` type exists with: type, peerIp, username, timestamp
- Add if missing

**Validation:**
- Types compile
- Match backend event structure

**Dependencies:** None

---

### Task 5.3: Update Contacts component for offline events

**File:** `src/components/contacts/Contacts.tsx` (MODIFY)

**Description:** Ensure Contacts component handles peer offline events.

**Implementation:**
- Subscribe to `peer-offline` events
- Update contact status to offline
- Update online count
- Show "last seen" time

**Validation:**
- Manual test: kill peer, verify status updates
- Online count decrements

**Dependencies:** Task 5.2

---

### Task 5.4: Update ContactItem to show last seen prominently

**File:** `src/components/contacts/ContactItem.tsx` (MODIFY)

**Description:** Display last seen time for offline contacts.

**Implementation:**
- If status is offline, show "X minutes ago online" or similar
- Use existing `formatLastSeen()` utility
- Make last seen timestamp visible

**Validation:**
- Visual inspection: last seen shown for offline contacts

**Dependencies:** Task 5.3

---

## Phase 6: Testing

### Task 6.1: Add unit tests for heartbeat task

**File:** `src-tauri/src/modules/peer/heartbeat.rs` (tests module)

**Description:** Add comprehensive unit tests for heartbeat functionality.

**Implementation:**
- Test heartbeat sent at correct interval (mock time)
- Test metrics updated correctly
- Test error handling when broadcast fails
- Test shutdown signal received

**Validation:**
- All tests pass
- Coverage >80% for heartbeat module

**Dependencies:** Task 1.4

---

### Task 6.2: Add unit tests for timeout checker

**File:** `src-tauri/src/modules/peer/timeout_checker.rs` (tests module)

**Description:** Add comprehensive unit tests for timeout detection.

**Implementation:**
- Test peer marked offline after timeout
- Test peer kept online if recent activity
- Test multiple peers checked in batch
- Test localhost skipped
- Test event emission

**Validation:**
- All tests pass
- Coverage >80% for timeout checker module

**Dependencies:** Task 2.4

---

### Task 6.3: Add unit tests for cleanup task

**File:** `src-tauri/src/modules/peer/cleanup.rs` (tests module)

**Description:** Add comprehensive unit tests for cleanup functionality.

**Implementation:**
- Test stale offline peers removed
- Test recent offline peers kept
- Test localhost never removed
- Test cleanup count returned

**Validation:**
- All tests pass
- Coverage >80% for cleanup module

**Dependencies:** Task 3.4

---

### Task 6.4: Integration test for end-to-end timeout detection

**File:** `src-tauri/tests/peer_timeout.rs` (NEW)

**Description:** Create integration test simulating peer crash and detection.

**Implementation:**
- Start two PeerManager instances
- Add peer to first manager
- Simulate time passing (advance last_seen)
- Run timeout checker
- Verify peer marked offline
- Verify event emitted

**Validation:**
- Test passes consistently
- Timeout detected within expected time

**Dependencies:** Task 6.2

---

### Task 6.5: Manual testing checklist

**Description:** Create and execute manual testing checklist.

**Testing Scenarios:**

1. **Heartbeat Verification**
   - [ ] Start application, check logs for initial heartbeat
   - [ ] Wait 60 seconds, verify second heartbeat in logs
   - [ ] Check network capture for BR_ENTRY broadcasts

2. **Timeout Detection**
   - [ ] Start application on two machines
   - [ ] Verify both peers show as online
   - [ ] Force quit one peer (kill process)
   - [ ] Wait 3+ minutes (timeout + grace)
   - [ ] Verify peer shows as offline on other machine
   - [ ] Check logs for timeout detection message

3. **Cleanup Verification**
   - [ ] Add peers to peer list
   - [ ] Set some peers offline with old last_seen
   - [ ] Wait 5+ minutes
   - [ ] Verify old peers removed from memory
   - [ ] Verify recent offline peers kept

4. **Configuration Changes**
   - [ ] Change heartbeat interval to 30 seconds
   - [ ] Verify new interval used
   - [ ] Restart application
   - [ ] Verify configuration persisted

5. **Frontend Updates**
   - [ ] Verify contact list updates when peer goes offline
   - [ ] Verify last seen time displayed correctly
   - [ ] Verify online count updates

**Validation:**
- All manual tests pass
- Document any issues found

**Dependencies:** All previous tasks

---

## Task Dependencies Graph

```
Phase 1: Heartbeat
├── 1.1 → 1.2 → 1.3 → 1.4

Phase 2: Timeout Detection
├── 2.1 → 2.2 → 2.3 → 2.4 (depends on 1.4)

Phase 3: Cleanup
├── 3.1 → 3.2 → 3.3 → 3.4 (depends on 2.4)

Phase 4: IPC Commands
├── 4.1 → 4.2 → 4.3 → 4.4 → 4.5 → 4.6

Phase 5: Frontend
├── 5.1 (depends on 4.6)
├── 5.2
├── 5.3 (depends on 5.2)
└── 5.4 (depends on 5.3)

Phase 6: Testing
├── 6.1 (depends on 1.4)
├── 6.2 (depends on 2.4)
├── 6.3 (depends on 3.4)
├── 6.4 (depends on 6.2)
└── 6.5 (depends on all)
```

---

## Parallelization Opportunities

The following tasks can be done in parallel:

**Parallel Group A:**
- Task 1.1 (Heartbeat module structure)
- Task 2.1 (Timeout checker module structure)
- Task 3.1 (Cleanup module structure)
- Task 4.1 (Create DTOs)
- Task 5.2 (Event types)

**Parallel Group B (after A):**
- Task 1.2, 2.2, 3.2 (Implement core logic for each module)
- Task 5.1 (API wrappers - after DTOs)

**Parallel Group C (after B):**
- Task 1.3, 2.3, 3.3 (Implement task loops)

**Sequential Required:**
- Integration tasks (1.4, 2.4, 3.4) must be in order
- IPC commands (4.2-4.5) have dependencies
- Frontend tasks (5.3-5.4) depend on event types
- Tests (6.1-6.4) depend on implementation

---

## Definition of Done

Each task is complete when:
- [ ] Code implemented per description
- [ ] Code compiles without warnings
- [ ] Unit tests pass (if applicable)
- [ ] Manual verification complete (for integration tasks)
- [ ] Code reviewed (if working in team)

**Overall Feature Complete When:**
- [ ] All 24 tasks complete
- [ ] All tests passing (unit + integration)
- [ ] Manual testing checklist complete
- [ ] No memory leaks detected
- [ ] CPU usage within acceptable limits
- [ ] Documentation updated (if needed)
