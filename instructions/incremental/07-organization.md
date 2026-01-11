# Milestone 7: Organization Chart Section

## Overview

Implement the Organization Chart section with department tree, colleague list, and search functionality.

---

## Step 1: Create Types

Create `src/lib/types/organization.ts`:

```typescript
export type UserStatus = 'online' | 'away' | 'offline'

export interface Department {
  id: string
  name: string
  parentId: string | null
  memberCount: number
}

export interface User {
  id: string
  name: string
  pinyin: string
  avatar: string
  position: string
  department: string
  departmentId: string
  status: UserStatus
  email: string
  phone: string
}

export interface OrganizationChartProps {
  currentUser: User
  departments: Department[]
  users: User[]
  onDepartmentSelect: (id: string | null) => void
  onStartChat: (userId: string) => void
  onViewDetails: (userId: string) => void
  onSearch: (query: string) => void
}
```

---

## Step 2: Copy Components

Copy the following files from `product-plan/sections/organization/` to `src/components/organization/`:

1. `OrganizationChart.tsx` - Main organization container
2. `DepartmentTree.tsx` - Expandable tree component
3. `UserCard.tsx` - User card with actions
4. Create `index.ts`

---

## Step 3: Create Index File

Create `src/components/organization/index.ts`:

```typescript
export { OrganizationChart } from './OrganizationChart'
export { DepartmentTree } from './DepartmentTree'
export { UserCard } from './UserCard'
```

---

## Step 4: Implement Organization Store

Create `src/stores/organizationStore.ts`:

```typescript
import { create } from 'zustand'
import type { Department, User } from '@/lib/types/organization'

interface OrganizationStore {
  departments: Department[]
  users: User[]
  selectedDepartmentId: string | null
  searchQuery: string
  expandedDepartmentIds: Set<string>

  setDepartments: (departments: Department[]) => void
  setUsers: (users: User[]) => void
  setSelectedDepartment: (id: string | null) => void
  setSearchQuery: (query: string) => void
  toggleDepartmentExpansion: (id: string) => void
  expandAllDepartments: () => void
  collapseAllDepartments: () => void

  getFilteredUsers: () => User[]
  buildDepartmentTree: (parentId: string | null) => Department[]
  getOnlineStats: () => { online: number; away: number; offline: number; total: number }
}

export const useOrganizationStore = create<OrganizationStore>((set, get) => ({
  departments: [],
  users: [],
  selectedDepartmentId: null,
  searchQuery: '',
  expandedDepartmentIds: new Set(),

  setDepartments: (departments) => set({ departments }),
  setUsers: (users) => set({ users }),

  setSelectedDepartment: (id) => set({ selectedDepartmentId: id }),

  setSearchQuery: (query) => set({ searchQuery: query }),

  toggleDepartmentExpansion: (id) => set((state) => {
    const expanded = new Set(state.expandedDepartmentIds)
    if (expanded.has(id)) {
      expanded.delete(id)
    } else {
      expanded.add(id)
    }
    return { expandedDepartmentIds: expanded }
  }),

  expandAllDepartments: () => set((state) => ({
    expandedDepartmentIds: new Set(state.departments.map(d => d.id))
  })),

  collapseAllDepartments: () => set({ expandedDepartmentIds: new Set() }),

  getFilteredUsers: () => {
    const { users, searchQuery, selectedDepartmentId } = get()
    let filtered = users

    // Filter by search query
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase()
      filtered = filtered.filter(user =>
        user.name.toLowerCase().includes(query) ||
        user.pinyin.toLowerCase().includes(query) ||
        user.department.toLowerCase().includes(query) ||
        user.position.toLowerCase().includes(query)
      )
    }

    // Filter by selected department
    if (selectedDepartmentId) {
      filtered = filtered.filter(user => user.departmentId === selectedDepartmentId)
    }

    return filtered
  },

  buildDepartmentTree: (parentId) => {
    const { departments } = get()
    return departments
      .filter(dept => dept.parentId === parentId)
      .sort((a, b) => a.name.localeCompare(b.name, 'zh-CN'))
  },

  getOnlineStats: () => {
    const { users } = get()
    return {
      online: users.filter(u => u.status === 'online').length,
      away: users.filter(u => u.status === 'away').length,
      offline: users.filter(u => u.status === 'offline').length,
      total: users.length
    }
  }
}))
```

---

## Step 5: Implement Search Hook

Create `src/hooks/useUserSearch.ts`:

```typescript
import { useMemo } from 'react'
import type { User } from '@/lib/types/organization'

export function useUserSearch(users: User[], query: string) {
  return useMemo(() => {
    if (!query.trim()) return users

    const q = query.toLowerCase()
    return users.filter(user =>
      user.name.toLowerCase().includes(q) ||
      user.pinyin.toLowerCase().includes(q) ||
      user.department.toLowerCase().includes(q) ||
      user.position.toLowerCase().includes(q)
    )
  }, [users, query])
}
```

---

## Step 6: Integrate Organization Chart

