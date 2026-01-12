# Spec: IPC Connection Layer

**Capability:** `ipc-connection`
**Change ID:** `connect-frontend-backend`
**Status:** Proposed

## Overview

This specification defines the IPC (Inter-Process Communication) service layer that connects the React frontend to the Rust/Tauri backend. It provides type-safe wrappers for all backend commands with consistent error handling and type conversion.

---

## ADDED Requirements

### Requirement: API Service Layer

The application MUST provide a service layer for all Tauri IPC commands that includes type-safe wrappers, error handling, and type conversion.

**ID:** IPC-001

#### Scenario: Basic command invocation

**Given** the service layer is implemented
**When** a component calls `peersApi.getPeers()`
**Then** the command should invoke `get_peers` in the backend
**And** the result should be returned as a Promise<Peer[]>

#### Scenario: Error handling

**Given** the service layer is implemented
**When** a backend command fails
**Then** the error should be wrapped in an `IpcError`
**And** the error should include the command name and original error

#### Scenario: Type conversion

**Given** the backend returns data with i64 timestamps
**When** the API layer processes the response
**Then** timestamps should be converted to ISO strings
**And** status enums should be mapped to frontend values

---

### Requirement: Peers API

The application MUST provide a `peersApi` object in `src/lib/api/peers.ts` that exposes all peer-related IPC commands.

**ID:** IPC-002

#### Scenario: Get all peers

**Given** the peersApi exists
**When** `peersApi.getPeers()` is called
**Then** it should invoke the `get_peers` command
**And** return a Promise<Peer[]>
**And** each Peer should have ISO timestamp strings

#### Scenario: Get online peers only

**Given** the peersApi exists
**When** `peersApi.getOnlinePeers()` is called
**Then** it should invoke the `get_online_peers` command
**And** return only peers with status 'online'

#### Scenario: Get peer by IP

**Given** the peersApi exists
**When** `peersApi.getPeerByIp('192.168.1.100')` is called
**Then** it should invoke `get_peer_by_ip` with the IP parameter
**And** return the matching Peer or throw if not found

#### Scenario: Get peer statistics

**Given** the peersApi exists
**When** `peersApi.getPeerStats()` is called
**Then** it should invoke the `get_peer_stats` command
**And** return peer statistics

---

### Requirement: Config API

The application MUST provide a `configApi` object in `src/lib/api/config.ts` that exposes all configuration-related IPC commands.

**ID:** IPC-003

#### Scenario: Get current configuration

**Given** the configApi exists
**When** `configApi.getConfig()` is called
**Then** it should invoke the `get_config` command
**And** return a Promise<Config>

#### Scenario: Update configuration

**Given** the configApi exists
**When** `configApi.setConfig(newConfig)` is called with partial config
**Then** it should invoke the `set_config` command with the config object
**And** the changes should persist to the backend

#### Scenario: Reset to defaults

**Given** the configApi exists
**When** `configApi.resetConfig()` is called
**Then** it should invoke the `reset_config` command
**And** all settings should return to default values

#### Scenario: Get single config value

**Given** the configApi exists
**When** `configApi.getConfigValue('username')` is called
**Then** it should invoke the `get_config_value` command
**And** return only the username value

#### Scenario: Set single config value

**Given** the configApi exists
**When** `configApi.setConfigValue('username', 'Alice')` is called
**Then** it should invoke the `set_config_value` command
**And** only the username should be updated

---

### Requirement: Messages API

The application MUST provide a `messagesApi` object in `src/lib/api/messages.ts` that exposes all message-related IPC commands.

**ID:** IPC-004

#### Scenario: Send a message

**Given** the messagesApi exists
**When** `messagesApi.sendMessage('Hello', '192.168.1.100')` is called
**Then** it should invoke the `send_message` command
**And** return the sent Message object
**And** the message should be delivered to the peer

#### Scenario: Get message history

**Given** the messagesApi exists
**When** `messagesApi.getMessages({ senderIp: '192.168.1.100' })` is called
**Then** it should invoke the `get_messages` command with filters
**And** return only messages from that peer

#### Scenario: Get all messages

**Given** the messagesApi exists
**When** `messagesApi.getMessages()` is called without filters
**Then** it should invoke the `get_messages` command
**And** return all messages from history

---

### Requirement: File Transfers API

The application MUST provide a `transfersApi` object in `src/lib/api/transfers.ts` that exposes all file transfer-related IPC commands.

**ID:** IPC-005

#### Scenario: Accept file transfer

**Given** the transfersApi exists
**And** a file transfer request is pending
**When** `transfersApi.acceptFileTransfer('transfer-123')` is called
**Then** it should invoke the `accept_file_transfer` command
**And** the file transfer should begin

#### Scenario: Reject file transfer

**Given** the transfersApi exists
**And** a file transfer request is pending
**When** `transfersApi.rejectFileTransfer('transfer-123')` is called
**Then** it should invoke the `reject_file_transfer` command
**And** the file transfer should be cancelled

#### Scenario: Get all transfers

**Given** the transfersApi exists
**When** `transfersApi.getFileTransfers()` is called
**Then** it should invoke the `get_file_transfers` command
**And** return a list of all transfers

#### Scenario: Cancel active transfer

