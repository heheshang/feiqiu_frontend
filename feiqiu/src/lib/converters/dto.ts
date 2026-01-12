/**
 * DTO (Data Transfer Object) adapters
 *
 * Converts backend DTO types to frontend types with proper field naming and type conversion.
 */

import { toIsoDate } from './timestamp'
import { mapPeerStatus, mapTransferStatus } from './status'
import type { PeerDto as BackendPeerDto } from '../api/types'
import type { MessageDto as BackendMessageDto } from '../api/types'
import type { TaskDto as BackendTaskDto } from '../api/types'
import type { ConfigDto as BackendConfigDto } from '../api/types'

// Frontend types (from existing type definitions)
import type { User as MessagingUser } from '../types/messaging'
import type { User as SettingsUser, UserStatus } from '../types/basic-settings'
import type { FileTransfer as FrontendFileTransfer } from '../types/file-transfer'

/**
 * Frontend Peer type (unified across all contexts)
 */
export interface Peer {
  ip: string
  port: number
  username: string
  hostname: string
  nickname: string | null
  avatar: string | null
  groups: string[]
  status: 'online' | 'offline' | 'away' | 'busy'
  displayName: string
  lastSeen: string // ISO string
  id: string // derived from IP
  name: string // alias for displayName
  department?: string // optional field
}

/**
 * Frontend Message type
 */
export interface Message {
  id: string
  msgId: string
  senderIp: string
  senderName: string
  receiverIp: string
  msgType: number
  content: string
  isEncrypted: boolean
  isOffline: boolean
  sentAt: string // ISO string
  receivedAt: string // ISO string
  createdAt: string // ISO string
  // Additional fields for UI compatibility
  timestamp: string // same as createdAt
  senderId: string // derived from senderIp
  type: 'text' | 'image' | 'file' | 'system'
  status: 'sending' | 'sent' | 'read' | 'unread' | 'failed'
}

/**
 * Frontend Config type
 */
export interface Config {
  username: string
  hostname: string
  avatar: string | null
  status: string
  bindIp: string
  udpPort: number
  tcpPortStart: number
  tcpPortEnd: number
  heartbeatInterval: number
  peerTimeout: number
  encryptionEnabled: boolean
  encryptionKey: string | null
  offlineMessageRetentionDays: number
  autoAcceptFiles: boolean
  fileSaveDir: string
  logLevel: string
}

/**
 * Frontend FileTransfer type
 * Re-exported from file-transfer types
 */
export type FileTransfer = FrontendFileTransfer

/**
 * Message filters for getMessages
 */
export interface MessageFilters {
  /** Filter by sender IP address */
  senderIp?: string
  /** Filter by receiver IP address */
  receiverIp?: string
  /** Filter by minimum timestamp (i64 milliseconds) */
  after?: number
  /** Filter by maximum timestamp (i64 milliseconds) */
  before?: number
  /** Limit number of results */
  limit?: number
}

/**
 * Converts backend PeerDto to frontend Peer
 */
export function toFrontendPeer(dto: BackendPeerDto): Peer {
  return {
    ip: dto.ip,
    port: dto.port,
    username: dto.username || '',
    hostname: dto.hostname || '',
    nickname: dto.nickname || null,
    avatar: dto.avatar || null,
    groups: dto.groups || [],
    status: mapPeerStatus((dto.status || 'offline') as any),
    displayName: dto.displayName || dto.username || dto.ip,
    lastSeen: toIsoDate(dto.lastSeen),
    id: dto.ip, // Use IP as unique identifier
    name: dto.displayName || dto.username || dto.ip,
  }
}

/**
 * Converts backend MessageDto to frontend Message
 */
