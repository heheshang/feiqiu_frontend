# Design: Peer Heartbeat and Automatic Offline Detection

## Overview

This document describes the architectural design for implementing peer heartbeat and automatic offline detection in FeiQiu. The solution adds three background tasks to the existing peer management system while maintaining compatibility with the IPMsg protocol and current architecture.

## Architecture

### System Context

```
┌─────────────────────────────────────────────────────────────────┐
│                         FeiQiu Application                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────┐      ┌──────────────────────────────────┐    │
│  │   Frontend    │      │         Backend (Rust)            │    │
│  │   (React)     │      │                                  │    │
│  │               │      │  ┌────────────────────────────┐  │    │
│  │  - Contacts   │◄─────┤  │   PeerManager              │  │    │
│  │    List       │ IPC │  │                            │  │    │
│  │  - Events     │      │  │  ┌──────────────────────┐ │  │    │
│  │    Handler    │      │  │  │ Peer Map (Arc<Mutex)│ │  │    │
│  └───────────────┘      │  │  └──────────────────────┘ │  │    │
│                         │  │                            │  │    │
│                         │  │  ┌──────────────────────┐ │  │    │
│                         │  │  │ Discovery Service    │ │  │    │
│                         │  │  └──────────────────────┘ │  │    │
│                         │  └────────────────────────────┘  │    │
│                         │                                  │    │
│                         │  ┌────────────────────────────┐  │    │
│                         │  │  Background Tasks          │  │    │
│                         │  │  ┌──────────────────────┐ │  │    │
│                         │  │  │ Heartbeat Task       │ │  │    │
│                         │  │  │ (every 60s)          │ │  │    │
│                         │  │  └──────────────────────┘ │  │    │
│                         │  │  ┌──────────────────────┐ │  │    │
│                         │  │  │ Timeout Checker      │ │  │    │
│                         │  │  │ (every 30s)          │ │  │    │
│                         │  │  └──────────────────────┘ │  │    │
│                         │  │  ┌──────────────────────┐ │  │    │
│                         │  │  │ Cleanup Task         │ │  │    │
│                         │  │  │ (every 5 min)        │ │  │    │
│                         │  │  └──────────────────────┘ │  │    │
│                         │  └────────────────────────────┘  │    │
│                         │                                  │    │
│                         └──────────────────────────────────┘    │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘

                            ┌─────────────────┐
                            │   LAN Network   │
                            │                 │
                            │  UDP Broadcast  │
                            │  Port 2425      │
                            └─────────────────┘
                                   ▲
                                   │
                         ┌─────────┴─────────┐
                         │   Other Peers     │
                         │   (IPMsg/FeiQiu)  │
                         └───────────────────┘
```

## Component Design

### 1. Heartbeat Task

**Module:** `src-tauri/src/modules/peer/heartbeat.rs`

**Purpose:** Periodically broadcast peer presence to maintain visibility in the peer list.

**Responsibility:**
- Send `IPMSG_BR_ENTRY` broadcast at configured interval
- Update own last_seen timestamp in peer map
- Track heartbeat count for metrics

**Structure:**
```rust
pub struct HeartbeatTask {
    interval: Duration,
    discovery: Arc<Discovery>,
    peer_map: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
    metrics: Arc<HeartbeatMetrics>,
}

impl HeartbeatTask {
    pub fn new(
        interval_secs: u64,
        discovery: Arc<Discovery>,
        peer_map: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
    ) -> Self;

    pub async fn run(&self, shutdown: Receiver<()>) -> Result<()>;

    fn send_heartbeat(&self) -> Result<()>;
}

pub struct HeartbeatMetrics {
    pub heartbeats_sent: AtomicU64,
    pub last_heartbeat_at: Mutex<Option<SystemTime>>,
}
```

**Algorithm:**
```
loop {
    select! {
        _ = sleep(interval) => {
            broadcast IPMSG_BR_ENTRY
            update own last_seen timestamp
            increment heartbeat counter
            log at DEBUG level
        }
        _ = shutdown.recv() => {
            break gracefully
        }
    }
}
```

