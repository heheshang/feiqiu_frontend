export interface NavItem {
  id: string
  label: string
  icon: string
  path: string
}

export interface UserMenuItem {
  id: string
  label: string
  icon: string
  action: () => void
}

export type NavTab = 'chat' | 'contacts' | 'organization'|'files'

export type UserStatus = 'online' | 'away' | 'busy' | 'offline'

export interface User {
  name: string
  avatar: string
  status: UserStatus
}
