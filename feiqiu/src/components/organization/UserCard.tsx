'use client'

import { User } from '../../lib/types/organization'
import { MessageSquare, User as UserIcon, Mail, Phone, Building2 } from 'lucide-react'

interface UserCardProps {
  user: User
  onStartChat?: (userId: string) => void
  onViewDetails?: (userId: string) => void
}

const statusConfig: Record<string, { color: string; label: string }> = {
  online: { color: 'bg-emerald-500', label: '在线' },
  away: { color: 'bg-amber-500', label: '离开' },
  offline: { color: 'bg-slate-400', label: '离线' }
}

export function UserCard({ user, onStartChat, onViewDetails }: UserCardProps) {
  const status = statusConfig[user.status]

  return (
    <div className="bg-slate-50 dark:bg-slate-800 rounded-xl p-4 hover:shadow-md transition-shadow">
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

      <div className="space-y-2 mb-4">
        <div className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
          <Building2 className="w-4 h-4 flex-shrink-0" />
          <span className="truncate">{user.department}</span>
        </div>
        <div className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
          <Mail className="w-4 h-4 flex-shrink-0" />
          <span className="truncate">{user.email}</span>
        </div>
        <div className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
          <Phone className="w-4 h-4 flex-shrink-0" />
          <span>{user.phone}</span>
        </div>
      </div>

      <div className="flex items-center gap-2">
        <button
          onClick={() => onStartChat?.(user.id)}
          className="flex-1 px-3 py-2 bg-emerald-500 hover:bg-emerald-600 text-white text-sm font-medium rounded-lg transition-colors flex items-center justify-center gap-1"
        >
          <MessageSquare className="w-4 h-4" />
          聊天
        </button>
        <button
          onClick={() => onViewDetails?.(user.id)}
          className="px-3 py-2 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-slate-700 dark:text-slate-300 text-sm font-medium rounded-lg transition-colors"
        >
          <UserIcon className="w-4 h-4" />
        </button>
      </div>
    </div>
  )
}
