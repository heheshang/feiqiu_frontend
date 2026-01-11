# Milestone 6: Collaboration Tools Section

## Overview

Implement the Collaboration Tools section with screen capture, annotation, and screenshot history.

---

## Step 1: Create Types

Create `src/lib/types/collaboration.ts`:

```typescript
export type AnnotationType = 'arrow' | 'rectangle' | 'text' | 'blur'

export interface Annotation {
  id: string
  type: AnnotationType
  x: number
  y: number
  width?: number
  height?: number
  text?: string
  color: string
}

export interface Screenshot {
  id: string
  imageUrl: string
  timestamp: Date
  annotations: Annotation[]
}

export interface CollaborationToolsProps {
  screenshots: Screenshot[]
  selectedScreenshotId: string | null
  onCapture: () => void
  onScreenshotSelect: (id: string) => void
  onAnnotationAdd: (screenshotId: string, annotation: Omit<Annotation, 'id'>) => void
  onAnnotationDelete: (screenshotId: string, annotationId: string) => void
  onScreenshotDelete: (id: string) => void
  onShare: (screenshotId: string, targetUserId: string) => void
}
```

---

## Step 2: Copy Components

Copy the following files from `product-plan/sections/collaboration/` to `src/components/collaboration/`:

1. `CollaborationTools.tsx` - Main collaboration container
2. Create `index.ts`

---

## Step 3: Create Index File

Create `src/components/collaboration/index.ts`:

```typescript
export { CollaborationTools } from './CollaborationTools'
```

---

## Step 4: Implement Screenshot Capture

Create `src/lib/screenshot.ts`:

```typescript
export async function captureScreen(): Promise<string> {
  try {
    // Use Screen Capture API
    const stream = await navigator.mediaDevices.getDisplayMedia({
      video: {
        cursor: 'always'
      },
      audio: false
    })

    const video = document.createElement('video')
    video.srcObject = stream
    await video.play()

    const canvas = document.createElement('canvas')
    canvas.width = video.videoWidth
    canvas.height = video.videoHeight

    const ctx = canvas.getContext('2d')!
    ctx.drawImage(video, 0, 0)

    // Stop all tracks to release the screen
    stream.getTracks().forEach(track => track.stop())

    return canvas.toDataURL('image/png')
  } catch (err) {
    console.error('Screenshot capture failed:', err)
    throw err
  }
}

export async function captureElement(element: HTMLElement): Promise<string> {
  try {
    // Use html2canvas library
    const html2canvas = (await import('html2canvas')).default
    const canvas = await html2canvas(element)
    return canvas.toDataURL('image/png')
  } catch (err) {
    console.error('Element capture failed:', err)
    throw err
  }
}

export function downloadScreenshot(dataUrl: string, filename: string = 'screenshot.png') {
  const link = document.createElement('a')
  link.href = dataUrl
  link.download = filename
  link.click()
}
```

---

## Step 5: Implement Annotation Tools

Create `src/lib/annotations.ts`:

```typescript
import type { Annotation } from '@/lib/types/collaboration'

export class AnnotationCanvas {
  private canvas: HTMLCanvasElement
  private ctx: CanvasRenderingContext2D
  private annotations: Annotation[] = []
  private currentTool: AnnotationType = 'arrow'
  private currentColor: string = '#ef4444'
  private isDrawing: boolean = false
  private startX: number = 0
  private startY: number = 0

  constructor(canvas: HTMLCanvasElement, imageUrl: string) {
    this.canvas = canvas
    this.ctx = canvas.getContext('2d')!

    this.loadImage(imageUrl)
    this.setupEventListeners()
  }

  private async loadImage(url: string) {
    const img = new Image()
    img.onload = () => {
      this.ctx.drawImage(img, 0, 0)
      this.redrawAnnotations()
    }
    img.src = url
  }

  private setupEventListeners() {
    this.canvas.addEventListener('mousedown', (e) => this.startDrawing(e))
    this.canvas.addEventListener('mousemove', (e) => this.draw(e))
    this.canvas.addEventListener('mouseup', () => this.stopDrawing())
    this.canvas.addEventListener('mouseleave', () => this.stopDrawing())
  }

  private startDrawing(e: MouseEvent) {
    this.isDrawing = true
    const rect = this.canvas.getBoundingClientRect()
    this.startX = e.clientX - rect.left
    this.startY = e.clientY - rect.top
  }

  private draw(e: MouseEvent) {
    if (!this.isDrawing) return

    const rect = this.canvas.getBoundingClientRect()
    const x = e.clientX - rect.left
    const y = e.clientY - rect.top

    // Clear and redraw
    this.redrawAnnotations()

    // Draw current shape preview
    this.ctx.strokeStyle = this.currentColor
    this.ctx.lineWidth = 2
    this.ctx.beginPath()

    if (this.currentTool === 'rectangle') {
      this.ctx.strokeRect(this.startX, this.startY, x - this.startX, y - this.startY)
    } else if (this.currentTool === 'arrow') {
      this.drawArrow(this.startX, this.startY, x, y)
    }
  }

  private stopDrawing() {
    if (!this.isDrawing) return
    this.isDrawing = false

    // In a real implementation, you'd save the annotation here
  }

  private drawArrow(fromX: number, fromY: number, toX: number, toY: number) {
    const headlen = 10
    const angle = Math.atan2(toY - fromY, toX - fromX)

    this.ctx.moveTo(fromX, fromY)
    this.ctx.lineTo(toX, toY)
    this.ctx.lineTo(toX - headlen * Math.cos(angle - Math.PI / 6), toY - headlen * Math.sin(angle - Math.PI / 6))
    this.ctx.moveTo(toX, toY)
    this.ctx.lineTo(toX - headlen * Math.cos(angle + Math.PI / 6), toY - headlen * Math.sin(angle + Math.PI / 6))
    this.ctx.stroke()
  }

  private redrawAnnotations() {
    // Redraw all saved annotations
    this.annotations.forEach(annotation => {
      // Implementation depends on annotation type
    })
  }

  setTool(tool: AnnotationType) {
    this.currentTool = tool
  }

  setColor(color: string) {
    this.currentColor = color
  }

  getAnnotatedImage(): string {
    return this.canvas.toDataURL('image/png')
  }
}
```

