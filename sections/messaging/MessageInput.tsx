import { useState } from 'react'

interface MessageInputProps {
  onSendMessage: (content: string) => void
  onImageUpload?: (file: File) => void
  disabled?: boolean
  placeholder?: string
}

const EMOJIS = ['😀', '😊', '😂', '🥰', '😎', '🤔', '👍', '👎', '❤️', '🎉', '🔥', '💯']

export function MessageInput({
  onSendMessage,
  onImageUpload,
  disabled = false,
  placeholder = '输入消息...'
}: MessageInputProps) {
  const [text, setText] = useState('')
  const [showEmojiPicker, setShowEmojiPicker] = useState(false)

  const handleSend = () => {
    const trimmed = text.trim()
    if (trimmed && !disabled) {
      onSendMessage(trimmed)
      setText('')
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (file && onImageUpload) {
      onImageUpload(file)
    }
    e.target.value = ''
  }

  return (
    <div className="border-t border-slate-200 dark:border-slate-700 p-4 bg-white dark:bg-slate-900">
      {/* Toolbar */}
      <div className="flex items-center gap-2 mb-2">
        {/* Emoji button */}
        <div className="relative">
          <button
            onClick={() => setShowEmojiPicker(!showEmojiPicker)}
            className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors"
            title="表情"
          >
            <svg className="w-5 h-5 text-slate-600 dark:text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </button>

          {/* Emoji picker popover */}
          {showEmojiPicker && (
            <>
              <div
                className="fixed inset-0 z-10"
                onClick={() => setShowEmojiPicker(false)}
              />
              <div className="absolute bottom-full left-0 mb-2 p-3 bg-white dark:bg-slate-800 rounded-xl shadow-xl border border-slate-200 dark:border-slate-700 z-20">
                <div className="grid grid-cols-6 gap-1">
                  {EMOJIS.map((emoji) => (
                    <button
                      key={emoji}
                      onClick={() => {
                        setText(text + emoji)
                        setShowEmojiPicker(false)
                      }}
                      className="p-2 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg text-xl transition-colors"
                    >
                      {emoji}
                    </button>
                  ))}
                </div>
              </div>
            </>
          )}
        </div>

        {/* Image upload button */}
        <label className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors cursor-pointer" title="图片">
          <input
            type="file"
            accept="image/*"
            onChange={handleImageUpload}
            className="hidden"
          />
          <svg className="w-5 h-5 text-slate-600 dark:text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
        </label>

        {/* @ mention button */}
        <button
          className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors"
          title="@提醒"
        >
          <svg className="w-5 h-5 text-slate-600 dark:text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
        </button>
      </div>

      {/* Input area */}
      <div className="flex items-end gap-2">
        <div className="flex-1 relative">
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            onKeyDown={handleKeyDown}
            disabled={disabled}
            placeholder={placeholder}
            rows={1}
            className="w-full px-4 py-3 pr-12 bg-slate-100 dark:bg-slate-800 rounded-2xl resize-none text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 dark:focus:ring-emerald-400 placeholder-slate-400 dark:text-white"
            style={{ minHeight: '44px', maxHeight: '120px' }}
          />
        </div>

        {/* Send button */}
        <button
          onClick={handleSend}
          disabled={disabled || !text.trim()}
          className={`
            p-3 rounded-xl transition-all duration-200 flex-shrink-0
            ${disabled || !text.trim()
              ? 'bg-slate-200 dark:bg-slate-700 text-slate-400 dark:text-slate-500 cursor-not-allowed'
              : 'bg-emerald-500 hover:bg-emerald-600 text-white dark:bg-emerald-600 dark:hover:bg-emerald-700'
            }
          `}
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
          </svg>
        </button>
      </div>

      <p className="text-xs text-slate-400 dark:text-slate-500 mt-2">按 Enter 发送，Shift + Enter 换行</p>
    </div>
  )
}
