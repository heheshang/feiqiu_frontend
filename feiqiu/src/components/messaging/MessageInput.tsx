import { useState } from 'react'
import { cn } from '../../lib/utils'
import { Image, AtSign, Send } from 'lucide-react'

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
  placeholder = '输入消息...',
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
    <div className="border-t border-slate-200/80 dark:border-slate-700/80 p-4 bg-white/90 dark:bg-slate-900/90 backdrop-blur-sm">
      <div className="flex items-center gap-2 mb-2.5">
        <div className="relative">
          <button
            onClick={() => setShowEmojiPicker(!showEmojiPicker)}
            className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-xl transition-colors font-medium"
            title="表情"
          >
            <span className="text-lg">😊</span>
          </button>

          {showEmojiPicker && (
            <>
              <div
                className="fixed inset-0 z-10"
                onClick={() => setShowEmojiPicker(false)}
              />
              <div className="absolute bottom-full left-0 mb-2 p-3 bg-white dark:bg-slate-800 rounded-2xl shadow-[0_10px_40px_-10px_rgba(0,0,0,0.15),0_4px_12px_-4px_rgba(0,0,0,0.1)] dark:shadow-[0_10px_40px_-10px_rgba(0,0,0,0.4),0_4px_12px_-4px_rgba(0,0,0,0.3)] border border-slate-200/90 dark:border-slate-700/90 z-20">
                <div className="grid grid-cols-6 gap-1">
                  {EMOJIS.map((emoji) => (
                    <button
                      key={emoji}
                      onClick={() => {
                        setText(text + emoji)
                        setShowEmojiPicker(false)
                      }}
                      className="p-2 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-xl text-xl transition-colors"
                    >
                      {emoji}
                    </button>
                  ))}
                </div>
              </div>
            </>
          )}
        </div>

        <label className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-xl transition-colors cursor-pointer" title="图片">
          <input
            type="file"
            accept="image/*"
            onChange={handleImageUpload}
            className="hidden"
          />
          <Image className="w-5 h-5 text-slate-600 dark:text-slate-400" />
        </label>

        <button
          className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-xl transition-colors"
          title="@提醒"
        >
          <AtSign className="w-5 h-5 text-slate-600 dark:text-slate-400" />
        </button>
      </div>

      <div className="flex items-end gap-2">
        <div className="flex-1 relative">
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            onKeyDown={handleKeyDown}
            disabled={disabled}
            placeholder={placeholder}
            rows={1}
            className={cn(
              'w-full px-4 py-3 pr-12 bg-slate-100 dark:bg-slate-800 rounded-2xl resize-none text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500/50 dark:focus:ring-emerald-400/50 focus:border-emerald-500 placeholder-slate-400 dark:text-white border border-transparent focus:border-emerald-500/50 dark:focus:border-emerald-400/50 transition-all duration-200 shadow-sm',
              disabled && 'opacity-50 cursor-not-allowed'
            )}
            style={{ minHeight: '44px', maxHeight: '120px' }}
          />
        </div>

        <button
          onClick={handleSend}
          disabled={disabled || !text.trim()}
          className={cn(
            'p-3 rounded-xl transition-all duration-200 flex-shrink-0',
            disabled || !text.trim()
              ? 'bg-slate-200 dark:bg-slate-700 text-slate-400 dark:text-slate-500 cursor-not-allowed'
              : 'bg-emerald-500 hover:bg-emerald-600 text-white dark:bg-emerald-600 dark:hover:bg-emerald-700 shadow-md shadow-emerald-500/20 dark:shadow-emerald-500/30 hover:shadow-lg hover:shadow-emerald-500/30 dark:hover:shadow-emerald-500/40'
          )}
        >
          <Send className="w-5 h-5" />
        </button>
      </div>

      <p className="text-xs text-slate-400 dark:text-slate-500 mt-2.5 font-medium">按 Enter 发送，Shift + Enter 换行</p>
    </div>
  )
}
