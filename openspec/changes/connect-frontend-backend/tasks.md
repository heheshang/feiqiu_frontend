# Tasks: Connect Frontend to Backend

**Change ID:** `connect-frontend-backend`
**Status:** Draft

## Overview

This is an ordered list of implementation tasks. Tasks are designed to be small, verifiable, and deliver user-visible progress.

---

## Phase 1: Core Connection Layer

### 1.1 Install and Verify Dependencies

**Description:** Ensure Tauri API packages are installed and working.

**Files:**
- `feiqiu/package.json`

**Steps:**
1. Check if `@tauri-apps/api` is installed
2. If not, install: `npm install @tauri-apps/api`
3. Verify version compatibility with Tauri version
4. Test basic import: `import { invoke } from '@tauri-apps/api/core'`

**Validation:**
- [ ] Package contains `@tauri-apps/api`
- [ ] TypeScript compiles without import errors
- [ ] Can successfully invoke `get_system_info` command

**Dependencies:** None
**Estimated time:** 30 minutes

---

### 1.2 Create Type Converters

**Description:** Create utility functions for converting between backend and frontend data formats.

**Files:**
- `feiqiu/src/lib/converters/timestamp.ts` (CREATE)
- `feiqiu/src/lib/converters/status.ts` (CREATE)
- `feiqiu/src/lib/converters/dto.ts` (CREATE)

**Steps:**
1. Create `timestamp.ts` with `toIsoDate()` and `fromIsoDate()` functions
2. Create `status.ts` with `mapPeerStatus()`, `mapTransferStatus()` functions
3. Create `dto.ts` with `toFrontendPeer()`, `toFrontendMessage()`, `toFrontendTransfer()` functions

**Validation:**
- [ ] All converter functions have unit tests
- [ ] Timestamp converter handles edge cases (negative, zero, very large values)
- [ ] Status mapper handles all backend enum values
- [ ] DTO adapters convert all fields correctly

**Dependencies:** 1.1
**Estimated time:** 2 hours

---

### 1.3 Create API Base Infrastructure

**Description:** Create base API wrapper with error handling.

**Files:**
- `feiqiu/src/lib/api/base.ts` (CREATE)
- `feiqiu/src/lib/api/errors.ts` (CREATE)
- `feiqiu/src/lib/api/types.ts` (CREATE)

**Steps:**
1. Create `IpcError` and `NetworkError` classes in `errors.ts`
2. Create `invokeCommand<T>()` wrapper in `base.ts` with error handling
3. Define backend DTO types in `types.ts` (PeerDto, MessageDto, ConfigDto, TaskDto)

**Validation:**
- [ ] Error classes properly wrap Tauri errors
- [ ] invokeCommand catches and rethrows typed errors
- [ ] DTO types match backend Rust structs exactly
- [ ] TypeScript compiles without type errors

**Dependencies:** 1.2
**Estimated time:** 2 hours

---

### 1.4 Create Peers API

**Description:** Create API wrapper for peer-related commands.

**Files:**
- `feiqiu/src/lib/api/peers.ts` (CREATE)

**Steps:**
1. Create `peersApi` object with methods:
   - `getPeers()`
   - `getOnlinePeers()`
   - `getPeerByIp(ip)`
   - `getPeerStats()`
2. Use `invokeCommand` from base
3. Apply type converters to return frontend types
4. Add JSDoc comments for each method

**Validation:**
- [ ] All peer commands successfully invoke backend
- [ ] Returned data matches frontend Peer type
- [ ] Errors are properly caught and wrapped
- [ ] TypeScript types are correct

**Dependencies:** 1.3
**Estimated time:** 1 hour

---

### 1.5 Create Config API

**Description:** Create API wrapper for configuration commands.

**Files:**
- `feiqiu/src/lib/api/config.ts` (CREATE)

