# Milestone 3: Basic Settings Section

## Overview

Implement the Basic Settings section with personal info management and network configuration.

---

## Step 1: Create Types

Create `src/lib/types/basic-settings.ts`:

```typescript
export type UserStatus = 'online' | 'away' | 'busy' | 'offline'

export interface User {
  id: string
  name: string
  pinyin: string
  avatar: string
  signature: string
  status: UserStatus
  email: string
  phone: string
  department: string
}

export interface NetworkConfig {
  udpPort: number
  listenAddress: string
  autoDiscovery: boolean
  maxConnections: number
}

export interface NetworkStatus {
  isConnected: boolean
  currentPort: number
  connectedPeers: number
  latency: number
}

export interface BasicSettingsProps {
  currentUser: User
  networkConfig: NetworkConfig
  networkStatus: NetworkStatus
  onUserUpdate: (user: Partial<User>) => void
  onNetworkConfigChange: (config: Partial<NetworkConfig>) => void
  onTestConnection: () => void
}
```

---

## Step 2: Copy Components

Copy the following files from `product-plan/sections/basic-settings/` to `src/components/basic-settings/`:

1. `BasicSettings.tsx` - Main settings container
2. `NetworkStatusCard.tsx` - Network status display
3. Create `index.ts` to export components

---

## Step 3: Create Index File

Create `src/components/basic-settings/index.ts`:

```typescript
export { BasicSettings } from './BasicSettings'
export { NetworkStatusCard } from './NetworkStatusCard'
```

---

## Step 4: Implement Form State Management

Create `src/hooks/useBasicSettings.ts`:

```typescript
import { useState, useCallback } from 'react'
import type { User, NetworkConfig } from '@/lib/types/basic-settings'

export function useBasicSettings(
  initialUser: User,
  initialNetworkConfig: NetworkConfig,
  onUserUpdate: (user: Partial<User>) => void,
  onNetworkConfigChange: (config: Partial<NetworkConfig>) => void
) {
  const [activeTab, setActiveTab] = useState<'personal' | 'network'>('personal')
  const [isEditing, setIsEditing] = useState(false)

  const [userForm, setUserForm] = useState<User>(initialUser)
  const [networkForm, setNetworkForm] = useState(initialNetworkConfig)

  const handleUserFieldChange = useCallback((field: keyof User, value: string) => {
    setUserForm(prev => ({ ...prev, [field]: value }))
  }, [])

  const handleNetworkFieldChange = useCallback((field: keyof NetworkConfig, value: any) => {
    setNetworkForm(prev => ({ ...prev, [field]: value }))
  }, [])

  const handleSaveUser = useCallback(() => {
    onUserUpdate(userForm)
    setIsEditing(false)
  }, [userForm, onUserUpdate])

  const handleSaveNetwork = useCallback(() => {
    onNetworkConfigChange(networkForm)
  }, [networkForm, onNetworkConfigChange])

  const handleCancelEdit = useCallback(() => {
    setUserForm(initialUser)
    setIsEditing(false)
  }, [initialUser])

  return {
    activeTab,
    setActiveTab,
    isEditing,
    setIsEditing,
    userForm,
    networkForm,
    handleUserFieldChange,
    handleNetworkFieldChange,
    handleSaveUser,
    handleSaveNetwork,
    handleCancelEdit
  }
}
```

---

## Step 5: Add to Main Application

Update your main app to include the Basic Settings view:

```typescript
'use client'

import { useState } from 'react'
import { AppShell, MainNav, UserMenu } from '@/components/shell'
import { BasicSettings } from '@/components/basic-settings'
import type { NavTab } from '@/lib/types/shell'

// Sample data
const sampleUser = {
  id: 'user-1',
  name: '张伟',
  pinyin: 'zhangwei',
  avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=ZhangWei',
  signature: '努力工作，认真生活',
  status: 'online' as const,
  email: 'zhangwei@company.com',
  phone: '138-0001-0001',
  department: '前端组'
}

const sampleNetworkConfig = {
  udpPort: 2425,
  listenAddress: '0.0.0.0',
  autoDiscovery: true,
  maxConnections: 100
}

const sampleNetworkStatus = {
  isConnected: true,
  currentPort: 2425,
  connectedPeers: 15,
  latency: 12
}

export default function HomePage() {
  const [activeTab, setActiveTab] = useState<NavTab>('chat')
  const [showSettings, setShowSettings] = useState(false)

  const renderContent = () => {
    if (showSettings) {
      return (
        <BasicSettings
          currentUser={sampleUser}
          networkConfig={sampleNetworkConfig}
          networkStatus={sampleNetworkStatus}
          onUserUpdate={(data) => console.log('Update user:', data)}
          onNetworkConfigChange={(config) => console.log('Update network:', config)}
          onTestConnection={() => console.log('Test connection')}
        />
      )
    }

    // Other views will be rendered here
    return (
      <div className="flex-1 flex items-center justify-center">
        <p className="text-slate-400">Content for: {activeTab}</p>
      </div>
    )
  }

  return (
    <AppShell
      mainNav={<MainNav activeTab={activeTab} onTabChange={setActiveTab} />}
      userMenu={
        <UserMenu
          currentUser={sampleUser}
          onSettings={() => setShowSettings(true)}
          onNetworkConfig={() => setShowSettings(true)}
          onLogout={() => console.log('Logout')}
        />
      }
    >
      {renderContent()}
    </AppShell>
  )
}
```

---

## Step 6: Add Form Validation

Add validation to the settings form:

```typescript
function validateEmail(email: string): boolean {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
}

function validatePhone(phone: string): boolean {
  return /^[\d\-\+\s]+$/.test(phone)
}

function validatePort(port: number): boolean {
  return port >= 1024 && port <= 65535
}
```

---

## Verification

Test the Basic Settings section:

1. **Personal Info Tab**:
   - [ ] All fields display correctly
   - [ ] Edit mode toggles
   - [ ] Form validation works
   - [ ] Save button calls callback
   - [ ] Cancel reverts changes

2. **Network Tab**:
   - [ ] Configuration displays
   - [ ] Port validation works
   - [ ] Test connection button works
   - [ ] Status card shows current state

3. **UI/UX**:
   - [ ] Dark mode works correctly
   - [ ] Responsive on mobile
   - [ ] Transitions are smooth

---

## Next Steps

Once Basic Settings is complete, proceed to **[Milestone 4: Messaging](./04-messaging.md)**.