**Given** the transfersApi exists
**And** a file transfer is in progress
**When** `transfersApi.cancelFileTransfer('transfer-123')` is called
**Then** it should invoke the `cancel_file_transfer` command
**And** the transfer should stop

---

### Requirement: Type Conversion

The application MUST provide type converters for backend-frontend data transformation including timestamp converters, status mappers, and DTO adapters.

**ID:** IPC-006

#### Scenario: Timestamp conversion

**Given** a backend timestamp of 1704067200000 (i64 milliseconds)
**When** `toIsoDate(1704067200000)` is called
**Then** it should return '2024-01-01T00:00:00.000Z'

#### Scenario: Reverse timestamp conversion

**Given** an ISO date string '2024-01-01T00:00:00.000Z'
**When** `fromIsoDate('2024-01-01T00:00:00.000Z')` is called
**Then** it should return 1704067200000

#### Scenario: Peer status mapping

**Given** a backend status 'online'
**When** `mapPeerStatus('online')` is called
**Then** it should return 'online' as a FrontendStatus

#### Scenario: Transfer status mapping

**Given** a backend transfer status 'active'
**When** `mapTransferStatus('active')` is called
**Then** it should return 'transferring' as a FrontendTransferStatus

#### Scenario: DTO conversion

**Given** a backend PeerDto with i64 timestamps
**When** `toFrontendPeer(peerDto)` is called
**Then** the returned Peer should have:
- Timestamp fields as ISO strings
- Status mapped to frontend enum
- All other fields preserved

---

### Requirement: Error Types

The application MUST provide specific error types for IPC failures including IpcError and NetworkError with proper error wrapping.

**ID:** IPC-007

#### Scenario: IpcError creation

**Given** a backend command fails
**When** the error is caught
**Then** an `IpcError` should be thrown
**And** the error should contain:
- The command name that failed
- The original error object
- A descriptive error message

#### Scenario: Network error handling

**Given** a network operation fails
**When** the error is caught
**Then** a `NetworkError` should be thrown
**And** the error should contain a descriptive message

---

## Related Capabilities

- **realtime-events** - The IPC layer emits events that the realtime system consumes
- **data-hooks** - Data fetching hooks use the IPC API layer

---

## Type Definitions

```typescript
// Backend DTO types (matching Rust structs)
interface PeerDto {
  ip: string;
  port: number;
  username: string;
  hostname: string;
  nickname: string | null;
  avatar: string | null;
  groups: string[];
  status: 'online' | 'offline' | 'away';
  display_name: string;
  last_seen: number; // i64 milliseconds
}

interface MessageDto {
  id: string;
  msg_id: string;
  sender_ip: string;
  sender_name: string;
  receiver_ip: string;
  msg_type: number;
  content: string;
  is_encrypted: boolean;
  is_offline: boolean;
  sent_at: number; // i64 milliseconds
  received_at: number; // i64 milliseconds
  created_at: number; // i64 milliseconds
}

interface ConfigDto {
  username: string;
  hostname: string;
  avatar: string | null;
  status: string;
  bind_ip: string;
  udp_port: number;
  tcp_port_start: number;
  tcp_port_end: number;
  heartbeat_interval: number;
  peer_timeout: number;
  encryption_enabled: boolean;
  encryption_key: string | null;
  offline_message_retention_days: number;
  auto_accept_files: boolean;
  file_save_dir: string;
  log_level: string;
}

interface TaskDto {
  id: string;
  direction: 'incoming' | 'outgoing';
  peer_ip: string;
  file_name: string;
  file_size: number;
  md5: string | null;
  status: 'pending' | 'active' | 'paused' | 'completed' | 'failed' | 'cancelled';
  transferred_bytes: number;
  progress: number;
  port: number | null;
  error: string | null;
  created_at: number; // i64 milliseconds
  updated_at: number; // i64 milliseconds
}

// Frontend types (after conversion)
interface Peer {
  ip: string;
  port: number;
  username: string;
  hostname: string;
  nickname: string | null;
  avatar: string | null;
  groups: string[];
  status: 'online' | 'offline' | 'away' | 'busy';
  displayName: string;
  lastSeen: string; // ISO string
}

interface Message {
  id: string;
  msgId: string;
  senderIp: string;
  senderName: string;
  receiverIp: string;
  msgType: number;
  content: string;
  isEncrypted: boolean;
  isOffline: boolean;
  sentAt: string; // ISO string
  receivedAt: string; // ISO string
  createdAt: string; // ISO string
}

interface Config {
  username: string;
  hostname: string;
  avatar: string | null;
  status: string;
  bindIp: string;
  udpPort: number;
  tcpPortStart: number;
  tcpPortEnd: number;
  heartbeatInterval: number;
  peerTimeout: number;
  encryptionEnabled: boolean;
  encryptionKey: string | null;
  offlineMessageRetentionDays: number;
  autoAcceptFiles: boolean;
  fileSaveDir: string;
  logLevel: string;
}

interface FileTransfer {
  id: string;
  direction: 'incoming' | 'outgoing';
  peerIp: string;
  fileName: string;
  fileSize: number;
  md5: string | null;
  status: 'waiting' | 'transferring' | 'paused' | 'completed' | 'cancelled' | 'failed';
  transferredBytes: number;
  progress: number;
  port: number | null;
  error: string | null;
  createdAt: string; // ISO string
  updatedAt: string; // ISO string
}
```