**Steps:**
1. Create `configApi` object with methods:
   - `getConfig()`
   - `setConfig(config)`
   - `resetConfig()`
   - `getConfigValue(key)`
   - `setConfigValue(key, value)`
2. Use `invokeCommand` from base
3. Apply type converters

**Validation:**
- [ ] All config commands work
- [ ] Config changes persist across restarts
- [ ] Type conversion works for all config fields
- [ ] Invalid config values are rejected

**Dependencies:** 1.3
**Estimated time:** 1 hour

---

### 1.6 Create Messages API

**Description:** Create API wrapper for message commands.

**Files:**
- `feiqiu/src/lib/api/messages.ts` (CREATE)

**Steps:**
1. Create `messagesApi` object with methods:
   - `sendMessage(content, receiverIp)`
   - `sendTextMessage(content, receiverIp)` (alias)
   - `getMessages(filters?)`
2. Use `invokeCommand` from base
3. Apply type converters

**Validation:**
- [ ] Can send messages to peers
- [ ] Can retrieve message history
- [ ] Timestamps are converted correctly
- [ ] Message status updates work

**Dependencies:** 1.3
**Estimated time:** 1 hour

---

### 1.7 Create File Transfers API

**Description:** Create API wrapper for file transfer commands.

**Files:**
- `feiqiu/src/lib/api/transfers.ts` (CREATE)

**Steps:**
1. Create `transfersApi` object with methods:
   - `acceptFileTransfer(transferId)`
   - `rejectFileTransfer(transferId)`
   - `getFileTransfers()`
   - `cancelFileTransfer(transferId)`
2. Use `invokeCommand` from base
3. Apply type converters

**Validation:**
- [ ] Can retrieve file transfer list
- [ ] Can accept incoming transfers
- [ ] Can reject transfers
- [ ] Can cancel active transfers

**Dependencies:** 1.3
**Estimated time:** 1 hour

---

### 1.8 Create Events System

**Description:** Create centralized event listener management.

**Files:**
- `feiqiu/src/lib/events/types.ts` (CREATE)
- `feiqiu/src/lib/events/manager.ts` (CREATE)
- `feiqiu/src/lib/events/index.ts` (CREATE)

**Steps:**
1. Define event types in `types.ts` (MessageReceivedEvent, PeerOnlineEvent, etc.)
2. Create `EventManager` class with:
   - `start()` - Register all listeners
   - `stop()` - Cleanup all listeners
   - Event handlers that call callback functions
3. Export singleton instance in `index.ts`

**Validation:**
- [ ] EventManager starts and stops without errors
- [ ] All event types are properly typed
- [ ] Listeners cleanup correctly (no memory leaks)
- [ ] Events are received when triggered by backend

**Dependencies:** 1.3
**Estimated time:** 2 hours

---

## Phase 2: Data Fetching Hooks

### 2.1 Create usePeers Hook

**Description:** Create custom hook for peer data management.

**Files:**
- `feiqiu/src/hooks/usePeers.ts` (CREATE)

**Steps:**
1. Create hook with state: `peers`, `loading`, `error`
2. Implement `refreshPeers()` function using `peersApi`
3. Add auto-refresh on component mount
4. Return values and refresh function
5. Add JSDoc documentation

**Validation:**
- [ ] Hook loads peers on mount
- [ ] Loading state works correctly
- [ ] Errors are caught and stored
- [ ] Refresh function updates peer list
- [ ] Hook works in multiple components simultaneously

**Dependencies:** 1.4, 1.8
**Estimated time:** 1.5 hours

---

### 2.2 Create useMessages Hook

**Description:** Create custom hook for message data management.

**Files:**
- `feiqiu/src/hooks/useMessages.ts` (CREATE)

**Steps:**
1. Create hook with state: `messages`, `loading`, `sending`
2. Implement `sendMessage(content, receiverIp)` with optimistic update
3. Implement `loadMessages(peerIp)` for history
4. Auto-update on `message-received` event
5. Return values and actions

