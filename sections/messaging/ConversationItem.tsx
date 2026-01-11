import type { Conversation } from '@/../product/sections/messaging/types'

interface ConversationItemProps {
  conversation: Conversation
  isActive: boolean
  onClick: () => void
}

export function ConversationItem({ conversation, isActive, onClick }: ConversationItemProps) {
  const isGroup = conversation.type === 'group'
  const displayName = isGroup ? conversation.group?.name : conversation.participant?.name
  const displayAvatar = isGroup ? conversation.group?.avatar : conversation.participant?.avatar
  const lastMessageTime = formatTime(conversation.lastMessage.timestamp)

  // Status indicator for single chat
  const userStatus = !isGroup ? conversation.participant?.status : undefined

  const containerClass = isActive
    ? 'flex items-center gap-3 px-4 py-3 cursor-pointer transition-all duration-200 bg-emerald-500 dark:bg-emerald-600'
    : 'flex items-center gap-3 px-4 py-3 cursor-pointer transition-all duration-200 hover:bg-slate-100 dark:hover:bg-slate-800'

  const nameClass = isActive
    ? 'font-medium text-sm truncate pr-2 text-white dark:text-white'
    : 'font-medium text-sm truncate pr-2 text-slate-900 dark:text-slate-100'

  const timeClass = isActive
    ? 'text-xs flex-shrink-0 text-emerald-100 dark:text-emerald-200'
    : 'text-xs flex-shrink-0 text-slate-500 dark:text-slate-400'

  const messageClass = isActive
    ? 'text-xs truncate text-emerald-100 dark:text-emerald-200'
    : 'text-xs truncate text-slate-500 dark:text-slate-400'

  const badgeClass = isActive
    ? 'flex-shrink-0 px-2 py-0.5 rounded-full text-xs font-medium bg-white text-emerald-600 dark:bg-slate-200 dark:text-emerald-700'
    : 'flex-shrink-0 px-2 py-0.5 rounded-full text-xs font-medium bg-emerald-500 text-white dark:bg-emerald-600'

  return (
    <div onClick={onClick} className={containerClass}>
      {/* Avatar with status indicator */}
      <div className="relative flex-shrink-0">
        <img
          src={displayAvatar}
          alt={displayName}
          className="w-12 h-12 rounded-lg object-cover"
        />
        {userStatus === 'online' && (
          <span className="absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 bg-emerald-500 border-2 border-white dark:border-slate-900 rounded-full" />
        )}
        {userStatus === 'away' && (
          <span className="absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 bg-amber-500 border-2 border-white dark:border-slate-900 rounded-full" />
        )}
      </div>

      {/* Content */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center justify-between mb-0.5">
          <h4 className={nameClass}>
            {conversation.pinned && (
              <span className="mr-1 text-xs">📌</span>
            )}
            {displayName}
          </h4>
          <span className={timeClass}>
            {lastMessageTime}
          </span>
        </div>

        <div className="flex items-center justify-between gap-2">
          <p className={messageClass}>
            {conversation.lastMessage.type === 'image' && (
              <span className="mr-1">📷</span>
            )}
            {conversation.lastMessage.senderId !== 'user-1' && (
              <span className="font-medium mr-1">
                {isGroup ? `${conversation.lastMessage.senderName}: ` : ''}
              </span>
            )}
            {conversation.lastMessage.content}
          </p>

          {conversation.unreadCount > 0 && (
            <span className={badgeClass}>
              {conversation.unreadCount > 99 ? '99+' : conversation.unreadCount}
            </span>
          )}
        </div>
      </div>
    </div>
  )
}

function formatTime(timestamp: string): string {
  const date = new Date(timestamp)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMs / 3600000)
  const diffDays = Math.floor(diffMs / 86400000)

  if (diffMins < 1) return '刚刚'
  if (diffMins < 60) return `${diffMins}分钟前`
  if (diffHours < 24) return `${diffHours}小时前`
  if (diffDays < 7) return `${diffDays}天前`

  const month = date.getMonth() + 1
  const day = date.getDate()
  return `${month}/${day}`
}
