# Spec: Real-time Events System

**Capability:** `realtime-events`
**Change ID:** `connect-frontend-backend`
**Status:** Proposed

## Overview

This specification defines the real-time event system that handles backend-generated events and updates the frontend state. The system manages event listeners, handles event lifecycles, and ensures proper cleanup to prevent memory leaks.

---

## ADDED Requirements

### Requirement: Event Manager

The application MUST provide a centralized EventManager class for managing Tauri event listeners that handles start, stop, and automatic restart.

**ID:** RTE-001

#### Scenario: Start event listeners

**Given** the application is running
**When** `eventManager.start()` is called
**Then** all event listeners should be registered with Tauri
**And** the manager should store unlisten functions for cleanup

#### Scenario: Stop event listeners

**Given** the EventManager is running
**When** `eventManager.stop()` is called
**Then** all event listeners should be unregistered
**And** all unlisten functions should be called
**And** the internal listeners map should be cleared

#### Scenario: Automatic restart

**Given** the EventManager was stopped
**When** `eventManager.start()` is called again
**Then** new listeners should be registered
**And** the manager should replace previous unlisten functions

---

### Requirement: Message Events

The application MUST handle message-related events from the backend including message-received and message-receipt-ack events.

**ID:** RTE-002

#### Scenario: Receive new message

**Given** a user is logged in
**When** the backend emits a `message-received` event
**Then** the frontend should receive the event
**And** the message should be added to the message store
**And** the UI should update to show the new message

#### Scenario: Message acknowledgment

**Given** a message was sent
**When** the backend emits a `message-receipt-ack` event
**Then** the frontend should update the message status to 'delivered'
**And** the UI should reflect the status change

---

### Requirement: Peer Events

The application MUST handle peer-related events from the backend including peer-online, peer-offline, and peers-discovered events.

**ID:** RTE-003

#### Scenario: Peer comes online

**Given** the application is running
**When** the backend emits a `peer-online` event with IP '192.168.1.100'
**Then** the frontend should add the peer to the peer store
**And** the peer's status should be set to 'online'
**And** the UI should show the peer in the online list

#### Scenario: Peer goes offline

**Given** a peer '192.168.1.100' is in the peer store
**When** the backend emits a `peer-offline` event for that IP
**Then** the peer's status should be updated to 'offline'
**And** the peer's last_seen timestamp should be updated
**And** the UI should reflect the offline status

#### Scenario: Initial peer discovery

**Given** the application just started
**When** the backend emits a `peers-discovered` event
**Then** all discovered peers should be added to the peer store
**And** the peer list UI should populate with all peers

---

### Requirement: File Transfer Events

The application MUST handle file transfer events from the backend including file-transfer-request, progress, completion, and failure events.

**ID:** RTE-004

#### Scenario: Incoming file transfer request

**Given** a peer sends a file
**When** the backend emits a `file-transfer-request` event
**Then** the transfer should be added to the transfer store with status 'waiting'
**And** the UI should show a file transfer notification
**And** the user should be able to accept or reject

#### Scenario: Transfer progress update

**Given** a file transfer is in progress
**When** the backend emits periodic progress events
**Then** the transfer's progress should be updated in the store
**And** the UI should show the progress bar updating

#### Scenario: Transfer completion

**Given** a file transfer is active
**When** the backend emits a `file-transfer-completed` event
**Then** the transfer status should be set to 'completed'
**And** the UI should show the transfer as complete
**And** a success notification should appear

#### Scenario: Transfer failure

**Given** a file transfer is active
**When** the backend emits a `file-transfer-failed` event
**Then** the transfer status should be set to 'failed'
**And** the error message should be stored
**And** the UI should show the failure

---

### Requirement: Event Type Safety

The application MUST provide TypeScript types for all event payloads including MessageReceivedEvent, PeerOnlineEvent, and FileTransferRequestEvent.

**ID:** RTE-005

#### Scenario: MessageReceivedEvent type

**Given** a `message-received` event is received
**When** the event payload is typed as MessageReceivedEvent
**Then** it should have:
- `msgId: string`
- `senderIp: string`
- `senderName: string`
- `content: string`
- `sentAt: number` (i64 milliseconds)

#### Scenario: PeerOnlineEvent type

**Given** a `peer-online` event is received
**When** the event payload is typed as PeerOnlineEvent
**Then** it should have:
- `ip: string`
- `username: string`
- `hostname: string`

#### Scenario: FileTransferRequestEvent type

**Given** a `file-transfer-request` event is received
**When** the event payload is typed as FileTransferRequestEvent
**Then** it should have:
- `transferId: string`
- `peerIp: string`
- `fileName: string`
- `fileSize: number`
- `md5: string | null`

---

### Requirement: Event Listener Lifecycle

The application MUST properly manage event listener lifecycles including cleanup on component unmount and support for multiple callbacks.

**ID:** RTE-006

#### Scenario: Cleanup on component unmount

**Given** a component has registered an event callback
**When** the component unmounts
**Then** the callback should be removed
**And** no memory leaks should occur