**Validation:**
- [ ] Can load message history
- [ ] Can send new messages
- [ ] Optimistic updates work
- [ ] Real-time messages appear immediately
- [ ] Sending state updates correctly

**Dependencies:** 1.6, 1.8
**Estimated time:** 2 hours

---

### 2.3 Create useConfig Hook

**Description:** Create custom hook for configuration management.

**Files:**
- `feiqiu/src/hooks/useConfig.ts` (CREATE)

**Steps:**
1. Create hook with state: `config`, `loading`, `updating`
2. Implement `updateConfig(newConfig)` function
3. Implement `resetConfig()` function
4. Load config on mount
5. Return values and update functions

**Validation:**
- [ ] Config loads on mount
- [ ] Updates persist to backend
- [ ] Reset works correctly
- [ ] Individual value updates work
- [ ] Changes reflect across all components

**Dependencies:** 1.5
**Estimated time:** 1.5 hours

---

### 2.4 Create useFileTransfers Hook

**Description:** Create custom hook for file transfer management.

**Files:**
- `feiqiu/src/hooks/useFileTransfers.ts` (CREATE)

**Steps:**
1. Create hook with state: `transfers`, `loading`
2. Implement `acceptTransfer(transferId)`
3. Implement `rejectTransfer(transferId)`
4. Implement `cancelTransfer(transferId)`
5. Auto-update on transfer events
6. Return values and actions

**Validation:**
- [ ] Can retrieve transfer list
- [ ] Can accept transfers
- [ ] Can reject transfers
- [ ] Can cancel transfers
- [ ] Real-time updates work

**Dependencies:** 1.7, 1.8
**Estimated time:** 1.5 hours

---

### 2.5 Create useRealtimeEvents Hook

**Description:** Create hook for automatic event listener management.

**Files:**
- `feiqiu/src/hooks/useRealtimeEvents.ts` (CREATE)

**Steps:**
1. Create hook that accepts event callbacks
2. Start EventManager on mount
3. Stop EventManager on unmount
4. Provide event registration functions
5. Return event emitter for manual triggering

**Validation:**
- [ ] EventManager starts on mount
- [ ] EventManager stops on unmount
- [ ] No memory leaks after unmount
- [ ] Can register additional callbacks
- [ ] Works in multiple components

**Dependencies:** 1.8
**Estimated time:** 1 hour

---

## Phase 3: Component Integration

### 3.1 Connect Messaging Component

**Description:** Replace mock data with real data in Messaging component.

**Files:**
- `feiqiu/src/components/messaging/Messaging.tsx` (MODIFY)

**Steps:**
1. Import `usePeers` and `useMessages` hooks
2. Remove mock peer data
3. Use `usePeers()` to get peer list
4. Use `useMessages()` to get messages
5. Wire up `sendMessage` to message input
6. Handle loading and error states
7. Update ConversationItem and MessageBubble props

**Validation:**
- [ ] Displays real peers from LAN
- [ ] Can select real peer conversations
- [ ] Can send messages to selected peer
- [ ] Incoming messages appear automatically
- [ ] Loading states show correctly
- [ ] Error states handled gracefully

**Dependencies:** 2.1, 2.2
**Estimated time:** 3 hours

---

### 3.2 Connect BasicSettings Component

**Description:** Connect settings to backend config.

**Files:**
- `feiqiu/src/components/basic-settings/BasicSettings.tsx` (MODIFY)

**Steps:**
1. Import `useConfig` hook
2. Remove mock config data
3. Use `useConfig()` to get current config
4. Wire up form inputs to config fields
5. Implement save functionality
6. Add reset to defaults button
7. Update NetworkStatusCard to use real data

**Validation:**
- [ ] Shows current config from backend
- [ ] Can update username
- [ ] Can update network settings
- [ ] Changes persist after restart
- [ ] Reset to defaults works
- [ ] Network status displays real data

