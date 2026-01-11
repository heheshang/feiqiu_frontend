import { useState, useRef, useEffect } from 'react'
import { User, UserMenuItem } from '../../lib/types/shell'
import { Settings, Network, LogOut, ChevronDown } from 'lucide-react'
import { cn } from '../../lib/utils'

export interface UserMenuProps {
  user: User
  menuItems: UserMenuItem[]
}

const statusColors: Record<string, string> = {
  online: 'bg-emerald-500 ring-2 ring-emerald-500/30',
  away: 'bg-amber-500 ring-2 ring-amber-500/30',
  busy: 'bg-amber-500 ring-2 ring-amber-500/30',
  offline: 'bg-slate-400 ring-2 ring-slate-400/30'
}

const iconMap: Record<string, React.ElementType> = {
  settings: Settings,
  network: Network,
  logout: LogOut
}

export function UserMenu({ user, menuItems }: UserMenuProps) {
  const [isOpen, setIsOpen] = useState(false)
  const menuRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsOpen(false)
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => document.removeEventListener('mousedown', handleClickOutside)
  }, [])

  return (
    <div className="relative" ref={menuRef}>
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center justify-center p-2.5 hover:bg-slate-100 dark:hover:bg-slate-800/80 transition-all duration-300 ease-out rounded-xl"
      >
        <div className="relative flex-shrink-0">
          <div className="w-11 h-11 rounded-xl bg-gradient-to-br from-emerald-400 to-emerald-600 flex items-center justify-center text-white text-base font-bold ring-2 ring-white dark:ring-slate-900/50">
            {user.avatar || user.name[0]}
          </div>
          <span
            className={cn(
              'absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 border-2 border-white dark:border-slate-900 rounded-full',
              statusColors[user.status]
            )}
          />
        </div>
      </button>

      {isOpen && (
        <div className="absolute bottom-full left-0 mb-2 w-56 bg-white/95 dark:bg-slate-800/95 backdrop-blur-md border border-slate-200/90 dark:border-slate-700/90 rounded-2xl overflow-hidden z-50 animate-in fade-in slide-in-from-bottom-2 duration-200">
          <div className="p-2">
            <div className="px-3 py-2 mb-1">
              <p className="text-sm font-semibold text-slate-800 dark:text-slate-100">{user.name}</p>
              <p className="text-xs text-slate-500 dark:text-slate-400 capitalize mt-0.5">{user.status}</p>
            </div>
            <div className="border-t border-slate-200/70 dark:border-slate-700/70 my-1" />
            {menuItems.map((item) => {
              const Icon = iconMap[item.id] || Settings
              return (
                <button
                  key={item.id}
                  onClick={() => {
                    item.action()
                    setIsOpen(false)
                  }}
                  className="w-full flex items-center gap-3 px-3 py-2.5 text-sm text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700/80 hover:text-emerald-600 dark:hover:text-emerald-400 rounded-xl transition-all duration-200 font-medium"
                >
                  <Icon className="w-4 h-4 text-slate-400 dark:text-slate-500" />
                  <span>{item.label}</span>
                </button>
              )
            })}
          </div>
        </div>
      )}
    </div>
  )
}
