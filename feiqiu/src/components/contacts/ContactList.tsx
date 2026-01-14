/**
 * ContactList Component
 *
 * Main list view for displaying contacts with support for multiple view modes
 * (list, grid, groups, departments).
 */

import { useState, useCallback, useMemo, useEffect } from 'react'
import { ContactItem } from './ContactItem'
import { ContactFilters as ContactFiltersComponent } from './ContactFilters'
import { Contact, ContactGroup, ContactView, ContactSortBy, ContactFilters } from '@/lib/types/contacts'
import { cn } from '@/lib/utils'

interface ContactListProps {
  /** All contacts */
  contacts: Contact[]
  /** All groups */
  groups: ContactGroup[]
  /** Whether data is loading */
  isLoading: boolean
  /** Error state */
  error: Error | null
  /** Contact click handler */
  onContactClick?: (contact: Contact) => void
  /** Favorite toggle handler */
  onToggleFavorite?: (contactId: number) => void
  /** Delete contact handler */
  onDeleteContact?: (contactId: number) => void
  /** Create contact handler */
  onCreateContact?: () => void
  /** Batch delete handler */
  onBatchDelete?: (contactIds: number[]) => Promise<void>
  /** Batch move to group handler */
  onBatchMoveToGroup?: (contactIds: number[], groupId: number) => Promise<void>
  /** Batch export handler */
  onBatchExport?: (contactIds: number[]) => void
  /** Additional CSS classes */
  className?: string
}

/**
 * Empty state component
 */
function EmptyState({
  type,
  onClearFilters,
  onCreateContact
}: {
  type: 'no_contacts' | 'no_results' | 'error'
  onClearFilters?: () => void
  onCreateContact?: () => void
}) {
  const content = {
    no_contacts: {
      icon: '👥',
      title: '暂无联系人',
      description: '开始添加联系人来管理您的网络',
    },
    no_results: {
      icon: '🔍',
      title: '未找到匹配的联系人',
      description: '尝试调整搜索条件或筛选器',
    },
    error: {
      icon: '⚠️',
      title: '加载失败',
      description: '无法加载联系人数据',
    },
  }[type]

  return (
    <div className="flex flex-col items-center justify-center h-full px-8 py-16 text-center">
      <div className="w-20 h-20 mb-4 text-6xl flex items-center justify-center bg-slate-100 dark:bg-slate-800 rounded-full">
        {content.icon}
      </div>
      <h3 className="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
        {content.title}
      </h3>
      <p className="text-sm text-slate-500 dark:text-slate-400 mb-4 max-w-md">
        {content.description}
      </p>
      {type === 'no_results' && onClearFilters && (
        <button
          onClick={onClearFilters}
          className="px-4 py-2 text-sm font-medium text-emerald-700 dark:text-emerald-400 bg-emerald-100 dark:bg-emerald-900/20 rounded-lg hover:bg-emerald-200 dark:hover:bg-emerald-900/30 transition-colors"
        >
          清除筛选条件
        </button>
      )}
      {type === 'no_contacts' && onCreateContact && (
        <button
          onClick={onCreateContact}
          className="px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 transition-colors"
        >
          添加第一个联系人
        </button>
      )}
    </div>
  )
}

/**
 * Loading skeleton
 */
function LoadingSkeleton() {
  return (
    <div className="divide-y divide-slate-200 dark:divide-slate-800">
      {Array.from({ length: 8 }).map((_, i) => (
        <div key={i} className="flex items-center gap-3 px-4 py-3.5 animate-pulse">
          <div className="w-12 h-12 bg-slate-200 dark:bg-slate-700 rounded-xl flex-shrink-0" />
          <div className="flex-1 min-w-0">
            <div className="h-4 bg-slate-200 dark:bg-slate-700 rounded w-1/3 mb-2" />
            <div className="h-3 bg-slate-200 dark:bg-slate-700 rounded w-1/2" />
          </div>
          <div className="h-3 bg-slate-200 dark:bg-slate-700 rounded w-16 flex-shrink-0" />
        </div>
      ))}
    </div>
  )
}

/**
 * List view - standard vertical list
 */