**Dependencies:** 2.3
**Estimated time:** 2 hours

---

### 3.3 Connect FileTransfer Component

**Description:** Connect file transfer UI to backend.

**Files:**
- `feiqiu/src/components/file-transfer/FileTransfer.tsx` (MODIFY)

**Steps:**
1. Import `useFileTransfers` hook
2. Remove mock transfer data
3. Use `useFileTransfers()` to get transfer list
4. Wire up accept/reject/cancel buttons
5. Display real progress updates
6. Handle transfer completion/failure

**Validation:**
- [ ] Shows active and completed transfers
- [ ] Can accept incoming file requests
- [ ] Can reject incoming file requests
- [ ] Can cancel active transfers
- [ ] Progress updates in real-time
- [ ] Transfer status updates correctly

**Dependencies:** 2.4
**Estimated time:** 2 hours

---

### 3.4 Update AppShell Component

**Description:** Connect shell components to system info.

**Files:**
- `feiqiu/src/components/shell/AppShell.tsx` (MODIFY)
- `feiqiu/src/components/shell/UserMenu.tsx` (MODIFY)

**Steps:**
1. Import `useConfig` hook
2. Display current username from config
3. Display user status from config
4. Wire up user status change
5. Connect to get_system_info for platform info

**Validation:**
- [ ] Displays current username
- [ ] User status is shown and can be changed
- [ ] Avatar displays correctly (if set)
- [ ] Platform info is accurate

**Dependencies:** 2.3
**Estimated time:** 1.5 hours

---

### 3.5 Clean Up App.tsx Mock Data

**Description:** Remove all mock data from App.tsx.

**Files:**
- `feiqiu/src/App.tsx` (MODIFY)

**Steps:**
1. Remove hardcoded mock data objects
2. Remove mock data props from components
3. Add ErrorBoundary for IPC errors
4. Add loading fallback for initial data load
5. Clean up unused imports

**Validation:**
- [ ] No mock data in App.tsx
- [ ] App loads with real data
- [ ] Error boundary catches IPC errors
- [ ] App works with backend running
- [ ] App shows appropriate message if backend unavailable

**Dependencies:** 3.1, 3.2, 3.3, 3.4
**Estimated time:** 1 hour

---

## Phase 4: Backend Enhancements

### 4.1 Connect get_messages to Database

**Description:** Implement actual database query for message history.

**Files:**
- `feiqiu/src-tauri/src/commands/message.rs` (MODIFY)

**Steps:**
1. Import message repository
2. Replace empty vector return with actual query
3. Filter by sender_ip/receiver_ip if provided
4. Apply date range filter if provided
5. Handle database errors gracefully
6. Add tests

**Validation:**
- [ ] Returns actual message history from database
- [ ] Filters work correctly
- [ ] Empty result set handled
- [ ] Database errors caught and returned
- [ ] Unit tests pass

**Dependencies:** None (backend only)
**Estimated time:** 2 hours

---

### 4.2 Implement get_network_status Command

**Description:** Create command for network status information.

**Files:**
- `feiqiu/src-tauri/src/commands/network_status.rs` (CREATE)
- `feiqiu/src-tauri/src/commands/mod.rs` (MODIFY)
- `feiqiu/src-tauri/src/lib.rs` (MODIFY)

**Steps:**
1. Create NetworkStatusDto struct
2. Implement `get_network_status` command
3. Get local IP address
4. Get MAC address (if available)
5. Get listening port from config
6. Return network status
7. Register command in lib.rs

**Validation:**
- [ ] Command returns valid network status
- [ ] IP address is accurate
- [ ] MAC address is provided if available
- [ ] Listening port matches config
- [ ] Registered in invoke_handler

**Dependencies:** None (backend only)
**Estimated time:** 2 hours

---

### 4.3 Complete File Transfer Implementation

