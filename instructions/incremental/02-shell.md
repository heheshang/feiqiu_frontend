# Milestone 2: Application Shell

## Overview

Implement the three-column application shell with navigation and user menu.

---

## Step 1: Create Shell Types

Create `src/lib/types/shell.ts`:

```typescript
export interface NavItem {
  id: string
  label: string
  icon: string
  path: string
}

export interface UserMenuItem {
  id: string
  label: string
  icon: string
  action: () => void
}
```

---

## Step 2: Implement AppShell Component

Create `src/components/shell/AppShell.tsx`:

```typescript
'use client'

import { ReactNode } from 'react'

interface AppShellProps {
  mainNav: ReactNode
  userMenu: ReactNode
  children: ReactNode
}

export function AppShell({ mainNav, userMenu, children }: AppShellProps) {
  return (
    <div className="h-screen flex bg-white dark:bg-slate-900 overflow-hidden">
      {/* Left sidebar - Main navigation */}
      <div className="w-[60px] bg-slate-900 dark:bg-slate-950 flex flex-col items-center py-4 flex-shrink-0">
        {mainNav}
      </div>

      {/* Middle column - Lists */}
      <div className="w-[280px] border-r border-slate-200 dark:border-slate-700 flex flex-col flex-shrink-0">
        {/* User menu header */}
        <div className="p-4 border-b border-slate-200 dark:border-slate-700">
          {userMenu}
        </div>

        <!-- Middle content will be rendered here -->
      </div>

      {/* Right column - Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {children}
      </div>
    </div>
  )
}
```

---

## Step 3: Implement MainNav Component

Create `src/components/shell/MainNav.tsx`:

```typescript
'use client'

import { useState } from 'react'
import { MessageSquare, Users, Building2 } from 'lucide-react'
import { cn } from '@/lib/utils'

type NavTab = 'chat' | 'contacts' | 'organization'

interface MainNavProps {
  activeTab: NavTab
  onTabChange: (tab: NavTab) => void
}

export function MainNav({ activeTab, onTabChange }: MainNavProps) {
  const navItems = [
    { id: 'chat' as NavTab, icon: MessageSquare, label: '聊天' },
    { id: 'contacts' as NavTab, icon: Users, label: '通讯录' },
    { id: 'organization' as NavTab, icon: Building2, label: '组织架构' },
  ]

  return (
    <nav className="flex-1 flex flex-col items-center gap-2 w-full">
      {navItems.map(item => {
        const Icon = item.icon
        const isActive = activeTab === item.id

        return (
          <button
            key={item.id}
            onClick={() => onTabChange(item.id)}
            className={cn(
              'w-10 h-10 rounded-lg flex items-center justify-center transition-all duration-200',
              isActive
                ? 'bg-emerald-500 text-white'
                : 'text-slate-400 hover:text-white hover:bg-slate-800'
            )}
            title={item.label}
          >
            <Icon className="w-5 h-5" />
          </button>
        )
      })}
    </nav>
  )
}
```

---

## Step 4: Implement UserMenu Component

Create `src/components/shell/UserMenu.tsx`:

