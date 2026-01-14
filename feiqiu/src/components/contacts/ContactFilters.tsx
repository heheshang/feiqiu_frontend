/**
 * ContactFilters Component
 *
 * Search and filter controls for the contacts list.
 */

import { useState, useCallback, useRef, useEffect } from 'react'
import { ContactFilters as ContactFiltersType, ContactView, ContactSortBy } from '@/lib/types/contacts'
import { cn } from '@/lib/utils'

interface ContactFiltersProps {
  /** Current filters */
  filters: ContactFiltersType
  /** Current view mode */
  viewMode: ContactView
  /** Current sort field */
  sortBy: ContactSortBy
  /** Available departments for filter dropdown */
  departments?: string[]
  /** Callback when filters change */
  onFiltersChange: (filters: ContactFiltersType) => void
  /** Callback when view mode changes */
  onViewModeChange: (view: ContactView) => void
  /** Callback when sort field changes */
  onSortByChange: (sortBy: ContactSortBy) => void
  /** Callback when search query changes */
  onSearch: (query: string) => void
  /** Callback when filters are cleared */
  onClearFilters: () => void
  /** Whether there are active filters */
  hasActiveFilters: boolean
  /** Additional CSS classes */
  className?: string
}

/**
 * View mode options
 */
const VIEW_MODES: Array<{ value: ContactView; label: string; icon: string }> = [
  { value: 'list', label: '列表', icon: '☰' },
  { value: 'grid', label: '网格', icon: '⊞' },
  { value: 'groups', label: '分组', icon: '📁' },
  { value: 'departments', label: '部门', icon: '🏢' },
]

/**
 * Sort options
 */
const SORT_OPTIONS: Array<{ value: ContactSortBy; label: string }> = [
  { value: 'name', label: '按名称' },
  { value: 'department', label: '按部门' },
  { value: 'lastSeen', label: '最后在线' },
  { value: 'created', label: '创建时间' },
  { value: 'favorites', label: '收藏优先' },
]

