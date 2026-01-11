'use client'

import { Department } from '../../lib/types/organization'
import { ChevronRight, ChevronDown, Users } from 'lucide-react'
import { useState } from 'react'

interface DepartmentTreeProps {
  departments: Department[]
  selectedDepartmentId?: string | null
  expandedDepartmentIds: Set<string>
  onToggleExpand: (departmentId: string) => void
  onSelectDepartment: (departmentId: string | null) => void
}

function buildDepartmentTree(departments: Department[]): Map<string, Department[]> {
  const tree = new Map<string, Department[]>()
  const rootDepartments: Department[] = []

  for (const dept of departments) {
    if (dept.parentId === null) {
      rootDepartments.push(dept)
    } else {
      const children = tree.get(dept.parentId) || []
      children.push(dept)
      tree.set(dept.parentId, children)
    }
  }

  tree.set('root', rootDepartments)
  return tree
}

interface DepartmentTreeNodeProps {
  department: Department
  children: Department[]
  depth: number
  allDepartments: Department[]
  selectedDepartmentId?: string | null
  expandedDepartmentIds: Set<string>
  onToggleExpand: (departmentId: string) => void
  onSelectDepartment: (departmentId: string | null) => void
}

function DepartmentTreeNode({
  department,
  children,
  depth,
  allDepartments,
  selectedDepartmentId,
  expandedDepartmentIds,
  onToggleExpand,
  onSelectDepartment,
}: DepartmentTreeNodeProps) {
  const isExpanded = expandedDepartmentIds.has(department.id)
  const isSelected = selectedDepartmentId === department.id
  const hasChildren = children.length > 0

  return (
    <div>
      <button
        onClick={() => {
          if (hasChildren) {
            onToggleExpand(department.id)
          }
          onSelectDepartment(isSelected ? null : department.id)
        }}
        className={`
          w-full flex items-center gap-2 px-2 py-1.5 rounded-lg text-left transition-colors
          ${isSelected
            ? 'bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400'
            : 'hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300'
          }
        `}
        style={{ paddingLeft: `${8 + depth * 20}px` }}
      >
        {hasChildren ? (
          isExpanded ? (
            <ChevronDown className="w-4 h-4 flex-shrink-0" />
          ) : (
            <ChevronRight className="w-4 h-4 flex-shrink-0" />
          )
        ) : (
          <span className="w-4" />
        )}
        <span className="text-sm font-medium truncate flex-1">{department.name}</span>
        <span className="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400">
          <Users className="w-3 h-3" />
          {department.memberCount}
        </span>
      </button>

      {isExpanded && hasChildren && (
        <div className="mt-0.5">
          {children.map((child) => (
            <DepartmentTreeNode
              key={child.id}
              department={child}
              children={buildDepartmentTree(allDepartments).get(child.id) || []}
              depth={depth + 1}
              allDepartments={allDepartments}
              selectedDepartmentId={selectedDepartmentId}
              expandedDepartmentIds={expandedDepartmentIds}
              onToggleExpand={onToggleExpand}
              onSelectDepartment={onSelectDepartment}
            />
          ))}
        </div>
      )}
    </div>
  )
}

export function DepartmentTree({
  departments,
  selectedDepartmentId,
  expandedDepartmentIds,
  onToggleExpand,
  onSelectDepartment,
}: DepartmentTreeProps) {
  const tree = buildDepartmentTree(departments)
  const rootDepartments = tree.get('root') || []

  return (
    <div className="space-y-0.5">
      <button
        onClick={() => onSelectDepartment(null)}
        className={`
          w-full flex items-center gap-2 px-2 py-1.5 rounded-lg text-left transition-colors
          ${selectedDepartmentId === null
            ? 'bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400'
            : 'hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300'
          }
        `}
      >
        <Users className="w-4 h-4 flex-shrink-0" />
        <span className="text-sm font-medium">全部同事</span>
        <span className="ml-auto text-xs text-slate-500 dark:text-slate-400">
          {departments.reduce((sum, d) => sum + d.memberCount, 0)}
        </span>
      </button>

      {rootDepartments.map((dept) => (
        <DepartmentTreeNode
          key={dept.id}
          department={dept}
          children={tree.get(dept.id) || []}
          depth={0}
          allDepartments={departments}
          selectedDepartmentId={selectedDepartmentId}
          expandedDepartmentIds={expandedDepartmentIds}
          onToggleExpand={onToggleExpand}
          onSelectDepartment={onSelectDepartment}
        />
      ))}
    </div>
  )
}
