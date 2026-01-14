/**
 * Group Dialogs Component
 *
 * Modal dialogs for creating, editing groups and managing group members.
 */

import { useState, useCallback, useMemo } from 'react'
import { ContactGroup, Contact } from '@/lib/types/contacts'
import { cn } from '@/lib/utils'

// ==================== Common Components ====================

interface ModalProps {
  isOpen: boolean
  onClose: () => void
  title: string
  children: React.ReactNode
  size?: 'sm' | 'md' | 'lg' | 'xl'
}

function Modal({ isOpen, onClose, title, children, size = 'md' }: ModalProps) {
  if (!isOpen) return null

  const sizeClass = {
    sm: 'max-w-sm',
    md: 'max-w-md',
    lg: 'max-w-lg',
    xl: 'max-w-xl',
  }[size]

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/50 backdrop-blur-sm"
        onClick={onClose}
      />

      {/* Modal */}
      <div className={cn(
        'relative bg-white dark:bg-slate-800 rounded-xl shadow-xl w-full',
        sizeClass
      )}>
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700">
          <h3 className="text-lg font-semibold text-slate-900 dark:text-slate-100">
            {title}
          </h3>
          <button
            onClick={onClose}
            className="p-1 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-colors"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Content */}
        <div className="px-6 py-4">
          {children}
        </div>
      </div>
    </div>
  )
}

// ==================== Color and Icon Selectors ====================

interface ColorSelectorProps {
  value: string
  onChange: (color: string) => void
  label?: string
}

const COLOR_OPTIONS = [
  { name: 'Emerald', value: '#10b981', class: 'bg-emerald-500' },
  { name: 'Blue', value: '#3b82f6', class: 'bg-blue-500' },
  { name: 'Purple', value: '#8b5cf6', class: 'bg-purple-500' },
  { name: 'Pink', value: '#ec4899', class: 'bg-pink-500' },
  { name: 'Orange', value: '#f97316', class: 'bg-orange-500' },
  { name: 'Red', value: '#ef4444', class: 'bg-red-500' },
  { name: 'Yellow', value: '#eab308', class: 'bg-yellow-500' },
  { name: 'Slate', value: '#64748b', class: 'bg-slate-500' },
]

function ColorSelector({ value, onChange, label = '分组颜色' }: ColorSelectorProps) {
  return (
    <div>
      <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
        {label}
      </label>
      <div className="flex flex-wrap gap-2">
        {COLOR_OPTIONS.map((color) => (
          <button
            key={color.value}
            type="button"
            onClick={() => onChange(color.value)}
            className={cn(
              'w-8 h-8 rounded-lg transition-all',
              color.class,
              value === color.value
                ? 'ring-2 ring-offset-2 ring-emerald-500 dark:ring-offset-slate-800 scale-110'
                : 'hover:scale-105'
            )}
            title={color.name}
          />
        ))}
      </div>
    </div>
  )
}

interface IconSelectorProps {
  value: string
  onChange: (icon: string) => void
  label?: string
}

const ICON_OPTIONS = [
  { name: 'Star', value: '⭐' },
  { name: 'Heart', value: '❤️' },
  { name: 'Fire', value: '🔥' },
  { name: 'Diamond', value: '💎' },
  { name: 'Rocket', value: '🚀' },
  { name: 'Bolt', value: '⚡' },
  { name: 'Briefcase', value: '💼' },
  { name: 'Family', value: '👨‍👩‍👧‍👦' },
  { name: 'Users', value: '👥' },
  { name: 'Flag', value: '🏁' },
  { name: 'Home', value: '🏠' },
  { name: 'Work', value: '💻' },
]

function IconSelector({ value, onChange, label = '分组图标' }: IconSelectorProps) {
  return (
    <div>
      <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
        {label}
      </label>
      <div className="flex flex-wrap gap-2">
        {ICON_OPTIONS.map((icon) => (
          <button
            key={icon.value}
            type="button"
            onClick={() => onChange(icon.value)}
            className={cn(
              'w-10 h-10 rounded-lg flex items-center justify-center text-lg transition-all',
              value === icon.value
                ? 'bg-emerald-100 dark:bg-emerald-900/20 ring-2 ring-emerald-500 scale-110'
                : 'bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700'
            )}
            title={icon.name}
          >
            {icon.value}
          </button>
        ))}
      </div>
    </div>
  )
}

// ==================== Create Group Dialog ====================

