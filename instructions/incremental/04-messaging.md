# Milestone 4: Messaging Section

## Overview

Implement the Messaging section with conversations, messages, and chat functionality.

---

## Step 1: Create Types

Create `src/lib/types/messaging.ts`:

```typescript
export type MessageStatus = 'sending' | 'sent' | 'delivered' | 'read' | 'failed'
export type MessageType = 'text' | 'image' | 'file' | 'system'

export interface Message {
  id: string
  conversationId: string
  senderId: string
  type: MessageType
  content: string
  timestamp: Date
  status: MessageStatus
  isRecalled?: boolean
  replyTo?: string
  reactions?: MessageReaction[]
}

export interface MessageReaction {
  emoji: string
  userId: string
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

export interface MessagingProps {
  currentUser: User
  conversations: Conversation[]
  users: Record<string, User>
  activeConversationId: string | null
  onConversationSelect: (id: string) => void
  onSendMessage: (conversationId: string, content: string, type: MessageType) => void
  onMessageRecall: (messageId: string) => void
  onMessageReply: (messageId: string) => void
  onMessageReact: (messageId: string, emoji: string) => void
  onSearch: (query: string) => void
}
```

---

## Step 2: Copy Components

Copy the following files from `product-plan/sections/messaging/` to `src/components/messaging/`:

1. `Messaging.tsx` - Main messaging container
2. `ConversationItem.tsx` - Conversation list item
3. `MessageBubble.tsx` - Message bubble with reactions
4. `MessageInput.tsx` - Input with emoji, file, @mentions
5. Create `index.ts`

---

## Step 3: Create Index File

Create `src/components/messaging/index.ts`:

```typescript
export { Messaging } from './Messaging'
export { ConversationItem } from './ConversationItem'
export { MessageBubble } from './MessageBubble'
export { MessageInput } from './MessageInput'
```

---

## Step 4: Implement Message Store

Create `src/stores/messageStore.ts` (using Zustand):

```typescript
import { create } from 'zustand'
import type { Conversation, Message, User } from '@/lib/types/messaging'

interface MessageStore {
  conversations: Conversation[]
  activeConversationId: string | null
  messages: Record<string, Message[]>
  users: Record<string, User>

  setActiveConversation: (id: string) => void
  addMessage: (message: Message) => void
  updateMessageStatus: (messageId: string, status: Message['status']) => void
  recallMessage: (messageId: string) => void
  addReaction: (messageId: string, emoji: string, userId: string) => void

  getActiveConversationMessages: () => Message[]
  getActiveConversation: () => Conversation | null
}

export const useMessageStore = create<MessageStore>((set, get) => ({
  conversations: [],
  activeConversationId: null,
  messages: {},
  users: {},

  setActiveConversation: (id) => set({ activeConversationId: id }),

  addMessage: (message) => set((state) => ({
    messages: {
      ...state.messages,
      [message.conversationId]: [
        ...(state.messages[message.conversationId] || []),
        message
      ]
    }
  })),

  updateMessageStatus: (messageId, status) => set((state) => {
    const messages = { ...state.messages }
    Object.keys(messages).forEach(convId => {
      messages[convId] = messages[convId].map(msg =>
        msg.id === messageId ? { ...msg, status } : msg
      )
    })
    return { messages }
  }),

  recallMessage: (messageId) => set((state) => {
    const messages = { ...state.messages }
    Object.keys(messages).forEach(convId => {
      messages[convId] = messages[convId].map(msg =>
        msg.id === messageId ? { ...msg, isRecalled: true } : msg
      )
    })
    return { messages }
  }),

  addReaction: (messageId, emoji, userId) => set((state) => {
    const messages = { ...state.messages }
    Object.keys(messages).forEach(convId => {
      messages[convId] = messages[convId].map(msg => {
        if (msg.id === messageId) {
          const reactions = msg.reactions || []
          const existingIndex = reactions.findIndex(r => r.userId === userId && r.emoji === emoji)
          if (existingIndex >= 0) {
            return {
              ...msg,
              reactions: reactions.filter((_, i) => i !== existingIndex)
            }
          }
          return {
            ...msg,
            reactions: [...reactions, { emoji, userId }]
          }
        }
        return msg
      })
    })
    return { messages }
  }),

  getActiveConversationMessages: () => {
    const { activeConversationId, messages } = get()
    if (!activeConversationId) return []
    return messages[activeConversationId] || []
  },

  getActiveConversation: () => {
    const { activeConversationId, conversations } = get()
    return conversations.find(c => c.id === activeConversationId) || null
  }
}))
```

