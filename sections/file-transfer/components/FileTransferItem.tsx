'use client'

import type { FileTransfer, User } from '@/../product/sections/file-transfer/types'

interface FileTransferItemProps {
  transfer: FileTransfer
  sender: User
  receiver: User
  currentUserId: string
  onPause?: () => void
  onResume?: () => void
  onCancel?: () => void
  onRetry?: () => void
  onOpenFolder?: () => void
  onRedownload?: () => void
}

export function FileTransferItem({
  transfer,
  sender,
  receiver,
  currentUserId,
  onPause,
  onResume,
  onCancel,
  onRetry,
  onOpenFolder,
  onRedownload
}: FileTransferItemProps) {
  const isSelf = transfer.direction === 'send'
  const otherUser = isSelf ? receiver : sender

  const statusInfo = getStatusInfo(transfer.status)

  return (
    <div className="px-6 py-4 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
      <div className="flex items-start gap-4">
        {/* Icon based on file type */}
        <div className="flex-shrink-0 w-12 h-12 rounded-lg bg-slate-100 dark:bg-slate-800 flex items-center justify-center">
          {getFileIcon(transfer.fileType)}
        </div>

        {/* Content */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center justify-between mb-1">
            <div className="flex items-center gap-2 min-w-0">
              <h3 className="font-medium text-slate-900 dark:text-slate-100 truncate">
                {transfer.fileName}
              </h3>
              <span className={`px-2 py-0.5 text-xs font-medium rounded ${statusInfo.badgeClass}`}>
                {statusInfo.label}
              </span>
            </div>
            <span className="text-sm text-slate-500 dark:text-slate-400 flex-shrink-0">
              {formatFileSize(transfer.fileSize)}
            </span>
          </div>

          {/* User info */}
          <div className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400 mb-2">
            <img
              src={otherUser.avatar}
              alt={otherUser.name}
              className="w-5 h-5 rounded-full"
            />
            <span>
              {isSelf ? `发送给 ${otherUser.name}` : `${otherUser.name} 发送`}
            </span>
          </div>

          {/* Progress bar for active transfers */}
          {['waiting', 'transferring', 'paused'].includes(transfer.status) && (
            <div className="mb-2">
              <div className="flex items-center justify-between text-xs text-slate-500 dark:text-slate-400 mb-1">
                <span>{transfer.progress}%</span>
                <div className="flex items-center gap-3">
                  {transfer.transferSpeed > 0 && (
                    <span>{formatSpeed(transfer.transferSpeed)}</span>
                  )}
                  {transfer.remainingTime > 0 && (
                    <span>剩余 {transfer.remainingTime} 分钟</span>
                  )}
                </div>
              </div>
              <div className="h-1.5 bg-slate-200 dark:bg-slate-700 rounded-full overflow-hidden">
                <div
                  className="h-full bg-emerald-500 rounded-full transition-all duration-300"
                  style={{ width: `${transfer.progress}%` }}
                />
              </div>
            </div>
          )}

          {/* Error message */}
          {transfer.status === 'failed' && transfer.errorMessage && (
            <p className="text-sm text-red-500 dark:text-red-400">
              {transfer.errorMessage}
            </p>
          )}

          {/* Action buttons */}
          <div className="flex items-center gap-2 mt-2">
            {transfer.status === 'transferring' && onPause && (
              <button
                onClick={onPause}
                className="px-3 py-1 text-sm bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
              >
                暂停
              </button>
            )}
            {transfer.status === 'paused' && onResume && (
              <button
                onClick={onResume}
                className="px-3 py-1 text-sm bg-emerald-500 text-white rounded-lg hover:bg-emerald-600 transition-colors"
              >
                继续
              </button>
            )}
            {['waiting', 'transferring', 'paused'].includes(transfer.status) && onCancel && (
              <button
                onClick={onCancel}
                className="px-3 py-1 text-sm text-slate-600 dark:text-slate-400 hover:text-red-500 dark:hover:text-red-400 transition-colors"
              >
                取消
              </button>
            )}
            {transfer.status === 'failed' && onRetry && (
              <button
                onClick={onRetry}
                className="px-3 py-1 text-sm bg-emerald-500 text-white rounded-lg hover:bg-emerald-600 transition-colors"
              >
                重试
              </button>
            )}
            {transfer.status === 'completed' && isSelf && onOpenFolder && (
              <button
                onClick={onOpenFolder}
                className="px-3 py-1 text-sm text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
              >
                打开文件夹
              </button>
            )}
            {transfer.status === 'completed' && !isSelf && onRedownload && (
              <button
                onClick={onRedownload}
                className="px-3 py-1 text-sm text-slate-600 dark:text-slate-400 hover:text-emerald-500 dark:hover:text-emerald-400 transition-colors"
              >
                重新下载
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}

function getStatusInfo(status: string) {
  const statusMap = {
    waiting: { label: '等待中', badgeClass: 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400' },
    transferring: { label: '传输中', badgeClass: 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400' },
    paused: { label: '已暂停', badgeClass: 'bg-amber-100 dark:bg-amber-900/30 text-amber-600 dark:text-amber-400' },
    completed: { label: '已完成', badgeClass: 'bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400' },
    cancelled: { label: '已取消', badgeClass: 'bg-slate-100 dark:bg-slate-800 text-slate-500 dark:text-slate-400' },
    failed: { label: '失败', badgeClass: 'bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400' }
  }
  return statusMap[status as keyof typeof statusMap] || statusMap.waiting
}

function getFileIcon(fileType: string) {
  const iconMap: Record<string, React.ReactNode> = {
    pdf: (
      <svg className="w-6 h-6 text-red-500" fill="currentColor" viewBox="0 0 20 20">
        <path fillRule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clipRule="evenodd" />
      </svg>
    ),
    docx: (
      <svg className="w-6 h-6 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
        <path fillRule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clipRule="evenodd" />
      </svg>
    ),
    xlsx: (
      <svg className="w-6 h-6 text-emerald-500" fill="currentColor" viewBox="0 0 20 20">
        <path fillRule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clipRule="evenodd" />
      </svg>
    ),
    png: (
      <svg className="w-6 h-6 text-purple-500" fill="currentColor" viewBox="0 0 20 20">
        <path fillRule="evenodd" d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z" clipRule="evenodd" />
      </svg>
    ),
    fig: (
      <svg className="w-6 h-6 text-pink-500" fill="currentColor" viewBox="0 0 20 20">
        <path d="M4 4a2 2 0 012-2h8a2 2 0 012 2v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" />
      </svg>
    ),
    mp4: (
      <svg className="w-6 h-6 text-indigo-500" fill="currentColor" viewBox="0 0 20 20">
        <path d="M2 6a2 2 0 012-2h6a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6zM14.553 7.106A1 1 0 0014 8v4a1 1 0 00.553.894l2 1A1 1 0 0018 13V7a1 1 0 00-1.447-.894l-2 1z" />
      </svg>
    )
  }
  return iconMap[fileType] || (
    <svg className="w-6 h-6 text-slate-400" fill="currentColor" viewBox="0 0 20 20">
      <path fillRule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clipRule="evenodd" />
    </svg>
  )
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const mb = bytes / (1024 * 1024)
  if (mb >= 1) return `${mb.toFixed(2)} MB`
  const kb = bytes / 1024
  return `${kb.toFixed(2)} KB`
}

function formatSpeed(bytesPerSecond: number): string {
  if (bytesPerSecond === 0) return '0 B/s'
  const mbps = bytesPerSecond / (1024 * 1024)
  if (mbps >= 1) return `${mbps.toFixed(1)} MB/s`
  const kbps = bytesPerSecond / 1024
  return `${kbps.toFixed(1)} KB/s`
}
