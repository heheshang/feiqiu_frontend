import { useState, useRef, useEffect } from 'react'
import type { Conversation, Message } from '@/../product/sections/messaging/types'
import { ConversationItem } from './ConversationItem'
import { MessageBubble } from './MessageBubble'
import { MessageInput } from './MessageInput'

interface MessagingProps {
  conversations: Conversation[]
  currentConversation: Conversation
  currentUser: {
    id: string
    name: string
    avatar: string
    status?: string
  }
  onSendMessage?: (content: string) => void
  onSendImage?: (file: File) => void
  onConversationSelect?: (conversationId: string) => void
  onMessageReply?: (messageId: string) => void
  onMessageReact?: (messageId: string, emoji: string) => void
  onMessageRetract?: (messageId: string) => void
}

export function Messaging({
  conversations,
  currentConversation,
  currentUser,
  onSendMessage,
  onSendImage,
  onConversationSelect,
  onMessageReply,
  onMessageReact,
  onMessageRetract
}: MessagingProps) {
  const [searchQuery, setSearchQuery] = useState('')
  const messagesEndRef = useRef<HTMLDivElement>(null)

  // Auto-scroll to bottom when messages change
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [currentConversation.messages])

  // Separate pinned and unpinned conversations
  const pinnedConversations = conversations.filter(c => c.pinned)
  const regularConversations = conversations.filter(c => !c.pinned)

  // Filter conversations by search
  const filterConversations = (convs: Conversation[]) => {
    if (!searchQuery.trim()) return convs
    return convs.filter(c => {
      const name = c.type === 'group' ? c.group?.name : c.participant?.name
      return name?.toLowerCase().includes(searchQuery.toLowerCase())
    })
  }

  const displayName = currentConversation.type === 'group'
    ? currentConversation.group?.name
    : currentConversation.participant?.name

  const memberCount = currentConversation.type === 'group'
    ? currentConversation.group?.memberCount
    : undefined

  const handleSendMessage = (content: string) => {
    onSendMessage?.(content)
  }

  const handleSendImage = (file: File) => {
    onSendImage?.(file)
  }

  const handleReply = (messageId: string) => {
    onMessageReply?.(messageId)
  }

  const handleReact = (messageId: string, emoji: string) => {
    onMessageReact?.(messageId, emoji)
  }

  const handleRetract = (messageId: string) => {
    onMessageRetract?.(messageId)
  }

  return (
    <div className="flex h-full bg-white dark:bg-slate-900">
      {/* Left sidebar - Conversation list */}
      <div className="w-[280px] border-r border-slate-200 dark:border-slate-700 flex flex-col flex-shrink-0">
        {/* Search */}
        <div className="p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="relative">
            <svg
              className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="搜索会话..."
              className="w-full pl-10 pr-4 py-2 bg-slate-100 dark:bg-slate-800 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 dark:text-white placeholder-slate-400"
            />
          </div>
        </div>

        {/* Conversation list */}
        <div className="flex-1 overflow-y-auto">
          {/* Pinned conversations */}
          {filterConversations(pinnedConversations).length > 0 && (
            <div className="px-4 py-2 text-xs font-medium text-slate-500 dark:text-slate-400 bg-slate-50 dark:bg-slate-800/50">
              置顶会话
            </div>
          )}
          {filterConversations(pinnedConversations).map(conv => (
            <ConversationItem
              key={conv.id}
              conversation={conv}
              isActive={conv.id === currentConversation.id}
              onClick={() => onConversationSelect?.(conv.id)}
            />
          ))}

          {/* Regular conversations */}
          {filterConversations(regularConversations).map(conv => (
            <ConversationItem
              key={conv.id}
              conversation={conv}
              isActive={conv.id === currentConversation.id}
              onClick={() => onConversationSelect?.(conv.id)}
            />
          ))}

          {filterConversations(conversations).length === 0 && (
            <div className="px-4 py-8 text-center text-sm text-slate-400 dark:text-slate-500">
              未找到匹配的会话
            </div>
          )}
        </div>
      </div>

      {/* Right - Chat window */}
      <div className="flex-1 flex flex-col min-w-0">
        {/* Header */}
        <div className="px-6 py-4 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between flex-shrink-0">
          <div className="flex items-center gap-3">
            <div>
              <h2 className="font-semibold text-slate-900 dark:text-slate-100">
                {displayName}
              </h2>
              {memberCount && (
                <p className="text-xs text-slate-500 dark:text-slate-400">
                  {memberCount} 位成员
                </p>
              )}
            </div>
          </div>

          <div className="flex items-center gap-2">
            <button className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors" title="更多">
              <svg className="w-5 h-5 text-slate-600 dark:text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
              </svg>
            </button>
          </div>
        </div>

        {/* Messages area */}
        <div className="flex-1 overflow-y-auto px-6 py-4">
          {currentConversation.messages && currentConversation.messages.length > 0 ? (
            <>
              {currentConversation.messages.map((message, index) => {
                const isSent = message.senderId === currentUser.id
                const showAvatar = !isSent && (
                  index === 0 ||
                  currentConversation.messages![index - 1].senderId !== message.senderId
                )

                const avatarUrl = isSent
                  ? currentUser.avatar
                  : (currentConversation.type === 'group'
                      ? currentConversation.group?.members.find(m => m.id === message.senderId)?.avatar
                      : currentConversation.participant?.avatar)

                return (
                  <MessageBubble
                    key={message.id}
                    message={message}
                    isSent={isSent}
                    showAvatar={showAvatar}
                    avatarUrl={avatarUrl}
                    onReply={handleReply}
                    onReact={handleReact}
                    onRetract={handleRetract}
                  />
                )
              })}
              <div ref={messagesEndRef} />
            </>
          ) : (
            <div className="h-full flex items-center justify-center">
              <p className="text-slate-400 dark:text-slate-500">暂无消息，开始聊天吧</p>
            </div>
          )}
        </div>

        {/* Input area */}
        <MessageInput
          onSendMessage={handleSendMessage}
          onImageUpload={handleSendImage}
          placeholder={`发给 ${displayName}...`}
        />
      </div>
    </div>
  )
}
