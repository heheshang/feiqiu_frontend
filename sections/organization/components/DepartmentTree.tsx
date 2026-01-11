'use client'

import type { Department } from '@/../product/sections/organization/types'

interface DepartmentTreeProps {
  departments: Department[]
  allDepartments: Department[]
  selectedId: string | null
  expandedIds: Set<string>
  onToggle: (id: string) => void
  onSelect: (id: string) => void
  buildTree: (parentId: string | null) => Department[]
  level?: number
}

export function DepartmentTree({
  departments,
  allDepartments,
  selectedId,
  expandedIds,
  onToggle,
  onSelect,
  buildTree,
  level = 0
}: DepartmentTreeProps) {
  if (departments.length === 0) return null

  return (
    <>
      {departments.map(dept => {
        const hasChildren = allDepartments.some(d => d.parentId === dept.id)
        const isExpanded = expandedIds.has(dept.id)
        const isSelected = selectedId === dept.id

        return (
          <div key={dept.id}>
            <button
              onClick={() => onSelect(dept.id)}
              className={`
                w-full flex items-center gap-2 px-3 py-2 rounded-lg text-left transition-all duration-200
                ${isSelected
                  ? 'bg-emerald-500 text-white'
                  : 'hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300'
                }
              `}
              style={{ paddingLeft: `${12 + level * 16}px` }}
            >
              {/* Expand/collapse icon */}
              {hasChildren ? (
                <svg
                  onClick={(e) => {
                    e.stopPropagation()
                    onToggle(dept.id)
                  }}
                  className={`w-4 h-4 flex-shrink-0 transition-transform ${isExpanded ? 'rotate-90' : ''} ${
                    isSelected ? 'text-emerald-100' : 'text-slate-400'
                  }`}
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                </svg>
              ) : (
                <span className="w-4 h-4 flex-shrink-0" />
              )}

              {/* Department name */}
              <span className="flex-1 truncate text-sm font-medium">{dept.name}</span>

              {/* Member count */}
              <span className={`text-xs flex-shrink-0 ${isSelected ? 'text-emerald-100' : 'text-slate-400'}`}>
                {dept.memberCount}
              </span>
            </button>

            {/* Children */}
            {hasChildren && isExpanded && (
              <DepartmentTree
                departments={buildTree(dept.id)}
                allDepartments={allDepartments}
                selectedId={selectedId}
                expandedIds={expandedIds}
                onToggle={onToggle}
                onSelect={onSelect}
                buildTree={buildTree}
                level={level + 1}
              />
            )}
          </div>
        )
      })}
    </>
  )
}