export function ContactFilters({
  filters,
  viewMode,
  sortBy,
  departments = [],
  onFiltersChange,
  onViewModeChange,
  onSortByChange,
  onSearch,
  onClearFilters,
  hasActiveFilters,
  className,
}: ContactFiltersProps) {
  const [searchQuery, setSearchQuery] = useState(filters.search || '')
  const [showViewMenu, setShowViewMenu] = useState(false)
  const [showSortMenu, setShowSortMenu] = useState(false)
  const [showFilterMenu, setShowFilterMenu] = useState(false)

  const viewMenuRef = useRef<HTMLDivElement>(null)
  const sortMenuRef = useRef<HTMLDivElement>(null)
  const filterMenuRef = useRef<HTMLDivElement>(null)
  const searchInputRef = useRef<HTMLInputElement>(null)

  // Close dropdowns when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (viewMenuRef.current && !viewMenuRef.current.contains(event.target as Node)) {
        setShowViewMenu(false)
      }
      if (sortMenuRef.current && !sortMenuRef.current.contains(event.target as Node)) {
        setShowSortMenu(false)
      }
      if (filterMenuRef.current && !filterMenuRef.current.contains(event.target as Node)) {
        setShowFilterMenu(false)
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => document.removeEventListener('mousedown', handleClickOutside)
  }, [])

  // Debounced search
  useEffect(() => {
    const timer = setTimeout(() => {
      onSearch(searchQuery)
    }, 300)

    return () => clearTimeout(timer)
  }, [searchQuery, onSearch])

  // Handle filter changes
  const handleFilterChange = useCallback(<K extends keyof ContactFiltersType>(
    key: K,
    value: ContactFiltersType[K]
  ) => {
    onFiltersChange({ ...filters, [key]: value })
  }, [filters, onFiltersChange])

  // Clear search input
  const clearSearch = useCallback(() => {
    setSearchQuery('')
    onSearch('')
  }, [onSearch])

  // Toggle favorite filter
  const toggleFavoriteFilter = useCallback(() => {
    handleFilterChange('isFavorite', filters.isFavorite ? undefined : true)
  }, [filters.isFavorite, handleFilterChange])

  // Toggle online filter
  const toggleOnlineFilter = useCallback(() => {
    handleFilterChange('isOnline', filters.isOnline === true ? undefined : true)
  }, [filters.isOnline, handleFilterChange])

  // Get current view mode label
  const currentViewMode = VIEW_MODES.find(m => m.value === viewMode)
  const currentSortOption = SORT_OPTIONS.find(o => o.value === sortBy)

  // Count active filters (excluding search)
  const activeFilterCount = [
    filters.isOnline,
    filters.isFavorite,
    filters.department,
    filters.groupId,
  ].filter(Boolean).length

  return (
    <div className={cn('flex flex-col gap-3 p-4 bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800', className)}>
      {/* Search bar */}
      <div className="relative">
        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <svg className="w-5 h-5 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </div>
        <input
          ref={searchInputRef}
          type="text"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          placeholder="搜索联系人（姓名、拼音、部门、职位）..."
          className="w-full pl-10 pr-10 py-2.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-sm text-slate-900 dark:text-slate-100 placeholder-slate-400 dark:placeholder-slate-500 focus:ring-2 focus:ring-emerald-500 dark:focus:ring-emerald-400 focus:bg-white dark:focus:bg-slate-900 transition-colors"
        />
        {searchQuery && (
          <button
            onClick={clearSearch}
            className="absolute inset-y-0 right-0 pr-3 flex items-center text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-colors"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        )}
      </div>

      {/* Action buttons row */}
      <div className="flex items-center justify-between gap-2">
        {/* Left side: Filter buttons */}
        <div className="flex items-center gap-2">
          {/* View mode dropdown */}
          <div className="relative" ref={viewMenuRef}>
            <button
              onClick={() => setShowViewMenu(!showViewMenu)}
              className="flex items-center gap-1.5 px-3 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-slate-800 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
            >
              <span>{currentViewMode?.icon}</span>
              <span className="hidden sm:inline">{currentViewMode?.label}</span>
              <svg className={cn('w-4 h-4 transition-transform', showViewMenu && 'rotate-180')} fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            {showViewMenu && (
              <div className="absolute z-10 mt-1 w-40 bg-white dark:bg-slate-800 rounded-lg shadow-lg border border-slate-200 dark:border-slate-700 py-1">
                {VIEW_MODES.map((mode) => (
                  <button
                    key={mode.value}
                    onClick={() => {
                      onViewModeChange(mode.value)
                      setShowViewMenu(false)
                    }}
                    className={cn(
                      'w-full flex items-center gap-2 px-3 py-2 text-sm text-left transition-colors',
                      viewMode === mode.value
                        ? 'bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400'
                        : 'text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700'
                    )}
                  >
                    <span>{mode.icon}</span>
                    <span>{mode.label}</span>
                  </button>
                ))}
              </div>
            )}
          </div>

          {/* Sort dropdown */}
          <div className="relative" ref={sortMenuRef}>
            <button
              onClick={() => setShowSortMenu(!showSortMenu)}
              className="flex items-center gap-1.5 px-3 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-slate-800 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
            >
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12" />
              </svg>
              <span className="hidden sm:inline">{currentSortOption?.label}</span>
              <svg className={cn('w-4 h-4 transition-transform', showSortMenu && 'rotate-180')} fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            {showSortMenu && (
              <div className="absolute z-10 mt-1 w-40 bg-white dark:bg-slate-800 rounded-lg shadow-lg border border-slate-200 dark:border-slate-700 py-1">
                {SORT_OPTIONS.map((option) => (
                  <button
                    key={option.value}
                    onClick={() => {
                      onSortByChange(option.value)
                      setShowSortMenu(false)
                    }}
                    className={cn(
                      'w-full px-3 py-2 text-sm text-left transition-colors',
                      sortBy === option.value
                        ? 'bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400'
                        : 'text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700'
                    )}
                  >
                    {option.label}
                  </button>
                ))}
              </div>
            )}
          </div>

          {/* Filter dropdown */}
          <div className="relative" ref={filterMenuRef}>
            <button
              onClick={() => setShowFilterMenu(!showFilterMenu)}
              className={cn(
                'flex items-center gap-1.5 px-3 py-2 text-sm font-medium rounded-lg transition-colors relative',
                hasActiveFilters
                  ? 'text-emerald-700 dark:text-emerald-400 bg-emerald-100 dark:bg-emerald-900/20 hover:bg-emerald-200 dark:hover:bg-emerald-900/30'
                  : 'text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700'
              )}
            >
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
              </svg>
              <span className="hidden sm:inline">筛选</span>
              {activeFilterCount > 0 && (
                <span className="absolute -top-1 -right-1 w-5 h-5 bg-emerald-500 dark:bg-emerald-600 text-white text-xs font-bold rounded-full flex items-center justify-center">
                  {activeFilterCount}
                </span>
              )}
              <svg className={cn('w-4 h-4 transition-transform', showFilterMenu && 'rotate-180')} fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            {showFilterMenu && (
              <div className="absolute z-10 mt-1 w-56 bg-white dark:bg-slate-800 rounded-lg shadow-lg border border-slate-200 dark:border-slate-700 py-2">
                {/* Online filter */}
                <button
                  onClick={toggleOnlineFilter}
                  className={cn(
                    'w-full flex items-center justify-between px-4 py-2 text-sm transition-colors',
                    filters.isOnline
                      ? 'bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400'
                      : 'text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700'
                  )}
                >
                  <span className="flex items-center gap-2">
                    <span className={cn('w-2 h-2 rounded-full', filters.isOnline ? 'bg-emerald-500' : 'bg-slate-300 dark:bg-slate-600')} />
                    仅显示在线
                  </span>
                  {filters.isOnline && (
                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                    </svg>
                  )}
                </button>

                {/* Favorite filter */}
                <button
                  onClick={toggleFavoriteFilter}
                  className={cn(
                    'w-full flex items-center justify-between px-4 py-2 text-sm transition-colors',
                    filters.isFavorite
                      ? 'bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400'
                      : 'text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700'
                  )}
                >
                  <span className="flex items-center gap-2">
                    <span className={filters.isFavorite ? 'text-yellow-500' : 'text-slate-400'}>★</span>
                    仅显示收藏
                  </span>
                  {filters.isFavorite && (
                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                    </svg>
                  )}
                </button>

                {/* Department filter */}
                {departments.length > 0 && (
                  <div className="px-4 py-2 border-t border-slate-200 dark:border-slate-700">
                    <label className="text-xs text-slate-500 dark:text-slate-400 mb-1.5 block">按部门筛选</label>
                    <select
                      value={filters.department || ''}
                      onChange={(e) => handleFilterChange('department', e.target.value || undefined)}
                      className="w-full px-2 py-1.5 text-sm bg-slate-100 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-md text-slate-700 dark:text-slate-300 focus:ring-2 focus:ring-emerald-500"
                    >
                      <option value="">全部部门</option>
                      {departments.map((dept) => (
                        <option key={dept} value={dept}>
                          {dept}
                        </option>
                      ))}
                    </select>
                  </div>
                )}

                {/* Clear all filters */}
                {hasActiveFilters && (
                  <button
                    onClick={() => {
                      onClearFilters()
                      setShowFilterMenu(false)
                    }}
                    className="w-full flex items-center justify-center gap-2 px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors border-t border-slate-200 dark:border-slate-700"
                  >
                    <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                    清除筛选
                  </button>
                )}
              </div>
            )}
          </div>
        </div>

        {/* Right side: Stats */}
        {hasActiveFilters && (
          <div className="text-xs text-slate-500 dark:text-slate-400">
            已应用 {activeFilterCount} 个筛选条件
          </div>
        )}
      </div>
    </div>
  )
}