#### Scenario: Multiple callbacks for same event

**Given** multiple components need the same event
**When** each component registers a callback
**Then** all callbacks should receive the event
**And** removing one callback should not affect others

#### Scenario: EventManager hook integration

**Given** a component uses the `useRealtimeEvents` hook
**When** the component mounts
**Then** the EventManager should be started
**When** the component unmounts
**Then** the EventManager should be stopped if no other components are using it

---

### Requirement: Event Error Handling

The application MUST handle errors in event processing gracefully including malformed payloads and store update failures.

**ID:** RTE-007

#### Scenario: Malformed event payload

**Given** an event is received with invalid data
**When** the event handler tries to process it
**Then** the error should be caught and logged
**And** the application should not crash
**And** other events should continue to process

#### Scenario: Store update failure

**Given** an event handler tries to update a store
**When** the store update fails
**Then** the error should be logged
**And** the event should be marked as failed
**And** a retry mechanism should be attempted

---

### Requirement: Event Callback Registration

The application MUST allow dynamic registration of event callbacks with support for multiple listeners per event type.

**ID:** RTE-008

#### Scenario: Register message callback

**Given** the EventManager is running
**When** `onMessageReceived(callback)` is called
**Then** the callback should be invoked for each `message-received` event
**And** the callback should receive the typed event payload

#### Scenario: Unregister message callback

**Given** a message callback is registered
**When** the returned unlisten function is called
**Then** the callback should stop receiving events
**And** the callback should be removed from internal storage

#### Scenario: Register multiple callbacks

**Given** multiple callbacks are registered for the same event
**When** the event is triggered
**Then** all registered callbacks should be invoked in order

---

## MODIFIED Requirements

### Requirement: Real-time Message Updates

The messaging system MUST update in real-time when backend emits events, supporting live message feeds and message sent confirmations.

**ID:** RTE-M001
**Modified from:** messaging capability

#### Scenario: Live message feed

**Given** a user is viewing a conversation
**When** a new message arrives from the current peer
**Then** the message should appear immediately
**And** the view should scroll to the new message
**And** the unread count should update

#### Scenario: Message sent confirmation

**Given** a user sends a message
**When** the backend acknowledges receipt
**Then** the message status should change from 'sending' to 'sent'
**And** the UI should reflect the status change

---

## Related Capabilities

- **ipc-connection** - Provides the invoke commands that emit events
- **data-hooks** - Hooks register callbacks with the event system
- **messaging** - Message events update the messaging UI

---

## Event Type Definitions

```typescript
// Event payloads from backend
interface MessageReceivedEvent {
  msgId: string;
  senderIp: string;
  senderName: string;
  receiverIp: string;
  msgType: number;
  content: string;
  isEncrypted: boolean;
  isOffline: boolean;
  sentAt: number; // i64 milliseconds
  receivedAt: number; // i64 milliseconds
}

interface MessageReceiptAckEvent {
  msgId: string;
  status: 'delivered' | 'read';
  timestamp: number; // i64 milliseconds
}

interface PeerOnlineEvent {
  ip: string;
  port: number;
  username: string;
  hostname: string;
  nickname: string | null;
  avatar: string | null;
  groups: string[];
  display_name: string;
}

interface PeerOfflineEvent {
  ip: string;
  lastSeen: number; // i64 milliseconds
}

interface PeersDiscoveredEvent {
  peers: Array<{
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
  }>;
}

interface FileTransferRequestEvent {
  transferId: string;
  peerIp: string;
  fileName: string;
  fileSize: number;
  md5: string | null;
}

interface FileTransferProgressEvent {
  transferId: string;
  transferredBytes: number;
  progress: number; // 0-100
}

interface FileTransferCompletedEvent {
  transferId: string;
  filePath: string;
  completedAt: number; // i64 milliseconds
}

interface FileTransferFailedEvent {
  transferId: string;
  error: string;
  failedAt: number; // i64 milliseconds
}

// Event callback types
type EventCallback<T> = (event: T) => void | Promise<void>;

// Event manager interface
interface EventManager {
  start(): Promise<void>;
  stop(): void;
  onMessageReceived(callback: EventCallback<MessageReceivedEvent>): () => void;
  onMessageReceiptAck(callback: EventCallback<MessageReceiptAckEvent>): () => void;
  onPeerOnline(callback: EventCallback<PeerOnlineEvent>): () => void;
  onPeerOffline(callback: EventCallback<PeerOfflineEvent>): () => void;
  onPeersDiscovered(callback: EventCallback<PeersDiscoveredEvent>): () => void;
  onFileTransferRequest(callback: EventCallback<FileTransferRequestEvent>): () => void;
  onFileTransferProgress(callback: EventCallback<FileTransferProgressEvent>): () => void;
  onFileTransferCompleted(callback: EventCallback<FileTransferCompletedEvent>): () => void;
  onFileTransferFailed(callback: EventCallback<FileTransferFailedEvent>): () => void;
}
```
