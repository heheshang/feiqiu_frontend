# Spec: Data Fetching Hooks

**Capability:** `data-hooks`
**Change ID:** `connect-frontend-backend`
**Status:** Proposed

## Overview

This specification defines custom React hooks that provide data fetching, state management, and real-time updates for each domain in the application. Hooks abstract the complexity of API calls, event handling, and state updates.

---

## ADDED Requirements

### Requirement: usePeers Hook

The application MUST provide a `usePeers` hook for peer data management that loads peers on mount, supports refresh, handles loading/error states, and updates in real-time.

**ID:** DH-001

#### Scenario: Load peers on mount

**Given** a component uses the `usePeers` hook
**When** the component mounts
**Then** the hook should call `peersApi.getPeers()`
**And** set the `peers` state with the result
**And** set `loading` to false

#### Scenario: Refresh peer list

**Given** the hook is mounted
**When** the `refreshPeers` function is called
**Then** the hook should call `peersApi.getPeers()` again
**And** update the `peers` state with fresh data

#### Scenario: Handle loading state

**Given** the hook is fetching peers
**When** the API call is in progress
**Then** `loading` should be true
**And** the component can show a loading indicator

#### Scenario: Handle errors

**Given** the hook is fetching peers
**When** the API call fails
**Then** `error` should contain the error message
**And** `loading` should be false
**And** the component can show an error message

#### Scenario: Real-time peer updates

**Given** the hook is mounted
**When** a `peer-online` event is received
**Then** the peer should be added/updated in the `peers` state
**And** the component should re-render with the new peer

---

### Requirement: useMessages Hook

The application MUST provide a `useMessages` hook for message data management that loads history, sends messages with optimistic updates, handles loading/sending states, and receives real-time messages.

**ID:** DH-002

#### Scenario: Load message history

**Given** a component uses the `useMessages` hook with `peerIp`
**When** the component mounts
**Then** the hook should call `messagesApi.getMessages({ peerIp })`
**And** set the `messages` state with the history

#### Scenario: Send a message

**Given** the hook is mounted
**When** `sendMessage('Hello', '192.168.1.100')` is called
**Then** an optimistic message should be added to `messages` with status 'sending'
**And** `messagesApi.sendMessage()` should be called
**And** on success, the message status should update to 'sent'

#### Scenario: Handle sending state

**Given** a message is being sent
**When** the API call is in progress
**Then** `sending` should be true
**And** the component can disable the send button

#### Scenario: Receive real-time message

**Given** the hook is mounted with a specific peerIp
**When** a `message-received` event is received for that peer
**Then** the message should be added to the `messages` state
**And** the component should re-render

#### Scenario: Load all messages (no filter)

**Given** a component uses the `useMessages` hook without `peerIp`
**When** the component mounts
**Then** the hook should call `messagesApi.getMessages()` without filters
**And** return all messages from all peers

---

### Requirement: useConfig Hook

The application MUST provide a `useConfig` hook for configuration management that loads config on mount, supports full and partial updates, handles resetting, and manages the updating state.

**ID:** DH-003

#### Scenario: Load config on mount

**Given** a component uses the `useConfig` hook
**When** the component mounts
**Then** the hook should call `configApi.getConfig()`
**And** set the `config` state with the result

#### Scenario: Update full config

**Given** the hook is mounted
**When** `updateConfig({ username: 'Alice', status: 'away' })` is called
**Then** `configApi.setConfig()` should be called with the new values
**And** the `config` state should update with the merged result

#### Scenario: Update single field

**Given** the hook is mounted
**When** `updateField('username', 'Bob')` is called
**Then** `configApi.setConfigValue()` should be called with the key and value
**And** only the username field should update in the `config` state

#### Scenario: Reset to defaults

**Given** the hook is mounted
**When** `resetConfig()` is called
**Then** `configApi.resetConfig()` should be called
**And** the `config` state should update with default values

#### Scenario: Handle updating state

**Given** config is being updated
**When** the API call is in progress
**Then** `updating` should be true
**And** the component can show a saving indicator

---

### Requirement: useFileTransfers Hook

The application MUST provide a `useFileTransfers` hook for file transfer management that loads transfers, supports accept/reject/cancel operations, and handles real-time progress updates.

**ID:** DH-004

#### Scenario: Load transfers on mount