function ListView({
  contacts,
  selectedIds,
  showCheckbox,
  onContactClick,
  onToggleSelect,
  onToggleFavorite,
}: {
  contacts: Contact[]
  selectedIds: Set<number>
  showCheckbox: boolean
  onContactClick?: (contact: Contact) => void
  onToggleSelect?: (id: number) => void
  onToggleFavorite?: (id: number) => void
}) {
  if (contacts.length === 0) {
    return <EmptyState type="no_results" />
  }

  return (
    <div role="list" className="divide-y divide-slate-200 dark:divide-slate-800" aria-label="联系人列表">
      {contacts.map((contact, index) => (
        <div
          key={contact.id}
          style={{ animation: `fadeIn 0.3s ease-out ${index * 0.05}s both` }}
        >
          <ContactItem
            contact={contact}
            isSelected={selectedIds.has(contact.id)}
            showCheckbox={showCheckbox}
            onClick={() => onContactClick?.(contact)}
            onToggleSelect={() => onToggleSelect?.(contact.id)}
            onToggleFavorite={() => onToggleFavorite?.(contact.id)}
          />
        </div>
      ))}
    </div>
  )
}

/**
 * Grid view - card-based layout
 */
function GridView({
  contacts,
  selectedIds,
  showCheckbox,
  onContactClick,
  onToggleSelect,
  onToggleFavorite,
}: {
  contacts: Contact[]
  selectedIds: Set<number>
  showCheckbox: boolean
  onContactClick?: (contact: Contact) => void
  onToggleSelect?: (id: number) => void
  onToggleFavorite?: (id: number) => void
}) {
  if (contacts.length === 0) {
    return <EmptyState type="no_results" />
  }

  return (
    <div role="list" className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 p-4" aria-label="联系人网格视图">
      {contacts.map((contact, index) => (
        <div
          key={contact.id}
          onClick={() => onContactClick?.(contact)}
          style={{ animation: `scaleIn 0.3s ease-out ${index * 0.03}s both` }}
          className={cn(
            'relative bg-white dark:bg-slate-800 rounded-xl p-4 shadow-sm hover:shadow-md transition-all duration-200 cursor-pointer border-2',
            selectedIds.has(contact.id)
              ? 'border-emerald-500 dark:border-emerald-400 shadow-lg shadow-emerald-500/10'
              : 'border-transparent hover:border-slate-300 dark:hover:border-slate-600'
          )}
          role="listitem"
          aria-label={`${contact.nickname || contact.name}, ${contact.isOnline ? '在线' : '离线'}`}
        >
          {showCheckbox && (
            <div
              onClick={(e) => {
                e.stopPropagation()
                onToggleSelect?.(contact.id)
              }}
              className="absolute top-3 right-3 w-5 h-5 rounded border-2 flex items-center justify-center transition-colors z-10"
              role="checkbox"
              aria-checked={selectedIds.has(contact.id)}
              tabIndex={0}
              onKeyDown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault()
                  onToggleSelect?.(contact.id)
                }
              }}
            >
              {selectedIds.has(contact.id) && (
                <svg
                  className="w-3 h-3 text-emerald-600 dark:text-emerald-400"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                  aria-hidden="true"
                >
                  <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                </svg>
              )}
            </div>
          )}

          <div className="flex flex-col items-center text-center">
            <div className="relative mb-3">
              {contact.avatar ? (
                <img
                  src={contact.avatar}
                  alt={`${contact.name} 的头像`}
                  className="w-20 h-20 rounded-full object-cover ring-2 ring-white dark:ring-slate-700 shadow-sm"
                />
              ) : (
                <div className="w-20 h-20 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center text-xl font-bold text-slate-600 dark:text-slate-300" aria-hidden="true">
                  {contact.name.slice(0, 2).toUpperCase()}
                </div>
              )}
              <span
                className={cn(
                  'absolute bottom-0 right-0 w-4 h-4 border-2 border-white dark:border-slate-800 rounded-full',
                  contact.isOnline
                    ? 'bg-emerald-500'
                    : 'bg-slate-400'
                )}
                aria-hidden="true"
              />
            </div>

            <h3 className="font-semibold text-sm text-slate-900 dark:text-slate-100 mb-1">
              {contact.nickname || contact.name}
            </h3>

            {contact.department && (
              <p className="text-xs text-slate-500 dark:text-slate-400 mb-2">
                {contact.department}
                {contact.position && ` · ${contact.position}`}
              </p>
            )}

            <div className="flex items-center gap-2">
              {contact.isOnline && (
                <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-emerald-100 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400">
                  在线
                </span>
              )}

              {contact.isFavorite && (
                <button
                  onClick={(e) => {
                    e.stopPropagation()
                    onToggleFavorite?.(contact.id)
                  }}
                  className="text-yellow-500 hover:text-yellow-600 dark:hover:text-yellow-400 transition-colors"
                  aria-label={`收藏 ${contact.name}`}
                  aria-pressed={true}
                  type="button"
                >
                  <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
                    <path d="M8 2L6.5 5.5L3 6L5.5 8.5L5 12L8 10L11 12L10.5 8.5L13 6L9.5 5.5L8 2Z" />
                  </svg>
                </button>
              )}
            </div>
          </div>
        </div>
      ))}
    </div>
  )
}