---

## Step 6: Implement Collaboration Store

Create `src/stores/collaborationStore.ts`:

```typescript
import { create } from 'zustand'
import type { Screenshot, Annotation } from '@/lib/types/collaboration'

interface CollaborationStore {
  screenshots: Screenshot[]
  selectedScreenshotId: string | null
  currentTool: Annotation['type']
  currentColor: string

  addScreenshot: (imageUrl: string) => string
  selectScreenshot: (id: string) => void
  addAnnotation: (screenshotId: string, annotation: Omit<Annotation, 'id'>) => void
  deleteAnnotation: (screenshotId: string, annotationId: string) => void
  deleteScreenshot: (id: string) => void
  setTool: (tool: Annotation['type']) => void
  setColor: (color: string) => void

  getSelectedScreenshot: () => Screenshot | null
}

export const useCollaborationStore = create<CollaborationStore>((set, get) => ({
  screenshots: [],
  selectedScreenshotId: null,
  currentTool: 'arrow',
  currentColor: '#ef4444',

  addScreenshot: (imageUrl) => {
    const id = crypto.randomUUID()
    const screenshot: Screenshot = {
      id,
      imageUrl,
      timestamp: new Date(),
      annotations: []
    }
    set((state) => ({
      screenshots: [screenshot, ...state.screenshots],
      selectedScreenshotId: id
    }))
    return id
  },

  selectScreenshot: (id) => set({ selectedScreenshotId: id }),

  addAnnotation: (screenshotId, annotation) => set((state) => ({
    screenshots: state.screenshots.map(s =>
      s.id === screenshotId
        ? {
            ...s,
            annotations: [...s.annotations, { ...annotation, id: crypto.randomUUID() }]
          }
        : s
    )
  })),

  deleteAnnotation: (screenshotId, annotationId) => set((state) => ({
    screenshots: state.screenshots.map(s =>
      s.id === screenshotId
        ? {
            ...s,
            annotations: s.annotations.filter(a => a.id !== annotationId)
          }
        : s
    )
  })),

  deleteScreenshot: (id) => set((state) => ({
    screenshots: state.screenshots.filter(s => s.id !== id),
    selectedScreenshotId: state.selectedScreenshotId === id ? null : state.selectedScreenshotId
  })),

  setTool: (tool) => set({ currentTool: tool }),
  setColor: (color) => set({ currentColor: color }),

  getSelectedScreenshot: () => {
    const { screenshots, selectedScreenshotId } = get()
    return screenshots.find(s => s.id === selectedScreenshotId) || null
  }
}))
```

---

## Step 7: Integrate Collaboration Tools

```typescript
'use client'

import { useState } from 'react'
import { useCollaborationStore } from '@/stores/collaborationStore'
import { CollaborationTools } from '@/components/collaboration'
import { captureScreen, downloadScreenshot } from '@/lib/screenshot'

function CollaborationPage() {
  const { screenshots, selectedScreenshotId, addScreenshot } = useCollaborationStore()

  const handleCapture = async () => {
    try {
      const imageUrl = await captureScreen()
      addScreenshot(imageUrl)
    } catch (err) {
      console.error('Capture failed:', err)
    }
  }

  const handleDownload = () => {
    if (selectedScreenshotId) {
      const screenshot = screenshots.find(s => s.id === selectedScreenshotId)
      if (screenshot) {
        downloadScreenshot(screenshot.imageUrl)
      }
    }
  }

  return (
    <CollaborationTools
      screenshots={screenshots}
      selectedScreenshotId={selectedScreenshotId}
      onCapture={handleCapture}
      onScreenshotSelect={(id) => console.log('Select:', id)}
      onAnnotationAdd={(id, annotation) => console.log('Add annotation:', id, annotation)}
      onAnnotationDelete={(id, annotationId) => console.log('Delete annotation:', id, annotationId)}
      onScreenshotDelete={(id) => console.log('Delete screenshot:', id)}
      onShare={(id, userId) => console.log('Share:', id, userId)}
    />
  )
}
```

---

## Verification

Test the Collaboration Tools section:

1. **Screen Capture**:
   - [ ] Capture button works
   - [ ] Screen selection dialog appears
   - [ ] Screenshot saves correctly
   - [ ] Downloads work

2. **Screenshot History**:
   - [ ] All screenshots display
   - [ ] Click to select
   - [ ] Delete works
   - [ ] Timestamps display

3. **Annotation Tools**:
   - [ ] Arrow tool draws arrows
   - [ ] Rectangle tool draws rectangles
   - [ ] Text tool adds text
   - [ ] Blur tool blurs areas
   - [ ] Color picker works

---

## Next Steps

Once Collaboration Tools is complete, proceed to **[Milestone 7: Organization Chart](./07-organization.md)**.
