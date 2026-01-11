# Milestone 5: File Transfer Section

## Overview

Implement the File Transfer section with LAN file/folder transfer, progress visualization, and transfer management.

---

## Step 1: Create Types

Create `src/lib/types/file-transfer.ts`:

```typescript
export type TransferDirection = 'upload' | 'download'
export type TransferStatus = 'pending' | 'transferring' | 'paused' | 'completed' | 'failed'
export type TransferFilter = 'all' | 'uploading' | 'downloading' | 'completed'

export interface FileTransfer {
  id: string
  fileName: string
  fileSize: number
  fileType: string
  direction: TransferDirection
  status: TransferStatus
  progress: number
  speed: number
  peerId?: string
  peerName?: string
  timestamp: Date
  filePath?: string
}

export interface FileTransferProps {
  transfers: FileTransfer[]
  activeFilter: TransferFilter
  onFilterChange: (filter: TransferFilter) => void
  onPause: (id: string) => void
  onResume: (id: string) => void
  onCancel: (id: string) => void
  onRetry: (id: string) => void
  onOpenFile: (id: string) => void
  onOpenFolder: (id: string) => void
}
```

---

## Step 2: Copy Components

Copy the following files from `product-plan/sections/file-transfer/` to `src/components/file-transfer/`:

1. `FileTransfer.tsx` - Main transfer list container
2. `FileTransferItem.tsx` - Individual transfer item
3. Create `index.ts`

---

## Step 3: Create Index File

Create `src/components/file-transfer/index.ts`:

```typescript
export { FileTransfer } from './FileTransfer'
export { FileTransferItem } from './FileTransferItem'
```

---

## Step 4: Implement File Transfer Store

Create `src/stores/fileTransferStore.ts`:

```typescript
import { create } from 'zustand'
import type { FileTransfer, TransferStatus, TransferDirection } from '@/lib/types/file-transfer'

interface FileTransferStore {
  transfers: FileTransfer[]
  activeFilter: 'all' | 'uploading' | 'downloading' | 'completed'

  addTransfer: (transfer: Omit<FileTransfer, 'id' | 'timestamp'>) => string
  updateTransferProgress: (id: string, progress: number, speed: number) => void
  updateTransferStatus: (id: string, status: TransferStatus) => void
  pauseTransfer: (id: string) => void
  resumeTransfer: (id: string) => void
  cancelTransfer: (id: string) => void
  retryTransfer: (id: string) => void
  setFilter: (filter: FileTransferStore['activeFilter']) => void

  getFilteredTransfers: () => FileTransfer[]
}

export const useFileTransferStore = create<FileTransferStore>((set, get) => ({
  transfers: [],
  activeFilter: 'all',

  addTransfer: (transfer) => {
    const id = crypto.randomUUID()
    const newTransfer: FileTransfer = {
      ...transfer,
      id,
      timestamp: new Date()
    }
    set((state) => ({
      transfers: [...state.transfers, newTransfer]
    }))
    return id
  },

  updateTransferProgress: (id, progress, speed) => set((state) => ({
    transfers: state.transfers.map(t =>
      t.id === id ? { ...t, progress, speed } : t
    )
  })),

  updateTransferStatus: (id, status) => set((state) => ({
    transfers: state.transfers.map(t =>
      t.id === id ? { ...t, status } : t
    )
  })),

  pauseTransfer: (id) => get().updateTransferStatus(id, 'paused'),
  resumeTransfer: (id) => get().updateTransferStatus(id, 'transferring'),
  cancelTransfer: (id) => set((state) => ({
    transfers: state.transfers.filter(t => t.id !== id)
  })),
  retryTransfer: (id) => get().updateTransferStatus(id, 'pending'),

  setFilter: (filter) => set({ activeFilter: filter }),

  getFilteredTransfers: () => {
    const { transfers, activeFilter } = get()
    switch (activeFilter) {
      case 'uploading':
        return transfers.filter(t => t.direction === 'upload' && t.status === 'transferring')
      case 'downloading':
        return transfers.filter(t => t.direction === 'download' && t.status === 'transferring')
      case 'completed':
        return transfers.filter(t => t.status === 'completed')
      default:
        return transfers
    }
  }
}))
```

---

## Step 5: Implement File Upload Handler

Create `src/lib/fileUpload.ts`:

