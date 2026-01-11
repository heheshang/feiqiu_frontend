'use client'

import { useState } from 'react'
import { CollaborationToolsProps, ScreenshotType } from '../../lib/types/collaboration'
import { cn } from '../../lib/utils'
import { 
  Monitor, 
  Crop, 
  Maximize, 
  Save, 
  Copy, 
  ArrowLeftRight, 
  Type, 
  Square, 
  Pen, 
  MousePointer2,
  X,
  Undo2,
  Redo2,
  Check
} from 'lucide-react'

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

  const getTypeLabel = (type: ScreenshotType): string => {
    const labels: Record<ScreenshotType, string> = {
      fullscreen: '全屏',
      region: '区域',
      window: '窗口'
    }
    return labels[type]
  }

  const getStatusBadge = (status: string): string => {
    const badges: Record<string, string> = {
      draft: '草稿',
      saved: '已保存',
      sent: '已发送'
    }
    return badges[status] || status
  }

  return (
    <div className="h-full flex flex-col bg-white dark:bg-slate-900">
      <div className="border-b border-slate-200 dark:border-slate-700 px-6 py-4">
        <h1 className="text-xl font-semibold text-slate-900 dark:text-slate-100 mb-4">协作工具</h1>

        <div className="flex items-center gap-3">
          <button
            onClick={() => handleScreenshot('fullscreen')}
            className="flex items-center gap-2 px-4 py-2 bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
          >
            <Monitor className="w-5 h-5" />
            <span>全屏截图</span>
          </button>

          <button
            onClick={() => handleScreenshot('region')}
            className="flex items-center gap-2 px-4 py-2 bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
          >
            <Crop className="w-5 h-5" />
            <span>区域选择</span>
          </button>

          <button
            onClick={() => handleScreenshot('window')}
            className="flex items-center gap-2 px-4 py-2 bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
          >
            <Maximize className="w-5 h-5" />
            <span>活动窗口</span>
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-6">
        {screenshots.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center">
            <div className="w-16 h-16 rounded-2xl bg-slate-100 dark:bg-slate-800 flex items-center justify-center mb-4">
              <Monitor className="w-8 h-8 text-slate-400 dark:text-slate-500" />
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
                className={cn(
                  'group relative bg-slate-50 dark:bg-slate-800 rounded-lg overflow-hidden cursor-pointer transition-all hover:shadow-lg',
                  selectedScreenshot === screenshot.id ? 'ring-2 ring-emerald-500' : ''
                )}
              >
                <div className="aspect-video bg-slate-200 dark:bg-slate-700">
                  <img
                    src={screenshot.thumbnailUrl}
                    alt={screenshot.title}
                    className="w-full h-full object-cover"
                  />
                </div>

                <div className="p-3">
                  <h4 className="font-medium text-sm text-slate-900 dark:text-slate-100 truncate mb-1">
                    {screenshot.title}
                  </h4>
                  <div className="flex items-center justify-between text-xs text-slate-500 dark:text-slate-400">
                    <span>{getTypeLabel(screenshot.type)}</span>
                    <span>{getStatusBadge(screenshot.status)}</span>
                  </div>
                </div>

                <div className="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center gap-2">
                  <button
                    onClick={(e) => {
                      e.stopPropagation()
                      onSave?.(screenshot.id)
                    }}
                    className="p-2 bg-white/90 rounded-lg hover:bg-white"
                    title="保存"
                  >
                    <Save className="w-4 h-4 text-slate-700" />
                  </button>
                  <button
                    onClick={(e) => {
                      e.stopPropagation()
                      onCopy?.(screenshot.id)
                    }}
                    className="p-2 bg-white/90 rounded-lg hover:bg-white"
                    title="复制"
                  >
                    <Copy className="w-4 h-4 text-slate-700" />
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {selectedScreenshotData && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white dark:bg-slate-800 rounded-2xl shadow-2xl max-w-5xl w-full max-h-[90vh] overflow-hidden flex flex-col">
            <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
              <h3 className="text-lg font-semibold text-slate-900 dark:text-slate-100">
                标注编辑器
              </h3>
              <button
                onClick={() => setSelectedScreenshot(null)}
                className="p-2 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
              >
                <X className="w-5 h-5 text-slate-500" />
              </button>
            </div>

            <div className="flex-1 overflow-auto p-4">
              <div className="relative inline-block">
                <img
                  src={selectedScreenshotData.imageUrl}
                  alt={selectedScreenshotData.title}
                  className="max-w-full h-auto rounded-lg"
                />
              </div>
            </div>

            <div className="border-t border-slate-200 dark:border-slate-700 p-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setCurrentTool('select')}
                    className={cn(
                      'p-2 rounded-lg transition-colors',
                      currentTool === 'select'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    )}
                    title="选择"
                  >
                    <MousePointer2 className="w-5 h-5" />
                  </button>

                  <button
                    onClick={() => setCurrentTool('arrow')}
                    className={cn(
                      'p-2 rounded-lg transition-colors',
                      currentTool === 'arrow'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    )}
                    title="箭头"
                  >
                    <ArrowLeftRight className="w-5 h-5" />
                  </button>

                  <button
                    onClick={() => setCurrentTool('rectangle')}
                    className={cn(
                      'p-2 rounded-lg transition-colors',
                      currentTool === 'rectangle'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    )}
                    title="矩形"
                  >
                    <Square className="w-5 h-5" />
                  </button>

                  <button
                    onClick={() => setCurrentTool('text')}
                    className={cn(
                      'p-2 rounded-lg transition-colors',
                      currentTool === 'text'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    )}
                    title="文字"
                  >
                    <Type className="w-5 h-5" />
                  </button>

                  <button
                    onClick={() => setCurrentTool('brush')}
                    className={cn(
                      'p-2 rounded-lg transition-colors',
                      currentTool === 'brush'
                        ? 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400'
                        : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700'
                    )}
                    title="画笔"
                  >
                    <Pen className="w-5 h-5" />
                  </button>

                  <div className="w-px h-6 bg-slate-300 dark:bg-slate-600 mx-2" />

                  <button
                    onClick={onUndo}
                    className="p-2 text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
                    title="撤销"
                  >
                    <Undo2 className="w-5 h-5" />
                  </button>

                  <button
                    onClick={onRedo}
                    className="p-2 text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
                    title="重做"
                  >
                    <Redo2 className="w-5 h-5" />
                  </button>
                </div>

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
