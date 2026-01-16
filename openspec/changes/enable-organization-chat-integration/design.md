# Design: Organization Chart Chat Integration

## Overview
This document describes the architectural design for enabling the organization chart to initiate chat conversations.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Frontend (React)                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────┐     ┌──────────────────────────────────────────┐      │
│  │  Organization   │────▶│         App.tsx                          │      │
│  │     Chart       │     │  ┌────────────────────────────────────┐  │      │
│  │                 │     │  │ handleStartChatFromOrganization()  │  │      │
│  │  - UserCard     │     │  │  1. Map userId → peer IP          │  │      │
│  │    (Chat btn)   │     │  │  2. Add to manuallyAddedConvs      │  │      │
│  │  - DeptTree     │     │  │  3. Switch to 'chat' tab           │  │      │
│  └─────────────────┘     │  │  4. Set activeConversationId       │  │      │
│                          │  └────────────────────────────────────┘  │      │
│                          └──────────────────────────────────────────┘      │
│                                          │                                  │
│                                          ▼                                  │
│                          ┌──────────────────────────────────────────┐      │
│                          │           Messaging Component             │      │
│                          │           (Conversation List)             │      │
│                          └──────────────────────────────────────────┘      │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                              IPC Layer (Tauri)                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  get_organization_data (NEW COMMAND)                                │   │
│  │  ┌─────────────────────────────────────────────────────────────┐    │   │
│  │  │ Input: None                                                  │    │   │
│  │  │ Output: OrganizationDataDto {                                │    │   │
│  │  │   departments: DepartmentDto[],  // Hierarchical tree        │    │   │
│  │  │   users: UserDto[]                // All users with deptId   │    │   │
│  │  │ }                                                             │    │   │
│  │  └─────────────────────────────────────────────────────────────┘    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                              Backend (Rust)                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────┐     ┌──────────────────────────────────────────┐      │
│  │   Peer Data     │────▶│   Organization Builder                   │      │
│  │  (PeerNode)     │     │   ┌────────────────────────────────────┐ │      │
│  │  - ip           │     │   │ 1. Extract unique departments       │ │      │
│  │  - username     │     │   │ 2. Build hierarchical tree          │ │      │
│  │  - groups       │     │   │ 3. Map PeerNode → UserDto           │      │
│  │  - (no dept)    │     │   │ 4. Link users to departments        │ │      │
│  └─────────────────┘     │   └────────────────────────────────────┘ │      │
│                          └──────────────────────────────────────────┘      │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Data Flow

### 1. Initial Load (Organization View)
```
User navigates to "organization" tab
    ↓
App.tsx calls get_organization_data()
    ↓
Backend transforms peer data → OrganizationDataDto
    ↓
Frontend converts Dto → Department[] + User[]
    ↓
OrganizationChart renders with data
```

### 2. Chat Initiation Flow
```
User clicks "聊天" button on UserCard
    ↓
UserCard.onStartChat(userId) is called
    ↓
App.tsx handleStartChatFromOrganization(userId)
    ↓
Resolve IP address from userId → IP mapping
    ↓
Add to manuallyAddedConversations
    ↓
setActiveTab('chat')
    ↓
setActiveConversationId(peerIp)
    ↓
Messaging component shows conversation with focus
```

## Data Structures

### Backend DTO (Rust)
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationDataDto {
    pub departments: Vec<DepartmentDto>,
    pub users: Vec<UserDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentDto {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub level: u32,
    pub member_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: String,           // IP address (for messaging compatibility)
    pub name: String,
    pub pinyin: String,       // For search
    pub avatar: Option<String>,
    pub position: String,     // Empty string if not set
    pub department: String,
    pub department_id: String,
    pub status: String,       // "online" | "away" | "offline"
    pub email: String,        // Empty string if not set
    pub phone: String,        // Empty string if not set
}
```

### Frontend Types (TypeScript)
```typescript
// Existing types in lib/types/organization.ts
export interface User {
  id: string          // Will be the peer IP address
  name: string
  pinyin: string
  avatar: string
  position: string
  department: string
  departmentId: string
  status: UserStatus
  email: string
  phone: string
}

export interface Department {
  id: string
  name: string
  parentId: string | null
  level: number
  memberCount: number
}
```

## Key Design Decisions

### Decision 1: Use IP Address as User ID
**Rationale**: The messaging system identifies conversations by IP address. Using IP as the organization User.id eliminates the need for a separate lookup table.

**Trade-offs**:
- ✅ Simple: No mapping layer needed
- ✅ Consistent: Same identifier across organization and messaging
- ❌ Less flexible: If we want to support non-IP-based messaging later, we'd need to refactor

### Decision 2: Department Data from Peer.groups
**Rationale**: PeerNode already has a `groups: Vec<String>` field. We can use the first group as the department name.

**Trade-offs**:
- ✅ Uses existing data, no new storage
- ❌ Limited: Only one department per peer, groups are user-defined
- Future: Could add `department` field to PeerNode and ContactModel

### Decision 3: Default Department for Users Without Groups
**Rationale**: Some peers may not have any group information. We need a fallback.

**Implementation**: Create a "未分组" (uncategorized) department with id="default" at the root level.

### Decision 4: Avatar Generation
**Rationale**: PeerDto.avatar is optional. The UI expects an avatar string.

**Implementation**: Use the existing `generateAvatar(name)` utility to create initials-based avatars when none is provided.

## Component Changes

### New Files
1. `feiqiu/src/lib/converters/organization.ts` - Transform DTO to domain types
2. `feiqiu/src-tauri/src/commands/organization.rs` - New IPC command handler

### Modified Files
1. `feiqiu/src/App.tsx` - Wire up OrganizationChart with data and handler
2. `feiqiu/src-tauri/src/lib.rs` - Register new command

## Error Handling
- **No peers found**: Return empty departments array + "未分组" department with 0 members
- **Invalid IP in userId**: Log warning and don't initiate chat
- **Conversation creation fails**: Show toast notification to user

## Testing Strategy
1. **Unit Tests**: Test organization builder logic with various peer configurations
2. **Integration Tests**: Test IPC command returns correct structure
3. **E2E Tests**: Test click flow from organization chart to messaging view

## Future Enhancements
1. Allow manual department assignment for peers
2. Support multi-level department hierarchy (currently flat)
3. Add organization settings to configure department sources
4. Cache organization data to reduce recomputation
