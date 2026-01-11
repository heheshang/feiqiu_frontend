# One-Shot Implementation Instructions

Complete guide to implementing 飞秋 (FeiQiu) in a single development cycle.

## Prerequisites

Before starting, confirm:
- [ ] Framework chosen (Next.js, Vite, CRA, etc.)
- [ ] State management approach (Redux, Zustand, Context, etc.)
- [ ] Backend solution determined (API routes, Express, etc.)
- [ ] Database selected (PostgreSQL, MySQL, MongoDB, etc.)
- [ ] Real-time communication approach (WebSocket, Socket.io, etc.)

---

## Milestone 1: Foundation (Project Setup)

### Step 1: Initialize Project

```bash
# For Next.js
npx create-next-app@latest feiqiu --typescript --tailwind --app

# For Vite
npm create vite@latest feiqiu -- --template react-ts

# For CRA
npx create-react-app feiqiu --template typescript
```

### Step 2: Configure Tailwind CSS v4

Install Tailwind CSS v4:
```bash
npm install -D tailwindcss@next @tailwindcss/postcss@next
```

Update `postcss.config.js`:
```js
module.exports = {
  plugins: {
    '@tailwindcss/postcss': {},
  },
}
```

Configure design tokens in `src/app/globals.css` (Next.js) or `src/index.css`:

```css
@import "tailwindcss";

@theme {
  --color-primary-50: #ecfdf5;
  --color-primary-100: #d1fae5;
  --color-primary-500: #10b981;
  --color-primary-600: #059669;

  --color-secondary-500: #3b82f6;
  --color-secondary-600: #2563eb;

  --font-sans: 'Inter', sans-serif;
  --font-mono: 'IBM Plex Mono', monospace;
}
```

### Step 3: Configure Dark Mode

Add dark mode toggle provider:
```tsx
// src/contexts/DarkModeContext.tsx
'use client'

import { createContext, useContext, useEffect, useState } from 'react'

type Theme = 'light' | 'dark'

const DarkModeContext = createContext<{
  theme: Theme
  toggleTheme: () => void
} | null>(null)

export function DarkModeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setTheme] = useState<Theme>('dark')

  useEffect(() => {
    document.documentElement.classList.toggle('dark', theme === 'dark')
  }, [theme])

  const toggleTheme = () => setTheme(t => t === 'light' ? 'dark' : 'light')

  return (
    <DarkModeContext.Provider value={{ theme, toggleTheme }}>
      {children}
    </DarkModeContext.Provider>
  )
}

export const useDarkMode = () => useContext(DarkModeContext)!
```

### Step 4: Create Folder Structure

```
src/
├── app/
├── components/
│   ├── shell/
│   ├── basic-settings/
│   ├── messaging/
│   ├── file-transfer/
│   ├── collaboration/
│   └── organization/
├── lib/
│   └── types/
└── contexts/
```

---

## Milestone 2: Application Shell

### Step 1: Implement AppShell

Create `src/components/shell/AppShell.tsx` following the provided component:

- Three-column layout (60px + 280px + flex-1)
- Responsive behavior for tablet/mobile
- Dark mode support

### Step 2: Implement MainNav

Create `src/components/shell/MainNav.tsx`:

- Navigation items: 聊天, 通讯录, 组织架构
- Active state highlighting
- Icon-based layout for left sidebar

### Step 3: Implement UserMenu

Create `src/components/shell/UserMenu.tsx`:

- User avatar display
- Dropdown menu (个人设置, 网络设置, 退出)
- Online status indicator

### Step 4: Integrate Shell

Update your main layout to use AppShell:
```tsx
import { AppShell } from '@/components/shell/AppShell'
import { MainNav } from '@/components/shell/MainNav'
import { UserMenu } from '@/components/shell/UserMenu'

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <AppShell
      mainNav={<MainNav />}
      userMenu={<UserMenu />}
    >
      {children}
    </AppShell>
  )
}
```

---

## Milestone 3: Basic Settings Section

### Step 1: Create Types

