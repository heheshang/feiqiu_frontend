# peer-heartbeat Specification

## Purpose

Defines requirements for the peer heartbeat mechanism and automatic offline detection system. This ensures accurate online/offline peer status tracking without relying solely on explicit exit messages.

## ADDED Requirements

### Requirement: Broadcast periodic heartbeat messages

The system MUST send periodic heartbeat broadcasts to maintain peer presence visibility on the LAN.

#### Scenario: Application starts and begins heartbeat broadcasts

**Given** the FeiQiu application has started
**And** the peer manager has initialized
**When** the heartbeat task is spawned
**Then** the system MUST broadcast an initial `IPMSG_BR_ENTRY` message
**And** the system MUST start a periodic heartbeat timer
**And** the heartbeat interval MUST be configurable (default: 60 seconds)

#### Scenario: Heartbeat broadcast sent at configured interval

**Given** the heartbeat task is running
**And** the heartbeat interval is set to 60 seconds
**When** 60 seconds have elapsed since the last heartbeat
**Then** the system MUST send a new `IPMSG_BR_ENTRY` broadcast message
**And** the system MUST update its own `last_seen` timestamp
**And** the system MUST increment the heartbeat counter metric
**And** the system MUST log the heartbeat at DEBUG level

#### Scenario: Heartbeat broadcast fails but task continues

**Given** the heartbeat task is running
**When** a heartbeat broadcast fails (network unavailable, socket error, etc.)
**Then** the system MUST log the failure at WARN level
**And** the heartbeat task MUST continue running
**And** the system MUST attempt the next heartbeat at the scheduled interval

#### Scenario: User configures custom heartbeat interval

**Given** the application is running
**And** the default heartbeat interval is 60 seconds
**When** the user invokes `update_heartbeat_interval` with value 30
**Then** the heartbeat interval MUST be updated to 30 seconds
**And** the new interval MUST take effect immediately
**And** the configuration MUST be persisted to the database
**And** the heartbeat task MUST use the new interval for the next broadcast

#### Scenario: Heartbeat interval validation prevents invalid values

**Given** the user attempts to set the heartbeat interval
**When** the provided value is less than 10 seconds
**Then** the system MUST reject the configuration
**And** return a validation error

**Given** the user attempts to set the heartbeat interval
**When** the provided value is greater than 600 seconds (10 minutes)
**Then** the system MUST reject the configuration
**And** return a validation error

---

### Requirement: Detect and mark peers offline after timeout

The system MUST monitor peer inactivity and automatically mark peers as offline when they exceed the configured timeout threshold.

#### Scenario: Peer detected offline after timeout period

**Given** a peer is currently marked as online
**And** the peer timeout is configured to 180 seconds
**And** the peer's `last_seen` timestamp is 200 seconds ago
**When** the timeout checker task runs
**Then** the peer's status MUST be changed to `Offline`
**And** a `PeerOffline` event MUST be emitted with reason `timeout`
**And** the system MUST log the timeout detection at INFO level

#### Scenario: Peer remains online when activity within timeout

**Given** a peer is currently marked as online
**And** the peer timeout is configured to 180 seconds
**And** the peer's `last_seen` timestamp is 100 seconds ago
**When** the timeout checker task runs
**Then** the peer's status MUST remain `Online`
**And** NO `PeerOffline` event MUST be emitted

#### Scenario: Peer explicitly exits before timeout

**Given** a peer is currently marked as online
**And** the peer sends an `IPMSG_BR_EXIT` message
**When** the exit message is processed
**Then** the peer's status MUST be changed to `Offline` immediately
**And** a `PeerOffline` event MUST be emitted with reason `explicit`
**And** the timeout checker MUST skip this peer (already offline)

#### Scenario: Timeout checker runs at regular intervals

**Given** the timeout checker task is running
**And** the check interval is set to 30 seconds
**When** 30 seconds have elapsed since the last check
**Then** the system MUST check all peers' `last_seen` timestamps
**And** the system MUST mark any qualifying peers as offline
**And** the system MUST update the last check timestamp metric

#### Scenario: Peer marked offline comes back online

