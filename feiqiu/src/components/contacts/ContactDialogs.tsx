/**
 * Contact Dialogs Component
 *
 * Modal dialogs for creating, editing contacts and viewing contact details.
 */

import { useState, useCallback, useMemo } from 'react'
import { Contact, ContactGroup, CreateContactInput, UpdateContactInput } from '@/lib/types/contacts'
import { cn } from '@/lib/utils'

// ==================== Modal Component ====================

interface ModalProps {
  isOpen: boolean
  onClose: () => void
  title: string
  children: React.ReactNode
  size?: 'sm' | 'md' | 'lg' | 'xl'
  footer?: React.ReactNode
}

function Modal({ isOpen, onClose, title, children, size = 'md', footer }: ModalProps) {
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
        'relative bg-white dark:bg-slate-800 rounded-xl shadow-xl w-full flex flex-col max-h-[90vh]',
        sizeClass
      )}>
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700 flex-shrink-0">
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
        <div className="px-6 py-4 overflow-y-auto flex-1">
          {children}
        </div>

        {/* Footer */}
        {footer && (
          <div className="px-6 py-4 border-t border-slate-200 dark:border-slate-700 flex-shrink-0">
            {footer}
          </div>
        )}
      </div>
    </div>
  )
}

// ==================== Form Components ====================

interface FormFieldProps {
  label: string
  required?: boolean
  error?: string
  children: React.ReactNode
  helperText?: string
}

function FormField({ label, required, error, children, helperText }: FormFieldProps) {
  return (
    <div>
      <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1.5">
        {label}
        {required && <span className="text-red-500 ml-1">*</span>}
      </label>
      {children}
      <div className="mt-1 flex items-center justify-between">
        {helperText && (
          <p className="text-xs text-slate-500 dark:text-slate-400">{helperText}</p>
        )}
        {error && (
          <p className="text-xs text-red-500 ml-auto">{error}</p>
        )}
      </div>
    </div>
  )
}

interface TextInputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size'> {
  label?: string
  error?: string
  helperText?: string
  required?: boolean
}

function TextInput({ label, error, helperText, required, className, ...props }: TextInputProps) {
  const inputElement = (
    <input
      {...props}
      className={cn(
        'w-full px-3 py-2 bg-slate-100 dark:bg-slate-900 border rounded-lg text-sm text-slate-900 dark:text-slate-100 placeholder-slate-400 dark:placeholder-slate-500 focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors',
        error
          ? 'border-red-300 dark:border-red-700 focus:ring-red-500'
          : 'border-slate-300 dark:border-slate-700',
        className
      )}
    />
  )

  if (!label) return inputElement

  return (
    <FormField label={label} required={required} error={error} helperText={helperText}>
      {inputElement}
    </FormField>
  )
}

// ==================== Create Contact Dialog ====================

export interface CreateContactDialogProps {
  isOpen: boolean
  onClose: () => void
  onSubmit: (data: CreateContactInput) => Promise<void>
  groups?: ContactGroup[]
  isLoading?: boolean
}

