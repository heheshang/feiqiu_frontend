# Tasks: Add Contacts Feature

**Change ID:** `add-contacts-feature`
**Total Tasks:** 25

---

## Phase 1: Database Schema & Backend Infrastructure (5 tasks)

### Task 1.1: Create database migration for contacts tables
**Priority:** P0 (Blocker)
**Effort:** Medium
**Description:** Create Sea-ORM migration for three new tables: `contacts`, `contact_groups`, and `contact_group_members`.

**Acceptance Criteria:**
- [ ] Migration file created in `src-tauri/src/migration/`
- [ ] `contacts` table with columns: id, peer_id (nullable), name, nickname, avatar, phone, email, department, position, notes, is_favorite, pinyin, created_at, updated_at
- [ ] `contact_groups` table with columns: id, name, color, icon, sort_order, created_at
- [ ] `contact_group_members` table with columns: id, contact_id, group_id, joined_at
- [ ] Foreign key constraints defined
- [ ] Indexes on: contacts(name), contacts(pinyin), contacts(department), contacts(peer_id), contact_group_members(contact_id, group_id)
- [ ] Migration runs successfully with `cargo test`

**Dependencies:** None
**Validation:** Run migration and verify schema with SQLite browser

---

### Task 1.2: Create Sea-ORM entity models
**Priority:** P0 (Blocker)
**Effort:** Low
**Description:** Generate Sea-ORM entity models for the three new tables.

**Acceptance Criteria:**
- [ ] `contacts.rs` entity created in `src-tauri/src/storage/entities/`
- [ ] `contact_groups.rs` entity created
- [ ] `contact_group_members.rs` entity created
- [ ] All entities derive Clone, Debug, Serialize, Deserialize
- [ ] Entities exported from `entities/mod.rs`

**Dependencies:** Task 1.1
**Validation:** Entity structs compile without errors

---

### Task 1.3: Implement ContactRepository
**Priority:** P0 (Blocker)
**Effort:** High
**Description:** Create `contact_repo.rs` with full CRUD operations for contacts and groups.

**Acceptance Criteria:**
- [ ] `ContactRepository` struct with db connection
- [ ] Methods: `find_all`, `find_by_id`, `create`, `update`, `delete`
- [ ] Group methods: `find_all_groups`, `create_group`, `update_group`, `delete_group`
- [ ] Membership methods: `add_to_group`, `remove_from_group`, `find_contacts_by_group`
- [ ] Search method: `search` with LIKE queries across name, nickname, pinyin, department, phone, email, notes
- [ ] Sync method: `sync_from_peers` to auto-create contacts from peer discovery
- [ ] All methods return proper `Result<T, NeoLanError>` types

**Dependencies:** Task 1.2
**Validation:** Unit tests for each repository method

---

### Task 1.4: Create Tauri IPC commands for contacts
**Priority:** P0 (Blocker)
**Effort:** Medium
**Description:** Create `commands/contacts.rs` with all IPC command handlers.

**Acceptance Criteria:**
- [ ] Command handlers for: `get_contacts`, `get_contact`, `create_contact`, `update_contact`, `delete_contact`
- [ ] Group commands: `get_contact_groups`, `create_contact_group`, `update_contact_group`, `delete_contact_group`
- [ ] Membership commands: `add_contacts_to_group`, `remove_contacts_from_group`
- [ ] Search command: `search_contacts`
- [ ] Statistics command: `get_contact_stats`
- [ ] All commands registered in `lib.rs` invoke_handler
- [ ] Proper error handling and logging

**Dependencies:** Task 1.3
**Validation:** Commands callable from frontend with correct request/response

---

### Task 1.5: Implement peer-to-contact sync service
**Priority:** P1 (High)
**Effort:** Medium
**Description:** Create a service that automatically syncs discovered peers to contacts table.

**Acceptance Criteria:**
- [ ] `ContactSyncService` in `modules/contacts/sync.rs`
- [ ] On peer discovered: check if contact exists, create or update
- [ ] On peer offline: update contact `is_online = false`
- [ ] On peer online: update contact `is_online = true`, `last_seen`
- [ ] User-edited fields take precedence over peer data
- [ ] Integration with existing `PeerManager` events

**Dependencies:** Task 1.4
**Validation:** Auto-creation of contacts when peers appear on LAN