**Description:** Finish file transfer command implementations.

**Files:**
- `feiqiu/src-tauri/src/commands/file_transfer.rs` (MODIFY)

**Steps:**
1. Implement actual TCP listener for incoming files
2. Implement TCP client for outgoing files
3. Connect to FileTransferManager
4. Track progress updates
5. Handle accept/reject logic
6. Add error handling for network failures

**Validation:**
- [ ] Can send file to peer
- [ ] Can receive file from peer
- [ ] Progress updates work
- [ ] Transfer completion detected
- [ ] Failed transfers handled correctly

**Dependencies:** None (backend only)
**Estimated time:** 4 hours

---

## Phase 5: State Management

### 5.1 Install Zustand

**Description:** Install and configure Zustand for state management.

**Files:**
- `feiqiu/package.json` (MODIFY)

**Steps:**
1. Install zustand: `npm install zustand`
2. Verify TypeScript support
3. Create base store structure

**Validation:**
- [ ] zustand installed
- [ ] TypeScript types work correctly
- [ ] Can create basic store

**Dependencies:** None
**Estimated time:** 30 minutes

---

### 5.2 Create Peers Store

**Description:** Create Zustand store for peer management.

**Files:**
- `feiqiu/src/lib/store/peers.ts` (CREATE)

**Steps:**
1. Create peers store with:
   - `peers` array
   - `loading` flag
   - `setPeers` action
   - `updatePeer` action
   - `removePeer` action
   - `getOnlinePeers` selector
2. Connect to event system for auto-updates
3. Add persistence middleware (optional)

**Validation:**
- [ ] Store created successfully
- [ ] Can read peer list
- [ ] Can update individual peers
- [ ] Can remove peers
- [ ] Selectors work correctly
- [ ] Events update store

**Dependencies:** 5.1, 1.8
**Estimated time:** 2 hours

---

### 5.3 Create Messages Store

**Description:** Create Zustand store for message management.

**Files:**
- `feiqiu/src/lib/store/messages.ts` (CREATE)

**Steps:**
1. Create messages store with:
   - `messages` array
   - `messagesByPeer` map
   - `addMessage` action
   - `addOptimisticMessage` action
   - `updateMessage` action
   - `getMessagesByPeer` selector
2. Connect to message-received event

**Validation:**
- [ ] Store created successfully
- [ ] Can add messages
- [ ] Can update optimistic messages
- [ ] Can filter by peer
- [ ] Real-time messages appear

**Dependencies:** 5.1, 1.8
**Estimated time:** 2 hours

---

### 5.4 Create Config Store

**Description:** Create Zustand store for configuration.

**Files:**
- `feiqiu/src/lib/store/config.ts` (CREATE)

**Steps:**
1. Create config store with:
   - `config` object
   - `loading` flag
   - `setConfig` action
   - `updateField` action
2. Load initial config from backend
3. Persist changes to backend

**Validation:**
- [ ] Store created successfully
- [ ] Config loads on init
- [ ] Can update full config
- [ ] Can update individual fields
- [ ] Changes persist to backend

**Dependencies:** 5.1
**Estimated time:** 1.5 hours

---

### 5.5 Create Transfers Store

**Description:** Create Zustand store for file transfers.

**Files:**
- `feiqiu/src/lib/store/transfers.ts` (CREATE)

**Steps:**
1. Create transfers store with:
   - `transfers` array
   - `addTransfer` action
   - `updateTransfer` action
   - `removeTransfer` action
   - `getActiveTransfers` selector
2. Connect to file-transfer events
3. Handle progress updates

**Validation:**
- [ ] Store created successfully
- [ ] Can add transfers
- [ ] Can update progress
- [ ] Can remove completed transfers
- [ ] Real-time updates work

**Dependencies:** 5.1, 1.8
**Estimated time:** 1.5 hours

---

### 5.6 Refactor Hooks to Use Stores