```typescript
import type { FileTransfer } from '@/lib/types/file-transfer'

export async function handleFileUpload(
  files: FileList,
  peerId: string,
  peerName: string,
  onProgress: (id: string, progress: number, speed: number) => void
): Promise<string[]> {
  const transferIds: string[] = []

  for (const file of Array.from(files)) {
    const id = crypto.randomUUID()

    // Simulate file upload (replace with actual LAN transfer logic)
    simulateFileTransfer(id, file.size, (progress, speed) => {
      onProgress(id, progress, speed)
    })

    transferIds.push(id)
  }

  return transferIds
}

async function simulateFileTransfer(
  id: string,
  fileSize: number,
  onProgress: (progress: number, speed: number) => void
) {
  let transferred = 0
  const speed = 1024 * 1024 * 5 // 5 MB/s

  const interval = setInterval(() => {
    transferred += speed * 0.1
    const progress = Math.min((transferred / fileSize) * 100, 100)

    onProgress(progress, speed)

    if (progress >= 100) {
      clearInterval(interval)
    }
  }, 100)
}
```

---

## Step 6: Integrate File Transfer

Add file transfer to your app:

```typescript
'use client'

import { useCallback } from 'react'
import { useFileTransferStore } from '@/stores/fileTransferStore'
import { FileTransfer } from '@/components/file-transfer'

function FileTransferPage() {
  const { getFilteredTransfers, activeFilter, setFilter } = useFileTransferStore()

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    const files = e.dataTransfer.files
    if (files.length > 0) {
      // Handle file upload
      console.log('Files dropped:', files)
    }
  }, [])

  const handleFileSelect = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files
    if (files) {
      console.log('Files selected:', files)
    }
  }, [])

  return (
    <div
      className="h-full flex flex-col"
      onDrop={handleDrop}
      onDragOver={(e) => e.preventDefault()}
    >
      <FileTransfer
        transfers={getFilteredTransfers()}
        activeFilter={activeFilter}
        onFilterChange={setFilter}
        onPause={(id) => console.log('Pause:', id)}
        onResume={(id) => console.log('Resume:', id)}
        onCancel={(id) => console.log('Cancel:', id)}
        onRetry={(id) => console.log('Retry:', id)}
        onOpenFile={(id) => console.log('Open:', id)}
        onOpenFolder={(id) => console.log('Open folder:', id)}
      />
    </div>
  )
}
```

---

## Step 7: Add Drag and Drop Zone

Create a drag-and-drop zone component:

```typescript
// src/components/file-transfer/DropZone.tsx
'use client'

import { useCallback, useState } from 'react'

interface DropZoneProps {
  onFilesDrop: (files: FileList) => void
  children: React.ReactNode
}

export function DropZone({ onFilesDrop, children }: DropZoneProps) {
  const [isDragging, setIsDragging] = useState(false)

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(true)
  }, [])

  const handleDragLeave = useCallback(() => {
    setIsDragging(false)
  }, [])

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(false)

    if (e.dataTransfer.files.length > 0) {
      onFilesDrop(e.dataTransfer.files)
    }
  }, [onFilesDrop])

  return (
    <div
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      className={`relative ${isDragging ? 'ring-2 ring-emerald-500' : ''}`}
    >
      {children}
      {isDragging && (
        <div className="absolute inset-0 bg-emerald-500/10 flex items-center justify-center z-10">
          <div className="text-center">
            <p className="text-lg font-medium text-emerald-600">Drop files to transfer</p>
          </div>
        </div>
      )}
    </div>
  )
}
```

---

## Verification

Test the File Transfer section:

1. **Transfer List**:
   - [ ] All transfers display
   - [ ] Filters work correctly
   - [ ] Sort by various criteria

2. **Transfer Items**:
   - [ ] File info displays correctly
   - [ ] Progress bars update
   - [ ] Speed calculations accurate
   - [ ] Status indicators show

3. **Actions**:
   - [ ] Pause/resume work
   - [ ] Cancel removes transfer
   - [ ] Retry restarts failed transfers
   - [ ] Open file/folder works

4. **Drag and Drop**:
   - [ ] Visual feedback on drag
   - [ ] Files can be dropped
   - [ ] Multiple files supported

---

## Next Steps

Once File Transfer is complete, proceed to **[Milestone 6: Collaboration Tools](./06-collaboration.md)**.