**Configuration:**
- Interval from `AppConfig.heartbeat_interval` (default: 60s)
- Configurable via `update_heartbeat_interval()` IPC command

**Error Handling:**
- Log broadcast failures at WARN level
- Continue task despite transient failures
- Do not exit on single failure

---

### 2. Timeout Checker Task

**Module:** `src-tauri/src/modules/peer/timeout_checker.rs`

**Purpose:** Detect peers that have silently gone offline (crash, network loss, etc.).

**Responsibility:**
- Check all peers' last_seen timestamps
- Mark peers offline if inactive beyond timeout
- Emit PeerOffline events for each transition
- Track timeout count for metrics

**Structure:**
```rust
pub struct TimeoutChecker {
    check_interval: Duration,
    peer_timeout: Duration,
    peer_map: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
    event_tx: mpsc::UnboundedSender<AppEvent>,
    metrics: Arc<TimeoutMetrics>,
}

impl TimeoutChecker {
    pub fn new(
        check_interval_secs: u64,
        timeout_secs: u64,
        peer_map: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
        event_tx: mpsc::UnboundedSender<AppEvent>,
    ) -> Self;

    pub async fn run(&self, shutdown: Receiver<()>) -> Result<()>;

    fn check_timeouts(&self) -> Vec<IpAddr>;
}

pub struct TimeoutMetrics {
    pub timeouts_detected: AtomicU64,
    pub last_check_at: Mutex<Option<SystemTime>>,
}
```

**Algorithm:**
```
loop {
    select! {
        _ = sleep(check_interval) => {
            now = SystemTime::now()
            offline_peers = []

            for each peer in peer_map {
                if peer.status == Online {
                    elapsed = now - peer.last_seen
                    if elapsed > peer_timeout {
                        peer.status = Offline
                        offline_peers.push(peer.ip)
                        emit PeerOffline event
                        log at INFO level
                    }
                }
            }

            update metrics
        }
        _ = shutdown.recv() => {
            break gracefully
        }
    }
}
```

**Configuration:**
- Check interval: 30 seconds (hardcoded, reasonable for responsive detection)
- Peer timeout: from `AppConfig.peer_timeout` (default: 180s)
- Timeout configurable via `update_peer_timeout()` IPC command

**Edge Cases:**
- Skip localhost (own peer) in timeout check
- Skip peers already marked offline
- Handle SystemTime::UNIX_EPOCH errors gracefully

---

### 3. Cleanup Task

**Module:** `src-tauri/src/modules/peer/cleanup.rs`

**Purpose:** Remove stale offline peers from in-memory map to prevent unbounded growth.

**Responsibility:**
- Remove peers offline for >24 hours from memory
- Keep database records for history
- Track cleanup count for metrics

**Structure:**
```rust
pub struct CleanupTask {
    cleanup_interval: Duration,
    offline_retention: Duration,
    peer_map: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
    metrics: Arc<CleanupMetrics>,
}

impl CleanupTask {
    pub fn new(
        cleanup_interval_secs: u64,
        offline_retention_secs: u64,
        peer_map: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
    ) -> Self;

    pub async fn run(&self, shutdown: Receiver<()>) -> Result<()>;

    fn cleanup_stale_peers(&self) -> usize;
}

pub struct CleanupMetrics {
    pub peers_cleaned_up: AtomicU64,
    pub last_cleanup_at: Mutex<Option<SystemTime>>,
}
```

**Algorithm:**
```
loop {
    select! {
        _ = sleep(cleanup_interval) => {
            now = SystemTime::now()
            removed_count = 0

            for each peer in peer_map {
                if peer.status == Offline {
                    offline_duration = now - peer.last_seen
                    if offline_duration > offline_retention {
                        remove peer from map
                        removed_count++
                    }
                }
            }

            log removed_count at INFO level
            update metrics
        }
        _ = shutdown.recv() => {
            break gracefully
        }
    }
}
```

**Configuration:**
- Cleanup interval: 5 minutes (300 seconds, hardcoded)
- Offline retention: 24 hours (86400 seconds, hardcoded)
- Future: Make configurable via AppConfig

