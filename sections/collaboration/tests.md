# Collaboration Tools - Test Instructions

## Component Under Test
`CollaborationTools.tsx`

## Test Data

```typescript
const testScreenshots = [
  {
    id: 'screenshot-1',
    imageUrl: 'data:image/png;base64,iVBORw0KGgo...',
    timestamp: new Date('2025-01-08T10:00:00'),
    annotations: [
      {
        id: 'annot-1',
        type: 'arrow' as const,
        x: 100,
        y: 100,
        color: '#ef4444'
      },
      {
        id: 'annot-2',
        type: 'rectangle' as const,
        x: 200,
        y: 150,
        width: 100,
        height: 50,
        color: '#3b82f6'
      }
    ]
  },
  {
    id: 'screenshot-2',
    imageUrl: 'data:image/png;base64,iVBORw0KGgo...',
    timestamp: new Date('2025-01-08T09:30:00'),
    annotations: []
  },
  {
    id: 'screenshot-3',
    imageUrl: 'data:image/png;base64,iVBORw0KGgo...',
    timestamp: new Date('2025-01-08T09:00:00'),
    annotations: [
      {
        id: 'annot-3',
        type: 'text' as const,
        x: 50,
        y: 50,
        text: '注意这个',
        color: '#10b981'
      }
    ]
  }
]
```

## Test Cases

### 1. Screenshot List Rendering
- [ ] All screenshots render without errors
- [ ] Screenshots sort by timestamp (newest first)
- [ ] Thumbnail displays correctly
- [ ] Timestamp shows relative time
- [ ] Selected screenshot is highlighted
- [ ] Screenshot count badge shows
- [ ] Empty state shows when no screenshots

### 2. Screenshot Capture
- [ ] Capture button triggers screen picker
- [ ] Screen picker shows available screens
- [ ] Selecting screen captures screenshot
- [ ] Capture is added to list
- [ ] New screenshot is automatically selected
- [ ] Capture button disabled during capture
- [ ] Multiple captures work in succession
- [ ] Cancel capture returns to list

### 3. Screenshot Display
- [ ] Selected screenshot displays at full size
- [ ] Image loads without distortion
- [ ] Pan and zoom work for large screenshots
- [ ] Scrollbars appear when needed
- [ ] Image fits in viewport initially

### 4. Annotation Tools
- [ ] Toolbar shows all annotation tools
- [ ] Arrow tool is selectable
- [ ] Rectangle tool is selectable
- [ ] Text tool is selectable
- [ ] Blur tool is selectable
- [ ] Active tool is highlighted
- [ ] Tool cursor changes to match tool
- [ ] Color picker shows preset colors
- [ ] Custom color can be selected

### 5. Drawing Annotations
- [ ] Arrow can be drawn on screenshot
- [ ] Arrow preview shows while drawing
- [ ] Rectangle can be drawn
- [ ] Rectangle preview shows while drawing
- [ ] Text can be placed
- [ ] Text input appears on click
- [ ] Text can be edited after placement
- [ ] Blur area can be drawn
- [ ] Blur effect is visible

### 6. Annotation Management
- [ ] Annotations appear on screenshot
- [ ] Annotations can be selected
- [ ] Selected annotation shows handles
- [ ] Annotations can be moved
- [ ] Annotations can be resized (where applicable)
- [ ] Annotations can be deleted
- [ ] Delete button removes selected annotation
- [ ] All annotations can be cleared

### 7. Screenshot Actions
- [ ] Download button saves image
- [ ] Downloaded image includes annotations
- [ ] Share button opens share dialog
- [ ] Share dialog shows user list
- [ ] Share sends to selected users
- [ ] Delete button removes screenshot
- [ ] Delete confirmation shows
- [ ] Delete can be cancelled

### 8. Screenshot History
- [ ] History shows all screenshots
- [ ] Clicking history item selects screenshot
- [ ] History thumbnails are readable
- [ ] History items show timestamp
- [ ] History items show annotation count
- [ ] History persists across sessions

### 9. Search and Filter
- [ ] Search filters by annotation text
- [ ] Search filters by timestamp
- [ ] Clear search resets list
- [ ] Empty search results show message

### 10. Empty States
- [ ] No screenshots shows empty illustration
- [ ] Empty state message is clear
- [ ] CTA button ("Capture screenshot") is visible
- [ ] First capture flow works

### 11. Dark Mode
- [ ] Toolbar adapts correctly
- [ ] Screenshot display area adapts
- [ ] Annotation colors remain visible
- [ ] Selected highlights visible
- [ ] Text contrast is sufficient

### 12. Responsive Behavior
- [ ] Mobile shows toolbar at bottom
- [ ] Screenshot fills available space
- [ ] History collapses to thumbnail strip
- [ ] Annotation tools remain accessible
- [ ] Pan/zoom works with touch gestures

## Edge Cases

- [ ] Very large screenshots (>4K) are handled
- [ ] Very long screenshots (scroll capture)
- [ ] Screenshot with no annotations
- [ ] Screenshot with many annotations (>50)
- [ ] Overlapping annotations render correctly
- [ ] Annotations off-screen are accessible
- [ ] Text annotation with very long text
- [ ] Text annotation with special characters
- [ ] Blur area covering entire image

## Performance Tests

- [ ] Screenshot capture completes in <2s
- [ ] Large image renders without lag
- [ ] Drawing annotations is smooth (60fps)
- [ ] Pan/zoom is smooth
- [ ] History with 100+ screenshots scrolls smoothly

## Integration Tests

- [ ] Screenshot can be shared to chat
- [ ] Shared screenshot appears in conversation
- [ ] Annotations are preserved when sharing
- [ ] Received screenshots can be annotated
- [ ] Screenshots persist across page reload
- [ ] Screenshots save to IndexedDB/local storage

## Browser Compatibility

- [ ] Screen capture works in Chrome
- [ ] Screen capture works in Edge
- [ ] Screen capture works in Firefox
- [ ] Graceful fallback if capture not supported
- [ ] Canvas rendering works in all browsers
