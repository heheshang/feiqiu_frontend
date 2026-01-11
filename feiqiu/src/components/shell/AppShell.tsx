import { ReactNode } from 'react'
import { useMediaQuery } from '../../hooks/useMediaQuery'
import { MainNav, MainNavProps } from './MainNav'
import { UserMenu, UserMenuProps } from './UserMenu'
import { NavTab } from '../../lib/types/shell'

interface AppShellProps {
  mainNav: Omit<MainNavProps, 'user' | 'onUserProfile'> & {
    user?: {
      name: string
      avatar: string
      status: string
    }
    onUserProfile?: () => void
  }
  userMenu?: UserMenuProps
  children: ReactNode
}

export function AppShell({ mainNav, userMenu, children }: AppShellProps) {
  const isMobile = useMediaQuery('(max-width: 768px)')
  const isTablet = useMediaQuery('(max-width: 1024px)')

  if (isMobile) {
    return (
      <div className="min-h-screen bg-slate-100/80 dark:bg-slate-950">
        <div className="flex flex-col h-screen">
          <div className="flex items-center justify-between px-5 py-4 border-b border-slate-200/90 dark:border-slate-800/90 bg-white/95 dark:bg-slate-900/95 backdrop-blur-md shadow-[0_1px_3px_0_rgba(0,0,0,0.06),0_1px_2px_0_rgba(0,0,0,0.04)] dark:shadow-[0_1px_3px_0_rgba(0,0,0,0.3),0_1px_2px_0_rgba(0,0,0,0.2)]">
            <h1 className="text-xl font-extrabold bg-gradient-to-r from-emerald-600 via-emerald-500 to-emerald-400 bg-clip-text text-transparent tracking-tight">飞秋</h1>
            <div className="w-12">
              {userMenu && <UserMenu {...userMenu} />}
            </div>
          </div>
          <div className="flex border-b border-slate-200/80 dark:border-slate-800/80 bg-white/60 dark:bg-slate-900/60 px-3 py-2">
            <div className="flex flex-row items-center gap-1.5 w-full justify-around">
              {(['chat', 'contacts', 'organization'] as NavTab[]).map((tab) => (
                <button
                  key={tab}
                  onClick={() => mainNav.onTabChange(tab)}
                  className={`px-5 py-2.5 rounded-xl text-sm font-semibold transition-all duration-300 ${
                    mainNav.activeTab === tab
                      ? 'bg-emerald-500 text-white shadow-lg shadow-emerald-500/30 scale-105'
                      : 'text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-200/70 dark:hover:bg-slate-800/70'
                  }`}
                >
                  {tab.charAt(0).toUpperCase() + tab.slice(1)}
                </button>
              ))}
            </div>
          </div>
          <div className="flex-1 overflow-auto">{children}</div>
        </div>
      </div>
    )
  }

  if (isTablet) {
    return (
      <div className="min-h-screen bg-slate-100/80 dark:bg-slate-950">
        <div className="flex h-screen">
          <MainNav {...mainNav as MainNavProps} />
          <main className="flex-1 overflow-auto">
            {children}
          </main>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-slate-100/80 dark:bg-slate-950">
      <div className="flex h-screen">
        <MainNav {...mainNav as MainNavProps} />
        <main className="flex-1 overflow-auto">
          {children}
        </main>
      </div>
    </div>
  )
}
