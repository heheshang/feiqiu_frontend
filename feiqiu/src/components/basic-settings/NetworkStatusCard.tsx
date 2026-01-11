import { NetworkStatus, ConnectionStatus } from '../../lib/types/basic-settings'
import { cn } from '../../lib/utils'

interface NetworkStatusCardProps {
  networkStatus: NetworkStatus
}

const statusConfig: Record<ConnectionStatus, { label: string; color: string; bgColor: string; icon: React.ReactNode }> = {
  connected: {
    label: '已连接',
    color: 'text-emerald-600 dark:text-emerald-400',
    bgColor: 'bg-emerald-100 dark:bg-emerald-900/30',
    icon: (
      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
      </svg>
    ),
  },
  disconnected: {
    label: '未连接',
    color: 'text-slate-600 dark:text-slate-400',
    bgColor: 'bg-slate-100 dark:bg-slate-900/30',
    icon: (
      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
      </svg>
    ),
  },
  connecting: {
    label: '连接中',
    color: 'text-amber-600 dark:text-amber-400',
    bgColor: 'bg-amber-100 dark:bg-amber-900/30',
    icon: (
      <svg className="w-5 h-5 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
      </svg>
    ),
  },
  error: {
    label: '连接错误',
    color: 'text-amber-600 dark:text-amber-400',
    bgColor: 'bg-amber-100 dark:bg-amber-900/30',
    icon: (
      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
    ),
  },
}

export function NetworkStatusCard({ networkStatus }: NetworkStatusCardProps) {
  const config = statusConfig[networkStatus.connectionStatus]

  return (
    <div className="bg-white dark:bg-slate-800 rounded-xl shadow-sm border border-slate-200 dark:border-slate-700 overflow-hidden">
      <div className="p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-slate-800 dark:text-slate-200">
            网络状态
          </h3>
          <div className={cn('inline-flex items-center gap-2 px-3 py-1.5 rounded-full', config.bgColor, config.color, 'font-medium text-sm')}>
            {config.icon}
            {config.label}
          </div>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
            <p className="text-xs text-slate-500 dark:text-slate-400 mb-1">IP 地址</p>
            <p className="text-sm font-mono font-medium text-slate-800 dark:text-slate-200">
              {networkStatus.ipAddress}
            </p>
          </div>

          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
            <p className="text-xs text-slate-500 dark:text-slate-400 mb-1">MAC 地址</p>
            <p className="text-sm font-mono font-medium text-slate-800 dark:text-slate-200">
              {networkStatus.macAddress}
            </p>
          </div>

          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
            <p className="text-xs text-slate-500 dark:text-slate-400 mb-1">监听端口</p>
            <p className="text-sm font-mono font-medium text-slate-800 dark:text-slate-200">
              {networkStatus.listeningPort}
            </p>
          </div>

          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
            <p className="text-xs text-slate-500 dark:text-slate-400 mb-1">在线用户</p>
            <p className="text-sm font-mono font-medium text-slate-800 dark:text-slate-200">
              {networkStatus.onlineUsers} 人
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
