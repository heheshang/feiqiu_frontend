# Design: Contacts Feature

**Change ID:** `add-contacts-feature`

## Architecture Overview

The Contacts feature introduces a new layer on top of the existing `peers` system. While `peers` represents transient LAN-discovered nodes, `contacts` provides persistent, user-managed contact records that can include:
- Auto-discovered peers (persisted for offline access)
- Manually added contacts
- User-editable metadata (notes, nicknames, groups)

### System Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        Frontend (React)                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │ ContactsList │  │ ContactGroup │  │ ContactEdit  │          │
│  │   Component  │  │   Manager    │  │   Dialog     │          │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘          │
│         │                  │                  │                  │
│         └──────────────────┼──────────────────┘                  │
│                            │                                     │
│                    ┌───────▼────────┐                           │
│                    │  useContacts   │                           │
│                    │     Hook       │                           │
│                    └───────┬────────┘                           │
└────────────────────────────┼─────────────────────────────────────┘
                             │ Tauri IPC
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Backend (Rust/Tauri)                       │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                   Commands Layer                          │  │
│  │  • get_contacts()     • create_contact()                  │  │
│  │  • update_contact()   • delete_contact()                  │  │
│  │  • get_groups()       • create_group()                    │  │
│  │  • update_group()     • delete_group()                    │  │
│  └──────────────────────────┬───────────────────────────────┘  │
│                             │                                   │
│  ┌──────────────────────────▼───────────────────────────────┐  │
│  │                   Contact Service                         │  │
│  │  • Sync with peers system                                 │  │
│  │  • Merge duplicates                                       │  │
│  │  • Business logic validation                              │  │
│  └──────────────────────────┬───────────────────────────────┘  │
│                             │                                   │
│  ┌──────────────────────────▼───────────────────────────────┐  │
│  │              Contact Repository                          │  │
│  │  • CRUD operations on contacts table                     │  │
│  │  • CRUD operations on contact_groups table               │  │
│  │  • Join operations for group members                     │  │
│  └──────────────────────────┬───────────────────────────────┘  │
│                             │                                   │
└─────────────────────────────┼───────────────────────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │    SQLite DB      │
                    │  • contacts       │
                    │  • contact_groups │
                    │  • peers (exist)  │
                    └───────────────────┘
```

## Data Model

### Core Entities

#### Contact (New Table)
The `contacts` table extends peer information with user-editable fields:

```rust
pub struct ContactModel {
    pub id: i32,
    pub peer_id: Option<i32>,        // Link to peers.peers.id (if from peer)
    pub name: String,                 // Display name (user-editable)
    pub nickname: Option<String>,     // User-set nickname
    pub avatar: Option<String>,       // Avatar URL
    pub phone: Option<String>,        // Phone number
    pub email: Option<String>,        // Email address
    pub department: Option<String>,   // Department name
    pub position: Option<String>,     // Job position
    pub notes: Option<String>,        // User notes
    pub is_favorite: bool,            // Starred/favorite status
    pub pinyin: Option<String>,       // Pinyin for search
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}
```

#### ContactGroup (New Table)
Custom groups for organizing contacts:

```rust
pub struct ContactGroupModel {
    pub id: i32,
    pub name: String,                 // Group name
    pub color: Option<String>,        // Optional color for UI
    pub icon: Option<String>,         // Optional icon
    pub sort_order: i32,              // Display order
    pub created_at: DateTime,
}
```

#### ContactGroupMember (New Table)
Many-to-many relationship between contacts and groups:

```rust
pub struct ContactGroupMemberModel {
    pub id: i32,
    pub contact_id: i32,
    pub group_id: i32,
    pub joined_at: DateTime,
}
```

### Relationship with Existing Peers System

```
┌─────────────┐         ┌─────────────┐
│    peers    │         │  contacts   │
├─────────────┤    ┌───▶├─────────────┤
│ id          │──┐    │  │ id          │
│ ip          │  │    │  │ peer_id     │◄──┐
│ port        │  │    │  │ name        │   │
│ username    │  │    │  │ nickname    │   │
│ hostname    │  │    │  │ ...         │   │
│ last_seen   │  │    │  └─────────────┘   │
└─────────────┘  │    │                      │
                  │    │  ┌──────────────────┤
                  │    └──│ contact_groups   │
                  │       │──────────────────│
                  │       │ id               │
                  │       │ name             │
                  │       └──────────────────┘
                  │                   │
                  │                   │
                  │    ┌──────────────▼───────┐
                  └────│ contact_group_members │
                       │──────────────────────│
                       │ contact_id            │
                       │ group_id              │
                       └───────────────────────┘
