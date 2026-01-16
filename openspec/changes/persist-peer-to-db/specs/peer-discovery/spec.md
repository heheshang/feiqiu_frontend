## ADDED Requirements

### Requirement: Peer Database Persistence

The system SHALL persist all peer information to the database, ensuring peer data survives application restarts and provides historical tracking capabilities.

#### Scenario: Peer discovered via UDP broadcast
- **WHEN** a peer presence message (IPMSG_BR_ENTRY or IPMSG_ANSENTRY) is received via UDP broadcast
- **THEN** the system SHALL upsert the peer information to the `peers` table with IP, port, username, hostname, and current timestamp

#### Scenario: Peer offline detection
- **WHEN** a peer exit message (IPMSG_BR_EXIT) is received
- **THEN** the system SHALL update the peer's `last_seen` timestamp in the database
- **AND** the peer's online status SHALL be computed as offline based on the timeout threshold

#### Scenario: Peer data retrieval after restart
- **WHEN** the application restarts after a shutdown
- **THEN** the system SHALL load previously discovered peer information from the database
- **AND** peers SHALL be displayed with their last-known usernames and hostnames
- **AND** peer status SHALL reflect online/offline based on `last_seen` timestamp

### Requirement: Database-Backed Peer Queries

The system SHALL query the database for all peer list operations, replacing in-memory HashMap lookups.

#### Scenario: Get all peers
- **WHEN** the frontend requests all peers via `get_peers` command
- **THEN** the system SHALL query the database for all peer records
- **AND** return peers with computed online/offline status based on `last_seen` timestamp

#### Scenario: Get online peers only
- **WHEN** the frontend requests online peers via `get_online_peers` command
- **THEN** the system SHALL query the database for peers with `last_seen` within the timeout threshold (default: 180 seconds)
- **AND** return only peers considered online

#### Scenario: Get specific peer by IP
- **WHEN** the frontend requests a specific peer via `get_peer_by_ip` command
- **THEN** the system SHALL query the database for the peer with matching IP address
- **AND** return the peer record or null if not found

#### Scenario: Peer count statistics
- **WHEN** the frontend requests peer statistics via `get_peer_stats` command
- **THEN** the system SHALL compute total, online, and offline counts from database records
- **AND** return accurate counts reflecting current database state

### Requirement: Async-to-Sync Database Bridge

The system SHALL safely execute async database operations from the synchronous peer discovery message handler.

#### Scenario: Sync context calling async DB upsert
- **WHEN** a UDP message handler (synchronous context) needs to upsert peer data to the database
- **THEN** the system SHALL use `tokio::runtime::Handle::block_on()` to execute the async database operation
- **AND** block until the database operation completes
- **AND** return the result to the synchronous caller

#### Scenario: Database operation error handling
- **WHEN** a database operation fails during peer discovery
- **THEN** the system SHALL log the error with tracing
- **AND** continue peer discovery operation (best-effort persistence)
- **AND** not block the UDP message listener

### Requirement: Peer Status Computation

The system SHALL compute peer online/offline status from the `last_seen` database field rather than storing a separate status flag.

#### Scenario: Compute online status from last_seen
- **WHEN** querying peers from the database
- **THEN** the system SHALL compare each peer's `last_seen` timestamp against the timeout threshold (180 seconds)
- **AND** mark peer as online if `last_seen` is within threshold
- **AND** mark peer as offline if `last_seen` exceeds threshold

#### Scenario: Heartbeat updates last_seen
- **WHEN** a heartbeat message is received from a peer
- **THEN** the system SHALL update the peer's `last_seen` timestamp to current time
- **AND** the peer SHALL be considered online after this update

### Requirement: Type Conversion Between Layers

The system SHALL provide conversion between in-memory `PeerNode` types and database `PeerModel` types.

#### Scenario: Database model to in-memory node
- **WHEN** retrieving a `PeerModel` from the database
- **THEN** the system SHALL convert it to `PeerNode` with appropriate type conversions (String → IpAddr, NaiveDateTime → SystemTime)
- **AND** compute peer status from `last_seen` timestamp

#### Scenario: In-memory node to database parameters
- **WHEN** persisting a `PeerNode` to the database
- **THEN** the system SHALL extract relevant fields and convert to database-compatible types (IpAddr → String, SystemTime → NaiveDateTime)
- **AND** pass parameters to `peer_repo.upsert()` or equivalent method

### Requirement: Coordination with MessageHandler

The system SHALL allow both `PeerManager` and `MessageHandler` to update peer records without conflict.

#### Scenario: MessageHandler updates peer during message
- **WHEN** `MessageHandler` processes a text message from a peer
- **THEN** the system SHALL upsert peer information via `peer_repo` in MessageHandler
- **AND** this SHALL NOT conflict with PeerManager's database operations

#### Scenario: PeerManager updates peer during discovery
- **WHEN** `PeerManager` receives a discovery message from a peer
- **THEN** the system SHALL upsert peer information via `peer_repo` in PeerManager
- **AND** both upsert operations SHALL use the same database connection
- **AND** last write SHALL win (acceptable for peer metadata)
