/**
 * ContactItem Component
 *
 * Displays a single contact in a list with avatar, name, status, and actions.
 */

import { memo } from 'react'
import { Contact } from '@/lib/types/contacts'
import { cn } from '@/lib/utils'
import { getContactDisplayName } from '@/lib/api'

interface ContactItemProps {
  /** Contact data */
  contact: Contact
  /** Whether this contact is selected */
  isSelected?: boolean
  /** Whether to show checkbox for batch operations */
  showCheckbox?: boolean
  /** Whether to show favorite star */
  showFavorite?: boolean
  /** Click handler */
  onClick?: () => void
  /** Selection toggle handler */
  onToggleSelect?: () => void
  /** Favorite toggle handler */
  onToggleFavorite?: () => void
  /** Additional CSS classes */
  className?: string
}

/**
 * Format timestamp for display
 */
function formatLastSeen(timestamp?: number): string {
  if (!timestamp) return '未知'

  const date = new Date(timestamp)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMs / 3600000)
  const diffDays = Math.floor(diffMs / 86400000)

  if (diffMins < 1) return '刚刚在线'
  if (diffMins < 60) return `${diffMins}分钟前在线`
  if (diffHours < 24) return `${diffHours}小时前在线`
  if (diffDays < 7) return `${diffDays}天前在线`

  const month = date.getMonth() + 1
  const day = date.getDate()
  return `${month}月${day}日`
}

/**
 * Get initials for avatar fallback
 */
function getInitials(name: string): string {
  const trimmed = name.trim()
  if (trimmed.length === 0) return '?'

  // For Chinese names, take first 2 characters
  if (/[\u4e00-\u9fa5]/.test(trimmed)) {
    return trimmed.slice(0, 2).toUpperCase()
  }

  // For Western names, take first letter of each word
  const parts = trimmed.split(/\s+/)
  if (parts.length >= 2) {
    return (parts[0][0] + parts[1][0]).toUpperCase()
  }
  return trimmed.slice(0, 2).toUpperCase()
}