**Given** a component uses the `useFileTransfers` hook
**When** the component mounts
**Then** the hook should call `transfersApi.getFileTransfers()`
**And** set the `transfers` state with the result

#### Scenario: Accept incoming transfer

**Given** a transfer has status 'waiting'
**When** `acceptTransfer(transferId)` is called
**Then** `transfersApi.acceptFileTransfer(transferId)` should be called
**And** the transfer status should update to 'transferring' in the state

#### Scenario: Reject incoming transfer

**Given** a transfer has status 'waiting'
**When** `rejectTransfer(transferId)` is called
**Then** `transfersApi.rejectFileTransfer(transferId)` should be called
**And** the transfer should be removed from the state

#### Scenario: Cancel active transfer

**Given** a transfer has status 'transferring'
**When** `cancelTransfer(transferId)` is called
**Then** `transfersApi.cancelFileTransfer(transferId)` should be called
**And** the transfer status should update to 'cancelled'

#### Scenario: Real-time transfer updates

**Given** the hook is mounted
**When** a `file-transfer-progress` event is received
**Then** the corresponding transfer's `progress` should update
**And** the `transferredBytes` should update
**And** the component should re-render

---

### Requirement: useRealtimeEvents Hook

The application MUST provide a `useRealtimeEvents` hook for event system integration that starts EventManager on mount, stops on unmount, supports multiple components, and allows callback registration.

**ID:** DH-005

#### Scenario: Start event manager on mount

**Given** a component uses the `useRealtimeEvents` hook
**When** the component mounts
**Then** the EventManager should be started
**And** the hook should return the event emitter

#### Scenario: Stop event manager on unmount

**Given** the hook is mounted
**When** the component unmounts
**Then** the EventManager should be stopped
**And** all event listeners should be cleaned up

#### Scenario: Register event callback

**Given** the hook is mounted
**When** `on('message-received', callback)` is called
**Then** the callback should be registered for that event
**And** the callback should be invoked when the event occurs

#### Scenario: Unregister callback

**Given** a callback is registered
**When** the returned unlisten function is called
**Then** the callback should stop receiving events

#### Scenario: Multiple components using hook

**Given** multiple components use the `useRealtimeEvents` hook
**When** all components are mounted
**Then** the EventManager should only start once
**When** all components unmount
**Then** the EventManager should stop

---

### Requirement: Hook Return Types

All hooks MUST return consistent types with state, actions, and flags including peers, messages, config, and transfers hooks.

**ID:** DH-006

#### Scenario: usePeers return type

**Given** the `usePeers` hook is called
**When** the return value is destructured
**Then** it should provide:
- `peers: Peer[]` - The list of peers
- `loading: boolean` - True while fetching
- `error: Error | null` - Error if fetch failed
- `refreshPeers: () => Promise<void>` - Function to refresh the list

#### Scenario: useMessages return type

**Given** the `useMessages` hook is called
**When** the return value is destructured
**Then** it should provide:
- `messages: Message[]` - The list of messages
- `loading: boolean` - True while fetching history
- `sending: boolean` - True while sending a message
- `error: Error | null` - Error if operation failed
- `sendMessage: (content: string, receiverIp: string) => Promise<void>` - Send function
- `loadMessages: (peerIp?: string) => Promise<void>` - Load history function

#### Scenario: useConfig return type

**Given** the `useConfig` hook is called
**When** the return value is destructured
**Then** it should provide:
- `config: Config` - The current configuration
- `loading: boolean` - True while loading
- `updating: boolean` - True while saving
- `error: Error | null` - Error if operation failed
- `updateConfig: (updates: Partial<Config>) => Promise<void>` - Update function
- `updateField: (key: string, value: any) => Promise<void>` - Update single field
- `resetConfig: () => Promise<void>` - Reset to defaults

#### Scenario: useFileTransfers return type

**Given** the `useFileTransfers` hook is called
**When** the return value is destructured
**Then** it should provide:
- `transfers: FileTransfer[]` - The list of transfers
- `loading: boolean` - True while fetching
- `error: Error | null` - Error if operation failed
- `acceptTransfer: (id: string) => Promise<void>` - Accept function
- `rejectTransfer: (id: string) => Promise<void>` - Reject function
- `cancelTransfer: (id: string) => Promise<void>` - Cancel function

---

### Requirement: Hook State Persistence

Hooks MUST maintain state consistency across component re-renders and properly handle dependency changes.

**ID:** DH-007

