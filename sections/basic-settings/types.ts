// =============================================================================
// Data Types
// =============================================================================

export type UserStatus = 'online' | 'away' | 'busy' | 'offline'

export type ConnectionStatus = 'connected' | 'disconnected' | 'connecting' | 'error'

export interface User {
  /** 用户唯一标识 */
  id: string
  /** 用户昵称 */
  name: string
  /** 头像URL（可选） */
  avatarUrl?: string
  /** 个性签名 */
  signature: string
  /** 在线状态 */
  status: UserStatus
  /** 所属部门 */
  department: string
}

export interface NetworkConfig {
  /** 配置唯一标识 */
  id: string
  /** UDP监听端口 */
  udpPort: number
  /** 绑定地址 */
  bindAddress: string
  /** 广播地址 */
  broadcastAddress: string
  /** 最大重试次数 */
  maxRetries: number
  /** 超时时间（毫秒） */
  timeout: number
}

export interface NetworkStatus {
  /** 当前IP地址 */
  ipAddress: string
  /** MAC地址 */
  macAddress: string
  /** 连接状态 */
  connectionStatus: ConnectionStatus
  /** 当前监听端口 */
  listeningPort: number
  /** 最后上线时间 */
  lastSeen: string
  /** 局域网内在线用户数 */
  onlineUsers: number
}

// =============================================================================
// Component Props
// =============================================================================

export interface BasicSettingsProps {
  /** 当前用户信息 */
  user: User
  /** 网络配置 */
  networkConfig: NetworkConfig
  /** 当前网络状态 */
  networkStatus: NetworkStatus
  /** 当前激活的选项卡 */
  activeTab?: 'profile' | 'network'
  /** 切换选项卡时调用 */
  onTabChange?: (tab: 'profile' | 'network') => void
  /** 更新用户信息时调用 */
  onUpdateUser?: (user: Partial<User>) => void
  /** 上传头像时调用 */
  onUploadAvatar?: (file: File) => void
  /** 更改在线状态时调用 */
  onStatusChange?: (status: UserStatus) => void
  /** 保存网络配置时调用 */
  onSaveNetworkConfig?: (config: NetworkConfig) => void
  /** 取消网络配置修改时调用 */
  onCancelNetworkConfig?: () => void
}
