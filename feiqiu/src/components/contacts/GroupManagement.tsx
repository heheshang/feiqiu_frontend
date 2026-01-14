/**
 * GroupManagement Component
 *
 * Displays and manages contact groups with options to create, edit, delete
 * and manage group memberships.
 */

import { useState, useCallback, useMemo } from 'react'
import { ContactGroup, Contact } from '@/lib/types/contacts'
import { cn } from '@/lib/utils'
import { CreateGroupDialog, EditGroupDialog, ManageGroupMembersDialog } from './GroupDialogs'

interface GroupManagementProps {
  /** All groups */
  groups: ContactGroup[]
  /** All contacts (for member assignment) */
  contacts: Contact[]
  /** Callback when a group is created */
  onCreateGroup: (group: { name: string; color?: string; icon?: string }) => Promise<void>
  /** Callback when a group is updated */
  onUpdateGroup: (id: number, updates: { name?: string; color?: string; icon?: string }) => Promise<void>
  /** Callback when a group is deleted */
  onDeleteGroup: (id: number) => Promise<void>
  /** Callback when contacts are added to a group */
  onAddContacts: (groupId: number, contactIds: number[]) => Promise<void>
  /** Callback when contacts are removed from a group */
  onRemoveContacts: (groupId: number, contactIds: number[]) => Promise<void>
  /** Whether operations are in progress */
  isLoading?: boolean
  /** Additional CSS classes */
  className?: string
}

interface GroupItemProps {
  group: ContactGroup
  contacts: Contact[]
  onEdit: (group: ContactGroup) => void
  onDelete: (id: number) => void
  onManageMembers: (group: ContactGroup) => void
}

function GroupItem({ group, contacts, onEdit, onDelete, onManageMembers }: GroupItemProps) {
  // Get group members
  const members = useMemo(() => {
    return contacts.filter(c => c.groups.includes(group.name))
  }, [contacts, group.name])

  return (
    <div className="flex items-center gap-3 p-4 bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700 hover:border-emerald-300 dark:hover:border-emerald-600 transition-colors">
      {/* Group icon/color indicator */}
      <div
        className="w-12 h-12 rounded-xl flex items-center justify-center text-xl shadow-sm flex-shrink-0"
        style={{
          backgroundColor: group.color || '#10b981',
        }}
      >
        {group.icon || '📁'}
      </div>

      {/* Group info */}
      <div className="flex-1 min-w-0">
        <h3 className="font-semibold text-slate-900 dark:text-slate-100 truncate">
          {group.name}
        </h3>
        <p className="text-sm text-slate-500 dark:text-slate-400">
          {members.length} 位成员
        </p>
      </div>

      {/* Action buttons */}
      <div className="flex items-center gap-1">
        <button
          onClick={() => onManageMembers(group)}
          className="p-2 text-slate-500 hover:text-emerald-600 dark:text-slate-400 dark:hover:text-emerald-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
          title="管理成员"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
        </button>
        <button
          onClick={() => onEdit(group)}
          className="p-2 text-slate-500 hover:text-blue-600 dark:text-slate-400 dark:hover:text-blue-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
          title="编辑分组"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
          </svg>
        </button>
        <button
          onClick={() => onDelete(group.id)}
          className="p-2 text-slate-500 hover:text-red-600 dark:text-slate-400 dark:hover:text-red-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
          title="删除分组"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
        </button>
      </div>
    </div>
  )
}

export function GroupManagement({
  groups,
  contacts,
  onCreateGroup,
  onUpdateGroup,
  onDeleteGroup,
  onAddContacts,
  onRemoveContacts,
  isLoading = false,
  className,
}: GroupManagementProps) {
  // Dialog states
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [showEditDialog, setShowEditDialog] = useState(false)
  const [showMembersDialog, setShowMembersDialog] = useState(false)
  const [editingGroup, setEditingGroup] = useState<ContactGroup | null>(null)
  const [managingGroup, setManagingGroup] = useState<ContactGroup | null>(null)

  // Sort groups by sort order
  const sortedGroups = useMemo(() => {
    return [...groups].sort((a, b) => a.sortOrder - b.sortOrder)
  }, [groups])

  // Handlers
  const handleCreateGroup = useCallback(async (data: { name: string; color?: string; icon?: string }) => {
    await onCreateGroup(data)
  }, [onCreateGroup])

  const handleEditGroup = useCallback((group: ContactGroup) => {
    setEditingGroup(group)
    setShowEditDialog(true)
  }, [])

  const handleUpdateGroup = useCallback(async (id: number, data: { name?: string; color?: string; icon?: string }) => {
    await onUpdateGroup(id, data)
    setShowEditDialog(false)
    setEditingGroup(null)
  }, [onUpdateGroup])

  const handleDeleteGroup = useCallback(async (id: number) => {
    if (window.confirm('确定要删除此分组吗？分组中的联系人不会被删除。')) {
      await onDeleteGroup(id)
    }
  }, [onDeleteGroup])

  const handleManageMembers = useCallback((group: ContactGroup) => {
    setManagingGroup(group)
    setShowMembersDialog(true)
  }, [])

  const handleAddMembers = useCallback(async (groupId: number, contactIds: number[]) => {
    await onAddContacts(groupId, contactIds)
  }, [onAddContacts])

  const handleRemoveMembers = useCallback(async (groupId: number, contactIds: number[]) => {
    await onRemoveContacts(groupId, contactIds)
  }, [onRemoveContacts])

  return (
    <div className={cn('flex flex-col h-full bg-white dark:bg-slate-900', className)}>
      {/* Header */}
      <div className="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-800">
        <div>
          <h2 className="text-xl font-bold text-slate-900 dark:text-slate-100">分组管理</h2>
          <p className="text-sm text-slate-500 dark:text-slate-400 mt-1">
            共 {groups.length} 个分组
          </p>
        </div>

        <button
          onClick={() => setShowCreateDialog(true)}
          className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 transition-colors"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          新建分组
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-6">
        {/* Groups list */}
        {sortedGroups.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-16 text-center">
            <div className="w-16 h-16 mb-4 text-4xl flex items-center justify-center bg-slate-100 dark:bg-slate-800 rounded-full">
              📁
            </div>
            <h3 className="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
              暂无分组
            </h3>
            <p className="text-sm text-slate-500 dark:text-slate-400 mb-4">
              点击上方"新建分组"按钮创建第一个分组
            </p>
          </div>
        ) : (
          <div className="grid gap-3">
            {sortedGroups.map((group) => (
              <GroupItem
                key={group.id}
                group={group}
                contacts={contacts}
                onEdit={handleEditGroup}
                onDelete={handleDeleteGroup}
                onManageMembers={handleManageMembers}
              />
            ))}
          </div>
        )}
      </div>

      {/* Dialogs */}
      <CreateGroupDialog
        isOpen={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
        onSubmit={handleCreateGroup}
        isLoading={isLoading}
      />

      <EditGroupDialog
        isOpen={showEditDialog}
        onClose={() => {
          setShowEditDialog(false)
          setEditingGroup(null)
        }}
        group={editingGroup}
        onSubmit={handleUpdateGroup}
        isLoading={isLoading}
      />

      <ManageGroupMembersDialog
        isOpen={showMembersDialog}
        onClose={() => {
          setShowMembersDialog(false)
          setManagingGroup(null)
        }}
        group={managingGroup}
        allContacts={contacts}
        onAddMembers={handleAddMembers}
        onRemoveMembers={handleRemoveMembers}
        isLoading={isLoading}
      />
    </div>
  )
}