#### Scenario: State persists across re-renders

**Given** a component using `usePeers` re-renders
**When** the re-render completes
**Then** the `peers` state should remain the same
**And** no additional API calls should be made

#### Scenario: Dependencies trigger refetch

**Given** a component using `useMessages(peerIp)` changes the `peerIp` prop
**When** the component re-renders
**Then** the hook should call `loadMessages(newPeerIp)`
**And** the `messages` state should update with the new peer's messages

#### Scenario: Memoized callbacks

**Given** a component using `useConfig` destructures the returned functions
**When** the component re-renders
**Then** the function references should remain stable (useCallback)
**And** child components should not unnecessarily re-render

---

### Requirement: Hook Error Recovery

Hooks MUST provide mechanisms for error recovery including retry functionality, error boundary integration, and graceful degradation.

**ID:** DH-008

#### Scenario: Retry on error

**Given** a hook call fails with an error
**When** the user triggers a retry action
**Then** the hook should attempt the API call again
**And** update the state with the new result

#### Scenario: Error boundary integration

**Given** a hook throws an unexpected error
**When** the error propagates to the ErrorBoundary
**Then** the component should show the fallback UI
**And** the app should not crash

#### Scenario: Graceful degradation

**Given** the backend is unavailable
**When** a hook tries to fetch data
**Then** the hook should set an error state
**And** the component can show cached data if available
**And** the component should not crash

---

## Hook Type Definitions

```typescript
// usePeers
interface UsePeersReturn {
  peers: Peer[];
  loading: boolean;
  error: Error | null;
  refreshPeers: () => Promise<void>;
}

function usePeers(): UsePeersReturn;

// useMessages
interface UseMessagesOptions {
  peerIp?: string;
  autoLoad?: boolean;
}

interface UseMessagesReturn {
  messages: Message[];
  loading: boolean;
  sending: boolean;
  error: Error | null;
  sendMessage: (content: string, receiverIp: string) => Promise<void>;
  loadMessages: (peerIp?: string) => Promise<void>;
}

function useMessages(options?: UseMessagesOptions): UseMessagesReturn;

// useConfig
interface UseConfigReturn {
  config: Config;
  loading: boolean;
  updating: boolean;
  error: Error | null;
  updateConfig: (updates: Partial<Config>) => Promise<void>;
  updateField: (key: keyof Config, value: any) => Promise<void>;
  resetConfig: () => Promise<void>;
}

function useConfig(): UseConfigReturn;

// useFileTransfers
interface UseFileTransfersReturn {
  transfers: FileTransfer[];
  loading: boolean;
  error: Error | null;
  acceptTransfer: (id: string) => Promise<void>;
  rejectTransfer: (id: string) => Promise<void>;
  cancelTransfer: (id: string) => Promise<void>;
}

function useFileTransfers(): UseFileTransfersReturn;

// useRealtimeEvents
interface UseRealtimeEventsReturn {
  on: <T extends EventName>(
    event: T,
    callback: EventCallback<T>
  ) => () => void;
  emit: <T extends EventName>(event: T, data: EventData[T]) => void;
}

function useRealtimeEvents(): UseRealtimeEventsReturn;

// Event types
type EventName =
  | 'message-received'
  | 'message-receipt-ack'
  | 'peer-online'
  | 'peer-offline'
  | 'peers-discovered'
  | 'file-transfer-request'
  | 'file-transfer-progress'
  | 'file-transfer-completed'
  | 'file-transfer-failed';

interface EventData {
  'message-received': MessageReceivedEvent;
  'message-receipt-ack': MessageReceiptAckEvent;
  'peer-online': PeerOnlineEvent;
  'peer-offline': PeerOfflineEvent;
  'peers-discovered': PeersDiscoveredEvent;
  'file-transfer-request': FileTransferRequestEvent;
  'file-transfer-progress': FileTransferProgressEvent;
  'file-transfer-completed': FileTransferCompletedEvent;
  'file-transfer-failed': FileTransferFailedEvent;
}

type EventCallback<T extends EventName> = (data: EventData[T]) => void;
```

---

## Related Capabilities

- **ipc-connection** - Hooks use the API layer for data fetching
- **realtime-events** - Hooks register callbacks with the event system
- **messaging** - useMessages hook provides data to messaging components
- **basic-settings** - useConfig hook provides data to settings components
- **file-transfer** - useFileTransfers hook provides data to transfer components