```typescript
'use client'

import { OrganizationChart } from '@/components/organization'
import { useOrganizationStore } from '@/stores/organizationStore'

// Sample data
const sampleDepartments = [
  { id: 'dept-root', name: '公司', parentId: null, memberCount: 20 },
  { id: 'dept-tech', name: '技术部', parentId: 'dept-root', memberCount: 12 },
  { id: 'dept-tech-frontend', name: '前端组', parentId: 'dept-tech', memberCount: 5 },
  { id: 'dept-tech-backend', name: '后端组', parentId: 'dept-tech', memberCount: 7 },
  // ... more departments
]

const sampleUsers = [
  {
    id: 'user-1',
    name: '张伟',
    pinyin: 'zhangwei',
    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=ZhangWei',
    position: '前端工程师',
    department: '前端组',
    departmentId: 'dept-tech-frontend',
    status: 'online' as const,
    email: 'zhangwei@company.com',
    phone: '138-0001-0001'
  },
  // ... more users
]

function OrganizationPage() {
  const { setDepartments, setUsers } = useOrganizationStore()

  // Initialize store with data
  useState(() => {
    setDepartments(sampleDepartments)
    setUsers(sampleUsers)
  })

  const currentUser = sampleUsers[0]

  return (
    <OrganizationChart
      currentUser={currentUser}
      departments={sampleDepartments}
      users={sampleUsers}
      onDepartmentSelect={(id) => console.log('Select department:', id)}
      onStartChat={(userId) => console.log('Start chat:', userId)}
      onViewDetails={(userId) => console.log('View details:', userId)}
      onSearch={(query) => console.log('Search:', query)}
    />
  )
}
```

---

## Step 7: Add Department Tree Helper

Create `src/lib/departmentTree.ts`:

```typescript
import type { Department } from '@/lib/types/organization'

export interface TreeNode extends Department {
  children: TreeNode[]
}

export function buildDepartmentTree(
  departments: Department[],
  parentId: string | null = null
): TreeNode[] {
  return departments
    .filter(dept => dept.parentId === parentId)
    .sort((a, b) => a.name.localeCompare(b.name, 'zh-CN'))
    .map(dept => ({
      ...dept,
      children: buildDepartmentTree(departments, dept.id)
    }))
}

export function flattenTree(tree: TreeNode[]): Department[] {
  const result: Department[] = []

  function traverse(nodes: TreeNode[]) {
    for (const node of nodes) {
      result.push(node)
      if (node.children.length > 0) {
        traverse(node.children)
      }
    }
  }

  traverse(tree)
  return result
}

export function findDepartmentPath(
  departments: Department[],
  targetId: string
): Department[] {
  const path: Department[] = []
  let current = departments.find(d => d.id === targetId)

  while (current) {
    path.unshift(current)
    current = departments.find(d => d.id === current!.parentId)
  }

  return path
}
```

---

## Step 8: Add User Status Indicators

Create a reusable status indicator component:

```typescript
// src/components/organization/StatusIndicator.tsx
'use client'

import type { UserStatus } from '@/lib/types/organization'
import { cn } from '@/lib/utils'

interface StatusIndicatorProps {
  status: UserStatus
  size?: 'sm' | 'md' | 'lg'
  showLabel?: boolean
}

export function StatusIndicator({ status, size = 'md', showLabel = false }: StatusIndicatorProps) {
  const statusConfig = {
    online: { color: 'bg-emerald-500', label: '在线' },
    away: { color: 'bg-amber-500', label: '离开' },
    offline: { color: 'bg-slate-400', label: '离线' }
  }

  const config = statusConfig[status]
  const sizeClasses = {
    sm: 'w-2 h-2',
    md: 'w-2.5 h-2.5',
    lg: 'w-3 h-3'
  }

  return (
    <div className={cn('flex items-center gap-1.5')}>
      <span className={cn('rounded-full', sizeClasses[size], config.color)} />
      {showLabel && (
        <span className="text-xs text-slate-500">{config.label}</span>
      )}
    </div>
  )
}
```

---

## Verification

Test the Organization Chart section:

1. **Department Tree**:
   - [ ] All departments display hierarchically
   - [ ] Expand/collapse works
   - [ ] Selection highlighting works
   - [ ] Member counts accurate

2. **User List**:
   - [ ] All users display correctly
   - [ ] Filtering by department works
   - [ ] Status indicators show
   - [ ] User cards display properly

3. **Search**:
   - [ ] Search by name works
   - [ ] Search by pinyin works
   - [ ] Search by department works
   - [ ] Search by position works
   - [ ] Clear results show all users

4. **User Actions**:
   - [ ] Start chat works
   - [ ] View details works
   - [ ] Status displays correctly

5. **Statistics**:
   - [ ] Online count accurate
   - [ ] Away count accurate
   - [ ] Offline count accurate
   - [ ] Total count accurate

---

## Completion Checklist

After completing this milestone, you should have:

- [ ] All 7 milestones implemented
- [ ] Full navigation between sections working
- [ ] Dark mode support throughout
- [ ] Responsive design verified
- [ ] All components props-based and portable
- [ ] Sample data integrated
- [ ] Basic error handling in place

---

## Post-Implementation Tasks

1. **Testing**: Comprehensive testing of all features
2. **Performance**: Optimize for large data sets
3. **Accessibility**: Verify keyboard navigation and screen readers
4. **Documentation**: Add inline code documentation
5. **Deployment**: Prepare for production deployment

---

Congratulations! You have completed the implementation of 飞秋 (FeiQiu).