export function CreateContactDialog({
  isOpen,
  onClose,
  onSubmit,
  groups = [],
  isLoading = false,
}: CreateContactDialogProps) {
  const [name, setName] = useState('')
  const [nickname, setNickname] = useState('')
  const [phone, setPhone] = useState('')
  const [email, setEmail] = useState('')
  const [department, setDepartment] = useState('')
  const [position, setPosition] = useState('')
  const [notes, setNotes] = useState('')
  const [selectedGroups, setSelectedGroups] = useState<Set<number>>(new Set())
  const [error, setError] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')

    if (!name.trim()) {
      setError('请输入联系人姓名')
      return
    }

    try {
      const data: CreateContactInput = {
        name: name.trim(),
        nickname: nickname.trim() || undefined,
        phone: phone.trim() || undefined,
        email: email.trim() || undefined,
        department: department.trim() || undefined,
        position: position.trim() || undefined,
        notes: notes.trim() || undefined,
        pinyin: undefined, // Backend will generate
      }

      await onSubmit(data)

      // Reset form on success
      setName('')
      setNickname('')
      setPhone('')
      setEmail('')
      setDepartment('')
      setPosition('')
      setNotes('')
      setSelectedGroups(new Set())
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : '创建联系人失败')
    }
  }

  const handleClose = useCallback(() => {
    setName('')
    setNickname('')
    setPhone('')
    setEmail('')
    setDepartment('')
    setPosition('')
    setNotes('')
    setSelectedGroups(new Set())
    setError('')
    onClose()
  }, [onClose])

  const footer = (
    <div className="flex justify-end gap-3">
      <button
        type="button"
        onClick={handleClose}
        disabled={isLoading}
        className="px-4 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-600 transition-colors disabled:opacity-50"
      >
        取消
      </button>
      <button
        onClick={handleSubmit}
        disabled={!name.trim() || isLoading}
        className="px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 transition-colors disabled:opacity-50"
      >
        {isLoading ? '创建中...' : '创建'}
      </button>
    </div>
  )

  return (
    <Modal isOpen={isOpen} onClose={handleClose} title="新建联系人" size="lg" footer={footer}>
      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Name */}
        <TextInput
          label="姓名"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="输入联系人姓名"
          required
          maxLength={50}
          autoFocus
        />

        {/* Nickname */}
        <TextInput
          label="昵称"
          value={nickname}
          onChange={(e) => setNickname(e.target.value)}
          placeholder="输入昵称（可选）"
          maxLength={50}
        />

        {/* Phone & Email in one row */}
        <div className="grid grid-cols-2 gap-4">
          <TextInput
            label="电话"
            value={phone}
            onChange={(e) => setPhone(e.target.value)}
            placeholder="输入电话号码"
            type="tel"
          />
          <TextInput
            label="邮箱"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="输入邮箱地址"
            type="email"
          />
        </div>

        {/* Department & Position in one row */}
        <div className="grid grid-cols-2 gap-4">
          <TextInput
            label="部门"
            value={department}
            onChange={(e) => setDepartment(e.target.value)}
            placeholder="输入部门名称"
            list="departments"
          />
          <TextInput
            label="职位"
            value={position}
            onChange={(e) => setPosition(e.target.value)}
            placeholder="输入职位"
          />
        </div>

        {/* Notes */}
        <div>
          <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1.5">
            备注
          </label>
          <textarea
            value={notes}
            onChange={(e) => setNotes(e.target.value)}
            placeholder="添加备注信息..."
            rows={3}
            maxLength={500}
            className="w-full px-3 py-2 bg-slate-100 dark:bg-slate-900 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 placeholder-slate-400 dark:placeholder-slate-500 focus:ring-2 focus:ring-emerald-500 focus:border-transparent resize-none"
          />
          <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">
            {notes.length}/500
          </p>
        </div>

        {/* Group selection */}
        {groups.length > 0 && (
          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
              添加到分组
            </label>
            <div className="flex flex-wrap gap-2">
              {groups.map((group) => (
                <button
                  key={group.id}
                  type="button"
                  onClick={() => {
                    setSelectedGroups(prev => {
                      const next = new Set(prev)
                      if (next.has(group.id)) next.delete(group.id)
                      else next.add(group.id)
                      return next
                    })
                  }}
                  className={cn(
                    'px-3 py-1.5 text-sm rounded-lg border transition-colors',
                    selectedGroups.has(group.id)
                      ? 'border-emerald-500 bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400'
                      : 'border-slate-300 dark:border-slate-600 hover:border-emerald-500 text-slate-600 dark:text-slate-400'
                  )}
                >
                  {group.icon && <span className="mr-1">{group.icon}</span>}
                  {group.name}
                </button>
              ))}
            </div>
          </div>
        )}

        {error && (
          <div className="p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
            <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
          </div>
        )}
      </form>
    </Modal>
  )
}

// ==================== Edit Contact Dialog ====================

export interface EditContactDialogProps {
  isOpen: boolean
  onClose: () => void
  contact: Contact | null
  onSubmit: (id: number, data: UpdateContactInput) => Promise<void>
  groups?: ContactGroup[]
  isLoading?: boolean
}

