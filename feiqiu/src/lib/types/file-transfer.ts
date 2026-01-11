export type TransferDirection = 'send' | 'receive'
export type TransferStatus = 'waiting' | 'transferring' | 'paused' | 'completed' | 'cancelled' | 'failed'

export interface User {
  id: string
  name: string
  avatar: string
}

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

export interface FileTransferProps {
  currentUser: User
  fileTransfers: FileTransfer[]
  users: Record<string, User>
  onPause?: (id: string) => void
  onResume?: (id: string) => void
  onCancel?: (id: string) => void
  onRetry?: (id: string) => void
  onOpenFolder?: (id: string) => void
  onRedownload?: (id: string) => void
  onSendFile?: (files: File[]) => void
}
