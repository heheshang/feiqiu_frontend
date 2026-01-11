export type UserStatus = 'online' | 'away' | 'offline'

export interface Department {
  id: string
  name: string
  parentId: string | null
  level: number
  memberCount: number
}

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

export interface OrganizationChartProps {
  currentUser: User
  departments: Department[]
  users: User[]
  onDepartmentSelect?: (departmentId: string) => void
  onStartChat?: (userId: string) => void
  onViewDetails?: (userId: string) => void
  onSearch?: (query: string) => void
}