Create `src/lib/types/basic-settings.ts`:
```typescript
export interface User {
  id: string
  name: string
  pinyin: string
  avatar: string
  signature: string
  status: 'online' | 'away' | 'busy' | 'offline'
  email: string
  phone: string
  department: string
}

export interface NetworkConfig {
  udpPort: number
  listenAddress: string
  autoDiscovery: boolean
  maxConnections: number
}

export interface NetworkStatus {
  isConnected: boolean
  currentPort: number
  connectedPeers: number
  latency: number
}
```

### Step 2: Implement Components

Copy components from `product-plan/sections/basic-settings/`:
- `BasicSettings.tsx` - Main settings container
- `NetworkStatusCard.tsx` - Network status display

### Step 3: Integrate Section

Add routing or state to display basic settings:
```tsx
import { BasicSettings } from '@/components/basic-settings/BasicSettings'

function BasicSettingsPage() {
  return (
    <BasicSettings
      currentUser={sampleData.user}
      networkConfig={sampleData.networkConfig}
      networkStatus={sampleData.networkStatus}
      onUserUpdate={(data) => console.log('Update user:', data)}
      onNetworkConfigChange={(config) => console.log('Update config:', config)}
      onTestConnection={() => console.log('Test connection')}
    />
  )
}
```

---

## Milestone 4: Messaging Section

### Step 1: Create Types

Create `src/lib/types/messaging.ts`:
```typescript
export type MessageStatus = 'sending' | 'sent' | 'delivered' | 'read' | 'failed'

export interface Message {
  id: string
  conversationId: string
  senderId: string
  type: 'text' | 'image' | 'file' | 'system'
  content: string
  timestamp: Date
  status: MessageStatus
  isRecalled?: boolean
  replyTo?: string
  reactions?: Array<{ emoji: string; userId: string }>
}

export interface Conversation {
  id: string
  type: 'single' | 'group'
  name: string
  avatar: string
  lastMessage?: Message
  unreadCount: number
  isPinned: boolean
  isMuted: boolean
  timestamp: Date
}

export interface User {
  id: string
  name: string
  pinyin: string
  avatar: string
  status: 'online' | 'away' | 'offline'
}
```

### Step 2: Implement Components

Copy components from `product-plan/sections/messaging/`:
- `Messaging.tsx` - Main messaging container
- `ConversationItem.tsx` - Conversation list item
- `MessageBubble.tsx` - Message bubble with reactions
- `MessageInput.tsx` - Input with emoji, file, @mentions

### Step 3: Add Message State Management

Implement message state with your chosen solution:

**Using Zustand:**
```typescript
import { create } from 'zustand'

interface MessageStore {
  conversations: Conversation[]
  activeConversation: string | null
  messages: Record<string, Message[]>
  setActiveConversation: (id: string) => void
  sendMessage: (conversationId: string, content: string) => void
}

export const useMessageStore = create<MessageStore>((set) => ({
  conversations: [],
  activeConversation: null,
  messages: {},
  setActiveConversation: (id) => set({ activeConversation: id }),
  sendMessage: (conversationId, content) => {
    // Implement send logic
  },
}))
```

---

## Milestone 5: File Transfer Section

### Step 1: Create Types

Create `src/lib/types/file-transfer.ts`:
```typescript
export type TransferDirection = 'upload' | 'download'
export type TransferStatus = 'pending' | 'transferring' | 'paused' | 'completed' | 'failed'

export interface FileTransfer {
  id: string
  fileName: string
  fileSize: number
  fileType: string
  direction: TransferDirection
  status: TransferStatus
  progress: number
  speed: number
  peerId?: string
  peerName?: string
  timestamp: Date
}
```

### Step 2: Implement Components

Copy from `product-plan/sections/file-transfer/`:
- `FileTransfer.tsx` - Transfer list container
- `FileTransferItem.tsx` - Individual transfer item

### Step 3: Add File Transfer Logic

Implement file handling:
```typescript
async function handleFileUpload(files: FileList) {
  for (const file of Array.from(files)) {
    const transfer: FileTransfer = {
      id: crypto.randomUUID(),
      fileName: file.name,
      fileSize: file.size,
      fileType: file.type,
      direction: 'upload',
      status: 'pending',
      progress: 0,
      speed: 0,
      timestamp: new Date(),
    }
    // Add to state and start transfer
  }
}
```

