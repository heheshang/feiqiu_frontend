# Change: Connect Frontend to Backend

## Why

The frontend currently operates entirely with mock data while the backend has fully implemented commands that are not being utilized. This disconnect prevents users from using any actual functionality like peer discovery, messaging, or file transfer.

## What Changes

- Create IPC service layer (`src/lib/api/`) with type-safe wrappers for all backend commands
- Implement real-time event system to handle backend events (`message-received`, `peer-online`, etc.)
- Create custom React hooks (`usePeers`, `useMessages`, `useConfig`, `useFileTransfers`, `useRealtimeEvents`)
- Add type converters for backend-frontend data transformation (i64 timestamps ↔ ISO strings, status mapping)
- Replace mock data in components with real API calls
- Connect backend database integration for message history and peer persistence
- Add Zustand stores for global state management

## Impact

- **User-facing**: Users can now discover LAN peers, send/receive messages, and transfer files
- **Technical**: Establishes the foundation for all frontend-backend communication
- **Breaking**: Existing mock data in App.tsx will be removed (components will need to use hooks)
- **Dependencies**: Requires `@tauri-apps/api` and `zustand` packages

---

## Summary

**Change ID:** `connect-frontend-backend`
**Status:** Draft
**Created:** 2025-01-12
**Author:** AI Assistant

## Problem Statement

### Current State

**Frontend (React/TypeScript):**
- All components use hardcoded sample data from `App.tsx`
- No Tauri API imports (`@tauri-apps/api/core`, `@tauri-apps/api/event`)
- No IPC communication with backend
- No real-time event listeners
- Types are defined but don't match backend DTOs exactly

**Backend (Rust/Tauri):**
- Fully functional peer discovery and messaging system
- Complete IPC commands registered (peers, config, messages, file transfers, events)
- Event emission system for real-time updates
- Database entities defined (but not fully integrated)
- IPMsg protocol implementation working

### The Gap

The frontend is a **UI mock** disconnected from the backend. Users cannot:
- See real peers on the LAN
- Send/receive actual messages
- Transfer files
- Configure network settings
- Experience real-time updates

### Type Mismatches

| Aspect | Frontend | Backend | Issue |
|--------|----------|---------|-------|
| Timestamps | ISO string | i64 milliseconds | Conversion needed |
| User Status | `online\|away\|busy\|offline` | `online\|offline\|away` | Missing 'busy' |
| Transfer Status | `waiting\|transferring` | `pending\|active` | Different naming |

## Proposed Solution

### Phase 1: Core Connection Layer

Create the foundational infrastructure for frontend-backend communication:

1. **IPC Service Layer** (`src/lib/api/`)
   - Type-safe wrappers for all backend commands
   - Error handling and retry logic
   - Type conversion between frontend and backend formats

2. **Real-time Event System** (`src/lib/events/`)
   - Event listener management
   - Event type definitions
   - Automatic reconnection on failure

3. **Type Conversion Utilities** (`src/lib/converters/`)
   - Timestamp converters (i64 ms ↔ ISO string)
   - Status mappers (backend ↔ frontend enums)
   - DTO adapters

### Phase 2: Data Fetching Hooks

Create React hooks that abstract API calls and state management:

1. **`usePeers`** - Peer discovery and management
2. **`useMessages`** - Message history and sending
3. **`useConfig`** - Application configuration
4. **`useFileTransfers`** - File transfer management
5. **`useRealtimeEvents`** - Event listening wrapper

### Phase 3: Component Integration

Replace mock data with real data in components:

1. **Messaging** - Connect to peer list and message commands
2. **Basic Settings** - Connect to config commands
3. **File Transfer** - Connect to transfer commands
4. **Shell** - Connect to system info and user status

### Phase 4: Backend Enhancements

Complete partially implemented backend features:

1. **Message Repository** - Connect `get_messages` to database
2. **Peer Repository** - Connect peer commands to database
3. **Network Status Command** - Add command for network info
4. **File Transfer Implementation** - Complete transfer logic

### Phase 5: State Management

Implement global state with Zustand:

1. **Peer Store** - Cached peer list with real-time updates
2. **Message Store** - Message history by conversation
3. **Config Store** - Application settings
4. **Transfer Store** - Active and completed transfers

## Affected Components

### Frontend Files to Create

