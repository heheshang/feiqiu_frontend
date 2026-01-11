// =============================================================================
// Data Types
// =============================================================================

/**
 * 用户信息
 */
export interface User {
  id: string
  name: string
  avatar: string
}

/**
 * 截图类型
 */
export type ScreenshotType = 'fullscreen' | 'region' | 'window'

/**
 * 截图状态
 */
export type ScreenshotStatus = 'draft' | 'saved' | 'sent'

/**
 * 标注类型
 */
export type AnnotationType = 'arrow' | 'rectangle' | 'text' | 'brush'

/**
 * 点坐标（用于画笔路径）
 */
export interface Point {
  x: number
  y: number
}

/**
 * 标注元素
 */
export interface Annotation {
  id: string
  type: AnnotationType
  color: string
  // 箭头属性
  startX?: number
  startY?: number
  endX?: number
  endY?: number
  // 矩形属性
  x?: number
  y?: number
  width?: number
  height?: number
  // 文字属性
  content?: string
  fontSize?: number
  // 画笔属性
  points?: Point[]
  lineWidth?: number
}

/**
 * 截图记录
 */
export interface Screenshot {
  id: string
  type: ScreenshotType
  title: string
  imageUrl: string
  thumbnailUrl: string
  createdAt: string
  createdBy: string
  annotations: Annotation[]
  status: ScreenshotStatus
  sentTo?: string
}

// =============================================================================
// Component Props
// =============================================================================

/**
 * CollaborationTools 组件的 Props
 */
export interface CollaborationToolsProps {
  /** 当前用户 */
  currentUser: User
  /** 截图列表 */
  screenshots: Screenshot[]
  /** 用户映射表 */
  users: Record<string, User>
  /** 截图回调 */
  onScreenshot?: (type: ScreenshotType) => void
  /** 添加标注 */
  onAddAnnotation?: (screenshotId: string, annotation: Omit<Annotation, 'id'>) => void
  /** 删除标注 */
  onDeleteAnnotation?: (screenshotId: string, annotationId: string) => void
  /** 保存截图 */
  onSave?: (screenshotId: string) => void
  /** 复制截图 */
  onCopy?: (screenshotId: string) => void
  /** 发送给联系人 */
  onSendToContact?: (screenshotId: string, contactId: string) => void
  /** 发送到聊天 */
  onSendToChat?: (screenshotId: string, conversationId: string) => void
  /** 撤销操作 */
  onUndo?: () => void
  /** 重做操作 */
  onRedo?: () => void
}
