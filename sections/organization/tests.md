# Organization Chart - Test Instructions

## Components Under Test
`OrganizationChart.tsx`, `DepartmentTree.tsx`, `UserCard.tsx`

## Test Data

```typescript
const testDepartments = [
  {
    id: 'dept-root',
    name: '公司',
    parentId: null,
    memberCount: 20
  },
  {
    id: 'dept-tech',
    name: '技术部',
    parentId: 'dept-root',
    memberCount: 12
  },
  {
    id: 'dept-tech-frontend',
    name: '前端组',
    parentId: 'dept-tech',
    memberCount: 5
  },
  {
    id: 'dept-tech-backend',
    name: '后端组',
    parentId: 'dept-tech',
    memberCount: 7
  },
  {
    id: 'dept-product',
    name: '产品部',
    parentId: 'dept-root',
    memberCount: 4
  },
  {
    id: 'dept-design',
    name: '设计部',
    parentId: 'dept-root',
    memberCount: 3
  }
]

const testUsers = [
  {
    id: 'user-1',
    name: '张伟',
    pinyin: 'zhangwei',
    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=ZhangWei',
    position: '前端工程师',
    department: '前端组',
    departmentId: 'dept-tech-frontend',
    status: 'online' as const,
    email: 'zhangwei@company.com',
    phone: '138-0001-0001'
  },
  {
    id: 'user-2',
    name: '李娜',
    pinyin: 'lina',
    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=LiNa',
    position: '产品经理',
    department: '产品部',
    departmentId: 'dept-product',
    status: 'away' as const,
    email: 'lina@company.com',
    phone: '138-0002-0002'
  },
  {
    id: 'user-3',
    name: '王强',
    pinyin: 'wangqiang',
    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=WangQiang',
    position: 'UI设计师',
    department: '设计部',
    departmentId: 'dept-design',
    status: 'offline' as const,
    email: 'wangqiang@company.com',
    phone: '138-0003-0003'
  }
]
```

## Test Cases

### 1. Department Tree Rendering
- [ ] All departments render without errors
- [ ] Tree structure displays hierarchy correctly
- [ ] Root department shows at top level
- [ ] Child departments are indented
- [ ] Member count displays for each department
- [ ] Expand/collapse icon shows for parent departments
- [ ] Leaf departments have no expand icon

### 2. Tree Navigation
- [ ] Clicking expand icon shows children
- [ ] Clicking collapse icon hides children
- [ ] Expanded state persists
- [ ] Expand/collapse is animated smoothly
- [ ] Can expand multiple departments simultaneously
- [ ] Can collapse all departments
- [ ] Can expand all departments

### 3. Department Selection
- [ ] Clicking department selects it
- [ ] Selected department is highlighted
- [ ] Only one department can be selected at a time
- [ ] Clicking already selected department keeps selection
- [ ] Selecting department filters user list
- [ ] Selection can be cleared

### 4. User List Display
- [ ] All users render without errors
- [ ] Users display in grid layout
- [ ] User cards show correct information
- [ ] Avatar displays correctly
- [ ] Name displays correctly
- [ ] Position displays correctly
- [ ] Department displays correctly
- [ ] Status indicator shows correct state
- [ ] Email displays correctly
- [ ] Phone displays correctly

### 5. User Card
- [ ] Card displays all user info
- [ ] Avatar has status indicator
- [ ] Status indicator color matches state
- [ ] Online = green dot
- [ ] Away = orange dot
- [ ] Offline = gray dot
- [ ] Hover effect works
- [ ] Long names truncate properly
- [ ] Long positions truncate properly

### 6. User Actions
- [ ] "Chat" button initiates conversation
- [ ] "Details" button shows user profile
- [ ] Actions trigger correct callbacks
- [ ] Disabled users don't show action buttons
- [ ] Multiple rapid clicks handled correctly

### 7. Search Functionality
- [ ] Search input filters users in real-time
- [ ] Search by name works
- [ ] Search by pinyin works
- [ ] Search by department works
- [ ] Search by position works
- [ ] Search is case-insensitive
- [ ] Clear search resets user list
- [ ] Empty search results show message
- [ ] Search preserves department filter

### 8. Statistics Display
- [ ] Online count is accurate
- [ ] Away count is accurate
- [ ] Offline count is accurate
- [ ] Total count is accurate
- [ ] Counts update when users change status
- [ ] Color indicators match status colors

### 9. Filtering
- [ ] Selecting department filters users
- [ ] Filter shows only department members
- [ ] Filter result count updates
- [ ] Clear selection shows all users
- [ ] Search within department filter works
- [ ] Search and filter work together

### 10. Empty States
- [ ] No users shows empty state
- [ ] Empty search results show message
- [ ] Empty department shows "No members"
- [ ] Empty state illustration displays
- [ ] Clear action to reset filters

### 11. Dark Mode
- [ ] Department tree adapts correctly
- [ ] Selected item visible in dark mode
- [ ] User cards adapt correctly
- [ ] Status indicators visible
- [ ] Text contrast is sufficient
- [ ] Borders visible

### 12. Responsive Behavior
- [ ] Mobile shows single column
- [ ] Department tree collapses to dropdown
- [ ] User grid becomes single column
- [ ] Search remains accessible
- [ ] Statistics hide on small screens
- [ ] Desktop shows dual-pane layout

## Edge Cases

- [ ] Department with 0 members
- [ ] Department with very long name
- [ ] User with very long name (>20 chars)
- [ ] User with very long position
- [ ] User with missing avatar (fallback)
- [ ] User with special characters in name
- [ ] Department with special characters
- [ ] Very deep hierarchy (>5 levels)
- [ ] Very wide hierarchy (>10 siblings)
- [ ] User with duplicate names
- [ ] Search with special characters
- [ ] Search with very long query

## Performance Tests

- [ ] Renders 100+ departments without lag
- [ ] Renders 500+ users without lag
- [ ] Search filters instantly
- [ ] Expand/collapse is smooth
- [ ] Scroll is smooth with many users
- [ ] Tree with 10+ levels handles correctly

## Integration Tests

- [ ] Starting chat navigates to messaging
- [ ] User details show modal/side panel
- [ ] Status updates reflect in real-time
- [ ] New users appear automatically
- [ ] Department changes update display
