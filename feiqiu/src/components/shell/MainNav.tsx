import { LucideIcon, MessageSquare, Users, Building2, User } from 'lucide-react'
import { NavTab } from '../../lib/types/shell'
import { cn } from '../../lib/utils'

export interface MainNavProps {
  activeTab: NavTab
  onTabChange: (tab: NavTab) => void
  user?: {
    name: string
    avatar: string
    status: string
  }
  onUserProfile?: () => void
}

const navItems = [
  {
    id: 'chat' as NavTab,
    icon: MessageSquare,
    label: 'Chat'
  },
  {
    id: 'contacts' as NavTab,
    icon: Users,
    label: 'Contacts'
  },
  {
    id: 'organization' as NavTab,
    icon: Building2,
    label: 'Organization'
  }
]

const statusColors: Record<string, string> = {
  online: 'bg-emerald-500',
  away: 'bg-amber-500',
  busy: 'bg-amber-500',
  offline: 'bg-slate-400'
}

export function MainNav({ activeTab, onTabChange, user, onUserProfile }: MainNavProps) {
  return (
    <nav className="w-16 bg-white/80 dark:bg-slate-900/90 border-r border-slate-200/80 dark:border-slate-800/80 flex flex-col shadow-[0_0_15px_-3px_rgba(0,0,0,0.07),0_0_6px_-2px_rgba(0,0,0,0.05)] dark:shadow-[0_0_15px_-3px_rgba(0,0,0,0.4),0_0_6px_-2px_rgba(0,0,0,0.3)] backdrop-blur-sm">
      <div className="flex-1 flex flex-col items-center py-8 gap-3">
        {user ? (
          <button
            onClick={onUserProfile}
            className="w-11 h-11 rounded-xl flex items-center justify-center transition-all duration-300 ease-out group relative bg-gradient-to-br from-emerald-400 to-emerald-600 text-white text-base font-bold ring-2 ring-white dark:ring-slate-900/50"
            title="Profile"
          >
            <span className="flex items-center justify-center w-full h-full">
              {user.avatar || user.name[0]}
            </span>
            <span
              className={cn(
                'absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 border-2 border-white dark:border-slate-900 rounded-full',
                statusColors[user.status] || 'bg-slate-400'
              )}
            />
          </button>
        ) : (
          <button
            onClick={onUserProfile}
            className="w-11 h-11 rounded-xl flex items-center justify-center transition-all duration-300 ease-out group relative text-slate-500 dark:text-slate-400 hover:text-emerald-600 dark:hover:text-emerald-400 hover:bg-slate-100 dark:hover:bg-slate-800/80"
            title="Profile"
          >
            <User className="w-5 h-5 transition-transform duration-300 group-hover:scale-110" />
          </button>
        )}

        {navItems.map((item) => {
          const Icon = item.icon
          const isActive = activeTab === item.id

          return (
            <button
              key={item.id}
              onClick={() => onTabChange(item.id)}
              className={cn(
                'w-11 h-11 rounded-xl flex items-center justify-center transition-all duration-300 ease-out group relative',
                isActive
                  ? 'bg-emerald-500 text-white shadow-lg shadow-emerald-500/30 scale-105 ring-2 ring-emerald-500/20'
                  : 'text-slate-500 dark:text-slate-400 hover:text-emerald-600 dark:hover:text-emerald-400 hover:bg-slate-100 dark:hover:bg-slate-800/80'
              )}
              aria-label={item.label}
              title={item.label}
            >
              <Icon className="w-5 h-5 transition-transform duration-300 group-hover:scale-110" />
              {isActive && (
                <span className="absolute -right-1 w-1.5 h-1.5 bg-emerald-400 rounded-full shadow-[0_0_10px_rgba(52,211,153,0.8)] dark:shadow-[0_0_12px_rgba(52,211,153,0.6)]" />
              )}
            </button>
          )
        })}
      </div>
    </nav>
  )
}