**Given** a peer was previously marked offline due to timeout
**And** the peer has restarted and sent an `IPMSG_BR_ENTRY` message
**When** the online message is received
**Then** the peer's status MUST be changed to `Online`
**And** the peer's `last_seen` timestamp MUST be updated
**And** a `PeerOnline` event MUST be emitted

#### Scenario: User configures custom peer timeout

**Given** the application is running
**And** the default peer timeout is 180 seconds
**When** the user invokes `update_peer_timeout` with value 300
**Then** the peer timeout MUST be updated to 300 seconds
**And** the new timeout MUST take effect immediately
**And** the configuration MUST be persisted to the database

#### Scenario: Peer timeout validation ensures relationship to heartbeat

**Given** the user attempts to set the peer timeout
**When** the heartbeat interval is 60 seconds
**And** the provided timeout value is 30 seconds (less than heartbeat)
**Then** the system MUST reject the configuration
**And** return a validation error
**And** the error message MUST indicate timeout must be greater than heartbeat interval

---

### Requirement: Clean up stale offline peers from memory

The system MUST periodically remove peers that have been offline for an extended period to prevent unbounded memory growth.

#### Scenario: Stale offline peers removed during cleanup

**Given** a peer has been offline for 25 hours
**And** the offline retention period is 24 hours
**When** the cleanup task runs
**Then** the peer MUST be removed from the in-memory peer map
**And** the peer's database record MUST be preserved (for history)
**And** the system MUST log the removal at DEBUG level

#### Scenario: Recently offline peers retained in memory

**Given** a peer has been offline for 2 hours
**And** the offline retention period is 24 hours
**When** the cleanup task runs
**Then** the peer MUST NOT be removed from the peer map
**And** the peer MUST remain visible in the contacts list with offline status

#### Scenario: Local peer never removed during cleanup

**Given** the local peer (localhost) exists in the peer map
**When** the cleanup task runs
**Then** the local peer MUST NEVER be removed
**Regardless** of how long it has been inactive

#### Scenario: Cleanup task runs at regular intervals

**Given** the cleanup task is running
**And** the cleanup interval is set to 5 minutes (300 seconds)
**When** 5 minutes have elapsed since the last cleanup
**Then** the system MUST scan all peers in the peer map
**And** the system MUST remove peers offline longer than retention period
**And** the system MUST report the number of peers removed

#### Scenario: Cleanup metrics tracked for monitoring

**Given** the cleanup task has completed a run
**And** 5 peers were removed
**When** querying the cleanup metrics
**Then** `peersCleanedUp` MUST equal 5
**And** `lastCleanupAt` MUST reflect the timestamp of the cleanup

---

### Requirement: Emit real-time events for peer status changes

The system MUST emit frontend events whenever a peer's online status changes, enabling real-time UI updates.

#### Scenario: PeerOffline event emitted on timeout detection

**Given** a peer has timed out and is marked offline
**When** the status change occurs
**Then** a `PeerOffline` event MUST be emitted via Tauri event system
**And** the event payload MUST contain:
  - `peerIp`: The IP address of the offline peer
  - `username`: The peer's username (if available)
  - `reason`: The string "timeout"
  - `timestamp`: Unix timestamp in milliseconds

#### Scenario: PeerOffline event emitted on explicit exit

**Given** a peer has sent an `IPMSG_BR_EXIT` message
**When** the exit message is processed
**Then** a `PeerOffline` event MUST be emitted
**And** the event payload `reason` MUST be the string "explicit"

#### Scenario: PeerOnline event emitted when peer returns

**Given** a previously offline peer sends an `IPMSG_BR_ENTRY` message
**When** the online message is processed
**Then** a `PeerOnline` event MUST be emitted
**And** the event payload MUST contain:
  - `peerIp`: The IP address of the peer
  - `username`: The peer's username (if available)
  - `timestamp`: Unix timestamp in milliseconds

#### Scenario: Frontend receives and handles peer offline event

**Given** the frontend is subscribed to `peer-offline` events
**And** the contacts list is displayed
**When** a `PeerOffline` event is received
**Then** the contact's status indicator MUST change to offline (gray)
**And** the contact's "last seen" time MUST be displayed
**And** the online count MUST decrement by 1
**And** the update MUST complete within 1 second