---

## Phase 2: Frontend Types and API Layer (3 tasks)

### Task 2.1: Create TypeScript types for contacts
**Priority:** P0 (Blocker)
**Effort:** Low
**Description:** Define TypeScript interfaces for contacts in `src/lib/types/contacts.ts`.

**Acceptance Criteria:**
- [ ] `Contact` interface with all fields matching backend
- [ ] `ContactGroup` interface
- [ ] `ContactFilters` interface for search/filter
- [ ] `ContactView` type for view modes
- [ ] `ContactStats` interface for statistics
- [ ] All types exported

**Dependencies:** Task 1.2
**Validation:** Types match backend entity structure

---

### Task 2.2: Create useContacts hook
**Priority:** P0 (Blocker)
**Effort:** Medium
**Description:** Implement custom React hook for contacts state management and API calls.

**Acceptance Criteria:**
- [ ] `useContacts` hook in `src/hooks/useContacts.ts`
- [ ] State: `contacts`, `groups`, `filters`, `stats`
- [ ] Methods: `getContacts`, `createContact`, `updateContact`, `deleteContact`
- [ ] Group methods: `getGroups`, `createGroup`, `updateGroup`, `deleteGroup`
- [ ] Search method: `searchContacts`
- [ ] Auto-refresh on peer events
- [ ] Loading and error states

**Dependencies:** Task 2.1
**Validation:** Hook returns correct data types and handles errors

---

### Task 2.3: Create API client functions
**Priority:** P1 (High)
**Effort:** Low
**Description:** Create typed API wrapper functions in `src/lib/api/contacts.ts`.

**Acceptance Criteria:**
- [ ] Wrapper functions for all contact IPC commands
- [ ] Type-safe request/response handling
- [ ] Error handling with descriptive messages
- [ ] JSDoc comments for each function

**Dependencies:** Task 2.1
**Validation:** Functions compile and have correct types

---

## Phase 3: Contacts List Component (4 tasks)

### Task 3.1: Create ContactsList main component
**Priority:** P0 (Blocker)
**Effort:** High
**Description:** Create the main contacts list view with search, filter, and status indicators.

**Acceptance Criteria:**
- [ ] `ContactsList.tsx` component in `src/components/contacts/`
- [ ] Search input with real-time filtering (300ms debounce)
- [ ] Status filter tabs: All, Online, Offline, Favorites
- [ ] Contact cards with avatar, name, status indicator, department
- [ ] Online status indicator (green/yellow/gray dots)
- [ ] Checkbox for multi-select
- [ ] Star icon for favorites
- [ ] Empty state when no contacts
- [ ] Loading state during data fetch
- [ ] Responsive layout (handles 100+ contacts)

**Dependencies:** Task 2.2
**Validation:** Component renders with mock data

---

### Task 3.2: Create ContactCard component
**Priority:** P0 (Blocker)
**Effort:** Medium
**Description:** Create individual contact card component with all display elements.

**Acceptance Criteria:**
- [ ] `ContactCard.tsx` component
- [ ] Avatar with fallback to initials
- [ ] Name/nickname display (nickname takes precedence)
- [ ] Status badge (Online/Offline)
- [ ] Department and position subtext
- [ ] Phone and email icons (when available)
- [ ] Favorite star toggle
- [ ] Checkbox for selection
- [ ] Click to view details
- [ ] Hover state with action buttons

**Dependencies:** Task 3.1
**Validation:** Card displays all contact fields correctly

---

### Task 3.3: Create ContactDetailsModal component
**Priority:** P1 (High)
**Effort:** High
**Description:** Create modal for viewing and editing full contact details.

**Acceptance Criteria:**
- [ ] `ContactDetailsModal.tsx` component
- [ ] Large avatar display
- [ ] All contact fields shown (name, nickname, phone, email, dept, position, notes)
- [ ] Edit mode toggle
- [ ] Form inputs for editable fields
- [ ] Save/Cancel buttons
- [ ] Delete button with confirmation
- [ ] Groups membership display
- [ ] Last seen timestamp
- [ ] Close button (X) and backdrop click to close

**Dependencies:** Task 3.2
**Validation:** Modal opens, edits, and saves correctly

---

