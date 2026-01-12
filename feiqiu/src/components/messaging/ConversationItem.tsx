import { Conversation } from '../../lib/types/messaging'
import { cn } from '../../lib/utils'

interface ConversationItemProps {
  conversation: Conversation
  isActive: boolean
  onClick: () => void
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

export function ConversationItem({ conversation, isActive, onClick }: ConversationItemProps) {
  const isGroup = conversation.type === 'group'
  const displayName = isGroup ? conversation.group?.name : conversation.participant?.name
  const displayAvatar = isGroup ? conversation.group?.avatar : conversation.participant?.avatar
  const lastMessageTime = conversation.lastMessage ? formatTime(conversation.lastMessage.timestamp) : ''
  const userStatus = !isGroup ? conversation.participant?.status : undefined

  const containerClass = cn(
    'flex items-center gap-3 px-4 py-3.5 cursor-pointer transition-all duration-200 border-l-2 border-transparent',
    isActive
      ? 'bg-emerald-500 dark:bg-emerald-600 border-l-emerald-600 dark:border-l-emerald-500 shadow-lg shadow-emerald-500/10 dark:shadow-emerald-500/20'
      : 'hover:bg-slate-100/80 dark:hover:bg-slate-800/80 border-l-transparent'
  )

  const nameClass = cn(
    'font-bold text-sm truncate pr-2',
    isActive
      ? 'text-white dark:text-white'
      : 'text-slate-900 dark:text-slate-100'
  )

  const timeClass = cn(
    'text-xs flex-shrink-0 font-medium',
    isActive
      ? 'text-emerald-100 dark:text-emerald-200'
      : 'text-slate-500 dark:text-slate-400'
  )

  const messageClass = cn(
    'text-xs truncate font-medium',
    isActive
      ? 'text-emerald-100 dark:text-emerald-200 opacity-90'
      : 'text-slate-500 dark:text-slate-400'
  )

  const badgeClass = cn(
    'flex-shrink-0 px-2 py-0.5 rounded-full text-xs font-bold',
    isActive
      ? 'bg-white text-emerald-600 dark:bg-slate-200 dark:text-emerald-700 shadow-sm'
      : 'bg-emerald-500 text-white dark:bg-emerald-600 shadow-md shadow-emerald-500/20'
  )

  const statusColor = cn({
    'bg-emerald-500 ring-2 ring-emerald-500/30': userStatus === 'online',
    'bg-amber-500 ring-2 ring-amber-500/30': userStatus === 'away',
    'bg-slate-400 ring-2 ring-slate-400/30': userStatus === 'offline',
  })

  return (
    <div onClick={onClick} className={containerClass}>
      <div className="relative flex-shrink-0">
        <img
          src={displayAvatar}
          alt={displayName}
          className="w-12 h-12 rounded-xl object-cover ring-2 ring-white dark:ring-slate-900/50 shadow-sm"
        />
        {userStatus && (
          <span className={cn(
            'absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 border-2 border-white dark:border-slate-900 rounded-full',
            statusColor
          )} />
        )}
      </div>

      <div className="flex-1 min-w-0">
        <div className="flex items-center justify-between mb-1">
          <h4 className={nameClass}>
            {conversation.pinned && <span className="mr-1 text-xs">📌</span>}
            {displayName}
          </h4>
          <span className={timeClass}>{lastMessageTime}</span>
        </div>

        <div className="flex items-center justify-between gap-2">
          {conversation.lastMessage ? (
            <p className={messageClass}>
              {conversation.lastMessage.type === 'image' && <span className="mr-1">📷</span>}
              {conversation.lastMessage.senderId !== 'user-1' && (
                <span className="font-bold mr-1">
                  {isGroup ? `${conversation.lastMessage.senderName}: ` : ''}
                </span>
              )}
              {conversation.lastMessage.content}
            </p>
          ) : (
            <p className={messageClass}>暂无消息</p>
          )}

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