export interface CreateGroupDialogProps {
  isOpen: boolean
  onClose: () => void
  onSubmit: (data: { name: string; color?: string; icon?: string }) => Promise<void>
  isLoading?: boolean
}

export function CreateGroupDialog({
  isOpen,
  onClose,
  onSubmit,
  isLoading = false,
}: CreateGroupDialogProps) {
  const [name, setName] = useState('')
  const [color, setColor] = useState(COLOR_OPTIONS[0].value)
  const [icon, setIcon] = useState(ICON_OPTIONS[0].value)
  const [error, setError] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')

    if (!name.trim()) {
      setError('请输入分组名称')
      return
    }

    try {
      await onSubmit({
        name: name.trim(),
        color,
        icon,
      })
      // Reset form on success
      setName('')
      setColor(COLOR_OPTIONS[0].value)
      setIcon(ICON_OPTIONS[0].value)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : '创建分组失败')
    }
  }

  // Reset form when dialog closes
  const handleClose = useCallback(() => {
    setName('')
    setColor(COLOR_OPTIONS[0].value)
    setIcon(ICON_OPTIONS[0].value)
    setError('')
    onClose()
  }, [onClose])

  return (
    <Modal isOpen={isOpen} onClose={handleClose} title="创建新分组" size="md">
      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Group name */}
        <div>
          <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1.5">
            分组名称 <span className="text-red-500">*</span>
          </label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="例如：家人、同事、好友..."
            className="w-full px-3 py-2 bg-slate-100 dark:bg-slate-900 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 placeholder-slate-400 dark:placeholder-slate-500 focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
            maxLength={50}
            autoFocus
          />
          <div className="mt-1 flex items-center justify-between">
            <p className="text-xs text-slate-500 dark:text-slate-400">
              {name.length}/50
            </p>
            {error && (
              <p className="text-xs text-red-500">{error}</p>
            )}
          </div>
        </div>

        {/* Color selector */}
        <ColorSelector value={color} onChange={setColor} />

        {/* Icon selector */}
        <IconSelector value={icon} onChange={setIcon} />

        {/* Preview */}
        <div className="p-4 bg-slate-50 dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-700">
          <label className="block text-xs font-medium text-slate-500 dark:text-slate-400 mb-3">
            预览
          </label>
          <div className="flex items-center gap-3">
            <div
              className="w-12 h-12 rounded-xl flex items-center justify-center text-2xl shadow-sm"
              style={{ backgroundColor: color }}
            >
              {icon}
            </div>
            <div>
              <p className="font-semibold text-slate-900 dark:text-slate-100">
                {name || '分组名称'}
              </p>
              <p className="text-sm text-slate-500 dark:text-slate-400">
                新建分组
              </p>
            </div>
          </div>
        </div>

        {/* Actions */}
        <div className="flex justify-end gap-3 pt-2">
          <button
            type="button"
            onClick={handleClose}
            disabled={isLoading}
            className="px-4 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-600 transition-colors disabled:opacity-50"
          >
            取消
          </button>
          <button
            type="submit"
            disabled={!name.trim() || isLoading}
            className="px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 transition-colors disabled:opacity-50"
          >
            {isLoading ? '创建中...' : '创建分组'}
          </button>
        </div>
      </form>
    </Modal>
  )
}

// ==================== Edit Group Dialog ====================

export interface EditGroupDialogProps {
  isOpen: boolean
  onClose: () => void
  group: ContactGroup | null
  onSubmit: (id: number, data: { name?: string; color?: string; icon?: string }) => Promise<void>
  isLoading?: boolean
}