export function EditContactDialog({
  isOpen,
  onClose,
  contact,
  onSubmit,
  groups = [],
  isLoading = false,
}: EditContactDialogProps) {
  const [name, setName] = useState('')
  const [nickname, setNickname] = useState('')
  const [phone, setPhone] = useState('')
  const [email, setEmail] = useState('')
  const [department, setDepartment] = useState('')
  const [position, setPosition] = useState('')
  const [notes, setNotes] = useState('')
  const [isFavorite, setIsFavorite] = useState(false)
  const [error, setError] = useState('')

  // Initialize form when contact changes
  useState(() => {
    if (contact) {
      setName(contact.name)
      setNickname(contact.nickname || '')
      setPhone(contact.phone || '')
      setEmail(contact.email || '')
      setDepartment(contact.department || '')
      setPosition(contact.position || '')
      setNotes(contact.notes || '')
      setIsFavorite(contact.isFavorite)
    }
  })

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')

    if (!contact) return
    if (!name.trim()) {
      setError('请输入联系人姓名')
      return
    }

    try {
      const data: UpdateContactInput = {
        name: name.trim(),
        nickname: nickname.trim() || undefined,
        phone: phone.trim() || undefined,
        email: email.trim() || undefined,
        department: department.trim() || undefined,
        position: position.trim() || undefined,
        notes: notes.trim() || undefined,
        isFavorite,
      }

      await onSubmit(contact.id, data)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : '更新联系人失败')
    }
  }

  const handleDelete = useCallback(async () => {
    if (!contact) return
    if (window.confirm(`确定要删除联系人 "${contact.name}" 吗？此操作不可恢复。`)) {
      // This would be handled by parent component
      onClose()
    }
  }, [contact, onClose])

  if (!contact) return null

  const footer = (
    <div className="flex items-center justify-between">
      <button
        type="button"
        onClick={handleDelete}
        disabled={isLoading}
        className="px-4 py-2 text-sm font-medium text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors disabled:opacity-50"
      >
        删除联系人
      </button>
      <div className="flex gap-3">
        <button
          type="button"
          onClick={onClose}
          disabled={isLoading}
          className="px-4 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-600 transition-colors disabled:opacity-50"
        >
          取消
        </button>
        <button
          onClick={handleSubmit}
          disabled={!name.trim() || isLoading}
          className="px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 transition-colors disabled:opacity-50"
        >
          {isLoading ? '保存中...' : '保存'}
        </button>
      </div>
    </div>
  )

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="编辑联系人" size="lg" footer={footer}>
      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Avatar display */}
        <div className="flex items-center gap-4 p-4 bg-slate-50 dark:bg-slate-900 rounded-lg">
          {contact.avatar ? (
            <img
              src={contact.avatar}
              alt={contact.name}
              className="w-16 h-16 rounded-full object-cover ring-2 ring-white dark:ring-slate-700"
            />
          ) : (
            <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center text-2xl font-bold text-slate-600 dark:text-slate-300">
              {contact.name.slice(0, 2).toUpperCase()}
            </div>
          )}
          <div>
            <p className="font-semibold text-slate-900 dark:text-slate-100">{contact.name}</p>
            <p className="text-sm text-slate-500 dark:text-slate-400">
              {contact.isOnline ? '在线' : '离线'}
            </p>
          </div>
          <button
            type="button"
            onClick={() => setIsFavorite(!isFavorite)}
            className={cn(
              'ml-auto p-2 rounded-lg transition-colors',
              isFavorite
                ? 'text-yellow-500 bg-yellow-50 dark:bg-yellow-900/20'
                : 'text-slate-400 hover:text-yellow-500 hover:bg-slate-100 dark:hover:bg-slate-700'
            )}
            title={isFavorite ? '取消收藏' : '添加收藏'}
          >
            <svg className="w-6 h-6" fill={isFavorite ? 'currentColor' : 'none'} viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.56.57-.218 1.196 1.118 1.518l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.783-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" />
            </svg>
          </button>
        </div>

        {/* Name */}
        <TextInput
          label="姓名"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="输入联系人姓名"
          required
          maxLength={50}
          autoFocus
        />

        {/* Nickname */}
        <TextInput
          label="昵称"
          value={nickname}
          onChange={(e) => setNickname(e.target.value)}
          placeholder="输入昵称（可选）"
          maxLength={50}
        />

        {/* Phone & Email */}
        <div className="grid grid-cols-2 gap-4">
          <TextInput
            label="电话"
            value={phone}
            onChange={(e) => setPhone(e.target.value)}
            placeholder="输入电话号码"
            type="tel"
          />
          <TextInput
            label="邮箱"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="输入邮箱地址"
            type="email"
          />
        </div>

        {/* Department & Position */}
        <div className="grid grid-cols-2 gap-4">
          <TextInput
            label="部门"
            value={department}
            onChange={(e) => setDepartment(e.target.value)}
            placeholder="输入部门名称"
          />
          <TextInput
            label="职位"
            value={position}
            onChange={(e) => setPosition(e.target.value)}
            placeholder="输入职位"
          />
        </div>

        {/* Notes */}
        <div>
          <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1.5">
            备注
          </label>
          <textarea
            value={notes}
            onChange={(e) => setNotes(e.target.value)}
            placeholder="添加备注信息..."
            rows={3}
            maxLength={500}
            className="w-full px-3 py-2 bg-slate-100 dark:bg-slate-900 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 placeholder-slate-400 dark:placeholder-slate-500 focus:ring-2 focus:ring-emerald-500 focus:border-transparent resize-none"
          />
          <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">
            {notes.length}/500
          </p>
        </div>

        {/* Meta info */}
        <div className="p-3 bg-slate-50 dark:bg-slate-900 rounded-lg">
          <p className="text-xs text-slate-500 dark:text-slate-400">
            ID: {contact.id} · 创建于: {new Date(contact.createdAt).toLocaleDateString('zh-CN')}
          </p>
        </div>

        {error && (
          <div className="p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
            <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
          </div>
        )}
      </form>
    </Modal>
  )
}

