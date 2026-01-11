'use client'

import type { User } from '@/../product/sections/organization/types'

interface UserCardProps {
  user: User
  onStartChat?: (userId: string) => void
  onViewDetails?: (userId: string) => void
}

export function UserCard({ user, onStartChat, onViewDetails }: UserCardProps) {
  const statusConfig = {
    online: { color: 'bg-emerald-500', label: '在线' },
    away: { color: 'bg-amber-500', label: '离开' },
    offline: { color: 'bg-slate-400', label: '离线' }
  }

  const status = statusConfig[user.status]

  return (
    <div className="bg-slate-50 dark:bg-slate-800 rounded-xl p-4 hover:shadow-md transition-shadow">
      {/* Header with avatar and status */}
      <div className="flex items-start gap-3 mb-3">
        <div className="relative flex-shrink-0">
          <img
            src={user.avatar}
            alt={user.name}
            className="w-12 h-12 rounded-full object-cover"
          />
          <span className={`absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 ${status.color} border-2 border-white dark:border-slate-800 rounded-full`} />
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="font-medium text-slate-900 dark:text-slate-100 truncate">{user.name}</h3>
          <p className="text-sm text-slate-500 dark:text-slate-400 truncate">{user.position}</p>
        </div>
      </div>

      {/* Department info */}
      <div className="space-y-2 mb-4">
        <div className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
          <svg className="w-4 h-4 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
          </svg>
          <span className="truncate">{user.department}</span>
        </div>
        <div className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
          <svg className="w-4 h-4 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
          <span className="truncate">{user.email}</span>
        </div>
        <div className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
          <svg className="w-4 h-4 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 8V5z" />
          </svg>
          <span>{user.phone}</span>
        </div>
      </div>

      {/* Action buttons */}
      <div className="flex items-center gap-2">
        <button
          onClick={() => onStartChat?.(user.id)}
          className="flex-1 px-3 py-2 bg-emerald-500 hover:bg-emerald-600 text-white text-sm font-medium rounded-lg transition-colors flex items-center justify-center gap-1"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
          </svg>
          聊天
        </button>
        <button
          onClick={() => onViewDetails?.(user.id)}
          className="px-3 py-2 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-slate-700 dark:text-slate-300 text-sm font-medium rounded-lg transition-colors"
        >
          详情
        </button>
      </div>
    </div>
  )
}
