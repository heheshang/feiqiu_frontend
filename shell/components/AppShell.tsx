import { MainNav } from './MainNav'
import { UserMenu } from './UserMenu'

export interface AppShellProps {
  children: React.ReactNode
  navigationItems: Array<{
    id: string
    label: string
    icon: string
    href: string
    isActive?: boolean
  }>
  currentUser?: {
    name: string
    avatarUrl?: string
    status?: 'online' | 'away' | 'busy' | 'offline'
  }
  activeNav?: string
  onNavigate?: (id: string) => void
  onLogout?: () => void
  onSettings?: () => void
}

export function AppShell({
  children,
  navigationItems,
  currentUser,
  activeNav,
  onNavigate,
  onLogout,
  onSettings,
}: AppShellProps) {
  return (
    <div className="flex h-screen bg-slate-50 dark:bg-slate-900">
      {/* Left Sidebar - Main Navigation (60px) */}
      <aside className="w-15 bg-slate-100 dark:bg-slate-800 flex flex-col items-center py-4 border-r border-slate-200 dark:border-slate-700">
        <MainNav
          items={navigationItems}
          activeId={activeNav}
          onSelect={onNavigate}
        />
      </aside>

      {/* Middle Column - List Area (280px) */}
      <div className="w-70 bg-white dark:bg-slate-800 border-r border-slate-200 dark:border-slate-700">
        {/* Middle column content will be injected here */}
        <div className="p-4">
          <h2 className="text-lg font-semibold text-slate-800 dark:text-slate-200">
            {navigationItems.find((item) => item.id === activeNav)?.label || '列表'}
          </h2>
        </div>
      </div>

      {/* Right Column - Content Area (flex-1) */}
      <div className="flex-1 flex flex-col">
        {/* Header with User Menu */}
        <header className="h-14 bg-white dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between px-4">
          <h1 className="text-lg font-medium text-slate-800 dark:text-slate-200">
            飞秋
          </h1>
          {currentUser && (
            <UserMenu
              user={currentUser}
              onSettings={onSettings}
              onLogout={onLogout}
            />
          )}
        </header>

        {/* Main Content Area */}
        <main className="flex-1 overflow-auto">
          {children}
        </main>
      </div>
    </div>
  )
}
