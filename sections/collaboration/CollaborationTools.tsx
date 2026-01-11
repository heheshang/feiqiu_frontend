'use client'

import { useState } from 'react'
import type { CollaborationToolsProps, ScreenshotType } from '@/../product/sections/collaboration/types'

export function CollaborationTools({
  currentUser,
  screenshots,
  users,
  onScreenshot,
  onSave,
  onCopy,
  onSendToContact,
  onSendToChat,
  onUndo,
  onRedo
}: CollaborationToolsProps) {
  const [selectedScreenshot, setSelectedScreenshot] = useState<string | null>(null)
  const [currentTool, setCurrentTool] = useState<'select' | 'arrow' | 'rectangle' | 'text' | 'brush'>('select')

  const handleScreenshot = (type: ScreenshotType) => {
    onScreenshot?.(type)
  }

  const selectedScreenshotData = screenshots.find(s => s.id === selectedScreenshot)

  return (
    <div className="h-full flex flex-col bg-white dark:bg-slate-900">
      {/* Toolbar */}
      <div className="border-b border-slate-200 dark:border-slate-700 px-6 py-4">
        <h1 className="text-xl font-semibold text-slate-900 dark:text-slate-100 mb-4">协作工具</h1>

        {/* Screenshot buttons */}
        <div className="flex items-center gap-3">
          <button
            onClick={() => handleScreenshot('fullscreen')}
            className="flex items-center gap-2 px-4 py-2 bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 4l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" />
            </svg>
            <span>全屏截图</span>
          </button>

          <button
            onClick={() => handleScreenshot('region')}
            className="flex items-center gap-2 px-4 py-2 bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 5a1 1 0 011-1h4a1 1 0 010 2H6v3a1 1 0 01-2 0V5zm16 0a1 1 0 00-1-1h-4a1 1 0 000 2h3v3a1 1 0 002 0V5zM4 19a1 1 0 001 1h4a1 1 0 000-2H6v-3a1 1 0 00-2 0v4zm16 0a1 1 0 01-1 1h-4a1 1 0 010-2h3v-3a1 1 0 012 0v4z" />
            </svg>
            <span>区域选择</span>
          </button>

          <button
            onClick={() => handleScreenshot('window')}
            className="flex items-center gap-2 px-4 py-2 bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2" />
            </svg>
            <span>活动窗口</span>
          </button>
        </div>
      </div>

      {/* Screenshot history or empty state */}
      <div className="flex-1 overflow-y-auto p-6">
        {screenshots.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center">
            <div className="w-16 h-16 rounded-2xl bg-slate-100 dark:bg-slate-800 flex items-center justify-center mb-4">
              <svg className="w-8 h-8 text-slate-400 dark:text-slate-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" />
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
            </div>
            <h3 className="text-lg font-medium text-slate-900 dark:text-slate-100 mb-2">暂无截图</h3>
            <p className="text-sm text-slate-500 dark:text-slate-400 mb-4">点击上方按钮开始截图</p>
            <p className="text-xs text-slate-400 dark:text-slate-500">支持全屏、区域选择和活动窗口截图</p>
          </div>
        ) : (
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
            {screenshots.map(screenshot => (
              <div
                key={screenshot.id}
                onClick={() => setSelectedScreenshot(screenshot.id)}
                className={`group relative bg-slate-50 dark:bg-slate-800 rounded-lg overflow-hidden cursor-pointer transition-all hover:shadow-lg ${
                  selectedScreenshot === screenshot.id ? 'ring-2 ring-emerald-500' : ''
                }`}
              >
                {/* Thumbnail */}
                <div className="aspect-video bg-slate-200 dark:bg-slate-700">
                  <img
                    src={screenshot.thumbnailUrl}
                    alt={screenshot.title}
                    className="w-full h-full object-cover"
                  />
                </div>

                {/* Info overlay */}
                <div className="p-3">
                  <h4 className="font-medium text-sm text-slate-900 dark:text-slate-100 truncate mb-1">
                    {screenshot.title}
                  </h4>
                  <div className="flex items-center justify-between text-xs text-slate-500 dark:text-slate-400">
                    <span>{getTypeLabel(screenshot.type)}</span>
                    <span>{getStatusBadge(screenshot.status)}</span>
                  </div>
                </div>

                {/* Hover actions */}
                <div className="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center gap-2">
                  <button
                    onClick={(e) => {
                      e.stopPropagation()
                      onSave?.(screenshot.id)
                    }}
                    className="p-2 bg-white/90 rounded-lg hover:bg-white"
                    title="保存"
                  >
                    <svg className="w-4 h-4 text-slate-700" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
                    </svg>
                  </button>
                  <button
                    onClick={(e) => {
                      e.stopPropagation()
                      onCopy?.(screenshot.id)
                    }}
                    className="p-2 bg-white/90 rounded-lg hover:bg-white"
                    title="复制"
                  >
                    <svg className="w-4 h-4 text-slate-700" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                    </svg>
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Annotation editor modal */}
      {selectedScreenshotData && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white dark:bg-slate-800 rounded-2xl shadow-2xl max-w-5xl w-full max-h-[90vh] overflow-hidden flex flex-col">
            {/* Header */}
            <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
              <h3 className="text-lg font-semibold text-slate-900 dark:text-slate-100">
                标注编辑器
              </h3>
              <button
                onClick={() => setSelectedScreenshot(null)}
                className="p-2 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
              >
                <svg className="w-5 h-5 text-slate-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>

            {/* Editor content */}
            <div className="flex-1 overflow-auto p-4">
              <div className="relative inline-block">
                <img
                  src={selectedScreenshotData.imageUrl}
                  alt={selectedScreenshotData.title}
                  className="max-w-full h-auto rounded-lg"
                />
                {/* Annotations would be rendered here */}
              </div>
            </div>

            {/* Toolbar */}
            <div className="border-t border-slate-200 dark:border-slate-700 p-4">
              <div className="flex items-center justify-between">
                {/* Annotation tools */}
                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setCurrentTool('select')}
                    className={`p-2 rounded-lg transition-colors ${
                      currentTool === 'select'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    }`}
                    title="选择"
                  >
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122" />
                    </svg>
                  </button>

                  <button
                    onClick={() => setCurrentTool('arrow')}
                    className={`p-2 rounded-lg transition-colors ${
                      currentTool === 'arrow'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    }`}
                    title="箭头"
                  >
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" />
                    </svg>
                  </button>

                  <button
                    onClick={() => setCurrentTool('rectangle')}
                    className={`p-2 rounded-lg transition-colors ${
                      currentTool === 'rectangle'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    }`}
                    title="矩形"
                  >
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <rect x="3" y="3" width="18" height="18" rx="2" strokeWidth={2} />
                    </svg>
                  </button>

                  <button
                    onClick={() => setCurrentTool('text')}
                    className={`p-2 rounded-lg transition-colors ${
                      currentTool === 'text'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    }`}
                    title="文字"
                  >
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h7" />
                    </svg>
                  </button>

                  <button
                    onClick={() => setCurrentTool('brush')}
                    className={`p-2 rounded-lg transition-colors ${
                      currentTool === 'brush'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    }`}
                    title="画笔"
                  >
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                    </svg>
                  </button>

                  <div className="w-px h-6 bg-slate-300 dark:bg-slate-600 mx-2" />

                  <button
                    onClick={onUndo}
                    className="p-2 text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
                    title="撤销"
                  >
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
                    </svg>
                  </button>

                  <button
                    onClick={onRedo}
                    className="p-2 text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
                    title="重做"
                  >
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 10h-10a8 8 0 00-8 8v2M21 10l-6 6m6-6l-6-6" />
                    </svg>
                  </button>
                </div>

                {/* Action buttons */}
                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setSelectedScreenshot(null)}
                    className="px-4 py-2 text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
                  >
                    取消
                  </button>
                  <button
                    onClick={() => {
                      onSave?.(selectedScreenshotData.id)
                      setSelectedScreenshot(null)
                    }}
                    className="px-4 py-2 bg-emerald-500 hover:bg-emerald-600 text-white rounded-lg transition-colors"
                  >
                    完成
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

function getTypeLabel(type: string): string {
  const labels = {
    fullscreen: '全屏',
    region: '区域',
    window: '窗口'
  }
  return labels[type as keyof typeof labels] || type
}

function getStatusBadge(status: string): string {
  const badges = {
    draft: '草稿',
    saved: '已保存',
    sent: '已发送'
  }
  return badges[status as keyof typeof badges] || status
}
