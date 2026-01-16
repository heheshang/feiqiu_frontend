# Tasks: Enable Contacts List Messaging

## Overview

This document breaks down the implementation of contacts-to-messaging functionality into ordered, verifiable work items.

---

## Phase 1: Data Model & Database

### 1.1 Add `ip_address` column to contacts table

**File:** `src-tauri/src/migration/src/m_*.rs`

- [x] Create new migration file (e.g., `m_20250116_0001_add_contact_ip.rs`)
- [x] Add `ip_address: Option<String>` column to `contacts` table
- [x] Add index on `ip_address` for efficient lookups
- [x] Write `up()` migration to add column
- [x] Write `down()` migration for rollback

**Validation:** Migration runs successfully with `cargo test`

---

### 1.2 Update Contact entity

**File:** `src-tauri/src/storage/entities/contacts.rs`

- [x] Add `ip_address` field to Contact model
- [x] Mark as Option<String> (nullable)
- [x] Update related entity methods if needed

**Validation:** `cargo build` succeeds without errors

---

### 1.3 Update ContactDto

**File:** `src-tauri/src/types/dtos.rs` (or appropriate DTO file)

- [x] Add `ip_address: Option<String>` to ContactDto
- [x] Add `#[serde(rename_all = "camelCase")]` if needed
- [x] Update `From<Contact> for ContactDto` impl

**Validation:** DTO serializes/deserializes correctly

---

## Phase 2: Backend Commands

### 2.1 Add `get_peer_by_ip` command

**File:** `src-tauri/src/commands/peer.rs`

- [x] Create `get_peer_by_ip(ip: String) -> Option<PeerDto>` command
- [x] Search peer list by IP address
- [x] Return peer if found, None otherwise
- [x] Add documentation with TypeScript usage example
- [x] Register command in `lib.rs`

**Validation:** Command returns peer for valid IP, None for invalid IP

---

### 2.2 Update contact creation to store IP

**File:** `src-tauri/src/commands/contacts.rs`

- [x] In `create_contact`, check if peer with matching name exists
- [x] If found, store both `peerId` and `ipAddress`
- [x] Update `create_contact::CreateContactDto` to accept optional IP
- [x] Update `update_contact` to handle IP field

**Validation:** Created contacts have IP when linked peer exists

---

## Phase 3: Frontend Types & API

### 3.1 Update Contact interface

**File:** `feiqiu/src/lib/types/contacts.ts`

- [x] Add `ipAddress?: string` to Contact interface
- [x] Update CreateContactInput to include optional ipAddress
- [x] Update UpdateContactInput to include optional ipAddress

**Validation:** TypeScript compiles without errors

---

### 3.2 Add peer lookup API function

**File:** `feiqiu/src/lib/api/peers.ts`

- [x] Add `getPeerByIp(ip: string): Promise<Peer | null>` function
- [x] Wrap `invoke('get_peer_by_ip', { ip })`
- [x] Handle null return value
- [x] Add error handling

**Validation:** Function returns Peer or null as expected

---

### 3.3 Add contact-to-peer resolution utility

**File:** `feiqiu/src/lib/api/contacts.ts` (or new file)

- [x] Add `findPeerForContact(contact: Contact, peers: Peer[]): Peer | null` function
- [x] Try peerId first
- [x] Try ipAddress lookup second
- [x] Try name match as final fallback
- [x] Return null if no match found

**Validation:** Function correctly finds peer in various scenarios

---

## Phase 4: Frontend Components

### 4.1 Update Contacts.tsx handleSendMessage

**File:** `feiqiu/src/components/contacts/Contacts.tsx`

- [x] Import peer lookup utilities
- [x] Update `handleSendMessage` with multi-step fallback logic:
  - Check contact.peerId
  - Check contact.ipAddress and lookup peer
  - Try finding peer by name
  - Show notification if all fail
- [x] Update contact if peer is found by name/IP

**Validation:** Can start conversation with online contact regardless of peerId

---

### 4.2 Update ContactDetailDialog button

**File:** `feiqiu/src/components/contacts/ContactDialogs.tsx`

- [x] Add availability check to "发消息" button
- [x] Disable button if contact is offline with no stored IP
- [x] Add tooltip explaining disabled state
- [x] Update button styling based on state

**Validation:** Button state reflects contact availability

---

### 4.3 Add notification helper

**File:** `feiqiu/src/lib/utils.ts` or new file

- [x] Add `showNotification(message: string, type: 'info' | 'warning' | 'error')` function
- [x] Create toast notification component if not exists
- [x] Handle auto-dismiss after timeout

**Validation:** Notifications appear and dismiss correctly

---

## Phase 5: Testing & Validation

### 5.1 Test migration rollback

**Task:** Verify database migration can be reversed

- [x] Run `down()` migration
- [x] Verify column is removed
- [x] Verify data integrity

**Validation:** Rollback works without errors

---

### 5.2 Test contact messaging scenarios

**Task:** Manual testing of all scenarios

- [x] Test contact with peerId: should work
- [x] Test contact with ipAddress only: should work if peer online
- [x] Test contact with neither: should show error
- [x] Test offline contact: should show appropriate message
- [x] Test name-based peer resolution: should link and message

**Validation:** All scenarios produce expected behavior

---

### 5.3 Update existing tests

**Task:** Ensure tests pass with new changes

- [x] Update contact CRUD tests for ipAddress field
- [x] Add tests for peer lookup commands
- [x] Update component tests if any

**Validation:** All tests pass

---

## Dependencies

- Task 2.1 depends on: None
- Task 2.2 depends on: 1.1, 1.2, 1.3
- Task 3.1 depends on: 1.3
- Task 3.2 depends on: 2.1
- Task 3.3 depends on: 3.1, 3.2
- Task 4.1 depends on: 3.3
- Task 4.2 depends on: 3.1
- Task 4.3 depends on: None (can be parallel)
- Task 5.x depends on: All 4.x tasks

## Parallelizable Work

The following can be done in parallel:
- Phase 1 (1.1, 1.2, 1.3) with Phase 2 (2.1)
- Phase 3 tasks within themselves
- Phase 4.3 with other Phase 4 tasks
- All Phase 5 tasks after Phase 4 is complete
