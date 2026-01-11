# Messaging - Test Instructions

## Components Under Test
`Messaging.tsx`, `ConversationItem.tsx`, `MessageBubble.tsx`, `MessageInput.tsx`

## Test Data

```typescript
const testConversations = [
  {
    id: 'conv-1',
    type: 'single' as const,
    name: '李娜',
    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=LiNa',
    lastMessage: {
      id: 'msg-1',
      conversationId: 'conv-1',
      senderId: 'user-2',
      type: 'text' as const,
      content: '好的，明天见！',
      timestamp: new Date('2025-01-08T10:30:00'),
      status: 'read' as const
    },
    unreadCount: 2,
    isPinned: false,
    isMuted: false,
    timestamp: new Date('2025-01-08T10:30:00')
  },
  {
    id: 'conv-2',
    type: 'group' as const,
    name: '前端开发组',
    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Frontend',
    lastMessage: {
      id: 'msg-2',
      conversationId: 'conv-2',
      senderId: 'user-3',
      type: 'text' as const,
      content: '@张伟 请查看最新的设计稿',
      timestamp: new Date('2025-01-08T09:15:00'),
      status: 'delivered' as const
    },
    unreadCount: 5,
    isPinned: true,
    isMuted: false,
    timestamp: new Date('2025-01-08T09:15:00')
  }
]

const testMessages = [
  {
    id: 'msg-1',
    conversationId: 'conv-1',
    senderId: 'user-1',
    type: 'text' as const,
    content: '你好，明天有空吗？',
    timestamp: new Date('2025-01-08T10:00:00'),
    status: 'read' as const
  },
  {
    id: 'msg-2',
    conversationId: 'conv-1',
    senderId: 'user-2',
    type: 'text' as const,
    content: '有的，什么事？',
    timestamp: new Date('2025-01-08T10:05:00'),
    status: 'read' as const
  },
  {
    id: 'msg-3',
    conversationId: 'conv-1',
    senderId: 'user-1',
    type: 'image' as const,
    content: 'https://example.com/image.png',
    timestamp: new Date('2025-01-08T10:10:00'),
    status: 'delivered' as const
  }
]

const testUsers = {
  'user-1': {
    id: 'user-1',
    name: '张伟',
    pinyin: 'zhangwei',
    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=ZhangWei',
    status: 'online' as const
  },
  'user-2': {
    id: 'user-2',
    name: '李娜',
    pinyin: 'lina',
    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=LiNa',
    status: 'online' as const
  }
}
```

## Test Cases

### 1. Conversation List
- [ ] All conversations render without errors
- [ ] Conversations sort by timestamp (newest first)
- [ ] Pinned conversations appear at top
- [ ] Unread count badge displays
- [ ] Last message preview shows
- [ ] Avatar displays correctly
- [ ] Active conversation is highlighted
- [ ] Clicking conversation selects it

### 2. Conversation Item
- [ ] Avatar displays correctly
- [ ] Name displays correctly
- [ ] Last message truncates if too long
- [ ] Timestamp shows relative time
- [ ] Unread count badge shows correct number
- [ ] Mute icon shows when muted
- [ ] Pin icon shows when pinned
- [ ] Hover effect works
- [ ] Online status indicator shows (for single chats)

### 3. Message List
- [ ] All messages render without errors
- [ ] Messages ordered by timestamp
- [ ] Own messages align right
- [ ] Others' messages align left
- [ ] Date dividers show between days
- [ ] Scroll position maintains on new messages
- [ ] Auto-scroll to bottom on new message
- [ ] Manually scrolling prevents auto-scroll

### 4. Message Bubble
- [ ] Text messages display correctly
- [ ] Image messages display correctly
- [ ] File messages show file icon and name
- [ ] System messages have distinct style
- [ ] Timestamp shows on hover/long press
- [ ] Status indicator shows (sent/delivered/read)
- [ ] Reactions display below message
- [ ] Recalled messages show "已撤回"
- [ ] Reply preview shows above message

### 5. Message Input
- [ ] Text area expands with content
- [ ] Emoji button opens picker
- [ ] File upload button triggers file picker
- [ ] @ mention button shows user list
- [ ] Send button enables when there's content
- [ ] Send on Enter (without Shift)
- [ ] New line on Shift+Enter
- [ ] Character counter shows (if limit exists)

### 6. Message Actions
- [ ] Hover shows action buttons
- [ ] Reply button creates reply context
- [ ] Recall button removes message (own messages only)
- [ ] React button opens emoji picker
- [ ] Copy button copies text
- [ ] Forward button opens conversation selector
- [ ] Delete button removes message (own messages only)

### 7. Reactions
- [ ] Clicking emoji adds reaction
- [ ] Multiple reactions stack
- [ ] Same emoji from same user toggles off
- [ ] Reaction count displays correctly
- [ ] Hover shows who reacted
- [ ] Reactions persist across re-renders

### 8. Search Functionality
- [ ] Search input filters conversations
- [ ] Search by conversation name
- [ ] Search by participant name
- [ ] Search by message content
- [ ] Clear search resets list
- [ ] Search results highlight matches
- [ ] Empty search results show message

### 9. Empty States
- [ ] No conversations shows empty state
- [ ] No messages shows placeholder
- [ ] Search with no results shows message
- [ ] Selected conversation with no messages shows hint

### 10. Dark Mode
- [ ] Message bubbles adapt correctly
- [ ] Input area colors adjust
- [ ] Conversation list items adapt
- [ ] Action buttons visible in dark mode
- [ ] Text contrast is sufficient

### 11. Responsive Behavior
- [ ] Mobile shows conversation list only
- [ ] Selecting conversation slides to messages
- [ ] Back button returns to list
- [ ] Desktop shows split view
- [ ] Tablet adjusts column widths

### 12. Real-Time Updates
- [ ] New messages appear without refresh
- [ ] Message status updates (sent → delivered → read)
- [ ] Typing indicators show
- [ ] Online status updates in real-time
- [ ] Unread count updates

## Edge Cases

- [ ] Very long messages wrap correctly
- [ ] Very long words break
- [ ] Special characters display correctly
- [ ] Unicode emojis render properly
- [ ] Large images don't break layout
- [ ] Rapid message sending handles correctly
- [ ] Empty message cannot be sent
- [ ] Whitespace-only message cannot be sent
- [ ] Message at exactly 140 characters
- [ ] Image upload failure handled gracefully

## Performance Tests

- [ ] Renders 100+ conversations without lag
- [ ] Renders 100+ messages without lag
- [ ] Scroll is smooth with many messages
- [ ] Search filters instantly
- [ ] Message send feels instant
