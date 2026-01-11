/**
 * Messaging Section Types
 * 飞秋消息通讯功能类型定义
 */

// 消息类型
export type MessageType = 'text' | 'emoji' | 'image' | 'file' | 'system'

// 消息状态
export type MessageStatus = 'sending' | 'sent' | 'read' | 'unread' | 'failed'

// 用户在线状态
export type UserStatus = 'online' | 'offline' | 'away' | 'busy'

// 会话类型
export type ConversationType = 'single' | 'group'

// 表情反应
export interface MessageReaction {
  emoji: string
  users: Array<{
    id: string
    name: string
  }>
}

// 引用回复
export interface MessageQuote {
  messageId: string
  content: string
  senderName: string
}

// 用户信息
export interface User {
  id: string
  name: string
  avatar: string
  status?: UserStatus
  department?: string
}

// 消息
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

// 群组
export interface Group {
  id: string
  name: string
  avatar: string
  memberCount: number
  members: User[]
}

// 会话
export interface Conversation {
  id: string
  type: ConversationType
  pinned: boolean
  unreadCount: number
  lastMessage: {
    id: string
    content: string
    type: MessageType
    timestamp: string
    senderId: string
    senderName: string
  }
  participant?: User // 单聊时存在
  group?: Group // 群聊时存在
  messages?: Message[] // 当前会话的消息列表
}

// 聊天输入状态
export interface ChatInputState {
  text: string
  selectionStart: number
  selectionEnd: number
}

// 表情选择器
export interface EmojiCategory {
  name: string
  emojis: string[]
}

// @提及用户
export interface MentionUser {
  id: string
  name: string
  avatar: string
}