---

## Step 5: Integrate Messaging Section

Update your main app:

```typescript
'use client'

import { useState } from 'react'
import { AppShell, MainNav, UserMenu } from '@/components/shell'
import { Messaging } from '@/components/messaging'
import { BasicSettings } from '@/components/basic-settings'
import type { NavTab } from '@/lib/types/shell'

// Import sample data
const sampleConversations = [
  // ... from product-plan/sections/messaging/data.json
]

const sampleUsers = {
  // ... from product-plan/sections/messaging/data.json
}

export default function HomePage() {
  const [activeTab, setActiveTab] = useState<NavTab>('chat')
  const [showSettings, setShowSettings] = useState(false)

  const renderContent = () => {
    if (showSettings) {
      return <BasicSettings {...settingsProps} />
    }

    switch (activeTab) {
      case 'chat':
        return (
          <Messaging
            currentUser={currentUser}
            conversations={sampleConversations}
            users={sampleUsers}
            activeConversationId={activeConversationId}
            onConversationSelect={(id) => setActiveConversationId(id)}
            onSendMessage={(convId, content, type) => {
              console.log('Send message:', { convId, content, type })
            }}
            onMessageRecall={(id) => console.log('Recall:', id)}
            onMessageReply={(id) => console.log('Reply to:', id)}
            onMessageReact={(id, emoji) => console.log('React:', id, emoji)}
            onSearch={(q) => console.log('Search:', q)}
          />
        )
      default:
        return <div>Content for: {activeTab}</div>
    }
  }

  return (
    <AppShell
      mainNav={<MainNav activeTab={activeTab} onTabChange={setActiveTab} />}
      userMenu={<UserMenu {...userMenuProps} />}
    >
      {renderContent()}
    </AppShell>
  )
}
```

---

## Step 6: Add Real-Time Updates (Optional)

For real-time messaging, set up WebSocket:

```typescript
// src/lib/websocket.ts
class MessagingWebSocket {
  private ws: WebSocket | null = null
  private messageHandlers: Array<(message: Message) => void> = []

  connect(url: string) {
    this.ws = new WebSocket(url)

    this.ws.onmessage = (event) => {
      const message = JSON.parse(event.data)
      this.messageHandlers.forEach(handler => handler(message))
    }
  }

  sendMessage(message: any) {
    this.ws?.send(JSON.stringify(message))
  }

  onMessage(handler: (message: Message) => void) {
    this.messageHandlers.push(handler)
  }

  disconnect() {
    this.ws?.close()
  }
}

export const messagingWS = new MessagingWebSocket()
```

---

## Verification

Test the Messaging section:

1. **Conversation List**:
   - [ ] All conversations display
   - [ ] Unread counts show
   - [ ] Click to select conversation
   - [ ] Search filters conversations

2. **Message Display**:
   - [ ] Messages render correctly
   - [ ] Own messages on right, others on left
   - [ ] Timestamps display
   - [ ] Status indicators show

3. **Message Actions**:
   - [ ] Send text messages
   - [ ] React with emoji
   - [ ] Reply to messages
   - [ ] Recall own messages

4. **UI/UX**:
   - [ ] Auto-scroll to bottom
   - [ ] Dark mode works
   - [ ] Responsive layout

---

## Next Steps

Once Messaging is complete, proceed to **[Milestone 5: File Transfer](./05-file-transfer.md)**.
