# File Transfer - Test Instructions

## Components Under Test
`FileTransfer.tsx`, `FileTransferItem.tsx`

## Test Data

```typescript
const testTransfers = [
  {
    id: 'transfer-1',
    fileName: 'project-design.pdf',
    fileSize: 2545612,
    fileType: 'application/pdf',
    direction: 'upload' as const,
    status: 'transferring' as const,
    progress: 65,
    speed: 2048000,
    peerId: 'user-2',
    peerName: '李娜',
    timestamp: new Date('2025-01-08T10:00:00')
  },
  {
    id: 'transfer-2',
    fileName: 'presentation.pptx',
    fileSize: 5242880,
    fileType: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
    direction: 'download' as const,
    status: 'completed' as const,
    progress: 100,
    speed: 0,
    peerId: 'user-3',
    peerName: '王强',
    timestamp: new Date('2025-01-08T09:30:00')
  },
  {
    id: 'transfer-3',
    fileName: 'large-video.mp4',
    fileSize: 524288000,
    fileType: 'video/mp4',
    direction: 'upload' as const,
    status: 'paused' as const,
    progress: 35,
    speed: 0,
    peerId: 'user-4',
    peerName: '赵敏',
    timestamp: new Date('2025-01-08T08:00:00')
  },
  {
    id: 'transfer-4',
    fileName: 'document.docx',
    fileSize: 102400,
    fileType: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    direction: 'download' as const,
    status: 'failed' as const,
    progress: 45,
    speed: 0,
    peerId: 'user-5',
    peerName: '孙丽',
    timestamp: new Date('2025-01-08T07:00:00')
  }
]
```

## Test Cases

### 1. Transfer List Rendering
- [ ] All transfers render without errors
- [ ] Transfers sort by timestamp (newest first)
- [ ] Active filter displays correctly
- [ ] Filter tabs show counts
- [ ] Empty state shows when no transfers
- [ ] Loading state shows during operations

### 2. Transfer Item Display
- [ ] File icon matches file type
- [ ] File name displays correctly
- [ ] File size displays in human-readable format
- [ ] Direction indicator shows (up/down arrow)
- [ ] Peer name displays
- [ ] Timestamp shows relative time
- [ ] Status badge shows correct state
- [ ] Progress bar fills to correct percentage
- [ ] Transfer speed displays (for active transfers)
- [ ] ETA displays (for active transfers)

### 3. Progress Visualization
- [ ] Progress bar fills smoothly
- [ ] Progress percentage updates
- [ ] Speed calculation is accurate
- [ ] ETA calculation is reasonable
- [ ] Progress bar color matches status
- [ ] Paused transfers show distinct style
- [ ] Failed transfers show error indicator

### 4. Transfer Actions
- [ ] Pause button shows for active transfers
- [ ] Resume button shows for paused transfers
- [ ] Cancel button shows for non-completed transfers
- [ ] Retry button shows for failed transfers
- [ ] Open file button shows for completed downloads
- [ ] Open folder button shows for completed transfers
- [ ] Actions respond to clicks
- [ ] Actions update transfer status

### 5. Filter Functionality
- [ ] "All" filter shows all transfers
- [ ] "Uploading" filter shows only uploads
- [ ] "Downloading" filter shows only downloads
- [ ] "Completed" filter shows only completed
- [ ] Filter updates are instant
- [ ] Active filter is highlighted
- [ ] Filter counts are accurate

### 6. Drag and Drop
- [ ] Drag over shows visual feedback
- [ ] Drop zone highlights when dragging
- [ ] Multiple files can be dropped
- [ ] Folders can be dropped
- [ ] Cancel drag (leave zone) removes feedback
- [ ] Drop creates transfer items
- [ ] File names are preserved
- [ ] File sizes are calculated

### 7. File Upload
- [ ] File picker opens on button click
- [ ] Multiple files can be selected
- [ ] File type detection works
- [ ] Large files (>100MB) are handled
- [ ] Very large files (>1GB) are handled
- [ ] Upload starts immediately
- [ ] Upload can be paused
- [ ] Upload can be resumed
- [ ] Upload can be cancelled

### 8. File Download
- [ ] Download starts on accept
- [ ] Download location can be chosen
- [ ] Download can be paused
- [ ] Download can be resumed
- [ ] Download can be cancelled
- [ ] Completed files are accessible
- [ ] Download errors are handled

### 9. Status States
- [ ] **Pending**: Shows "等待中..."
- [ ] **Transferring**: Shows progress and speed
- [ ] **Paused**: Shows "已暂停", progress bar
- [ ] **Completed**: Shows "完成", 100% progress
- [ ] **Failed**: Shows "失败", error message, retry button

### 10. Empty States
- [ ] No transfers shows empty illustration
- [ ] Empty state message is clear
- [ ] CTA button is visible ("Select files to transfer")
- [ ] Filtered empty state shows appropriate message

### 11. Dark Mode
- [ ] Transfer items adapt correctly
- [ ] Progress bars visible in both modes
- [ ] Action buttons accessible
- [ ] Status badges visible
- [ ] File icons visible
- [ ] Text contrast is sufficient

### 12. Responsive Behavior
- [ ] Mobile layout stacks vertically
- [ ] Progress bars scale correctly
- [ ] Action buttons remain accessible
- [ ] File names truncate on small screens
- [ ] Peer info hides on very small screens

## Edge Cases

- [ ] File with very long name displays with truncation
- [ ] File with special characters in name
- [ ] File with unicode characters in name
- [ ] File with no extension
- [ ] Unknown file type shows generic icon
- [ ] Zero-byte file handled correctly
- [ ] Transfer speed 0 shows correctly
- [ ] Transfer speed updates from 0 to actual value
- [ ] Progress at exactly 0%
- [ ] Progress at exactly 100%
- [ ] Progress at 99.9% (rounding)

## Performance Tests

- [ ] Renders 100+ transfers without lag
- [ ] Progress updates are smooth (60fps)
- [ ] Speed calculations don't block UI
- [ ] Multiple simultaneous transfers work
- [ ] Drag and drop handles many files

## Integration Tests

- [ ] Transfers persist across page navigation
- [ ] Transfer history saved to local storage
- [ ] Completed transfers can be cleared
- [ ] Transfer can be initiated from chat
- [ ] Transfer can be initiated from file browser