### Task 3.4: Implement contact statistics display
**Priority:** P2 (Medium)
**Effort:** Low
**Description:** Add statistics summary showing total, online, favorites, and department breakdown.

**Acceptance Criteria:**
- [ ] `ContactStats.tsx` component
- [ ] Total contacts count
- [ ] Online contacts count
- [ ] Favorites count
- [ ] Department breakdown (collapsible)
- [ ] Updates in real-time as contacts change

**Dependencies:** Task 3.1
**Validation:** Stats match actual contact counts

---

## Phase 4: Contact Groups Feature (4 tasks)

### Task 4.1: Create ContactGroupsPanel sidebar
**Priority:** P0 (Blocker)
**Effort:** High
**Description:** Create sidebar with group navigation and management.

**Acceptance Criteria:**
- [ ] `ContactGroupsPanel.tsx` sidebar component
- [ ] "全部" (All) option
- [ ] List of custom groups with member counts
- [ ] Active group highlighting
- [ ] "+ 新建" (New Group) button
- [ ] Right-click context menu on groups (Edit, Delete)
- [ ] Collapsible panel
- [ ] Scrollable for many groups

**Dependencies:** Task 3.1
**Validation:** Sidebar displays groups and filters contacts

---

### Task 4.2: Create ContactGroupDialog component
**Priority:** P1 (High)
**Effort:** Medium
**Description:** Create dialog for creating and editing groups.

**Acceptance Criteria:**
- [ ] `ContactGroupDialog.tsx` component
- [ ] Group name input (required)
- [ ] Color picker for group color
- [ ] Icon selector (optional)
- [ ] Sort order input
- [ ] Create/Update button
- [ ] Cancel button
- [ ] Validation (name required)

**Dependencies:** Task 4.1
**Validation:** Dialog creates and updates groups

---

### Task 4.3: Implement add to group functionality
**Priority:** P1 (High)
**Effort:** Medium
**Description:** Allow adding contacts to groups from selection or details modal.

**Acceptance Criteria:**
- [ ] "添加到分组" (Add to Group) button in batch toolbar
- [ ] Group selector dropdown
- [ ] Multi-group selection support
- [ ] Add button with confirmation
- [ ] Success message
- [ ] Group member count updates
- [ ] Contact displays group badges

**Dependencies:** Task 4.1, Task 4.2
**Validation:** Contacts added to groups correctly

---

### Task 4.4: Implement department view mode
**Priority:** P2 (Medium)
**Effort:** High
**Description:** Add toggle between Groups and Department view modes.

**Acceptance Criteria:**
- [ ] View mode toggle: "分组" / "部门"
- [ ] Department tree in sidebar when in Department mode
- [ ] Department nodes expandable/collapsible
- [ ] Department member counts
- [ ] Filter contacts by selecting department
- [ ] Include sub-departments in filter
- [ ] Breadcrumb showing department path

**Dependencies:** Task 4.1
**Validation:** Department view shows contacts organized correctly

---

## Phase 5: Contact CRUD Operations (3 tasks)

### Task 5.1: Implement create contact functionality
**Priority:** P1 (High)
**Effort:** Medium
**Description:** Add ability to manually create new contacts.

**Acceptance Criteria:**
- [ ] "+ 添加联系人" (Add Contact) button
- [ ] Contact form modal with fields: name (required), phone, email, department, position, notes
- [ ] Form validation
- [ ] Duplicate name warning
- [ ] Save button creates contact
- [ ] Success message
- [ ] Contact appears in list

**Dependencies:** Task 3.3
**Validation:** New contacts persist and display correctly

---

### Task 5.2: Implement edit contact functionality
**Priority:** P1 (High)
**Effort:** Low
**Description:** Enable editing existing contact details.

**Acceptance Criteria:**
- [ ] Edit button in ContactDetailsModal
- [ ] Form pre-populated with existing data
- [ ] Save button updates contact
- [ ] User edits take precedence over peer sync
- [ ] Success message

**Dependencies:** Task 5.1
**Validation:** Edited contacts persist correctly

---

### Task 5.3: Implement delete contact functionality
**Priority:** P1 (High)
**Effort:** Low
**Description:** Add delete functionality with confirmation and warnings.

**Acceptance Criteria:**
- [ ] Delete button in ContactDetailsModal
- [ ] Confirmation dialog
- [ ] Special warning for synced contacts
- [ ] Delete removes contact from database
- [ ] Success message
- [ ] Contact disappears from list