**Edge Cases:**
- Never remove localhost (own peer)
- Consider preserving manually added contacts (future enhancement)
- Log IP addresses of removed peers for debugging

---

## PeerManager Integration

**Module:** `src-tauri/src/modules/peer/manager.rs`

**Changes:**

1. **Add shutdown channel** for graceful task termination:
```rust
pub struct PeerManager {
    // ... existing fields
    shutdown_tx: Option<broadcast::Sender<()>>,
}
```

2. **Update start() method** to spawn background tasks:
```rust
pub fn start(&self) -> Result<()> {
    // Create shutdown channel
    let (shutdown_tx, _) = broadcast::channel(1);
    self.shutdown_tx.set(shutdown_tx.clone());

    // Announce online presence
    self.discovery.announce_online()?;

    // Start existing listener
    let peers = self.peers.clone();
    let message_tx = self.message_tx.clone();
    self.discovery.listen_incoming(move |msg, sender| {
        Self::handle_message(&peers, msg, sender, &message_tx)
    })?;

    // Spawn heartbeat task
    let heartbeat = HeartbeatTask::new(
        self.config.heartbeat_interval,
        self.discovery.clone(),
        self.peers.clone(),
    );
    let shutdown_rx = shutdown_tx.subscribe();
    tokio::spawn(async move {
        if let Err(e) = heartbeat.run(shutdown_rx).await {
            error!("Heartbeat task failed: {}", e);
        }
    });

    // Spawn timeout checker task
    let timeout_checker = TimeoutChecker::new(
        30, // check interval
        self.config.peer_timeout,
        self.peers.clone(),
        self.event_tx.clone(),
    );
    let shutdown_rx = shutdown_tx.subscribe();
    tokio::spawn(async move {
        if let Err(e) = timeout_checker.run(shutdown_rx).await {
            error!("Timeout checker task failed: {}", e);
        }
    });

    // Spawn cleanup task
    let cleanup = CleanupTask::new(
        300, // cleanup interval
        86400, // offline retention
        self.peers.clone(),
    );
    let shutdown_rx = shutdown_tx.subscribe();
    tokio::spawn(async move {
        if let Err(e) = cleanup.run(shutdown_rx).await {
            error!("Cleanup task failed: {}", e);
        }
    });

    info!("Peer manager started with background tasks");
    Ok(())
}
```

3. **Add stop() method** for graceful shutdown:
```rust
pub fn stop(&self) -> Result<()> {
    if let Some(tx) = &self.shutdown_tx {
        let _ = tx.send(());
    }
    info!("Peer manager stopped");
    Ok(())
}
```

---

## IPC Commands

**Module:** `src-tauri/src/commands/peer.rs`

**New Commands:**

### Get Heartbeat Config
```rust
#[tauri::command]
pub fn get_heartbeat_config(state: tauri::State<AppState>) -> Result<HeartbeatConfigDto, Error> {
    let config = state.get_config();
    Ok(HeartbeatConfigDto {
        heartbeat_interval: config.heartbeat_interval,
        peer_timeout: config.peer_timeout,
    })
}

// Frontend Usage:
// const config = await invoke<HeartbeatConfigDto>("get_heartbeat_config");
```

### Update Heartbeat Interval
```rust
#[tauri::command]
pub async fn update_heartbeat_interval(
    state: tauri::State<AppState>,
    interval_secs: u64,
) -> Result<(), Error> {
    let mut config = state.get_config_mut();
    config.heartbeat_interval = interval_secs;

    // Restart heartbeat task with new interval
    state.peer_manager.restart_heartbeat(interval_secs).await?;

    // Persist to database
    state.config_repo.save_app_config(&config).await?;
    Ok(())
}

// Frontend Usage:
// await invoke("update_heartbeat_interval", { interval_secs: 30 });
```