```typescript
'use client'

import { useState, useRef, useEffect } from 'react'
import { ChevronDown, Settings, Network, LogOut } from 'lucide-react'
import { cn } from '@/lib/utils'

interface UserMenuProps {
  currentUser: {
    name: string
    avatar: string
    status: 'online' | 'away' | 'offline'
  }
  onSettings: () => void
  onNetworkConfig: () => void
  onLogout: () => void
}

export function UserMenu({ currentUser, onSettings, onNetworkConfig, onLogout }: UserMenuProps) {
  const [isOpen, setIsOpen] = useState(false)
  const menuRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsOpen(false)
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => document.removeEventListener('mousedown', handleClickOutside)
  }, [])

  const statusColors = {
    online: 'bg-emerald-500',
    away: 'bg-amber-500',
    offline: 'bg-slate-400'
  }

  const menuItems = [
    { id: 'settings', label: '个人设置', icon: Settings, action: onSettings },
    { id: 'network', label: '网络设置', icon: Network, action: onNetworkConfig },
    { id: 'logout', label: '退出登录', icon: LogOut, action: onLogout },
  ]

  return (
    <div ref={menuRef} className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center gap-3 p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
      >
        <div className="relative flex-shrink-0">
          <img
            src={currentUser.avatar}
            alt={currentUser.name}
            className="w-10 h-10 rounded-full object-cover"
          />
          <span className={cn(
            'absolute bottom-0 right-0 w-2.5 h-2.5 border-2 border-white dark:border-slate-900 rounded-full',
            statusColors[currentUser.status]
          )} />
        </div>
        <div className="flex-1 min-w-0 text-left">
          <p className="text-sm font-medium text-slate-900 dark:text-slate-100 truncate">
            {currentUser.name}
          </p>
        </div>
        <ChevronDown className={cn(
          'w-4 h-4 text-slate-400 transition-transform',
          isOpen && 'rotate-180'
        )} />
      </button>

      {isOpen && (
        <div className="absolute top-full left-0 right-0 mt-1 bg-white dark:bg-slate-800 rounded-lg shadow-lg border border-slate-200 dark:border-slate-700 py-1 z-50">
          {menuItems.map(item => {
            const Icon = item.icon
            return (
              <button
                key={item.id}
                onClick={() => {
                  item.action()
                  setIsOpen(false)
                }}
                className="w-full flex items-center gap-3 px-4 py-2 text-sm text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
              >
                <Icon className="w-4 h-4" />
                {item.label}
              </button>
            )
          })}
        </div>
      )}
    </div>
  )
}
```

---

## Step 5: Create Shell Index

Create `src/components/shell/index.ts`:

```typescript
export { AppShell } from './AppShell'
export { MainNav } from './MainNav'
export { UserMenu } from './UserMenu'
```

---

## Step 6: Integrate Shell

Update your main app file to use the shell:

**For Next.js** (`src/app/page.tsx`):

```typescript
'use client'

import { useState } from 'react'
import { AppShell, MainNav, UserMenu } from '@/components/shell'

type NavTab = 'chat' | 'contacts' | 'organization'

export default function HomePage() {
  const [activeTab, setActiveTab] = useState<NavTab>('chat')

  const currentUser = {
    name: '张伟',
    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=ZhangWei',
    status: 'online' as const
  }

  const handleSettings = () => console.log('Open settings')
  const handleNetworkConfig = () => console.log('Open network config')
  const handleLogout = () => console.log('Logout')

  return (
    <AppShell
      mainNav={
        <MainNav activeTab={activeTab} onTabChange={setActiveTab} />
      }
      userMenu={
        <UserMenu
          currentUser={currentUser}
          onSettings={handleSettings}
          onNetworkConfig={handleNetworkConfig}
          onLogout={handleLogout}
        />
      }
    >
      <div className="flex-1 flex items-center justify-center">
        <p className="text-slate-400">Current tab: {activeTab}</p>
      </div>
    </AppShell>
  )
}
```

---

## Step 7: Add Responsive Behavior

Update AppShell to handle mobile/tablet:

```typescript
'use client'

import { ReactNode } from 'react'
import { useMediaQuery } from '@/hooks/useMediaQuery'

interface AppShellProps {
  mainNav: ReactNode
  userMenu: ReactNode
  children: ReactNode
}

export function AppShell({ mainNav, userMenu, children }: AppShellProps) {
  const isMobile = useMediaQuery('(max-width: 767px)')
  const isTablet = useMediaQuery('(max-width: 1023px)')

  if (isMobile) {
    // Single column layout for mobile
    return <div>{children}</div>
  }

  if (isTablet) {
    // Collapsible nav for tablet
    return (
      <div className="h-screen flex">
        <div className="w-[60px]">{mainNav}</div>
        <div className="flex-1">{children}</div>
      </div>
    )
  }

  // Full three-column layout for desktop
  return (
    <div className="h-screen flex bg-white dark:bg-slate-900">
      <div className="w-[60px] bg-slate-900">{mainNav}</div>
      <div className="w-[280px]">{userMenu}</div>
      <div className="flex-1">{children}</div>
    </div>
  )
}
```

---

## Verification

Test the shell:

1. **Layout**: Three columns display correctly
2. **Navigation**: Clicking nav items updates active tab
3. **User Menu**: Dropdown opens and closes
4. **Dark Mode**: Colors adapt correctly
5. **Responsive**: Test at different screen sizes

---

## Next Steps

Once shell is complete, proceed to **[Milestone 3: Basic Settings](./03-basic-settings.md)**.
