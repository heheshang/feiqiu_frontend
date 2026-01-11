export type ScreenshotType = 'fullscreen' | 'region' | 'window'
export type ScreenshotStatus = 'draft' | 'saved' | 'sent'
export type AnnotationType = 'arrow' | 'rectangle' | 'text' | 'brush'

export interface Point {
  x: number
  y: number
}

export interface Annotation {
  id: string
  type: AnnotationType
  color: string
  startX?: number
  startY?: number
  endX?: number
  endY?: number
  x?: number
  y?: number
  width?: number
  height?: number
  content?: string
  fontSize?: number
  points?: Point[]
  lineWidth?: number
}

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

export interface User {
  id: string
  name: string
  avatar: string
}

export interface CollaborationToolsProps {
  currentUser: User
  screenshots: Screenshot[]
  users: Record<string, User>
  onScreenshot?: (type: ScreenshotType) => void
  onAddAnnotation?: (screenshotId: string, annotation: Omit<Annotation, 'id'>) => void
  onDeleteAnnotation?: (screenshotId: string, annotationId: string) => void
  onSave?: (screenshotId: string) => void
  onCopy?: (screenshotId: string) => void
  onSendToContact?: (screenshotId: string, contactId: string) => void
  onSendToChat?: (screenshotId: string, conversationId: string) => void
  onUndo?: () => void
  onRedo?: () => void
}