#### Scenario: Frontend receives and handles peer online event

**Given** the frontend is subscribed to `peer-online` events
**And** a contact is currently displayed as offline
**When** a `PeerOnline` event is received for that contact
**Then** the contact's status indicator MUST change to online (green)
**And** the "last seen" label MUST be removed or show "刚刚在线"
**And** the online count MUST increment by 1

#### Scenario: Event channel overflow handled gracefully

**Given** many peers timeout simultaneously
**And** the event channel is at capacity
**When** the system attempts to emit events
**Then** excess events MAY be dropped
**And** the system MUST log a warning about dropped events
**And** peer status MUST still be updated correctly in the peer map

---

### Requirement: Provide configuration and metrics APIs

The system MUST expose IPC commands for configuring heartbeat/timeout settings and retrieving health metrics.

#### Scenario: User retrieves current heartbeat configuration

**Given** the application is running
**And** the heartbeat interval is 60 seconds
**And** the peer timeout is 180 seconds
**When** the frontend invokes `get_heartbeat_config`
**Then** the system MUST return a `HeartbeatConfigDto` with:
  - `heartbeatInterval`: 60
  - `peerTimeout`: 180

#### Scenario: User retrieves peer health statistics

**Given** the application has been running for 10 minutes
**And** 10 heartbeats have been sent
**And** 2 peers have timed out
**And** 1 cleanup has run
**When** the frontend invokes `get_peer_health_stats`
**Then** the system MUST return a `PeerHealthStatsDto` with:
  - `heartbeatsSent`: 10
  - `timeoutsDetected`: 2
  - `peersCleanedUp`: 1
  - `lastHeartbeatAt`: <timestamp of last heartbeat>
  - `lastTimeoutCheck`: <timestamp of last check>

#### Scenario: Configuration changes persist across application restarts

**Given** the user has set heartbeat interval to 90 seconds
**And** the configuration has been saved to database
**When** the application is restarted
**Then** the loaded configuration MUST have `heartbeatInterval` of 90
**And** the heartbeat task MUST use the 90-second interval

#### Scenario: Invalid configuration values rejected with clear errors

**Given** the user attempts to set heartbeat interval to 5 seconds
**When** the `update_heartbeat_interval` command is invoked
**Then** the command MUST return an error
**And** the error message MUST indicate the value is too small
**And** the configuration MUST NOT be changed

---

### Requirement: Background tasks operate safely and efficiently

The system MUST ensure background tasks operate efficiently without causing performance issues or race conditions.

#### Scenario: Tasks shut down gracefully on application exit

**Given** all three background tasks are running
**When** the application begins shutdown
**Then** each task MUST receive a shutdown signal
**And** each task MUST complete its current operation
**And** each task MUST exit cleanly
**And** shutdown MUST complete within 5 seconds

#### Scenario: Peer map access is thread-safe

**Given** multiple background tasks are running
**And** the heartbeat task updates the peer map
**And** the timeout checker reads the peer map
**When** both operations occur simultaneously
**Then** NO race conditions MUST occur
**And** the peer map MUST remain consistent
**And** operations MUST be synchronized via `Arc<Mutex<>>`

#### Scenario: Timeout checker handles locked peer map gracefully

**Given** the peer map is locked by a long-running operation
**When** the timeout checker attempts to acquire the lock
**And** the lock cannot be acquired within 1 second
**Then** the timeout checker MUST skip this check cycle
**And** the system MUST log a warning
**And** the task MUST retry at the next interval

#### Scenario: CPU usage remains within acceptable limits

**Given** the application is running with 100 active peers
**And** all three background tasks are operational
**When** measuring CPU usage over 1 minute
**Then** the combined CPU usage of background tasks MUST be less than 1%

#### Scenario: Memory usage does not grow unbounded

**Given** the application has been running for 24 hours
**And** peers have been coming and going
**When** measuring memory usage
**Then** memory usage MUST be stable (not continuously growing)
**And** stale peers MUST have been removed by cleanup task