**Dependencies:** Task 5.2
**Validation:** Deleted contacts are removed and don't reappear

---

## Phase 6: Search and Batch Operations (4 tasks)

### Task 6.1: Implement full-text search
**Priority:** P0 (Blocker)
**Effort:** Medium
**Description:** Add real-time search across all contact fields with pinyin support.

**Acceptance Criteria:**
- [ ] Search input in ContactsList header
- [ ] 300ms debounce on input
- [ ] Search across: name, nickname, pinyin, department, position, phone, email, notes
- [ ] Highlight matching text in results
- [ ] Result count display
- [ ] Clear search button (X)
- [ ] Recent searches dropdown

**Dependencies:** Task 3.1
**Validation:** Search returns matching contacts

---

### Task 6.2: Implement multi-select functionality
**Priority:** P1 (High)
**Effort:** Medium
**Description:** Add checkbox selection with Ctrl+Click and Shift+Click support.

**Acceptance Criteria:**
- [ ] Checkboxes on all contact cards
- [ ] Ctrl+Click for multi-select
- [ ] Shift+Click for range selection
- [ ] Select All checkbox in header
- [ ] Selected contacts highlighted
- [ ] Selection count displayed
- [ ] Batch action toolbar appears on selection

**Dependencies:** Task 3.2
**Validation:** Multiple contacts can be selected

---

### Task 6.3: Implement batch delete and move
**Priority:** P1 (High)
**Effort:** Medium
**Description:** Add batch delete and batch move to group operations.

**Acceptance Criteria:**
- [ ] Batch delete button in toolbar
- [ ] Confirmation dialog with contact count
- [ ] Warning for synced contacts
- [ ] Batch move to group button
- [ ] Group selector for batch move
- [ ] Progress indicator for large batches
- [ ] Success messages

**Dependencies:** Task 6.2
**Validation:** Batch operations complete correctly

---

### Task 6.4: Implement batch export
**Priority:** P2 (Medium)
**Effort:** Medium
**Description:** Add export functionality for selected contacts.

**Acceptance Criteria:**
- [ ] Export button in batch toolbar
- [ ] Format selection: CSV or JSON
- [ ] Export includes: name, nickname, phone, email, department, position, notes
- [ ] File download with timestamp
- [ ] Export all contacts option (no selection)

**Dependencies:** Task 6.3
**Validation:** Exported file contains correct data

---

## Phase 7: Integration and Polish (2 tasks)

### Task 7.1: Update App.tsx to use Contacts component
**Priority:** P0 (Blocker)
**Effort:** Low
**Description:** Integrate Contacts component into main app navigation.

**Acceptance Criteria:**
- [ ] Import and render Contacts component when 'contacts' tab is active
- [ ] Remove placeholder "通讯录功能开发中" message
- [ ] Pass necessary props (user, config, etc.)
- [ ] Contacts tab shows correct content

**Dependencies:** Task 3.1
**Validation:** Contacts tab displays functional component

---

### Task 7.2: Add pinyin generation for Chinese names
**Priority:** P2 (Medium)
**Effort:** Medium
**Description:** Implement automatic pinyin generation for Chinese names to enable pinyin search.

**Acceptance Criteria:**
- [ ] Backend uses `pinyin` crate for Chinese conversion
- [ ] Pinyin auto-generated on contact create/update
- [ ] Stored in `pinyin` column
- [ ] Full pinyin (zhangsan) and acronym (zs) both stored
- [ ] Search works with both forms

**Dependencies:** Task 1.3
**Validation:** Chinese names searchable via pinyin

---

## Summary

**Total Tasks:** 25
**Estimated Effort:** ~8-10 days for one developer

**Critical Path:** Task 1.1 → 1.2 → 1.3 → 1.4 → 2.1 → 2.2 → 3.1 → 7.1

**Parallelizable Work:**
- Phase 2 (Frontend Types) can start after Task 1.2
- Phase 3 components can be developed in parallel after Task 2.2
- Phase 6 (Batch Operations) can be done in parallel with Phase 4-5

**Definition of Done:**
- All acceptance criteria met
- Code reviewed and merged
- Tests passing
- Documentation updated
- No regressions in existing functionality
