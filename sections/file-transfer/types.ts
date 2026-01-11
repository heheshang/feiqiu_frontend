// =============================================================================
// Data Types
// =============================================================================

/**
 * 用户信息
 */
export interface User {
  id: string
  name: string
  avatar: string
}

/**
 * 传输方向
 */
export type TransferDirection = 'send' | 'receive'

/**
 * 传输状态
 */
export type TransferStatus = 'waiting' | 'transferring' | 'paused' | 'completed' | 'cancelled' | 'failed'

/**
 * 文件传输记录
 */
export interface FileTransfer {
  id: string
  direction: TransferDirection
  senderId: string
  receiverId: string
  fileName: string
  fileSize: number
  fileType: string
  status: TransferStatus
  progress: number
  transferSpeed: number
  remainingTime: number
  startTime: string
  endTime: string | null
  errorMessage?: string
}

// =============================================================================
// Component Props
// =============================================================================

/**
 * FileTransfer 组件的 Props
 */
export interface FileTransferProps {
  /** 当前用户 */
  currentUser: User
  /** 文件传输列表 */
  fileTransfers: FileTransfer[]
  /** 用户映射表 */
  users: Record<string, User>
  /** 暂停传输 */
  onPause?: (id: string) => void
  /** 继续传输 */
  onResume?: (id: string) => void
  /** 取消传输 */
  onCancel?: (id: string) => void
  /** 重试失败的传输 */
  onRetry?: (id: string) => void
  /** 打开文件所在文件夹 */
  onOpenFolder?: (id: string) => void
  /** 重新下载文件 */
  onRedownload?: (id: string) => void
  /** 发送新文件 */
  onSendFile?: (files: File[]) => void
}
