import { User, Settings, LogOut, MoreVertical } from 'lucide-react'
import { useState } from 'react'

export interface UserMenuProps {
  user: {
    name: string
    avatarUrl?: string
    status?: 'online' | 'away' | 'busy' | 'offline'
  }
  onSettings?: () => void
  onLogout?: () => void
}

const statusColors: Record<string, string> = {
  online: 'bg-emerald-500',
  away: 'bg-amber-500',
  busy: 'bg-red-500',
  offline: 'bg-slate-400',
}

export function UserMenu({ user, onSettings, onLogout }: UserMenuProps) {
  const [isOpen, setIsOpen] = useState(false)

  return (
    <div className="relative">
      {/* User Avatar Button */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 px-3 py-1.5 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
      >
        {/* Avatar with Status Indicator */}
        <div className="relative">
          {user.avatarUrl ? (
            <img
              src={user.avatarUrl}
              alt={user.name}
              className="w-8 h-8 rounded-full object-cover"
            />
          ) : (
            <div className="w-8 h-8 rounded-full bg-emerald-500 dark:bg-emerald-600 flex items-center justify-center">
              <User className="w-5 h-5 text-white" />
            </div>
          )}
          {user.status && (
            <span
              className={`absolute bottom-0 right-0 w-2.5 h-2.5 ${statusColors[user.status]} border-2 border-white dark:border-slate-800 rounded-full`}
            />
          )}
        </div>

        {/* User Name */}
        <span className="text-sm font-medium text-slate-700 dark:text-slate-300 hidden sm:block">
          {user.name}
        </span>

        {/* Dropdown Icon */}
        <MoreVertical className="w-4 h-4 text-slate-500 dark:text-slate-400" />
      </button>

      {/* Dropdown Menu */}
      {isOpen && (
        <>
          <div
            className="fixed inset-0 z-10"
            onClick={() => setIsOpen(false)}
          />
          <div className="absolute right-0 top-full mt-1 w-48 bg-white dark:bg-slate-800 rounded-lg shadow-lg border border-slate-200 dark:border-slate-700 py-1 z-20">
            {/* User Info Header */}
            <div className="px-4 py-2 border-b border-slate-200 dark:border-slate-700">
              <p className="text-sm font-medium text-slate-800 dark:text-slate-200">
                {user.name}
              </p>
              <p className="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
                {user.status === 'online' && '在线'}
                {user.status === 'away' && '离开'}
                {user.status === 'busy' && '忙碌'}
                {user.status === 'offline' && '离线'}
              </p>
            </div>

            {/* Menu Items */}
            <button
              onClick={() => {
                onSettings?.()
                setIsOpen(false)
              }}
              className="w-full px-4 py-2 text-left text-sm text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 flex items-center gap-2"
            >
              <Settings className="w-4 h-4" />
              个人设置
            </button>

            <button
              onClick={() => {
                onLogout?.()
                setIsOpen(false)
              }}
              className="w-full px-4 py-2 text-left text-sm text-red-600 dark:text-red-400 hover:bg-slate-100 dark:hover:bg-slate-700 flex items-center gap-2"
            >
              <LogOut className="w-4 h-4" />
              退出登录
            </button>
          </div>
        </>
      )}
    </div>
  )
}