### Update Peer Timeout
```rust
#[tauri::command]
pub async fn update_peer_timeout(
    state: tauri::State<AppState>,
    timeout_secs: u64,
) -> Result<(), Error> {
    let mut config = state.get_config_mut();
    config.peer_timeout = timeout_secs;

    // Restart timeout checker with new timeout
    state.peer_manager.restart_timeout_checker(timeout_secs).await?;

    // Persist to database
    state.config_repo.save_app_config(&config).await?;
    Ok(())
}

// Frontend Usage:
// await invoke("update_peer_timeout", { timeout_secs: 300 });
```

### Get Peer Health Stats
```rust
#[tauri::command]
pub fn get_peer_health_stats(state: tauri::State<AppState>) -> Result<PeerHealthStatsDto, Error> {
    let stats = state.peer_manager.get_health_stats();
    Ok(PeerHealthStatsDto {
        heartbeats_sent: stats.heartbeats_sent,
        timeouts_detected: stats.timeouts_detected,
        peers_cleaned_up: stats.peers_cleaned_up,
        last_heartbeat_at: stats.last_heartbeat_at,
        last_timeout_check: stats.last_timeout_check,
    })
}

// Frontend Usage:
// const stats = await invoke<PeerHealthStatsDto>("get_peer_health_stats");
```