---

## Milestone 6: Collaboration Tools Section

### Step 1: Create Types

Create `src/lib/types/collaboration.ts`:
```typescript
export type AnnotationType = 'arrow' | 'rectangle' | 'text' | 'blur'

export interface Annotation {
  id: string
  type: AnnotationType
  x: number
  y: number
  width?: number
  height?: number
  text?: string
  color: string
}

export interface Screenshot {
  id: string
  imageUrl: string
  timestamp: Date
  annotations: Annotation[]
}
```

### Step 2: Implement Components

Copy from `product-plan/sections/collaboration/`:
- `CollaborationTools.tsx` - Toolbar and screenshot history

### Step 3: Add Screenshot Functionality

```typescript
async function captureScreen() {
  try {
    // Use Screen Capture API or integrate with desktop app
    const stream = await navigator.mediaDevices.getDisplayMedia({
      video: { cursor: 'always' },
    })

    const video = document.createElement('video')
    video.srcObject = stream
    await video.play()

    const canvas = document.createElement('canvas')
    canvas.width = video.videoWidth
    canvas.height = video.videoHeight

    const ctx = canvas.getContext('2d')!
    ctx.drawImage(video, 0, 0)

    const imageUrl = canvas.toDataURL('image/png')
    stream.getTracks().forEach(t => t.stop())

    return imageUrl
  } catch (err) {
    console.error('Screenshot failed:', err)
  }
}
```

---

## Milestone 7: Organization Chart Section

### Step 1: Create Types

Create `src/lib/types/organization.ts`:
```typescript
export interface Department {
  id: string
  name: string
  parentId: string | null
  memberCount: number
}

export interface User {
  id: string
  name: string
  pinyin: string
  avatar: string
  position: string
  department: string
  departmentId: string
  status: 'online' | 'away' | 'offline'
  email: string
  phone: string
}
```

### Step 2: Implement Components

Copy from `product-plan/sections/organization/`:
- `OrganizationChart.tsx` - Main container with dual layout
- `DepartmentTree.tsx` - Expandable tree component
- `UserCard.tsx` - User card with actions

### Step 3: Add Search and Filtering

```typescript
function useUserSearch(users: User[], query: string) {
  return useMemo(() => {
    if (!query.trim()) return users

    const q = query.toLowerCase()
    return users.filter(user =>
      user.name.toLowerCase().includes(q) ||
      user.pinyin.toLowerCase().includes(q) ||
      user.department.toLowerCase().includes(q) ||
      user.position.toLowerCase().includes(q)
    )
  }, [users, query])
}
```

---

## Final Integration Steps

### 1. Connect Navigation

Wire up MainNav to switch between sections:
```typescript
const [activeTab, setActiveTab] = useState<'chat' | 'contacts' | 'organization'>('chat')

function MainContent() {
  switch (activeTab) {
    case 'chat': return <Messaging />
    case 'contacts': return <div>Contacts view</div>
    case 'organization': return <OrganizationChart />
  }
}
```

### 2. Implement Real-Time Updates

Set up WebSocket/Socket.io for real-time features:
- Message delivery
- Online status updates
- File transfer progress
- Typing indicators

### 3. Add Data Persistence

- Local storage for user preferences
- IndexedDB for offline message cache
- Backend API for data synchronization

### 4. Polish and Test

- Verify dark mode works across all sections
- Test responsive behavior on mobile
- Add loading and error states
- Implement proper error handling

---

## Testing Checklist

- [ ] All sections render without errors
- [ ] Dark mode toggles correctly
- [ ] Navigation works between sections
- [ ] Message sending and receiving works
- [ ] File transfer progress updates
- [ ] Screenshots capture and display
- [ ] Organization search filters correctly
- [ ] Settings save and persist
- [ ] Responsive layout on mobile/tablet
- [ ] All callbacks fire correctly

---

## Next Steps

After completing implementation:
1. Deploy to staging environment
2. Conduct user acceptance testing
3. Gather feedback and iterate
4. Prepare for production deployment
