# Design: Frontend-Backend Connection Architecture

**Change ID:** `connect-frontend-backend`
**Status:** Draft
**Created:** 2025-01-12

## Overview

This document describes the architectural design for connecting the React frontend to the Rust/Tauri backend, establishing a clean separation of concerns while maintaining type safety and real-time capabilities.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         React Components                        │
│  (Messaging, FileTransfer, BasicSettings, etc.)                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Custom React Hooks                          │
│  usePeers, useMessages, useConfig, useFileTransfers, etc.      │
└─────────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                ▼                           ▼
┌──────────────────────────┐    ┌──────────────────────────┐
│    Zustand Stores        │    │      API Layer           │
│  (Global State)          │    │  (IPC Wrappers)          │
│  - peersStore            │    │  - peersApi              │
│  - messagesStore         │    │  - messagesApi           │
│  - configStore           │    │  - configApi             │
│  - transfersStore        │    │  - transfersApi          │
└──────────────────────────┘    └──────────────────────────┘
                │                           │
                └─────────────┬─────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Type Converters                             │
│  (i64 ms ↔ ISO, status mapping, DTO adaptation)                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Event System                                │
│  (listen/unlisten, event type definitions)                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Tauri IPC Layer                               │
│  invoke() commands, listen() events                             │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Rust Backend Commands                         │
│  peers, config, messages, file_transfer, events                 │
└─────────────────────────────────────────────────────────────────┘
```

## Design Decisions

### 1. Service Layer Pattern

**Decision:** Create a dedicated API service layer that wraps all Tauri IPC calls.

**Rationale:**
- **Centralized error handling**: All IPC errors can be handled in one place
- **Type safety**: TypeScript types can be enforced at the boundary
- **Testability**: Services can be mocked for unit testing
- **Consistency**: Uniform interface for all backend communication

**Implementation:**
```typescript
// src/lib/api/peers.ts
export const peersApi = {
  getPeers: (): Promise<Peer[]> => invoke('get_peers'),
  getOnlinePeers: (): Promise<Peer[]> => invoke('get_online_peers'),
  getPeerByIp: (ip: string): Promise<Peer> => invoke('get_peer_by_ip', { ip }),
  // ... error handling and type conversion
}
```

### 2. Custom Hooks Pattern

**Decision:** Create custom hooks that encapsulate state and side effects for each domain.

**Rationale:**
- **Separation of concerns**: Components focus on UI, hooks manage data
- **Reusability**: Same hook can be used across multiple components
- **Testing**: Hooks can be tested independently
- **React best practices**: Follows React conventions

**Implementation:**
```typescript
// src/hooks/usePeers.ts
export function usePeers() {
  const [peers, setPeers] = useState<Peer[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    refreshPeers();
  }, []);

  const refreshPeers = async () => {
    setLoading(true);
    try {
      const data = await peersApi.getPeers();
      setPeers(data);
    } finally {
      setLoading(false);
    }
  };

  return { peers, loading, refreshPeers };
}
```

### 3. State Management with Zustand

**Decision:** Use Zustand for global state management.

**Rationale:**
- **Simplicity**: Less boilerplate than Redux
- **Performance**: Fine-grained reactivity
- **TypeScript support**: Excellent type inference
- **No providers needed**: Can be used anywhere

**Trade-offs considered:**
- **TanStack Query**: Better for server state, but more complex for real-time events
- **Jotai**: More atomic, but more stores to manage
- **Redux**: Overkill for this application size

### 4. Type Conversion Strategy

**Decision:** Handle type conversion at the API layer boundary.

**Rationale:**
- Frontend uses ISO timestamps, backend uses i64 milliseconds
- Status enums have different values
- Conversion should be transparent to components

**Implementation:**
```typescript
// src/lib/converters/timestamp.ts
export function toIsoDate(millis: number): string {
  return new Date(millis).toISOString();
}

export function fromIsoDate(iso: string): number {
  return new Date(iso).getTime();
}

// src/lib/converters/dto.ts
export function toFrontendPeer(dto: PeerDto): Peer {
  return {
    ...dto,
    lastSeen: toIsoDate(dto.last_seen),
    status: mapPeerStatus(dto.status),
  };
}
```

### 5. Real-time Event System

**Decision:** Create a centralized event system that manages all Tauri event listeners.

**Rationale:**
- **Lifecycle management**: Ensure listeners are properly cleaned up
- **Type safety**: Strongly typed event payloads
- **Reusability**: Same event can update multiple stores

**Events to handle:**
- `message-received` → Update messagesStore
- `peer-online` → Update peersStore
- `peer-offline` → Update peersStore
- `file-transfer-request` → Update transfersStore, show notification
- `peers-discovered` → Update peersStore

**Implementation:**
```typescript
// src/lib/events/manager.ts
class EventManager {
  private listeners: Map<string, UnlistenFn> = new Map();

  async start() {
    this.listeners.set('message-received', await listen('message-received', handle));
    // ... other events
  }

