export type UserStatus = 'online' | 'away' | 'busy' | 'offline'

export type ConnectionStatus = 'connected' | 'disconnected' | 'connecting' | 'error'

export interface User {
  id: string
  name: string
  avatarUrl?: string
  signature: string
  status: UserStatus
  department: string
}

export interface NetworkConfig {
  id: string
  udpPort: number
  bindAddress: string
  broadcastAddress: string
  maxRetries: number
  timeout: number
}

export interface NetworkStatus {
  ipAddress: string
  macAddress: string
  connectionStatus: ConnectionStatus
  listeningPort: number
  lastSeen: string
  onlineUsers: number
}

export interface BasicSettingsProps {
  user: User
  networkConfig: NetworkConfig
  networkStatus: NetworkStatus
  activeTab?: 'profile' | 'network'
  onTabChange?: (tab: 'profile' | 'network') => void
  onUpdateUser?: (user: Partial<User>) => void
  onUploadAvatar?: (file: File) => void
  onStatusChange?: (status: UserStatus) => void
  onSaveNetworkConfig?: (config: NetworkConfig) => void
  onCancelNetworkConfig?: () => void
}
