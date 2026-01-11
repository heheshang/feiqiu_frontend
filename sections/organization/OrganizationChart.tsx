'use client'

import { useState, useMemo } from 'react'
import type { OrganizationChartProps } from '@/../product/sections/organization/types'
import { UserCard } from './UserCard'
import { DepartmentTree } from './DepartmentTree'

export function OrganizationChart({
  currentUser,
  departments,
  users,
  onDepartmentSelect,
  onStartChat,
  onViewDetails,
  onSearch
}: OrganizationChartProps) {
  const [selectedDepartmentId, setSelectedDepartmentId] = useState<string | null>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [expandedDepts, setExpandedDepts] = useState<Set<string>>(new Set(['dept-root', 'dept-tech', 'dept-product', 'dept-design', 'dept-marketing']))

  // Filter users based on search and selected department
  const filteredUsers = useMemo(() => {
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
    if (selectedDepartmentId && selectedDepartmentId !== 'dept-root') {
      filtered = filtered.filter(user => user.departmentId === selectedDepartmentId)
    }

    return filtered
  }, [users, searchQuery, selectedDepartmentId])

  // Handle search
  const handleSearch = (value: string) => {
    setSearchQuery(value)
    onSearch?.(value)
  }

  // Handle department selection
  const handleDepartmentSelect = (deptId: string) => {
    setSelectedDepartmentId(deptId)
    onDepartmentSelect?.(deptId)
  }

  // Toggle department expansion
  const toggleExpansion = (deptId: string) => {
    setExpandedDepts(prev => {
      const next = new Set(prev)
      if (next.has(deptId)) {
        next.delete(deptId)
      } else {
        next.add(deptId)
      }
      return next
    })
  }

  // Build department tree structure
  const buildTree = (parentId: string | null = null): typeof departments => {
    return departments
      .filter(dept => dept.parentId === parentId)
      .sort((a, b) => a.name.localeCompare(b.name, 'zh-CN'))
  }

  const stats = useMemo(() => {
    const online = users.filter(u => u.status === 'online').length
    const away = users.filter(u => u.status === 'away').length
    const offline = users.filter(u => u.status === 'offline').length
    return { online, away, offline, total: users.length }
  }, [users])

  return (
    <div className="h-full flex bg-white dark:bg-slate-900">
      {/* Left sidebar - Department tree */}
      <div className="w-[280px] border-r border-slate-200 dark:border-slate-700 flex flex-col flex-shrink-0">
        {/* Header */}
        <div className="p-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-3">组织架构</h2>
          {/* Search box */}
          <div className="relative">
            <svg
              className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => handleSearch(e.target.value)}
              placeholder="搜索姓名、部门、职位..."
              className="w-full pl-10 pr-4 py-2 bg-slate-100 dark:bg-slate-800 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 dark:text-white placeholder-slate-400"
            />
          </div>
        </div>

        {/* Department tree */}
        <div className="flex-1 overflow-y-auto p-2">
          <DepartmentTree
            departments={buildTree(null)}
            allDepartments={departments}
            selectedId={selectedDepartmentId}
            expandedIds={expandedDepts}
            onToggle={toggleExpansion}
            onSelect={handleDepartmentSelect}
            buildTree={buildTree}
          />
        </div>

        {/* Stats footer */}
        <div className="p-3 border-t border-slate-200 dark:border-slate-700 text-xs text-slate-500 dark:text-slate-400">
          <div className="flex items-center justify-between mb-1">
            <span>在线: {stats.online}</span>
            <span className="flex items-center gap-1">
              <span className="w-2 h-2 bg-emerald-500 rounded-full" />
            </span>
          </div>
          <div className="flex items-center justify-between mb-1">
            <span>离开: {stats.away}</span>
            <span className="flex items-center gap-1">
              <span className="w-2 h-2 bg-amber-500 rounded-full" />
            </span>
          </div>
          <div className="flex items-center justify-between">
            <span>离线: {stats.offline}</span>
            <span className="flex items-center gap-1">
              <span className="w-2 h-2 bg-slate-400 rounded-full" />
            </span>
          </div>
        </div>
      </div>

      {/* Right side - Colleague list */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Header */}
        <div className="px-6 py-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-lg font-semibold text-slate-900 dark:text-slate-100">
                {searchQuery ? `搜索结果 (${filteredUsers.length})` : '同事列表'}
              </h2>
              {!searchQuery && selectedDepartmentId && (
                <p className="text-sm text-slate-500 dark:text-slate-400 mt-1">
                  {departments.find(d => d.id === selectedDepartmentId)?.name}
                </p>
              )}
            </div>
          </div>
        </div>

        {/* User grid */}
        <div className="flex-1 overflow-y-auto p-6">
          {filteredUsers.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-full text-center">
              <div className="w-16 h-16 rounded-2xl bg-slate-100 dark:bg-slate-800 flex items-center justify-center mb-4">
                <svg className="w-8 h-8 text-slate-400 dark:text-slate-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
              </div>
              <h3 className="text-lg font-medium text-slate-900 dark:text-slate-100 mb-2">未找到同事</h3>
              <p className="text-sm text-slate-500 dark:text-slate-400">尝试其他搜索条件</p>
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredUsers.map(user => (
                <UserCard
                  key={user.id}
                  user={user}
                  onStartChat={onStartChat}
                  onViewDetails={onViewDetails}
                />
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
