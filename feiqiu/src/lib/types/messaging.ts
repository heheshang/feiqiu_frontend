export type MessageType = 'text' | 'emoji' | 'image' | 'file' | 'system'
export type MessageStatus = 'sending' | 'sent' | 'read' | 'unread' | 'failed'
export type UserStatus = 'online' | 'offline' | 'away' | 'busy'
export type ConversationType = 'single' | 'group'

export interface MessageReaction {
  emoji: string
  users: Array<{ id: string; name: string }>
}

export interface MessageQuote {
  messageId: string
  content: string
  senderName: string
}

export interface User {
  id: string
  name: string
  avatar: string
  status?: UserStatus
  department?: string
}

export interface Message {
  id: string
  type: MessageType
  content: string
  timestamp: string
  senderId: string
  senderName: string
  status: MessageStatus
  reactions: MessageReaction[]
  quote?: MessageQuote
  imageThumbnailUrl?: string
  imageUrl?: string
  fileUrl?: string
  fileName?: string
  fileSize?: number
}

export interface Group {
  id: string
  name: string
  avatar: string
  memberCount: number
  members: User[]
}

export interface Conversation {
  id: string
  type: ConversationType
  pinned: boolean
  unreadCount: number
  lastMessage?: {
    id: string
    content: string
    type: MessageType
    timestamp: string
    senderId: string
    senderName: string
  }
  participant?: User
  group?: Group
  messages?: Message[]
}