/**
 * Groups view - contacts organized by groups
 */
function GroupsView({
  contacts,
  groups,
  selectedIds,
  showCheckbox,
  expandedGroups,
  onToggleGroup,
  onContactClick,
  onToggleSelect,
  onToggleFavorite,
}: {
  contacts: Contact[]
  groups: ContactGroup[]
  selectedIds: Set<number>
  showCheckbox: boolean
  expandedGroups: Set<number>
  onToggleGroup: (groupId: number) => void
  onContactClick?: (contact: Contact) => void
  onToggleSelect?: (id: number) => void
  onToggleFavorite?: (id: number) => void
}) {
  // Group contacts by their group membership
  const contactsByGroup = useMemo(() => {
    const grouped = new Map<number, Contact[]>()
    grouped.set(0, []) // "Ungrouped" contacts

    contacts.forEach((contact) => {
      if (contact.groups.length === 0) {
        grouped.get(0)?.push(contact)
      } else {
        contact.groups.forEach((groupName) => {
          const group = groups.find(g => g.name === groupName)
          if (group) {
            if (!grouped.has(group.id)) {
              grouped.set(group.id, [])
            }
            grouped.get(group.id)?.push(contact)
          }
        })
      }
    })

    return grouped
  }, [contacts, groups])

  const groupEntries = Array.from(contactsByGroup.entries()) as Array<[number, Contact[]]>

  if (groupEntries.length === 0 || (groupEntries.length === 1 && groupEntries[0][1].length === 0)) {
    return <EmptyState type="no_contacts" />
  }

  return (
    <div className="divide-y divide-slate-200 dark:divide-slate-800">
      {groupEntries.map(([groupId, groupContacts]: [number, Contact[]]) => {
        if (groupContacts.length === 0) return null

        const group = groupId === 0 ? null : groups.find(g => g.id === groupId)
        const isExpanded = expandedGroups.has(groupId)

        return (
          <div key={groupId}>
            {/* Group header */}
            <button
              onClick={() => onToggleGroup(groupId)}
              className="w-full flex items-center gap-3 px-4 py-3 bg-slate-50 dark:bg-slate-900/50 hover:bg-slate-100 dark:hover:bg-slate-900 transition-colors"
            >
              <svg
                className={cn('w-4 h-4 text-slate-400 transition-transform', isExpanded && 'rotate-90')}
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
              </svg>

              {group?.icon && <span className="text-lg">{group.icon}</span>}

              <h3 className="flex-1 text-left font-semibold text-sm text-slate-900 dark:text-slate-100">
                {group?.name || '未分组'}
              </h3>

              <span className="text-xs text-slate-500 dark:text-slate-400">
                {groupContacts.length} 人
              </span>
            </button>

            {/* Group contacts */}
            {isExpanded && (
              <div className="divide-y divide-slate-200 dark:divide-slate-800">
                {groupContacts.map((contact) => (
                  <ContactItem
                    key={contact.id}
                    contact={contact}
                    isSelected={selectedIds.has(contact.id)}
                    showCheckbox={showCheckbox}
                    onClick={() => onContactClick?.(contact)}
                    onToggleSelect={() => onToggleSelect?.(contact.id)}
                    onToggleFavorite={() => onToggleFavorite?.(contact.id)}
                  />
                ))}
              </div>
            )}
          </div>
        )
      })}
    </div>
  )
}

/**
 * Departments view - contacts organized by department
 */
