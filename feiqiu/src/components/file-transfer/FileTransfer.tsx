'use client'

import { useState } from 'react'
import { FileTransferProps } from '../../lib/types/file-transfer'
import { FileTransferItem } from './FileTransferItem'
import { cn } from '../../lib/utils'
import { Upload, FolderOpen, File } from 'lucide-react'

export function FileTransfer({
  currentUser,
  fileTransfers,
  users,
  onPause,
  onResume,
  onCancel,
  onRetry,
  onOpenFolder,
  onRedownload,
  onSendFile
}: FileTransferProps) {
  const [filter, setFilter] = useState<'all' | 'sending' | 'receiving'>('all')
  const [showHistory, setShowHistory] = useState(false)

  const filteredTransfers = fileTransfers.filter(transfer => {
    const isActive = ['waiting', 'transferring', 'paused'].includes(transfer.status)
    if (!showHistory && !isActive) return false
    if (showHistory && isActive) return false

    if (filter === 'sending' && transfer.direction !== 'send') return false
    if (filter === 'receiving' && transfer.direction !== 'receive') return false

    return true
  })

  const activeTransfers = fileTransfers.filter(t => ['waiting', 'transferring', 'paused'].includes(t.status))
  const totalTransfers = activeTransfers.length
  const totalSpeed = activeTransfers.reduce((sum, t) => sum + (t.transferSpeed || 0), 0)

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
  }

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
    const files = Array.from(e.dataTransfer.files)
    if (files.length > 0 && onSendFile) {
      onSendFile(files)
    }
  }

  return (
    <div className="h-full flex flex-col bg-white dark:bg-slate-900">
      <div className="border-b border-slate-200 dark:border-slate-700 px-6 py-4">
        <div className="flex items-center justify-between mb-4">
          <h1 className="text-xl font-semibold text-slate-900 dark:text-slate-100">文件传输</h1>
          <div className="flex items-center gap-2">
            <button
              onClick={() => setFilter('all')}
              className={cn(
                'px-3 py-1.5 text-sm rounded-lg transition-colors',
                filter === 'all'
                  ? 'bg-emerald-500 text-white'
                  : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800'
              )}
            >
              全部
            </button>
            <button
              onClick={() => setFilter('sending')}
              className={cn(
                'px-3 py-1.5 text-sm rounded-lg transition-colors',
                filter === 'sending'
                  ? 'bg-emerald-500 text-white'
                  : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800'
              )}
            >
              发送中
            </button>
            <button
              onClick={() => setFilter('receiving')}
              className={cn(
                'px-3 py-1.5 text-sm rounded-lg transition-colors',
                filter === 'receiving'
                  ? 'bg-emerald-500 text-white'
                  : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800'
              )}
            >
              接收中
            </button>
          </div>
        </div>

        {totalTransfers > 0 && (
          <div className="flex items-center gap-6 text-sm text-slate-600 dark:text-slate-400">
            <span>{totalTransfers} 个传输任务</span>
            {totalSpeed > 0 && (
              <span>总速度: {formatSpeed(totalSpeed)}</span>
            )}
          </div>
        )}
      </div>

      <div className="px-6 py-3 border-b border-slate-200 dark:border-slate-700">
        <button
          onClick={() => setShowHistory(!showHistory)}
          className="text-sm text-slate-600 dark:text-slate-400 hover:text-emerald-500 dark:hover:text-emerald-400 transition-colors"
        >
          {showHistory ? '← 返回当前传输' : '查看历史记录 →'}
        </button>
      </div>

      <div
        className="flex-1 overflow-y-auto"
        onDragOver={handleDragOver}
        onDrop={handleDrop}
      >
        {filteredTransfers.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full p-8">
            <div className={cn(
              'w-16 h-16 rounded-2xl flex items-center justify-center mb-4',
              showHistory
                ? 'bg-slate-100 dark:bg-slate-800'
                : 'bg-emerald-100 dark:bg-emerald-900/30'
            )}>
              {showHistory ? (
                <File className={cn(
                  'w-8 h-8',
                  showHistory
                    ? 'text-slate-400 dark:text-slate-500'
                    : 'text-emerald-500 dark:text-emerald-400'
                )} />
              ) : (
                <Upload className={cn(
                  'w-8 h-8',
                  'text-emerald-500 dark:text-emerald-400'
                )} />
              )}
            </div>
            <p className="text-slate-600 dark:text-slate-400 mb-1">
              {showHistory ? '暂无传输历史' : '拖拽文件到此处发送'}
            </p>
            {!showHistory && (
              <p className="text-sm text-slate-400 dark:text-slate-500">或点击下方按钮选择文件</p>
            )}
            {!showHistory && onSendFile && (
              <label className="mt-4 px-4 py-2 bg-emerald-500 hover:bg-emerald-600 text-white rounded-lg cursor-pointer transition-colors">
                选择文件
                <input
                  type="file"
                  multiple
                  className="hidden"
                  onChange={(e) => {
                    const files = Array.from(e.target.files || [])
                    if (files.length > 0) onSendFile(files)
                  }}
                />
              </label>
            )}
          </div>
        ) : (
          <div className="divide-y divide-slate-100 dark:divide-slate-800">
            {filteredTransfers.map(transfer => (
              <FileTransferItem
                key={transfer.id}
                transfer={transfer}
                sender={users[transfer.senderId]}
                receiver={users[transfer.receiverId]}
                currentUserId={currentUser.id}
                onPause={() => onPause?.(transfer.id)}
                onResume={() => onResume?.(transfer.id)}
                onCancel={() => onCancel?.(transfer.id)}
                onRetry={() => onRetry?.(transfer.id)}
                onOpenFolder={() => onOpenFolder?.(transfer.id)}
                onRedownload={() => onRedownload?.(transfer.id)}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  )
}

function formatSpeed(bytesPerSecond: number): string {
  if (bytesPerSecond === 0) return '0 B/s'
  const mbps = bytesPerSecond / (1024 * 1024)
  if (mbps >= 1) return `${mbps.toFixed(1)} MB/s`
  const kbps = bytesPerSecond / 1024
  return `${kbps.toFixed(1)} KB/s`
}
