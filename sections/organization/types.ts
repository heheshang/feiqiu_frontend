// =============================================================================
// Data Types
// =============================================================================

/**
 * 在线状态
 */
export type UserStatus = 'online' | 'away' | 'offline'

/**
 * 部门
 */
export interface Department {
  id: string
  name: string
  parentId: string | null
  level: number
  memberCount: number
}

/**
 * 用户
 */
export interface User {
  id: string
  name: string
  pinyin: string
  avatar: string
  position: string
  department: string
  departmentId: string
  status: UserStatus
  email: string
  phone: string
}

// =============================================================================
// Component Props
// =============================================================================

/**
 * OrganizationChart 组件的 Props
 */
export interface OrganizationChartProps {
  /** 当前用户 */
  currentUser: User
  /** 部门列表 */
  departments: Department[]
  /** 用户列表 */
  users: User[]
  /** 选中部门变化时的回调 */
  onDepartmentSelect?: (departmentId: string) => void
  /** 发起聊天 */
  onStartChat?: (userId: string) => void
  /** 查看用户详情 */
  onViewDetails?: (userId: string) => void
  /** 搜索用户 */
  onSearch?: (query: string) => void
}