function DepartmentsView({
  contacts,
  selectedIds,
  showCheckbox,
  expandedDepartments,
  onToggleDepartment,
  onContactClick,
  onToggleSelect,
  onToggleFavorite,
}: {
  contacts: Contact[]
  selectedIds: Set<number>
  showCheckbox: boolean
  expandedDepartments: Set<string>
  onToggleDepartment: (department: string) => void
  onContactClick?: (contact: Contact) => void
  onToggleSelect?: (id: number) => void
  onToggleFavorite?: (id: number) => void
}) {
  // Group contacts by department
  const contactsByDepartment = useMemo(() => {
    const grouped = new Map<string, Contact[]>()
    grouped.set('未分类', [])

    contacts.forEach((contact) => {
      const dept = contact.department || '未分类'
      if (!grouped.has(dept)) {
        grouped.set(dept, [])
      }
      grouped.get(dept)?.push(contact)
    })

    return grouped
  }, [contacts])

  const departmentEntries = Array.from(contactsByDepartment.entries()).sort((a, b) => {
    if (a[0] === '未分类') return 1
    if (b[0] === '未分类') return -1
    return a[0].localeCompare(b[0], 'zh-CN')
  })

  if (departmentEntries.length === 0 || (departmentEntries.length === 1 && departmentEntries[0][1].length === 0)) {
    return <EmptyState type="no_contacts" />
  }

  return (
    <div className="divide-y divide-slate-200 dark:divide-slate-800">
      {departmentEntries.map(([department, deptContacts]) => {
        if (deptContacts.length === 0) return null

        const isExpanded = expandedDepartments.has(department)

        return (
          <div key={department}>
            {/* Department header */}
            <button
              onClick={() => onToggleDepartment(department)}
              className="w-full flex items-center gap-3 px-4 py-3 bg-slate-50 dark:bg-slate-900/50 hover:bg-slate-100 dark:hover:bg-slate-900 transition-colors"
            >
              <svg
                className={cn('w-4 h-4 text-slate-400 transition-transform', isExpanded && 'rotate-90')}
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
              </svg>

              <span className="text-lg">🏢</span>

              <h3 className="flex-1 text-left font-semibold text-sm text-slate-900 dark:text-slate-100">
                {department}
              </h3>

              <span className="text-xs text-slate-500 dark:text-slate-400">
                {deptContacts.length} 人
              </span>
            </button>

            {/* Department contacts */}
            {isExpanded && (
              <div className="divide-y divide-slate-200 dark:divide-slate-800">
                {deptContacts.map((contact) => (
                  <ContactItem
                    key={contact.id}
                    contact={contact}
                    isSelected={selectedIds.has(contact.id)}
                    showCheckbox={showCheckbox}
                    onClick={() => onContactClick?.(contact)}
                    onToggleSelect={() => onToggleSelect?.(contact.id)}
                    onToggleFavorite={() => onToggleFavorite?.(contact.id)}
                  />
                ))}
              </div>
            )}
          </div>
        )
      })}
    </div>
  )
}