**Description:** Update hooks to use Zustand stores instead of local state.

**Files:**
- `feiqiu/src/hooks/usePeers.ts` (MODIFY)
- `feiqiu/src/hooks/useMessages.ts` (MODIFY)
- `feiqiu/src/hooks/useConfig.ts` (MODIFY)
- `feiqiu/src/hooks/useFileTransfers.ts` (MODIFY)

**Steps:**
1. Replace local state with store selectors
2. Update actions to call store actions
3. Keep hook interface the same (no component changes needed)
4. Add store subscription logic

**Validation:**
- [ ] Hooks still work correctly
- [ ] Components don't need changes
- [ ] State is shared across components
- [ ] Real-time updates work everywhere

**Dependencies:** 5.2, 5.3, 5.4, 5.5
**Estimated time:** 2 hours

---

## Final Tasks

### F.1 Integration Testing

**Description:** Test full application flow end-to-end.

**Steps:**
1. Start application with backend running
2. Test peer discovery
3. Test sending messages
4. Test receiving messages
5. Test configuration changes
6. Test file transfer (if peer available)
7. Test error scenarios

**Validation:**
- [ ] All features work end-to-end
- [ ] No console errors
- [ ] No memory leaks
- [ ] Performance is acceptable
- [ ] UI updates smoothly

**Dependencies:** All previous tasks
**Estimated time:** 3 hours

---

### F.2 Documentation Update

**Description:** Update project documentation to reflect changes.

**Files:**
- `CLAUDE.md` (MODIFY)
- `openspec/project.md` (MODIFY)

**Steps:**
1. Update tech stack to include zustand
2. Update architecture diagrams
3. Document new API layer
4. Document new hooks
5. Update code examples

**Validation:**
- [ ] Documentation is accurate
- [ ] Code examples work
- [ ] Architecture is clear

**Dependencies:** F.1
**Estimated time:** 1 hour

---

### F.3 Clean Up and Polish

**Description:** Final code cleanup and optimization.

**Steps:**
1. Remove unused imports
2. Remove commented-out code
3. Add missing JSDoc comments
4. Fix any TypeScript warnings
5. Run linter and fix issues
6. Run formatter

**Validation:**
- [ ] No unused imports
- [ ] No commented code
- [ ] All public functions documented
- [ ] No TypeScript warnings
- [ ] Code is formatted

**Dependencies:** F.1
**Estimated time:** 2 hours

---

## Task Summary

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Phase 1: Core Connection Layer | 8 tasks | ~11 hours |
| Phase 2: Data Fetching Hooks | 5 tasks | ~7.5 hours |
| Phase 3: Component Integration | 5 tasks | ~9.5 hours |
| Phase 4: Backend Enhancements | 3 tasks | ~8 hours |
| Phase 5: State Management | 6 tasks | ~9.5 hours |
| Final Tasks | 3 tasks | ~6 hours |
| **Total** | **30 tasks** | **~51.5 hours** |

## Parallelization Opportunities

The following tasks can be done in parallel:

**Sprint 1 (Phase 1):**
- Tasks 1.4, 1.5, 1.6, 1.7 can be done in parallel after 1.3
- Task 1.8 can be done alongside 1.4-1.7

**Sprint 2 (Phase 2):**
- Tasks 2.1, 2.2, 2.3, 2.4 can be done in parallel after Phase 1
- Task 2.5 can be done after 1.8

**Sprint 3 (Phase 3):**
- Tasks 3.2, 3.3, 3.4 can be done in parallel
- Task 3.1 must be before 3.5

**Sprint 4 (Phase 4):**
- All tasks can be done in parallel (backend only)

**Sprint 5 (Phase 5):**
- Tasks 5.2, 5.3, 5.4, 5.5 can be done in parallel after 5.1
- Task 5.6 must wait for stores

With parallelization, the total timeline can be reduced to approximately **5-6 working days**.
