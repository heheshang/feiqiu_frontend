# One-Shot Implementation Prompt for 飞秋 (FeiQiu)

> **Before starting implementation**, ask your team the following clarifying questions:

## Clarifying Questions

1. **Authentication**: How will users authenticate?
   - Local account system?
   - LDAP/Active Directory integration?
   - SSO (Single Sign-On)?

2. **User Modeling**: How will user sessions be managed?
   - JWT tokens?
   - Session-based authentication?
   - How to handle multi-device login?

3. **Tech Stack**: Please confirm:
   - Framework: Next.js, Vite + React, Create React App, or other?
   - State management: Redux Toolkit, Zustand, Jotai, Context API?
   - Backend: Node.js + Express, Next.js API routes, or separate backend service?
   - Database: PostgreSQL, MySQL, MongoDB, SQLite, or other?
   - Real-time communication: WebSocket, Socket.io, or other?

4. **File Storage**: Where will files be stored?
   - Local filesystem?
   - Cloud storage (S3, Azure Blob)?
   - Temporary storage with cleanup?

5. **Network Configuration**: Any specific network requirements?
   - UDP port range for LAN discovery?
   - Proxy settings for corporate networks?
   - Firewall configuration requirements?

---

## Implementation Request

Implement the complete 飞秋 (FeiQiu) LAN instant messaging application based on the product design package.

### Product Overview

飞秋 is a LAN instant messaging tool for enterprise internal communication. Key features:
- Single/group chat with message enhancement (recall, reactions, read status)
- LAN file/folder transfer with progress visualization
- Screen capture and sharing for remote assistance
- Organization chart with department tree and colleague list
- Personal settings and network configuration

### Design System

- **Primary Color**: Emerald (emerald-500, emerald-600)
- **Secondary Color**: Blue (blue-500, blue-600)
- **Neutral Color**: Slate (slate-50 to slate-900)
- **Typography**: Inter for headings and body, IBM Plex Mono for code
- **Styling**: Tailwind CSS v4
- **Dark Mode**: Full support required

### Architecture

**Three-Column Layout** (WeChat-inspired):
- Left sidebar (60px): Main navigation icons
- Middle column (280px): Lists (conversations/contacts/departments)
- Right column (flex-1): Content area (chat/details/settings)

### Implementation Milestones

#### Milestone 1: Foundation
1. Project setup with your chosen tech stack
2. Configure Tailwind CSS v4 with design tokens
3. Set up dark mode support
4. Configure TypeScript and ESLint
5. Create base folder structure

#### Milestone 2: Application Shell
1. Implement AppShell with three-column layout
2. Create MainNav component (聊天, 通讯录, 组织架构)
3. Create UserMenu component (avatar, dropdown)
4. Implement responsive behavior

#### Milestone 3: Basic Settings Section
1. Personal info form (avatar, name, signature, status)
2. Network configuration form (UDP port, listen address)
3. Network status display card
4. Tab-based settings interface

#### Milestone 4: Messaging Section
1. Conversation list with unread counts
2. Message list with bubbles (text, image, file)
3. Message input with emoji, file upload, @mentions
4. Message actions (recall, reply, react)
5. Search functionality

#### Milestone 5: File Transfer Section
1. File transfer list with filters
2. Transfer items with progress bars
3. Pause/resume/cancel actions
4. Drag-and-drop file upload

#### Milestone 6: Collaboration Tools Section
1. Screenshot toolbar with annotation tools
2. Screenshot history list
3. Screen share controls

#### Milestone 7: Organization Chart Section
1. Department tree with expand/collapse
2. User card with status indicator
3. Search by name, department, position
4. Colleague statistics (online/away/offline)

### Data Models

Implement the following entities:

1. **User** - id, name, pinyin, avatar, position, department, status, email, phone
2. **Message** - id, conversationId, senderId, type, content, timestamp, status
3. **Conversation** - id, type (single/group), name, avatar, lastMessage, unreadCount
4. **Group** - id, name, description, members, creator
5. **FileTransfer** - id, fileName, fileSize, direction, status, progress
6. **Department** - id, name, parentId, memberCount
7. **NetworkConfig** - udpPort, listenAddress, autoDiscovery
8. **ScreenShare** - id, initiatorId, recipientId, status

### Implementation Requirements

1. **Props-Based Components**: All components receive data via props, no direct imports
2. **TypeScript**: Full type safety with exported interfaces
3. **Responsive**: Mobile-first design with Tailwind breakpoints
4. **Dark Mode**: All components support dark mode
5. **Accessibility**: Proper ARIA labels and keyboard navigation
6. **Performance**: Optimize for large lists (virtualization if needed)

### Testing Approach

For each section, verify:
- Component renders without errors
- All user interactions work correctly
- Dark mode toggles properly
- Responsive layout adapts to screen size
- Empty states display appropriately
- Loading states work correctly
- Error handling is in place

### Sample Data

Use the provided `data.json` files in each section directory for development and testing.

### Deliverables

1. Complete working application with all sections
2. All TypeScript types exported and documented
3. Dark mode fully functional
4. Responsive design verified
5. Sample data integrated for testing
6. Basic error handling and validation

---

## Next Steps

Once you confirm the answers to the clarifying questions above, proceed with implementation following the milestone order. Test each milestone before moving to the next.

For detailed implementation instructions, refer to:
- `instructions/one-shot-instructions.md` - Combined milestone guide
- `instructions/incremental/` - Individual milestone instructions
- `sections/*/tests.md` - TDD specifications for each section