// ==================== Contact Detail Dialog ====================

export interface ContactDetailDialogProps {
  isOpen: boolean
  onClose: () => void
  contact: Contact | null
  onEdit?: (contact: Contact) => void
  onMessage?: (contact: Contact) => void
  onDelete?: (contact: Contact) => void
  onToggleFavorite?: (contact: Contact) => void
  isLoading?: boolean
}

export function ContactDetailDialog({
  isOpen,
  onClose,
  contact,
  onEdit,
  onMessage,
  onDelete,
  onToggleFavorite,
  isLoading = false,
}: ContactDetailDialogProps) {
  if (!contact) return null

  const handleSendMessage = () => {
    onMessage?.(contact)
  }

  return (
    <Modal isOpen={isOpen} onClose={onClose} title={contact.name} size="md">
      <div className="space-y-6">
        {/* Header with avatar */}
        <div className="flex flex-col items-center text-center">
          {contact.avatar ? (
            <img
              src={contact.avatar}
              alt={contact.name}
              className="w-24 h-24 rounded-full object-cover ring-4 ring-white dark:ring-slate-700 shadow-lg mb-4"
            />
          ) : (
            <div className="w-24 h-24 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center text-3xl font-bold text-slate-600 dark:text-slate-300 mb-4">
              {contact.name.slice(0, 2).toUpperCase()}
            </div>
          )}

          <h3 className="text-xl font-bold text-slate-900 dark:text-slate-100">
            {contact.nickname || contact.name}
          </h3>

          {contact.nickname && (
            <p className="text-sm text-slate-500 dark:text-slate-400">
              昵称: {contact.nickname}
            </p>
          )}

          {/* Status */}
          <div className="flex items-center gap-2 mt-2">
            <span className={cn(
              'inline-flex items-center px-3 py-1 rounded-full text-sm font-medium',
              contact.isOnline
                ? 'bg-emerald-100 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400'
                : 'bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400'
            )}>
              <span className={cn(
                'w-2 h-2 rounded-full mr-2',
                contact.isOnline ? 'bg-emerald-500' : 'bg-slate-400'
              )} />
              {contact.isOnline ? '在线' : '离线'}
            </span>

            {contact.isFavorite && (
              <button
                onClick={() => onToggleFavorite?.(contact)}
                className="p-1.5 bg-yellow-100 dark:bg-yellow-900/20 rounded-lg text-yellow-600 dark:text-yellow-400 hover:bg-yellow-200 dark:hover:bg-yellow-900/30 transition-colors"
                title="收藏"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
                </svg>
              </button>
            )}
          </div>
        </div>

        {/* Contact information */}
        <div className="space-y-4">
          {/* Phone */}
          {contact.phone && (
            <div className="flex items-center gap-3 p-3 bg-slate-50 dark:bg-slate-900 rounded-lg">
              <div className="w-10 h-10 rounded-lg bg-emerald-100 dark:bg-emerald-900/20 flex items-center justify-center text-emerald-600 dark:text-emerald-400">
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 8V5z" />
                </svg>
              </div>
              <div className="flex-1">
                <p className="text-xs text-slate-500 dark:text-slate-400">电话</p>
                <p className="text-sm font-medium text-slate-900 dark:text-slate-100">{contact.phone}</p>
              </div>
            </div>
          )}

          {/* Email */}
          {contact.email && (
            <div className="flex items-center gap-3 p-3 bg-slate-50 dark:bg-slate-900 rounded-lg">
              <div className="w-10 h-10 rounded-lg bg-blue-100 dark:bg-blue-900/20 flex items-center justify-center text-blue-600 dark:text-blue-400">
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                </svg>
              </div>
              <div className="flex-1">
                <p className="text-xs text-slate-500 dark:text-slate-400">邮箱</p>
                <p className="text-sm font-medium text-slate-900 dark:text-slate-100 truncate">{contact.email}</p>
              </div>
            </div>
          )}

          {/* Department */}
          {contact.department && (
            <div className="flex items-center gap-3 p-3 bg-slate-50 dark:bg-slate-900 rounded-lg">
              <div className="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900/20 flex items-center justify-center text-purple-600 dark:text-purple-400">
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                </svg>
              </div>
              <div className="flex-1">
                <p className="text-xs text-slate-500 dark:text-slate-400">部门</p>
                <p className="text-sm font-medium text-slate-900 dark:text-slate-100">
                  {contact.department}
                  {contact.position && ` · ${contact.position}`}
                </p>
              </div>
            </div>
          )}

          {/* Groups */}
          {contact.groups.length > 0 && (
            <div className="flex items-start gap-3 p-3 bg-slate-50 dark:bg-slate-900 rounded-lg">
              <div className="w-10 h-10 rounded-lg bg-orange-100 dark:bg-orange-900/20 flex items-center justify-center text-orange-600 dark:text-orange-400 flex-shrink-0">
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
                </svg>
              </div>
              <div className="flex-1 min-w-0">
                <p className="text-xs text-slate-500 dark:text-slate-400 mb-1">分组</p>
                <div className="flex flex-wrap gap-1.5">
                  {contact.groups.map((group, index) => (
                    <span
                      key={index}
                      className="inline-flex items-center px-2 py-0.5 bg-white dark:bg-slate-800 rounded-md text-xs font-medium text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-slate-700"
                    >
                      {group}
                    </span>
                  ))}
                </div>
              </div>
            </div>
          )}

          {/* Notes */}
          {contact.notes && (
            <div className="p-3 bg-slate-50 dark:bg-slate-900 rounded-lg">
              <p className="text-xs text-slate-500 dark:text-slate-400 mb-1">备注</p>
              <p className="text-sm text-slate-700 dark:text-slate-300 whitespace-pre-wrap">{contact.notes}</p>
            </div>
          )}

          {/* Last seen */}
          {contact.lastSeen && !contact.isOnline && (
            <p className="text-xs text-slate-500 dark:text-slate-400 text-center">
              上次在线: {new Date(contact.lastSeen).toLocaleString('zh-CN')}
            </p>
          )}
        </div>

        {/* Action buttons */}
        <div className="flex justify-center gap-3 pt-4 border-t border-slate-200 dark:border-slate-700">
          <button
            onClick={handleSendMessage}
            className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
            </svg>
            发消息
          </button>
          <button
            onClick={() => onEdit?.(contact)}
            className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-600 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
            编辑
          </button>
        </div>
      </div>
    </Modal>
  )
}