export function ContactList({
  contacts,
  groups,
  isLoading,
  error,
  onContactClick,
  onToggleFavorite,
  onDeleteContact,
  onCreateContact,
  onBatchDelete,
  onBatchMoveToGroup,
  onBatchExport,
  className,
}: ContactListProps) {
  // Local state
  const [viewMode, setViewMode] = useState<ContactView>('list')
  const [sortBy, setSortBy] = useState<ContactSortBy>('name')
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc')
  const [filters, setFilters] = useState<ContactFilters>({})
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set())
  const [showCheckbox, setShowCheckbox] = useState(false)
  const [expandedGroups, setExpandedGroups] = useState<Set<number>>(new Set())
  const [expandedDepartments, setExpandedDepartments] = useState<Set<string>>(new Set())
  const [showMoveDialog, setShowMoveDialog] = useState(false)
  const [selectedTargetGroup, setSelectedTargetGroup] = useState<number | null>(null)

  // Get unique departments from contacts
  const departments = useMemo(() => {
    const depts = new Set<string>()
    contacts.forEach(c => {
      if (c.department) depts.add(c.department)
    })
    return Array.from(depts).sort()
  }, [contacts])

  // Check if there are active filters
  const hasActiveFilters = Object.values(filters).some(Boolean)

  // Handle contact selection
  const handleToggleSelect = useCallback((id: number) => {
    setSelectedIds(prev => {
      const next = new Set(prev)
      if (next.has(id)) {
        next.delete(id)
        if (next.size === 0) setShowCheckbox(false)
      } else {
        next.add(id)
      }
      return next
    })
  }, [])

  // Handle select all
  const handleSelectAll = useCallback(() => {
    setSelectedIds(new Set(contacts.map(c => c.id)))
    setShowCheckbox(true)
  }, [contacts])

  // Handle clear selection
  const handleClearSelection = useCallback(() => {
    setSelectedIds(new Set())
    setShowCheckbox(false)
  }, [])

  // Handle clear filters
  const handleClearFilters = useCallback(() => {
    setFilters({})
  }, [])

  // Batch operation handlers
  const handleBatchDelete = useCallback(async () => {
    if (!onBatchDelete || selectedIds.size === 0) return
    const ids = Array.from(selectedIds)
    if (window.confirm(`确定要删除选中的 ${ids.length} 个联系人吗？`)) {
      await onBatchDelete(ids)
      setSelectedIds(new Set())
      setShowCheckbox(false)
    }
  }, [selectedIds, onBatchDelete])

  const handleBatchMoveToGroup = useCallback(() => {
    if (selectedIds.size === 0) return
    setShowMoveDialog(true)
  }, [selectedIds])

  const handleConfirmMoveToGroup = useCallback(async () => {
    if (!onBatchMoveToGroup || selectedTargetGroup === null || selectedIds.size === 0) return
    const ids = Array.from(selectedIds)
    await onBatchMoveToGroup(ids, selectedTargetGroup)
    setShowMoveDialog(false)
    setSelectedTargetGroup(null)
    setSelectedIds(new Set())
    setShowCheckbox(false)
  }, [selectedIds, selectedTargetGroup, onBatchMoveToGroup])

  const handleBatchExport = useCallback(() => {
    if (!onBatchExport || selectedIds.size === 0) return
    const ids = Array.from(selectedIds)
    onBatchExport(ids)
  }, [selectedIds, onBatchExport])

  // Keyboard shortcuts for batch operations
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl/Cmd + A: Select all
      if ((e.ctrlKey || e.metaKey) && e.key === 'a') {
        e.preventDefault()
        handleSelectAll()
      }
      // Escape: Clear selection
      else if (e.key === 'Escape' && selectedIds.size > 0) {
        handleClearSelection()
      }
      // Delete: Batch delete (with confirmation)
      else if (e.key === 'Delete' && selectedIds.size > 0 && onBatchDelete) {
        handleBatchDelete()
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [selectedIds, onBatchDelete, handleSelectAll, handleClearSelection, handleBatchDelete])

  // Render appropriate view
  const renderContent = () => {
    if (isLoading) {
      return <LoadingSkeleton />
    }

    if (error) {
      return <EmptyState type="error" />
    }

    if (contacts.length === 0) {
      return <EmptyState type="no_contacts" onCreateContact={onCreateContact} />
    }

    switch (viewMode) {
      case 'grid':
        return (
          <GridView
            contacts={contacts}
            selectedIds={selectedIds}
            showCheckbox={showCheckbox}
            onContactClick={onContactClick}
            onToggleSelect={handleToggleSelect}
            onToggleFavorite={onToggleFavorite}
          />
        )
      case 'groups':
        return (
          <GroupsView
            contacts={contacts}
            groups={groups}
            selectedIds={selectedIds}
            showCheckbox={showCheckbox}
            expandedGroups={expandedGroups}
            onToggleGroup={(id) => {
              setExpandedGroups(prev => {
                const next = new Set(prev)
                if (next.has(id)) next.delete(id)
                else next.add(id)
                return next
              })
            }}
            onContactClick={onContactClick}
            onToggleSelect={handleToggleSelect}
            onToggleFavorite={onToggleFavorite}
          />
        )
      case 'departments':
        return (
          <DepartmentsView
            contacts={contacts}
            selectedIds={selectedIds}
            showCheckbox={showCheckbox}
            expandedDepartments={expandedDepartments}
            onToggleDepartment={(dept) => {
              setExpandedDepartments(prev => {
                const next = new Set(prev)
                if (next.has(dept)) next.delete(dept)
                else next.add(dept)
                return next
              })
            }}
            onContactClick={onContactClick}
            onToggleSelect={handleToggleSelect}
            onToggleFavorite={onToggleFavorite}
          />
        )
      default:
        return (
          <ListView
            contacts={contacts}
            selectedIds={selectedIds}
            showCheckbox={showCheckbox}
            onContactClick={onContactClick}
            onToggleSelect={handleToggleSelect}
            onToggleFavorite={onToggleFavorite}
          />
        )
    }
  }

  return (
    <div className={cn('flex flex-col h-full bg-white dark:bg-slate-900', className)}>
      {/* Filters */}
      <ContactFiltersComponent
        filters={filters}
        viewMode={viewMode}
        sortBy={sortBy}
        departments={departments}
        onFiltersChange={setFilters}
        onViewModeChange={setViewMode}
        onSortByChange={setSortBy}
        onSearch={(query) => setFilters(prev => ({ ...prev, search: query || undefined }))}
        onClearFilters={handleClearFilters}
        hasActiveFilters={hasActiveFilters}
      />

      {/* Selection toolbar */}
      {selectedIds.size > 0 && (
        <div className="flex items-center justify-between px-4 py-2 bg-emerald-50 dark:bg-emerald-900/20 border-b border-emerald-200 dark:border-emerald-800">
          <span className="text-sm font-medium text-emerald-700 dark:text-emerald-400">
            已选择 {selectedIds.size} 个联系人
          </span>
          <div className="flex items-center gap-2">
            <button
              onClick={handleSelectAll}
              className="text-xs px-3 py-1.5 text-emerald-700 dark:text-emerald-400 hover:bg-emerald-100 dark:hover:bg-emerald-900/30 rounded-lg transition-colors"
            >
              全选
            </button>
            <button
              onClick={handleClearSelection}
              className="text-xs px-3 py-1.5 text-emerald-700 dark:text-emerald-400 hover:bg-emerald-100 dark:hover:bg-emerald-900/30 rounded-lg transition-colors"
            >
              取消选择
            </button>
            <div className="w-px h-4 bg-emerald-300 dark:bg-emerald-700" />
            {onBatchMoveToGroup && (
              <button
                onClick={handleBatchMoveToGroup}
                className="text-xs px-3 py-1.5 text-blue-700 dark:text-blue-400 hover:bg-blue-100 dark:hover:bg-blue-900/30 rounded-lg transition-colors flex items-center gap-1"
              >
                <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                </svg>
                移动到分组
              </button>
            )}
            {onBatchExport && (
              <button
                onClick={handleBatchExport}
                className="text-xs px-3 py-1.5 text-blue-700 dark:text-blue-400 hover:bg-blue-100 dark:hover:bg-blue-900/30 rounded-lg transition-colors flex items-center gap-1"
              >
                <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                </svg>
                导出
              </button>
            )}
            {onBatchDelete && (
              <button
                onClick={handleBatchDelete}
                className="text-xs px-3 py-1.5 text-red-700 dark:text-red-400 hover:bg-red-100 dark:hover:bg-red-900/30 rounded-lg transition-colors flex items-center gap-1"
              >
                <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
                删除
              </button>
            )}
          </div>
        </div>
      )}

      {/* Contact list content */}
      <div className="flex-1 overflow-y-auto">
        {renderContent()}
      </div>

      {/* Move to Group Dialog */}
      {showMoveDialog && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
          <div
            className="absolute inset-0 bg-black/50 backdrop-blur-sm"
            onClick={() => {
              setShowMoveDialog(false)
              setSelectedTargetGroup(null)
            }}
          />
          <div className="relative bg-white dark:bg-slate-800 rounded-xl shadow-xl w-full max-w-md p-6">
            <h3 className="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">
              移动到分组
            </h3>
            <p className="text-sm text-slate-600 dark:text-slate-400 mb-4">
              将选中的 {selectedIds.size} 个联系人移动到：
            </p>
            <div className="space-y-2 max-h-64 overflow-y-auto mb-4">
              {groups.map((group) => (
                <button
                  key={group.id}
                  onClick={() => setSelectedTargetGroup(group.id)}
                  className={cn(
                    'w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-left',
                    selectedTargetGroup === group.id
                      ? 'bg-emerald-100 dark:bg-emerald-900/30 border-2 border-emerald-500'
                      : 'bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 border-2 border-transparent'
                  )}
                >
                  <div
                    className="w-10 h-10 rounded-lg flex items-center justify-center text-lg flex-shrink-0"
                    style={{ backgroundColor: group.color || '#10b981' }}
                  >
                    {group.icon || '📁'}
                  </div>
                  <span className="font-medium text-slate-900 dark:text-slate-100">
                    {group.name}
                  </span>
                  {selectedTargetGroup === group.id && (
                    <svg className="w-5 h-5 ml-auto text-emerald-600 dark:text-emerald-400" fill="currentColor" viewBox="0 0 20 20">
                      <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                    </svg>
                  )}
                </button>
              ))}
              {groups.length === 0 && (
                <div className="text-center py-8 text-slate-500 dark:text-slate-400">
                  暂无分组，请先创建分组
                </div>
              )}
            </div>
            <div className="flex justify-end gap-2">
              <button
                onClick={() => {
                  setShowMoveDialog(false)
                  setSelectedTargetGroup(null)
                }}
                className="px-4 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-slate-700 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors"
              >
                取消
              </button>
              <button
                onClick={handleConfirmMoveToGroup}
                disabled={selectedTargetGroup === null || groups.length === 0}
                className="px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                确定移动
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
