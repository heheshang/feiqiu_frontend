import { Message } from '../../lib/types/messaging'
import { cn } from '../../lib/utils'

interface MessageBubbleProps {
  message: Message
  isSent: boolean
  showAvatar?: boolean
  avatarUrl?: string
  onReply?: (messageId: string) => void
  onReact?: (messageId: string, emoji: string) => void
  onRetract?: (messageId: string) => void
}

function formatMessageTime(timestamp: string): string {
  const date = new Date(timestamp)
  const hours = date.getHours().toString().padStart(2, '0')
  const minutes = date.getMinutes().toString().padStart(2, '0')
  return `${hours}:${minutes}`
}

export function MessageBubble({
  message,
  isSent,
  showAvatar,
  avatarUrl,
  onReply,
  onReact,
  onRetract,
}: MessageBubbleProps) {
  const messageTime = formatMessageTime(message.timestamp)

  const bubbleClass = cn(
    'px-4 py-2.5 rounded-2xl break-words relative group shadow-sm',
    isSent
      ? 'bg-emerald-500 text-white rounded-br-md shadow-[0_2px_8px_rgba(16,185,129,0.25)] dark:shadow-[0_2px_8px_rgba(16,185,129,0.35)]'
      : 'bg-slate-100 dark:bg-slate-800 text-slate-900 dark:text-slate-100 rounded-bl-md border border-slate-200/50 dark:border-slate-700/50'
  )

  const quoteClass = cn(
    'mb-1.5 px-3 py-2 text-xs rounded-lg border-l-2 font-medium',
    isSent
      ? 'bg-emerald-100/60 dark:bg-emerald-900/40 border-emerald-400 dark:border-emerald-500 text-emerald-800 dark:text-emerald-300'
      : 'bg-slate-100/80 dark:bg-slate-800/80 border-slate-300 dark:border-slate-600 text-slate-600 dark:text-slate-400'
  )

  return (
    <div className={cn('flex gap-2.5 mb-5', isSent ? 'flex-row-reverse' : 'flex-row')}>
      {showAvatar && avatarUrl && (
        <img
          src={avatarUrl}
          alt={message.senderName}
          className="w-8 h-8 rounded-full flex-shrink-0 object-cover ring-2 ring-white dark:ring-slate-900/50 shadow-sm"
        />
      )}

      <div className={cn('max-w-[70%] flex flex-col', isSent ? 'items-end' : 'items-start')}>
        {!isSent && showAvatar && (
          <span className="text-xs font-semibold text-slate-500 dark:text-slate-400 mb-1 px-1">
            {message.senderName}
          </span>
        )}

        {message.quote && (
          <div className={quoteClass}>
            <span className="font-bold">{message.quote.senderName}: </span>
            {message.quote.content}
          </div>
        )}

        <div className={bubbleClass}>
          {message.type === 'text' && (
            <p className="text-sm leading-relaxed whitespace-pre-wrap font-medium">{message.content}</p>
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

          {message.reactions.length > 0 && (
            <div className="flex gap-1 mt-2 flex-wrap">
              {message.reactions.map((reaction, idx) => (
                <span
                  key={idx}
                  className="px-2 py-0.5 bg-white/20 dark:bg-black/20 rounded-full text-xs font-medium backdrop-blur-sm border border-white/10 dark:border-white/5"
                >
                  {reaction.emoji} {reaction.users.length}
                </span>
              ))}
            </div>
          )}

          <div
            className={cn(
              'absolute top-1/2 -translate-y-1/2 flex gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity duration-200',
              isSent ? '-left-24' : '-right-24'
            )}
          >
            <button
              onClick={() => onReact?.(message.id, '😊')}
              className="px-2 py-1 bg-white/90 dark:bg-slate-700 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-600 text-sm shadow-md border border-slate-200 dark:border-slate-600 font-medium"
              title="表情"
            >
              😊
            </button>
            <button
              onClick={() => onReply?.(message.id)}
              className="px-2 py-1 bg-white/90 dark:bg-slate-700 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-600 text-sm shadow-md border border-slate-200 dark:border-slate-600 font-medium"
              title="引用"
            >
              ↩
            </button>
            {isSent && (
              <button
                onClick={() => onRetract?.(message.id)}
                className="px-2 py-1 bg-white/90 dark:bg-slate-700 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-600 text-sm shadow-md border border-slate-200 dark:border-slate-600 font-medium"
                title="撤回"
              >
                ↩
              </button>
            )}
          </div>
        </div>

        <div className={cn('flex items-center gap-1.5 mt-1', isSent ? 'justify-end' : 'justify-start')}>
          <span className="text-xs text-slate-400 dark:text-slate-500 font-medium">{messageTime}</span>
          {isSent && message.status === 'read' && (
            <span className="text-xs text-emerald-500 dark:text-emerald-400 font-semibold">已读</span>
          )}
          {isSent && message.status === 'unread' && (
            <span className="text-xs text-slate-400 dark:text-slate-500 font-medium">未读</span>
          )}
        </div>
      </div>
    </div>
  )
}
