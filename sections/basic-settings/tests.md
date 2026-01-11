# Basic Settings - Test Instructions

## Component Under Test
`BasicSettings.tsx`, `NetworkStatusCard.tsx`

## Test Data

```typescript
const testUser = {
  id: 'user-1',
  name: '张伟',
  pinyin: 'zhangwei',
  avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=ZhangWei',
  signature: '努力工作，认真生活',
  status: 'online' as const,
  email: 'zhangwei@company.com',
  phone: '138-0001-0001',
  department: '前端组'
}

const testNetworkConfig = {
  udpPort: 2425,
  listenAddress: '0.0.0.0',
  autoDiscovery: true,
  maxConnections: 100
}

const testNetworkStatus = {
  isConnected: true,
  currentPort: 2425,
  connectedPeers: 15,
  latency: 12
}
```

## Test Cases

### 1. Component Rendering
- [ ] Component renders without errors
- [ ] Both tabs (Personal Info, Network) are visible
- [ ] Active tab is highlighted
- [ ] User avatar displays correctly
- [ ] All form fields render

### 2. Personal Info Tab
- [ ] All user fields display current values
- [ ] Name field is editable
- [ ] Signature field is editable
- [ ] Email field is editable
- [ ] Phone field is editable
- [ ] Status dropdown shows all options
- [ ] Avatar upload button works
- [ ] Save button is enabled only when changes are made
- [ ] Cancel button reverts unsaved changes

### 3. Network Config Tab
- [ ] UDP port field displays current value
- [ ] Listen address field displays current value
- [ ] Auto discovery toggle shows current state
- [ ] Max connections field displays current value
- [ ] Port validation rejects invalid values (< 1024, > 65535)
- [ ] Test connection button is clickable
- [ ] Save button works

### 4. Network Status Card
- [ ] Connection status displays (connected/disconnected)
- [ ] Current port displays
- [ ] Connected peers count displays
- [ ] Latency displays with unit (ms)
- [ ] Status indicator color matches state
- [ ] Card updates when props change

### 5. Tab Switching
- [ ] Clicking "Personal Info" tab switches view
- [ ] Clicking "Network Config" tab switches view
- [ ] Tab state persists across switches
- [ ] Unsaved changes warning shows when switching

### 6. Form Validation
- [ ] Email validation shows error for invalid format
- [ ] Phone validation shows error for invalid format
- [ ] Port validation shows error for out-of-range values
- [ ] Required field validation works
- [ ] Validation errors are clear and helpful

### 7. User Actions
- [ ] Save changes calls `onUserUpdate` with correct data
- [ ] Save network config calls `onNetworkConfigChange`
- [ ] Test connection calls `onTestConnection`
- [ ] Upload avatar triggers file picker
- [ ] Cancel edit reverts form to original values

### 8. Empty States
- [ ] Handles missing user signature
- [ ] Handles empty email/phone
- [ ] Handles missing network status

### 9. Dark Mode
- [ ] All colors adapt correctly in dark mode
- [ ] Input borders visible in both modes
- [ ] Text contrast is sufficient
- [ ] Icons display correctly

### 10. Responsive Behavior
- [ ] Layout adapts to mobile screens
- [ ] Form fields stack on small screens
- [ ] Tabs remain accessible on mobile
- [ ] Status card fits on small screens

## Edge Cases

- [ ] Very long user signatures display with truncation
- [ ] Very long email addresses display properly
- [ ] Avatar images fail to load (fallback)
- [ ] Network config with maximum port value (65535)
- [ ] Network config with minimum port value (1024)
- [ ] Zero connected peers displays correctly
- [ ] Very high latency values display correctly

## Integration Tests

- [ ] Changes persist when switching tabs
- [ ] Multiple rapid save operations handle correctly
- [ ] Network test updates status card
- [ ] Avatar upload updates preview immediately