export function EditGroupDialog({
  isOpen,
  onClose,
  group,
  onSubmit,
  isLoading = false,
}: EditGroupDialogProps) {
  const [name, setName] = useState('')
  const [color, setColor] = useState(COLOR_OPTIONS[0].value)
  const [icon, setIcon] = useState(ICON_OPTIONS[0].value)
  const [error, setError] = useState('')

  // Initialize form when group changes
  useState(() => {
    if (group) {
      setName(group.name)
      setColor(group.color || COLOR_OPTIONS[0].value)
      setIcon(group.icon || ICON_OPTIONS[0].value)
    }
  })

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')

    if (!group) return
    if (!name.trim()) {
      setError('请输入分组名称')
      return
    }

    try {
      await onSubmit(group.id, {
        name: name.trim(),
        color,
        icon,
      })
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : '更新分组失败')
    }
  }

  const handleClose = useCallback(() => {
    setError('')
    onClose()
  }, [onClose])

  if (!group) return null

  return (
    <Modal isOpen={isOpen} onClose={handleClose} title="编辑分组" size="md">
      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Group name */}
        <div>
          <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1.5">
            分组名称 <span className="text-red-500">*</span>
          </label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full px-3 py-2 bg-slate-100 dark:bg-slate-900 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
            maxLength={50}
            autoFocus
          />
          {error && (
            <p className="mt-1 text-xs text-red-500">{error}</p>
          )}
        </div>

        {/* Color selector */}
        <ColorSelector value={color} onChange={setColor} />

        {/* Icon selector */}
        <IconSelector value={icon} onChange={setIcon} />

        {/* Preview */}
        <div className="p-4 bg-slate-50 dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-700">
          <label className="block text-xs font-medium text-slate-500 dark:text-slate-400 mb-3">
            预览
          </label>
          <div className="flex items-center gap-3">
            <div
              className="w-12 h-12 rounded-xl flex items-center justify-center text-2xl shadow-sm"
              style={{ backgroundColor: color }}
            >
              {icon}
            </div>
            <div>
              <p className="font-semibold text-slate-900 dark:text-slate-100">
                {name}
              </p>
              <p className="text-sm text-slate-500 dark:text-slate-400">
                {group.memberCount} 位成员
              </p>
            </div>
          </div>
        </div>

        {/* Actions */}
        <div className="flex justify-end gap-3 pt-2">
          <button
            type="button"
            onClick={handleClose}
            disabled={isLoading}
            className="px-4 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-600 transition-colors disabled:opacity-50"
          >
            取消
          </button>
          <button
            type="submit"
            disabled={!name.trim() || isLoading}
            className="px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 transition-colors disabled:opacity-50"
          >
            {isLoading ? '保存中...' : '保存更改'}
          </button>
        </div>
      </form>
    </Modal>
  )
}

// ==================== Manage Group Members Dialog ====================

export interface ManageGroupMembersDialogProps {
  isOpen: boolean
  onClose: () => void
  group: ContactGroup | null
  allContacts: Contact[]
  onAddMembers: (groupId: number, contactIds: number[]) => Promise<void>
  onRemoveMembers: (groupId: number, contactIds: number[]) => Promise<void>
  isLoading?: boolean
}

