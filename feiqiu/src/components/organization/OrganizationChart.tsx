'use client'

import { useState, useMemo } from 'react'
import { User, Department, OrganizationChartProps } from '../../lib/types/organization'
import { DepartmentTree } from './DepartmentTree'
import { UserCard } from './UserCard'
import { Search, Filter, Users as UsersIcon } from 'lucide-react'

export function OrganizationChart({
  currentUser,
  departments,
  users,
  onDepartmentSelect,
  onStartChat,
  onViewDetails,
  onSearch,
}: OrganizationChartProps) {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedDepartmentId, setSelectedDepartmentId] = useState<string | null>(null)
  const [expandedDepartmentIds, setExpandedDepartmentIds] = useState<Set<string>>(() => {
    const initial = new Set<string>()
    departments.forEach((dept) => {
      if (dept.level === 0) {
        initial.add(dept.id)
      }
    })
    return initial
  })

  const handleToggleExpand = (departmentId: string) => {
    setExpandedDepartmentIds((prev) => {
      const next = new Set(prev)
      if (next.has(departmentId)) {
        next.delete(departmentId)
      } else {
        next.add(departmentId)
      }
      return next
    })
  }

  const handleSelectDepartment = (departmentId: string | null) => {
    setSelectedDepartmentId(departmentId)
    onDepartmentSelect?.(departmentId ?? '')
  }

  const handleSearchChange = (query: string) => {
    setSearchQuery(query)
    onSearch?.(query)
  }

  const filteredUsers = useMemo(() => {
    let filtered = users

    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase()
      filtered = filtered.filter(
        (user) =>
          user.name.toLowerCase().includes(query) ||
          user.pinyin.toLowerCase().includes(query) ||
          user.position.toLowerCase().includes(query) ||
          user.department.toLowerCase().includes(query) ||
          user.email.toLowerCase().includes(query)
      )
    }

    if (selectedDepartmentId) {
      filtered = filtered.filter((user) => user.departmentId === selectedDepartmentId)
    }

    return filtered
  }, [users, searchQuery, selectedDepartmentId])

  const selectedDepartment = useMemo(
    () => departments.find((d) => d.id === selectedDepartmentId),
    [departments, selectedDepartmentId]
  )

  return (
    <div className="h-full flex flex-col bg-slate-50 dark:bg-slate-900">
      {/* Header */}
      <div className="px-4 py-3 border-b border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-800">
        <h2 className="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-3">
          组织架构
        </h2>

        {/* Search */}
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
          <input
            type="text"
            placeholder="搜索姓名、职位、部门..."
            value={searchQuery}
            onChange={(e) => handleSearchChange(e.target.value)}
            className="w-full pl-9 pr-4 py-2 bg-slate-100 dark:bg-slate-700 border-0 rounded-lg text-sm text-slate-900 dark:text-slate-100 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500"
          />
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 flex overflow-hidden">
        {/* Department Tree */}
        <div className="w-56 flex-shrink-0 border-r border-slate-200 dark:border-slate-800 p-3 overflow-y-auto bg-white dark:bg-slate-800/50">
          <DepartmentTree
            departments={departments}
            selectedDepartmentId={selectedDepartmentId}
            expandedDepartmentIds={expandedDepartmentIds}
            onToggleExpand={handleToggleExpand}
            onSelectDepartment={handleSelectDepartment}
          />
        </div>

        {/* User Grid */}
        <div className="flex-1 overflow-y-auto p-4">
          {searchQuery && (
            <div className="mb-3 flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
              <Filter className="w-4 h-4" />
              <span>
                找到 <strong>{filteredUsers.length}</strong> 位同事
              </span>
            </div>
          )}

          {!searchQuery && selectedDepartment && (
            <div className="mb-3 flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
              <UsersIcon className="w-4 h-4" />
              <span>
                {selectedDepartment.name} · <strong>{filteredUsers.length}</strong> 人
              </span>
            </div>
          )}

          {!searchQuery && !selectedDepartment && (
            <div className="mb-3 flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
              <UsersIcon className="w-4 h-4" />
              <span>
                全部同事 · <strong>{filteredUsers.length}</strong> 人
              </span>
            </div>
          )}

          {filteredUsers.length > 0 ? (
            <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-3">
              {filteredUsers.map((user) => (
                <UserCard
                  key={user.id}
                  user={user}
                  onStartChat={onStartChat}
                  onViewDetails={onViewDetails}
                />
              ))}
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center h-48 text-slate-500 dark:text-slate-400">
              <UsersIcon className="w-12 h-12 mb-2 opacity-50" />
              <p className="text-sm">暂无同事</p>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