```

## Frontend Architecture

### Component Structure

```
src/components/contacts/
├── ContactsList.tsx         # Main list view with search/filter
├── ContactCard.tsx          # Individual contact display card
├── ContactGroupsPanel.tsx   # Sidebar with group navigation
├── ContactDetailsModal.tsx  # Contact detail/edit dialog
├── ContactGroupDialog.tsx   # Create/edit group dialog
├── ContactBatchActions.tsx  # Batch operations toolbar
└── index.ts
```

### Type Definitions

```typescript
// src/lib/types/contacts.ts
export type ContactView = 'all' | 'online' | 'offline' | 'favorites' | 'group'

export interface Contact {
  id: string
  peerId?: string
  name: string
  nickname?: string
  avatar?: string
  phone?: string
  email?: string
  department?: string
  position?: string
  notes?: string
  isFavorite: boolean
  isOnline: boolean            // Derived from peers system
  lastSeen?: string
  groups: string[]             // Group IDs
  pinyin?: string
  createdAt: string
  updatedAt?: string
}

export interface ContactGroup {
  id: string
  name: string
  color?: string
  icon?: string
  sortOrder: number
  memberCount: number
  createdAt: string
}

export interface ContactFilters {
  search?: string              // Full-text search
  status?: 'online' | 'offline' | 'all'
  groupId?: string             // Filter by group
  department?: string
  isFavorite?: boolean
}

export interface ContactStats {
  total: number
  online: number
  offline: number
  favorites: number
  byDepartment: Record<string, number>
}
```

### State Management

Using a custom hook pattern (consistent with existing codebase):

```typescript
// src/hooks/useContacts.ts
export function useContacts() {
  const [contacts, setContacts] = useState<Contact[]>([])
  const [groups, setGroups] = useState<ContactGroup[]>([])
  const [filters, setFilters] = useState<ContactFilters>({})

  // IPC calls via Tauri
  const { invoke } = window.__TAURI__

  const getContacts = async (filters?: ContactFilters) => {
    return await invoke('get_contacts', { filters })
  }

  const createContact = async (contact: Partial<Contact>) => {
    return await invoke('create_contact', { contact })
  }

  // ... more methods

  return {
    contacts,
    groups,
    filters,
    getContacts,
    createContact,
    updateContact,
    deleteContact,
    // ...
  }
}
```

## Backend Architecture

### New Commands

```rust
// src-tauri/src/commands/contacts.rs

#[tauri::command]
async fn get_contacts(
    state: State<'_, AppState>,
    filters: Option<ContactFilters>,
) -> Result<Vec<ContactDto>, NeoLanError>

#[tauri::command]
async fn get_contact(
    state: State<'_, AppState>,
    id: i32,
) -> Result<Option<ContactDto>, NeoLanError>

#[tauri::command]
async fn create_contact(
    state: State<'_, AppState>,
    contact: CreateContactDto,
) -> Result<ContactDto, NeoLanError>

#[tauri::command]
async fn update_contact(
    state: State<'_, AppState>,
    id: i32,
    contact: UpdateContactDto,
) -> Result<ContactDto, NeoLanError>

#[tauri::command]
async fn delete_contact(
    state: State<'_, AppState>,
    id: i32,
) -> Result<(), NeoLanError>

#[tauri::command]
async fn get_contact_groups(
    state: State<'_, AppState>,
) -> Result<Vec<ContactGroupDto>, NeoLanError>

#[tauri::command]
async fn create_contact_group(
    state: State<'_, AppState>,
    group: CreateContactGroupDto,
) -> Result<ContactGroupDto, NeoLanError>

#[tauri::command]
async fn update_contact_group(
    state: State<'_, AppState>,
    id: i32,
    group: UpdateContactGroupDto,
) -> Result<ContactGroupDto, NeoLanError>

#[tauri::command]
async fn delete_contact_group(
    state: State<'_, AppState>,
    id: i32,
) -> Result<(), NeoLanError>

#[tauri::command]
async fn add_contacts_to_group(
    state: State<'_, AppState>,
    group_id: i32,
    contact_ids: Vec<i32>,
) -> Result<(), NeoLanError>

#[tauri::command]
async fn remove_contacts_from_group(
    state: State<'_, AppState>,
    group_id: i32,
    contact_ids: Vec<i32>,
) -> Result<(), NeoLanError>
```

### Repository Layer

```rust
// src-tauri/src/storage/contact_repo.rs