| Path | Purpose |
|------|---------|
| `src/lib/api/peers.ts` | Peer API wrappers |
| `src/lib/api/config.ts` | Config API wrappers |
| `src/lib/api/messages.ts` | Message API wrappers |
| `src/lib/api/transfers.ts` | File transfer API wrappers |
| `src/lib/api/events.ts` | Event listening utilities |
| `src/lib/converters/timestamp.ts` | Time format conversion |
| `src/lib/converters/status.ts` | Status enum mapping |
| `src/lib/converters/dto.ts` | DTO type adapters |
| `src/hooks/usePeers.ts` | Peer data hook |
| `src/hooks/useMessages.ts` | Message data hook |
| `src/hooks/useConfig.ts` | Config data hook |
| `src/hooks/useFileTransfers.ts` | Transfer data hook |
| `src/hooks/useRealtimeEvents.ts` | Event listener hook |
| `src/lib/store/peers.ts` | Peer Zustand store |
| `src/lib/store/messages.ts` | Message Zustand store |
| `src/lib/store/config.ts` | Config Zustand store |
| `src/lib/store/transfers.ts` | Transfer Zustand store |

### Frontend Files to Modify

| Path | Changes |
|------|---------|
| `src/App.tsx` | Remove mock data, add providers |
| `src/components/messaging/Messaging.tsx` | Connect to real peers/messages |
| `src/components/basic-settings/BasicSettings.tsx` | Connect to config API |
| `src/components/file-transfer/FileTransfer.tsx` | Connect to transfer API |
| `src/components/shell/AppShell.tsx` | Connect to system info |
| `src/lib/types/messaging.ts` | Align with backend MessageDto |
| `src/lib/types/basic-settings.ts` | Align with backend ConfigDto |
| `src/lib/types/file-transfer.ts` | Align with backend TaskDto |

### Backend Files to Modify

| Path | Changes |
|------|---------|
| `src-tauri/src/commands/message.rs` | Connect `get_messages` to database |
| `src-tauri/src/commands/file_transfer.rs` | Complete transfer implementations |
| `src-tauri/src/commands/peer.rs` | Connect to database for persistence |
| `src-tauri/src/commands/mod.rs` | Add `get_network_status` command |
| `src-tauri/src/commands/network_status.rs` | **CREATE** Network status command |

## Dependencies

### Required Packages

```json
{
  "@tauri-apps/api": "^2.x",
  "zustand": "^4.x"
}
```

These may already be installed but need to be verified and properly imported.

## Success Criteria

### Functional Requirements

- [ ] Frontend can fetch and display real peers from LAN
- [ ] Users can send messages that are delivered to peers
- [ ] Users can receive and display incoming messages
- [ ] Configuration changes persist and take effect
- [ ] File transfers can be initiated and tracked
- [ ] Real-time events (peer online/offline, new messages) update UI

### Technical Requirements

- [ ] All type mismatches between frontend/backend resolved
- [ ] Error handling for all IPC calls
- [ ] Event listeners properly cleanup on unmount
- [ ] Database integration working for messages and peers
- [ ] State management (Zustand) implemented and working

### Performance Requirements

- [ ] Peer list updates within 1 second of discovery
- [ ] Message delivery within 100ms on LAN
- [ ] UI remains responsive during file transfers

## Alternatives Considered

### Alternative 1: Direct Component IPC Calls

Instead of a service layer, components would directly call `invoke()`.

**Pros:**
- Simpler initial implementation
- Less abstraction

**Cons:**
- Harder to test
- No centralized error handling
- Type safety issues
- Difficult to mock

**Decision:** Rejected - Service layer provides better maintainability.

### Alternative 2: Use React Query instead of Zustand

Use TanStack Query for data fetching instead of Zustand stores.

**Pros:**
- Built-in caching and refetching
- Excellent devtools
- Less boilerplate

**Cons:**
- More complex setup for real-time events
- Additional dependency
- Learning curve

**Decision:** Viable alternative, but Zustand is simpler for this use case. Can be added later if needed.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Breaking existing UI | High | Incremental migration, keep mock data fallback |
| Type mismatches causing runtime errors | Medium | Comprehensive type checking, validation |
| Event listener memory leaks | Medium | Proper cleanup in useEffect |
| Backend commands incomplete | Medium | Implement placeholder commands first |
| Performance issues with real-time updates | Low | Debouncing, throttling, selective updates |

## Timeline Estimate

**Phase 1:** Core Connection Layer - 2-3 days
**Phase 2:** Data Fetching Hooks - 2 days
**Phase 3:** Component Integration - 3-4 days
**Phase 4:** Backend Enhancements - 2-3 days
**Phase 5:** State Management - 2 days

**Total:** 11-14 days

## Related Specs

This change will create or modify the following specs:

- **NEW:** `ipc-connection` - IPC service layer requirements
- **NEW:** `realtime-events` - Event system requirements
- **NEW:** `data-hooks` - Custom hooks requirements
- **MODIFY:** `messaging` - Add real messaging integration
- **MODIFY:** `basic-settings` - Add config integration
- **MODIFY:** `file-transfer` - Add transfer integration

## Open Questions

1. Should we implement state management (Zustand) in Phase 2 or wait until Phase 5?
2. Should we implement a fallback to mock data if backend is unavailable?
3. Should we add error boundaries for IPC failures?
4. What is the priority of organization chart and collaboration features (both have no backend)?