export function toFrontendMessage(dto: BackendMessageDto): Message {
  return {
    id: dto.id,
    msgId: dto.msgId,
    senderIp: dto.senderIp,
    senderName: dto.senderName,
    receiverIp: dto.receiverIp,
    msgType: dto.msgType,
    content: dto.content,
    isEncrypted: dto.isEncrypted,
    isOffline: dto.isOffline,
    sentAt: toIsoDate(dto.sentAt),
    receivedAt: toIsoDate(dto.receivedAt),
    createdAt: toIsoDate(dto.createdAt),
    // Additional fields for UI compatibility
    timestamp: toIsoDate(dto.createdAt),
    senderId: dto.senderIp,
    type: 'text', // Default, could be derived from msg_type
    status: 'sent', // Could be derived from delivery status
  }
}

/**
 * Converts backend TaskDto to frontend FileTransfer
 */
export function toFrontendTransfer(dto: BackendTaskDto): FrontendFileTransfer {
  const status = mapTransferStatus((dto.status || 'pending') as any)
  const direction = dto.direction === 'incoming' ? 'receive' : 'send'

  return {
    id: dto.id,
    direction: direction as 'send' | 'receive',
    senderId: dto.direction === 'outgoing' ? 'local' : dto.peerIp,
    receiverId: dto.direction === 'incoming' ? 'local' : dto.peerIp,
    fileName: dto.fileName,
    fileSize: dto.fileSize,
    fileType: getFileExtension(dto.fileName),
    status: status,
    progress: dto.progress || 0,
    transferSpeed: 0, // Not provided by backend
    remainingTime: 0, // Calculated from progress
    startTime: toIsoDate(dto.createdAt),
    endTime: status === 'completed' ? toIsoDate(dto.updatedAt) : null,
    errorMessage: dto.error || undefined,
  }
}

/**
 * Converts backend ConfigDto to frontend Config
 */
export function toFrontendConfig(dto: BackendConfigDto): Config {
  return {
    username: dto.username || '',
    hostname: dto.hostname || '',
    avatar: dto.avatar || null,
    status: dto.status || 'online',
    bindIp: dto.bindIp || '0.0.0.0',
    udpPort: dto.udpPort || 2425,
    tcpPortStart: dto.tcpPortStart || 3000,
    tcpPortEnd: dto.tcpPortEnd || 3010,
    heartbeatInterval: dto.heartbeatInterval || 60,
    peerTimeout: dto.peerTimeout || 300,
    encryptionEnabled: dto.encryptionEnabled || false,
    encryptionKey: dto.encryptionKey || null,
    offlineMessageRetentionDays: dto.offlineMessageRetentionDays || 30,
    autoAcceptFiles: dto.autoAcceptFiles || false,
    fileSaveDir: dto.fileSaveDir || '',
    logLevel: dto.logLevel || 'info',
  }
}

/**
 * Converts frontend Peer to MessagingUser type
 */
export function toMessagingUser(peer: Peer): MessagingUser {
  return {
    id: peer.id,
    name: peer.name,
    avatar: peer.avatar || `https://api.dicebear.com/7.x/avataaars/svg?seed=${peer.ip}`,
    status: peer.status as any,
    department: peer.department,
  }
}

/**
 * Converts frontend Config to SettingsUser type
 */
export function toSettingsUser(config: Config, avatar: string = ''): SettingsUser {
  return {
    id: 'local', // Local user
    name: config.username,
    avatarUrl: config.avatar || avatar,
    signature: '', // Not stored in config
    status: config.status as UserStatus,
    department: '', // Not stored in config
  }
}

/**
 * Helper function to get file extension from filename
 */
function getFileExtension(filename: string): string {
  const parts = filename.split('.')
  return parts.length > 1 ? parts[parts.length - 1] : ''
}

/**
 * Batch converts an array of backend PeerDtos to frontend Peers
 */
export function toFrontendPeers(dtos: BackendPeerDto[]): Peer[] {
  return dtos.map(toFrontendPeer)
}

/**
 * Batch converts an array of backend MessageDtos to frontend Messages
 */
export function toFrontendMessages(dtos: BackendMessageDto[]): Message[] {
  return dtos.map(toFrontendMessage)
}

/**
 * Batch converts an array of backend TaskDtos to frontend FileTransfers
 */
export function toFrontendTransfers(dtos: BackendTaskDto[]): FrontendFileTransfer[] {
  return dtos.map(toFrontendTransfer)
}
