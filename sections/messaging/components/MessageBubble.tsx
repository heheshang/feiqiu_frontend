import type { Message } from '@/../product/sections/messaging/types'

interface MessageBubbleProps {
  message: Message
  isSent: boolean
  showAvatar?: boolean
  avatarUrl?: string
  onReply?: (messageId: string) => void
  onReact?: (messageId: string, emoji: string) => void
  onRetract?: (messageId: string) => void
}

export function MessageBubble({
  message,
  isSent,
  showAvatar,
  avatarUrl,
  onReply,
  onReact,
  onRetract
}: MessageBubbleProps) {
  const messageTime = formatMessageTime(message.timestamp)

  const bubbleClass = isSent
    ? 'bg-emerald-500 text-white rounded-br-sm'
    : 'bg-slate-200 dark:bg-slate-700 text-slate-900 dark:text-slate-100 rounded-bl-sm'

  return (
    <div className={`flex gap-2 mb-4 ${isSent ? 'flex-row-reverse' : 'flex-row'}`}>
      {/* Avatar */}
      {showAvatar && avatarUrl && (
        <img
          src={avatarUrl}
          alt={message.senderName}
          className="w-8 h-8 rounded-full flex-shrink-0 object-cover"
        />
      )}

      {/* Message content */}
      <div className={`max-w-[70%] ${isSent ? 'items-end' : 'items-start'} flex flex-col`}>
        {/* Sender name for group messages */}
        {!isSent && showAvatar && (
          <span className="text-xs text-slate-500 dark:text-slate-400 mb-1 px-1">
            {message.senderName}
          </span>
        )}

        {/* Quote */}
        {message.quote && (
          <div
            className={
              isSent
                ? 'mb-1 px-3 py-2 text-xs rounded-lg border-l-2 bg-emerald-100 dark:bg-emerald-900/30 border-emerald-400 dark:border-emerald-500 text-slate-600 dark:text-slate-300'
                : 'mb-1 px-3 py-2 text-xs rounded-lg border-l-2 bg-slate-100 dark:bg-slate-800 border-slate-300 dark:border-slate-600 text-slate-600 dark:text-slate-300'
            }
          >
            <span className="font-medium">{message.quote.senderName}: </span>
            {message.quote.content}
          </div>
        )}

        {/* Message bubble */}
        <div className={`px-4 py-2 rounded-2xl break-words relative group ${bubbleClass}`}>
          {/* Content */}
          {message.type === 'text' && (
            <p className="text-sm leading-relaxed whitespace-pre-wrap">{message.content}</p>
          )}

          {message.type === 'emoji' && (
            <span className="text-2xl">{message.content}</span>
          )}

          {message.type === 'image' && (
            <div className="rounded-lg overflow-hidden">
              <img
                src={message.imageUrl}
                alt="发送的图片"
                className="max-w-full h-auto"
              />
            </div>
          )}

          {/* Reactions */}
          {message.reactions.length > 0 && (
            <div className="flex gap-1 mt-2 flex-wrap">
              {message.reactions.map((reaction, idx) => (
                <span
                  key={idx}
                  className="px-2 py-0.5 bg-white/20 dark:bg-black/20 rounded-full text-xs backdrop-blur-sm"
                >
                  {reaction.emoji} {reaction.users.length}
                </span>
              ))}
            </div>
          )}

          {/* Hover actions */}
          <div
            className={`absolute ${isSent ? '-left-24' : '-right-24'} top-1/2 -translate-y-1/2 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity`}
          >
            <button
              onClick={() => onReact?.(message.id, '😊')}
              className="px-2 py-1 bg-slate-100 dark:bg-slate-700 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-600 text-sm"
              title="表情"
            >
              😊
            </button>
            <button
              onClick={() => onReply?.(message.id)}
              className="px-2 py-1 bg-slate-100 dark:bg-slate-700 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-600 text-sm"
              title="引用"
            >
              ↩
            </button>
            {isSent && (
              <button
                onClick={() => onRetract?.(message.id)}
                className="px-2 py-1 bg-slate-100 dark:bg-slate-700 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-600 text-sm"
                title="撤回"
              >
                ↩
              </button>
            )}
          </div>
        </div>

        {/* Timestamp and status */}
        <div className={`flex items-center gap-1 mt-1 ${isSent ? 'justify-end' : 'justify-start'}`}>
          <span className="text-xs text-slate-400 dark:text-slate-500">{messageTime}</span>
          {isSent && message.status === 'read' && (
            <span className="text-xs text-slate-400 dark:text-slate-500">已读</span>
          )}
          {isSent && message.status === 'unread' && (
            <span className="text-xs text-slate-400 dark:text-slate-500">未读</span>
          )}
        </div>
      </div>
    </div>
  )
}

function formatMessageTime(timestamp: string): string {
  const date = new Date(timestamp)
  const hours = date.getHours().toString().padStart(2, '0')
  const minutes = date.getMinutes().toString().padStart(2, '0')
  return `${hours}:${minutes}`
}