pub struct ContactRepository {
    db: DatabaseConnection,
}

impl ContactRepository {
    // Basic CRUD
    pub async fn find_all(&self, filters: Option<ContactFilters>) -> Result<Vec<ContactModel>>;
    pub async fn find_by_id(&self, id: i32) -> Result<Option<ContactModel>>;
    pub async fn create(&self, contact: CreateContact) -> Result<ContactModel>;
    pub async fn update(&self, id: i32, contact: UpdateContact) -> Result<ContactModel>;
    pub async fn delete(&self, id: i32) -> Result<()>;

    // Groups
    pub async fn find_all_groups(&self) -> Result<Vec<ContactGroupModel>>;
    pub async fn find_contacts_by_group(&self, group_id: i32) -> Result<Vec<ContactModel>>;
    pub async fn create_group(&self, group: CreateGroup) -> Result<ContactGroupModel>;
    pub async fn delete_group(&self, id: i32) -> Result<()>;
    pub async fn add_to_group(&self, contact_id: i32, group_id: i32) -> Result<()>;
    pub async fn remove_from_group(&self, contact_id: i32, group_id: i32) -> Result<()>;

    // Search
    pub async fn search(&self, query: &str) -> Result<Vec<ContactModel>>;

    // Sync with peers
    pub async fn sync_from_peers(&self, peers: Vec<PeerNode>) -> Result<()>;
}
```

## Peer-to-Contact Synchronization Strategy

### Automatic Sync

When a peer is discovered via UDP broadcast:
1. Check if contact exists with matching `peer_id`
2. If exists: update `last_seen`, `is_online` status
3. If not exists: create new contact record (auto-sync)

When a peer goes offline (heartbeat timeout):
1. Update contact `is_online = false`
2. Keep contact record (historical)

### Manual Override

Users can:
- Edit any field on auto-synced contacts (takes precedence)
- Add contacts manually (without peer association)
- Delete contacts (doesn't affect peer discovery)

## Search Implementation

### Search Strategy

Full-text search across multiple fields:

```rust
// SQL query pattern
SELECT * FROM contacts
WHERE
    -- Exact name match
    name LIKE :query
    OR nickname LIKE :query
    -- Pinyin search
    OR pinyin LIKE :query
    -- Department/position
    OR department LIKE :query
    OR position LIKE :query
    -- Email/phone
    OR email LIKE :query
    OR phone LIKE :query
    -- Notes
    OR notes LIKE :query
```

### Pinyin Generation

Use `pinyin` crate for Chinese name search:
- "张三" → "zhangsan"
- Stored in `pinyin` column for efficient indexing

## UI Design Decisions

### View Modes

1. **Mixed View**: Toggle between "Groups" and "Department" mode
   - Groups mode: Custom user-created groups
   - Department mode: Auto-organized by department field

2. **Status Filter**: All / Online / Offline / Favorites

3. **List/Grid Toggle**: Compact list vs card grid view

### Navigation Pattern

```
┌─────────────────────────────────────────────────────┐
│  🔍 搜索联系人...                    [+ 添加联系人]  │
├──────────┬──────────────────────────────────────────┤
│          │  ◆ 全部 (142)  ● 在线 (23)  ○ 离线      │
│  分组    │  ★ 收藏 (8)                              │
│  ──────  │                                          │
│ ▼ 全部   │  ┌─────────────────────────────────┐    │
│   同事   │  │ 👤 张三              在线   ★   │    │
│   VIP    │  │    产品经理 | 技术部           │    │
│   朋友   │  │    138****1234                │    │
│   家人   │  └─────────────────────────────────┘    │
│          │  ┌─────────────────────────────────┐    │
│ [+ 新建] │  │ 👤 李四              离线       │    │
│          │  │    设计师 | 设计部             │    │
│          │  └─────────────────────────────────┘    │
└──────────┴──────────────────────────────────────────┘
```

## Performance Considerations

1. **Caching**: Frontend caches contact list, invalidates on peer events
2. **Pagination**: Load contacts in batches (50 per page) for large lists
3. **Indexing**: Database indexes on `name`, `pinyin`, `department`, `peer_id`
4. **Debouncing**: Search queries debounced (300ms)

## Security Considerations

1. All contacts are local-only (no cloud sync)
2. No sensitive data transmitted over LAN
3. User notes are private (not shared with other peers)

## Migration Path

1. **Phase 1**: Database tables + backend CRUD
2. **Phase 2**: Frontend list + search
3. **Phase 3**: Groups functionality
4. **Phase 4**: Batch operations + advanced features