  stop() {
    this.listeners.forEach(fn => fn());
    this.listeners.clear();
  }
}
```

## Data Flow Examples

### Example 1: Loading Initial Peers

```
1. Component mounts → usePeers() hook called
2. Hook calls peersApi.getPeers()
3. API invokes 'get_peers' command
4. Backend returns PeerDto[]
5. API converts DTOs to frontend Peer format
6. Hook updates state
7. Component re-renders with peer data
```

### Example 2: Receiving a Real-time Message

```
1. Backend receives UDP packet from peer
2. Message handler processes packet
3. Backend emits 'message-received' event
4. Frontend event listener receives event
5. Event handler updates messagesStore
6. All components using messages store re-render
```

### Example 3: Sending a Message

```
1. User types message and clicks send
2. Component calls hook's sendMessage()
3. Hook calls messagesApi.sendMessage(text, peerIp)
4. API invokes 'send_message' command
5. Backend creates MessageDto, sends UDP packet
6. Backend updates database
7. Backend returns success to frontend
8. Hook adds message to local store (optimistic update)
9. Component shows message as sent
```

## Type System Alignment

### Frontend Types (Post-Conversion)

```typescript
// src/lib/types/messaging.ts
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
  sentAt: string;        // ISO 8601 string
  receivedAt: string;    // ISO 8601 string
  createdAt: string;     // ISO 8601 string
}

interface UserStatus {
  online: 'online';
  offline: 'offline';
  away: 'away';
  busy: 'busy';  // Added - frontend only
}
```

### Backend Types (Rust)

```rust
// src-tauri/src/commands/message.rs
#[derive(Serialize, Deserialize)]
pub struct MessageDto {
    pub id: String,
    pub msg_id: String,
    pub sender_ip: String,
    pub sender_name: String,
    pub receiver_ip: String,
    pub msg_type: u32,
    pub content: String,
    pub is_encrypted: bool,
    pub is_offline: bool,
    pub sent_at: i64,      // Milliseconds since epoch
    pub received_at: i64,  // Milliseconds since epoch
    pub created_at: i64,   // Milliseconds since epoch
}
```

### Status Mapping

```typescript
// src/lib/converters/status.ts
export type BackendStatus = 'online' | 'offline' | 'away';
export type FrontendStatus = 'online' | 'offline' | 'away' | 'busy';

export function mapPeerStatus(status: BackendStatus): FrontendStatus {
  return status as FrontendStatus; // 'busy' must be set by UI
}
```

## Error Handling Strategy

### Error Types

```typescript
// src/lib/api/errors.ts
export class IpcError extends Error {
  constructor(
    public command: string,
    public originalError: unknown
  ) {
    super(`IPC command '${command}' failed: ${originalError}`);
  }
}

export class NetworkError extends Error {
  constructor(message: string) {
    super(`Network error: ${message}`);
  }
}
```

### Error Handling in API Layer

```typescript
// src/lib/api/base.ts
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    // Handle specific Tauri errors
    if (error instanceof TauriError) {
      throw new IpcError(command, error);
    }
    throw error;
  }
}
```

### Error Boundaries

```typescript
// src/components/ErrorBoundary.tsx
// Wrap the entire app to catch IPC errors gracefully
```

## Performance Considerations

### 1. Optimistic Updates

When sending messages, add to local store immediately before confirmation:

```typescript
const sendMessage = async (content: string) => {
  const tempId = generateTempId();
  messagesStore.addOptimistic({ id: tempId, content, status: 'sending' });

  try {
    const result = await messagesApi.sendMessage(content, peerIp);
    messagesStore.update(tempId, result);
  } catch (error) {
    messagesStore.update(tempId, { status: 'failed' });
  }
};
```

### 2. Event Debouncing

Debounce rapid peer status updates:

```typescript
const debouncedUpdatePeers = debounce(() => {
  peersApi.getPeers().then(setPeers);
}, 500);
```

### 3. Selective Re-renders

Use Zustand selectors to prevent unnecessary re-renders:

```typescript
const onlinePeers = usePeersStore(state => state.peers.filter(p => p.status === 'online'));
```

## Security Considerations

### 1. Input Validation

Validate all data coming from backend before rendering:

```typescript
function sanitizeMessageContent(content: string): string {
  // Prevent XSS
  return content.replace(/<[^>]*>/g, '');
}
```

### 2. Type Validation

Use runtime type checking for critical data:

```typescript
import { z } from 'zod';

const MessageSchema = z.object({
  id: z.string(),
  content: z.string().max(10000),
  // ...
});
```

## Testing Strategy

### Unit Tests

- API layer functions with mocked `invoke()`
- Type converters
- Event handlers

### Integration Tests

- Hook behavior with mocked backend
- Store updates from events

### E2E Tests

- Full flow: send message → receive → display
- Peer discovery flow

## Migration Strategy

### Phase 1: Parallel Implementation

1. Create new API layer alongside existing mock data
2. Add feature flag to switch between mock/real
3. Test real backend without breaking UI

### Phase 2: Gradual Migration

1. Migrate one component at a time
2. Start with least critical (BasicSettings)
3. End with most critical (Messaging)

### Phase 3: Remove Mock Data

1. Once all components connected
2. Remove hardcoded data from App.tsx
3. Clean up unused mock data structures

## Open Issues

1. **State Synchronization**: How to handle conflicts between optimistic updates and server responses?
2. **Offline Handling**: What UI to show when backend is unavailable?
3. **Reconnection**: How to re-establish connection after Tauri restart?
4. **File Chunking**: Large files need chunking - implement on frontend or backend?