**DTOs:**
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatConfigDto {
    pub heartbeat_interval: u64,
    pub peer_timeout: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerHealthStatsDto {
    pub heartbeats_sent: u64,
    pub timeouts_detected: u64,
    pub peers_cleaned_up: u64,
    pub last_heartbeat_at: Option<i64>,
    pub last_timeout_check: Option<i64>,
}
```

---

## Frontend Integration

### Event Handling

**File:** `src/lib/events/types.ts`

Ensure event types are defined:
```typescript
export interface PeerOfflineEvent {
  type: 'peer-offline'
  peerIp: string
  username?: string
  reason: 'explicit' | 'timeout' | 'cleanup'
  timestamp: number
}
```

**File:** `src/components/contacts/Contacts.tsx`

Subscribe to offline events:
```typescript
useEffect(() => {
  const unsubscribe = eventsManager.onEvent<PeerOfflineEvent>(
    'peer-offline',
    (event) => {
      // Update contact status in UI
      setContacts((prev) =>
        prev.map((c) =>
          c.ip === event.peerIp
            ? { ...c, status: 'offline', lastSeen: event.timestamp }
            : c
        )
      )

      // Update online count
      setOnlineCount((prev) => Math.max(0, prev - 1))
    }
  )

  return () => unsubscribe.remove()
}, [])
```

### Configuration UI (Future)

**File:** `src/components/advanced-settings/NetworkSettings.tsx`

```typescript
export function NetworkSettings() {
  const [config, setConfig] = useState<HeartbeatConfigDto | null>(null)

  useEffect(() => {
    invoke<HeartbeatConfigDto>('get_heartbeat_config').then(setConfig)
  }, [])

  const handleHeartbeatChange = async (value: number) => {
    await invoke('update_heartbeat_interval', { intervalSecs: value })
    setConfig((prev) => (prev ? { ...prev, heartbeatInterval: value } : null))
  }

  return (
    <div>
      <label>
        Heartbeat Interval (seconds):
        <input
          type="number"
          value={config?.heartbeatInterval ?? 60}
          onChange={(e) => handleHeartbeatChange(Number(e.target.value))}
          min="30"
          max="300"
        />
      </label>

      <label>
        Peer Timeout (seconds):
        <input
          type="number"
          value={config?.peerTimeout ?? 180}
          onChange={(e) => handleTimeoutChange(Number(e.target.value))}
          min="60"
          max="600"
        />
      </label>
    </div>
  )
}
```

---

## Data Flow

### Normal Operation Flow

```
Time 0s:    Peer A starts
            ├─ Announce online (BR_ENTRY)
            └─ Start heartbeat task (60s interval)

Time 60s:   Heartbeat task fires
            ├─ Send BR_ENTRY broadcast
            └─ Update own last_seen

Time 120s:  Heartbeat task fires
            ├─ Send BR_ENTRY broadcast
            └─ Update own last_seen

Time 150s:  Peer B (monitoring) runs timeout checker
            └─ Peer A.last_seen = 120s ago (within 180s timeout)
                └─ Keep as online ✓

Time 180s:  Peer A crashes (silent failure)

Time 240s:  Heartbeat task WOULD have fired (but peer crashed)

Time 270s:  Timeout checker on Peer B runs
            └─ Peer A.last_seen = 150s ago
                └─ Exceeds 180s timeout!
                    ├─ Mark Peer A as offline
                    ├─ Emit PeerOffline event
                    └─ Log: "Peer 192.168.1.100 timed out"

Time 300s:  Frontend receives event
            └─ Update contact list UI
                └─ Show Peer A as offline with last_seen
```

### Recovery Flow

```
Time 360s:  Peer A restarts
            ├─ Announce online (BR_ENTRY)
            └─ Start heartbeat task again

Time 360s:  Peer B receives BR_ENTRY
            ├─ Find existing Peer A in map (status: offline)
            ├─ Update status to online
            ├─ Update last_seen to now
            └─ Emit PeerOnline event

Time 361s:  Frontend updates UI
            └─ Show Peer A as online again
```

---

## Error Handling Strategy

### Broadcast Failures

**Scenario:** Network interface down during heartbeat

**Handling:**
```rust
fn send_heartbeat(&self) -> Result<()> {
    match self.discovery.announce_online() {
        Ok(_) => {
            self.metrics.heartbeats_sent.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
        Err(e) => {
            warn!("Heartbeat broadcast failed: {}", e);
            // Continue task, don't exit
            Err(e.into())
        }
    }
}
```

### Peer Map Lock Contention

**Scenario:** Timeout checker blocked by long-running operation

**Handling:**
```rust
fn check_timeouts(&self) -> Vec<IpAddr> {
    let peers = self.peer_map.try_lock_for(Duration::from_secs(1));
    match peers {
        Some(map) => {
            // Perform timeout check
            // ...
        }
        None => {
            warn!("Peer map locked, skipping timeout check");
            vec![]
        }
    }
}
```

### Event Channel Overflow

**Scenario:** Many peers timeout simultaneously, event channel full

**Handling:**
```rust
if let Err(e) = self.event_tx.try_send(AppEvent::PeerOffline { ip }) {
    warn!("Failed to send peer offline event: {}", e);
    // Event dropped but peer marked offline (correct state)
}
```

---

## Performance Considerations

### CPU Usage

**Heartbeat Task:**
- Frequency: Every 60 seconds
- Operation: Single UDP broadcast
- Estimated CPU: <0.01% per broadcast

**Timeout Checker:**
- Frequency: Every 30 seconds
- Operation: Iterate 1000 peers, timestamp comparison
- Estimated CPU: <0.1% per check (1000 peers)

**Cleanup Task:**
- Frequency: Every 5 minutes
- Operation: Iterate 1000 peers, remove stale entries
- Estimated CPU: <0.05% per cleanup

**Total CPU Impact:** <0.2% on typical hardware

### Memory Usage

**Per-Task Overhead:**
- Task handle: ~1KB
- Metrics: ~100 bytes
- Channel buffers: ~1KB

**Total Additional Memory:** <5KB (negligible)

### Network Traffic

**Heartbeat Broadcasts:**
- Size: ~100 bytes per broadcast
- Frequency: 1 per peer per 60 seconds
- 100 peers = 100 bytes/minute = 1.6 KB/minute

**Impact:** Negligible on typical LAN (100 Mbps+)

---

## Testing Strategy

### Unit Tests

**Heartbeat Task:**
```rust
#[tokio::test]
async fn test_heartbeat_sent_at_interval() {
    let mock_discovery = MockDiscovery::new();
    let interval = Duration::from_secs(1); // Fast for testing

    let task = HeartbeatTask::new(interval, mock_discovery.clone(), peer_map);
    tokio::spawn(async move { task.run(shutdown).await });

    sleep(Duration::from_secs(3)).await;

    assert_eq!(mock_discovery.broadcast_count(), 3);
}
```

**Timeout Checker:**
```rust
#[tokio::test]
async fn test_peer_marked_offline_after_timeout() {
    let peer_map = create_peer_map_with_old_peer();
    let timeout = Duration::from_secs(10);
    let checker = TimeoutChecker::new(Duration::from_secs(1), timeout, ...);

    checker.check_timeouts();

    let peers = peer_map.lock().unwrap();
    assert_eq!(peers.get(&ip).unwrap().status, PeerStatus::Offline);
}
```

**Cleanup Task:**
```rust
#[tokio::test]
async fn test_stale_peers_removed() {
    let peer_map = create_peer_map_with_stale_offline_peers();
    let cleanup = CleanupTask::new(Duration::from_secs(1), Duration::from_secs(24), ...);

    let removed = cleanup.cleanup_stale_peers();

    assert!(removed > 0);
    let peers = peer_map.lock().unwrap();
    assert!(!peers.contains_key(&stale_ip));
}
```

### Integration Tests

**Scenario: Peer crash detection**
```rust
#[tokio::test]
async fn test_peer_crash_detected() {
    // Start peer manager
    let manager = PeerManager::start(config);

    // Add a peer
    manager.add_peer(test_peer);

    // Simulate time passing (mock SystemTime)
    mock_time_advance(Duration::from_secs(200));

    // Run timeout checker
    manager.timeout_checker.check_timeouts();

    // Verify peer marked offline
    let peers = manager.get_peers();
    assert_eq!(peers[0].status, PeerStatus::Offline);

    // Verify event emitted
    let events = manager.get_events();
    assert!(events.contains(|e| matches!(e, AppEvent::PeerOffline { ... })));
}
```

---

## Security Considerations

### No New Attack Vectors

- Heartbeat uses existing broadcast mechanism (no new ports/protocols)
- Timeout detection only reads internal state (no external input)
- Cleanup only removes memory entries (no data loss)

### Resource Exhaustion Prevention

**Peer Map Size:**
- Limit maximum peers to 10,000 (configurable)
- Reject new peers if limit reached

**Event Flood:**
- Use bounded channels for events
- Drop events if not consumed (graceful degradation)

**Configuration Bounds:**
- Validate heartbeat interval (min: 10s, max: 600s)
- Validate peer timeout (min: 30s, max: 3600s)
- Enforce timeout > heartbeat interval

---

## Monitoring and Observability

### Log Messages

**Heartbeat:**
```
[DEBUG] Heartbeat sent (interval: 60s, total: 42)
[WARN] Heartbeat broadcast failed: Network unreachable
```

**Timeout Detection:**
```
[INFO] Timeout check completed: 0 peers timed out
[INFO] Peer 192.168.1.100 timed out (last seen: 200s ago)
[WARN] Peer map locked, skipping timeout check
```

**Cleanup:**
```
[INFO] Cleanup completed: removed 3 stale peers
[DEBUG] Removed offline peer 192.168.1.50 (offline for 48h)
```

### Metrics

Expose via `get_peer_health_stats()`:
```json
{
  "heartbeatsSent": 1234,
  "timeoutsDetected": 5,
  "peersCleanedUp": 12,
  "lastHeartbeatAt": 1705420800000,
  "lastTimeoutCheck": 1705420830000
}
```

---

## Future Enhancements

### Adaptive Heartbeat

Adjust heartbeat interval based on network conditions:
- Increase interval if network congested
- Decrease interval if many peers timing out

### Peer Health Scoring

Track peer reliability:
- Count how often each peer times out
- Show "unreliable" indicator for flaky peers

### Differential Heartbeat

Send heartbeat more frequently to "important" peers (favorites, active conversations).

### Graceful Degradation

If network unavailable, stop heartbeat and retry later with exponential backoff.

---

## References

- IPMsg Protocol Specification: http://www.ipmsg.org/files/protocol.txt
- Tokio Runtime: https://tokio.rs/
- Existing Config: `feiqiu/src-tauri/src/config/app.rs`
- Existing Peer Manager: `feiqiu/src-tauri/src/modules/peer/manager.rs`
- Existing Discovery: `feiqiu/src-tauri/src/modules/peer/discovery.rs`