export const ContactItem = memo(function ContactItem({
  contact,
  isSelected = false,
  showCheckbox = false,
  showFavorite = true,
  onClick,
  onToggleSelect,
  onToggleFavorite,
  className,
}: ContactItemProps) {
  const displayName = getContactDisplayName(contact)
  const initials = getInitials(displayName)
  const lastSeenText = formatLastSeen(contact.lastSeen)
  const isOnline = contact.isOnline

  // Status indicator color
  const statusColor = cn(
    'absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 border-2 border-white dark:border-slate-900 rounded-full',
    isOnline
      ? 'bg-emerald-500 ring-2 ring-emerald-500/30'
      : 'bg-slate-400 ring-2 ring-slate-400/30'
  )

  // Container styles
  const containerClass = cn(
    'flex items-center gap-3 px-4 py-3 transition-all duration-200 border-l-2',
    isSelected
      ? 'bg-emerald-500 dark:bg-emerald-600 border-l-emerald-600 dark:border-l-emerald-500 shadow-lg shadow-emerald-500/10 dark:shadow-emerald-500/20'
      : 'hover:bg-slate-100/80 dark:hover:bg-slate-800/80 border-l-transparent cursor-pointer',
    onClick && !isSelected && 'cursor-pointer',
    className
  )

  // Name text color
  const nameClass = cn(
    'font-bold text-sm truncate',
    isSelected
      ? 'text-white dark:text-white'
      : 'text-slate-900 dark:text-slate-100'
  )

  // Department/info text color
  const infoClass = cn(
    'text-xs truncate',
    isSelected
      ? 'text-emerald-100 dark:text-emerald-200 opacity-90'
      : 'text-slate-500 dark:text-slate-400'
  )

  // Status text color
  const statusClass = cn(
    'text-xs flex-shrink-0 font-medium',
    isSelected
      ? 'text-emerald-100 dark:text-emerald-200'
      : isOnline
      ? 'text-emerald-600 dark:text-emerald-400'
      : 'text-slate-400 dark:text-slate-500'
  )

  // Avatar styles
  const avatarClass = cn(
    'w-12 h-12 rounded-xl object-cover ring-2 ring-white dark:ring-slate-900/50 shadow-sm flex items-center justify-center font-bold text-sm',
    isSelected
      ? 'bg-emerald-600 text-white ring-transparent'
      : 'bg-slate-200 dark:bg-slate-700 text-slate-600 dark:text-slate-300'
  )

  const checkboxClass = cn(
    'flex-shrink-0 w-5 h-5 rounded border-2 flex items-center justify-center transition-colors',
    isSelected
      ? 'bg-white dark:bg-slate-200 border-white dark:border-slate-200'
      : 'border-slate-300 dark:border-slate-600 hover:border-emerald-500 dark:hover:border-emerald-400'
  )

  const favoriteIconClass = cn(
    'transition-colors',
    contact.isFavorite
      ? isSelected
        ? 'text-yellow-300 dark:text-yellow-200'
        : 'text-yellow-500 dark:text-yellow-400'
      : isSelected
      ? 'text-emerald-200 dark:text-emerald-300 hover:text-white'
      : 'text-slate-300 dark:text-slate-600 hover:text-yellow-500 dark:hover:text-yellow-400'
  )

  return (
    <div
      className={containerClass}
      onClick={onClick}
      role="listitem"
      aria-label={`${displayName}, ${isOnline ? '在线' : lastSeenText}`}
      aria-selected={isSelected}
    >
      {/* Checkbox for selection */}
      {showCheckbox && (
        <div
          className={checkboxClass}
          onClick={(e) => {
            e.stopPropagation()
            onToggleSelect?.()
          }}
          role="checkbox"
          aria-checked={isSelected}
          aria-label={`选择 ${displayName}`}
          tabIndex={0}
          onKeyDown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault()
              onToggleSelect?.()
            }
          }}
        >
          {isSelected && (
            <svg
              className={isSelected ? 'text-emerald-600 dark:text-emerald-700' : ''}
              width="12"
              height="12"
              viewBox="0 0 12 12"
              fill="none"
              aria-hidden="true"
            >
              <path
                d="M2 6L5 9L10 3"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          )}
        </div>
      )}

      {/* Avatar with status indicator */}
      <div className="relative flex-shrink-0">
        {contact.avatar ? (
          <img
            src={contact.avatar}
            alt={`${displayName} 的头像`}
            className="w-12 h-12 rounded-xl object-cover ring-2 ring-white dark:ring-slate-900/50 shadow-sm"
          />
        ) : (
          <div className={avatarClass} aria-hidden="true">{initials}</div>
        )}
        <span
          className={statusColor}
          aria-hidden="true"
          title={isOnline ? '在线' : lastSeenText}
        />
      </div>

      {/* Contact info */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center justify-between mb-0.5">
          <h4 className={nameClass}>{displayName}</h4>
          <span className={statusClass} aria-live="polite">
            {isOnline ? '在线' : lastSeenText}
          </span>
        </div>

        <div className="flex items-center justify-between gap-2">
          <p className={infoClass}>
            {contact.department && (
              <>
                {contact.department}
                {contact.position && ` · ${contact.position}`}
              </>
            )}
            {!contact.department && !contact.position && (
              <>
                {contact.email || contact.phone || '无部门信息'}
              </>
            )}
          </p>

          {/* Favorite star */}
          {showFavorite && onToggleFavorite && (
            <button
              className={favoriteIconClass}
              onClick={(e) => {
                e.stopPropagation()
                onToggleFavorite()
              }}
              aria-label={contact.isFavorite ? `取消收藏 ${displayName}` : `收藏 ${displayName}`}
              aria-pressed={contact.isFavorite}
              type="button"
            >
              <svg width="16" height="16" viewBox="0 0 16 16" fill={contact.isFavorite ? 'currentColor' : 'none'} aria-hidden="true">
                <path d="M8 2L6.5 5.5L3 6L5.5 8.5L5 12L8 10L11 12L10.5 8.5L13 6L9.5 5.5L8 2Z" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
              </svg>
            </button>
          )}
        </div>
      </div>
    </div>
  )
})