export function ManageGroupMembersDialog({
  isOpen,
  onClose,
  group,
  allContacts,
  onAddMembers,
  onRemoveMembers,
  isLoading = false,
}: ManageGroupMembersDialogProps) {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedContactIds, setSelectedContactIds] = useState<Set<number>>(new Set())

  // Get current members and non-members
  const { members, nonMembers } = useMemo(() => {
    if (!group) return { members: [], nonMembers: [] }

    const members = allContacts.filter(c => c.groups.includes(group.name))
    const nonMembers = allContacts.filter(c => !c.groups.includes(group.name))
    return { members, nonMembers }
  }, [group, allContacts])

  // Filter by search query
  const filteredNonMembers = useMemo(() => {
    if (!searchQuery) return nonMembers
    const query = searchQuery.toLowerCase()
    return nonMembers.filter(c =>
      c.name.toLowerCase().includes(query) ||
      c.nickname?.toLowerCase().includes(query) ||
      c.department?.toLowerCase().includes(query)
    )
  }, [nonMembers, searchQuery])

  const handleToggleContact = useCallback((contactId: number) => {
    setSelectedContactIds(prev => {
      const next = new Set(prev)
      if (next.has(contactId)) {
        next.delete(contactId)
      } else {
        next.add(contactId)
      }
      return next
    })
  }, [])

  const handleAddSelected = async () => {
    if (!group || selectedContactIds.size === 0) return
    await onAddMembers(group.id, Array.from(selectedContactIds))
    setSelectedContactIds(new Set())
    setSearchQuery('')
  }

  const handleRemoveMember = async (contactId: number) => {
    if (!group) return
    if (window.confirm('确定要将此联系人从分组中移除吗？')) {
      await onRemoveMembers(group.id, [contactId])
    }
  }

  if (!group) return null

  return (
    <Modal isOpen={isOpen} onClose={onClose} title={`管理成员 - ${group.name}`} size="lg">
      <div className="space-y-4">
        {/* Current members */}
        <div>
          <h4 className="text-sm font-semibold text-slate-900 dark:text-slate-100 mb-2">
            当前成员 ({members.length})
          </h4>
          {members.length === 0 ? (
            <p className="text-sm text-slate-500 dark:text-slate-400 py-4 text-center">
              暂无成员
            </p>
          ) : (
            <div className="max-h-48 overflow-y-auto space-y-1">
              {members.map((contact) => (
                <div
                  key={contact.id}
                  className="flex items-center justify-between p-2 bg-slate-50 dark:bg-slate-900 rounded-lg"
                >
                  <div className="flex items-center gap-2">
                    {contact.avatar ? (
                      <img
                        src={contact.avatar}
                        alt={contact.name}
                        className="w-8 h-8 rounded-full object-cover"
                      />
                    ) : (
                      <div className="w-8 h-8 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center text-xs font-bold">
                        {contact.name.slice(0, 2).toUpperCase()}
                      </div>
                    )}
                    <div>
                      <p className="text-sm font-medium text-slate-900 dark:text-slate-100">
                        {contact.nickname || contact.name}
                      </p>
                      {contact.department && (
                        <p className="text-xs text-slate-500 dark:text-slate-400">
                          {contact.department}
                        </p>
                      )}
                    </div>
                  </div>
                  <button
                    onClick={() => handleRemoveMember(contact.id)}
                    disabled={isLoading}
                    className="p-1.5 text-slate-400 hover:text-red-600 dark:hover:text-red-400 transition-colors disabled:opacity-50"
                    title="移除"
                  >
                    <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Add members */}
        <div className="border-t border-slate-200 dark:border-slate-700 pt-4">
          <h4 className="text-sm font-semibold text-slate-900 dark:text-slate-100 mb-2">
            添加成员
          </h4>

          {/* Search */}
          <div className="relative mb-3">
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="搜索联系人..."
              className="w-full pl-9 pr-3 py-2 text-sm bg-slate-100 dark:bg-slate-900 border border-slate-300 dark:border-slate-700 rounded-lg focus:ring-2 focus:ring-emerald-500"
            />
            <svg
              className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>

          {/* Contact list with checkboxes */}
          <div className="max-h-64 overflow-y-auto space-y-1">
            {filteredNonMembers.length === 0 ? (
              <p className="text-sm text-slate-500 dark:text-slate-400 py-4 text-center">
                {searchQuery ? '未找到匹配的联系人' : '所有联系人已在分组中'}
              </p>
            ) : (
              filteredNonMembers.map((contact) => (
                <label
                  key={contact.id}
                  className={cn(
                    'flex items-center gap-3 p-2 rounded-lg cursor-pointer transition-colors',
                    selectedContactIds.has(contact.id)
                      ? 'bg-emerald-50 dark:bg-emerald-900/20'
                      : 'hover:bg-slate-50 dark:hover:bg-slate-900'
                  )}
                >
                  <input
                    type="checkbox"
                    checked={selectedContactIds.has(contact.id)}
                    onChange={() => handleToggleContact(contact.id)}
                    className="w-4 h-4 text-emerald-600 rounded border-slate-300 focus:ring-emerald-500"
                  />
                  {contact.avatar ? (
                    <img
                      src={contact.avatar}
                      alt={contact.name}
                      className="w-8 h-8 rounded-full object-cover"
                    />
                  ) : (
                    <div className="w-8 h-8 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center text-xs font-bold">
                      {contact.name.slice(0, 2).toUpperCase()}
                    </div>
                  )}
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-slate-900 dark:text-slate-100 truncate">
                      {contact.nickname || contact.name}
                    </p>
                    {contact.department && (
                      <p className="text-xs text-slate-500 dark:text-slate-400 truncate">
                        {contact.department}
                      </p>
                    )}
                  </div>
                </label>
              ))
            )}
          </div>

          {/* Add selected button */}
          {selectedContactIds.size > 0 && (
            <div className="flex items-center justify-between mt-3 pt-3 border-t border-slate-200 dark:border-slate-700">
              <span className="text-sm text-slate-600 dark:text-slate-400">
                已选择 {selectedContactIds.size} 位联系人
              </span>
              <button
                onClick={handleAddSelected}
                disabled={isLoading}
                className="px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 transition-colors disabled:opacity-50"
              >
                添加到分组
              </button>
            </div>
          )}
        </div>
      </div>
    </Modal>
  )
}
