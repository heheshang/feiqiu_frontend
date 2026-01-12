/**
 * Backend DTO (Data Transfer Object) types
 *
 * These types match the Rust structs in the backend exactly.
 * They are used for type-safe IPC communication.
 */

/**
 * Peer data transfer object from backend
 * Matches Rust PeerDto in src-tauri/src/commands/peer.rs
 */
export interface PeerDto {
  ip: string
  port: number
  username: string | null
  hostname: string | null
  nickname: string | null
  avatar: string | null
  groups: string[]
  status: string
  displayName: string
  lastSeen: number // i64 milliseconds
}

/**
 * Message data transfer object from backend
 */
export interface MessageDto {
  id: string
  msgId: string
  senderIp: string
  senderName: string
  receiverIp: string
  msgType: number
  content: string
  isEncrypted: boolean
  isOffline: boolean
  sentAt: number // i64 milliseconds
  receivedAt: number // i64 milliseconds
  createdAt: number // i64 milliseconds
}

/**
 * Configuration data transfer object from backend
 */
export interface ConfigDto {
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
 * File transfer task data transfer object from backend
 */
export interface TaskDto {
  id: string
  direction: 'incoming' | 'outgoing'
  peerIp: string
  fileName: string
  fileSize: number
  md5: string | null
  status: 'pending' | 'active' | 'paused' | 'completed' | 'failed' | 'cancelled'
  transferredBytes: number
  progress: number
  port: number | null
  error: string | null
  createdAt: number // i64 milliseconds
  updatedAt: number // i64 milliseconds
}

/**
 * System info data transfer object
 */
export interface SystemInfoDto {
  platform: string
  version: string
}

/**
 * Network status data transfer object (future implementation)
 */
export interface NetworkStatusDto {
  ipAddress: string
  macAddress: string | null
  listeningPort: number
  isConnected: boolean
}

/**
 * Peer statistics data transfer object
 */
export interface PeerStatsDto {
  totalPeers: number
  onlinePeers: number
  offlinePeers: number
  awayPeers: number
}

/**
 * Event data transfer objects (emitted from backend)
 */
export interface MessageReceivedEvent {
  msgId: string
  senderIp: string
  senderName: string
  receiverIp: string
  msgType: number
  content: string
  isEncrypted: boolean
  isOffline: boolean
  sentAt: number // i64 milliseconds
  receivedAt: number // i64 milliseconds
}

export interface MessageReceiptAckEvent {
  msgId: string
  senderIp: string
  senderName: string
  status: 'delivered' | 'read'
  timestamp: number // i64 milliseconds
}

export interface PeerOnlineEvent {
  ip: string
  port: number
  username: string
  hostname: string
  nickname: string | null
  avatar: string | null
  groups: string[]
  displayName: string
}

export interface PeerOfflineEvent {
  ip: string
  lastSeen: number // i64 milliseconds
}

export interface PeersDiscoveredEvent {
  peers: PeerDto[]
}

export interface FileTransferRequestEvent {
  transferId: string
  peerIp: string
  fileName: string
  fileSize: number
  md5: string | null
}

export interface FileTransferProgressEvent {
  transferId: string
  transferredBytes: number
  progress: number // 0-100
}

export interface FileTransferCompletedEvent {
  transferId: string
  filePath: string
  completedAt: number // i64 milliseconds
}

export interface FileTransferFailedEvent {
  transferId: string
  error: string
  failedAt: number // i64 milliseconds
}
