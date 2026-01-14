/**
 * Contacts Types
 *
 * Type definitions for the contacts feature including contacts, groups,
 * filters, and statistics. All types match the backend DTO structure
 * with camelCase naming convention.
 */

/**
 * Contact view mode determines how contacts are displayed
 */
export type ContactView = 'list' | 'grid' | 'groups' | 'departments'

/**
 * Contact sort options
 */
export type ContactSortBy =
  | 'name'
  | 'department'
  | 'lastSeen'
  | 'created'
  | 'favorites'

/**
 * Contact sort order
 */
export type ContactSortOrder = 'asc' | 'desc'

/**
 * Contact entity representing a single contact in the system
 */
export interface Contact {
  /** Database ID */
  id: number
  /** Reference to peer ID (if linked to a network peer) */
  peerId?: number
  /** Display name */
  name: string
  /** Custom nickname (optional) */
  nickname?: string
  /** Avatar URL or path */
  avatar?: string
  /** Phone number */
  phone?: string
  /** Email address */
  email?: string
  /** Department name */
  department?: string
  /** Job position/title */
  position?: string
  /** Additional notes */
  notes?: string
  /** Whether marked as favorite */
  isFavorite: boolean
  /** Pinyin for Chinese name search */
  pinyin?: string
  /** Online status */
  isOnline: boolean
  /** Last seen timestamp (milliseconds since epoch) */
  lastSeen?: number
  /** Creation timestamp (milliseconds since epoch) */
  createdAt: number
  /** Last update timestamp (milliseconds since epoch) */
  updatedAt?: number
  /** Groups this contact belongs to */
  groups: string[]
}

/**
 * Contact group for organizing contacts
 */
export interface ContactGroup {
  /** Database ID */
  id: number
  /** Group name */
  name: string
  /** Color code for UI display */
  color?: string
  /** Icon name or identifier */
  icon?: string
  /** Sort order for display */
  sortOrder: number
  /** Creation timestamp (milliseconds since epoch) */
  createdAt: number
  /** Number of members in the group */
  memberCount: number
}

/**
 * Filters for querying contacts
 */
export interface ContactFilters {
  /** Search query (searches name, nickname, pinyin, etc.) */
  search?: string
  /** Filter by online status */
  isOnline?: boolean
  /** Filter by favorite status */
  isFavorite?: boolean
  /** Filter by department */
  department?: string
  /** Filter by group ID */
  groupId?: number
}

/**
 * Contact statistics
 */
export interface ContactStats {
  /** Total number of contacts */
  total: number
  /** Number of online contacts */
  online: number
  /** Number of offline contacts */
  offline: number
  /** Number of favorite contacts */
  favorites: number
  /** Contacts breakdown by department */
  byDepartment: Record<string, number>
}

/**
 * Input data for creating a new contact
 */
export interface CreateContactInput {
  /** Reference to peer ID (optional) */
  peerId?: number
  /** Display name (required) */
  name: string
  /** Custom nickname */
  nickname?: string
  /** Avatar URL or path */
  avatar?: string
  /** Phone number */
  phone?: string
  /** Email address */
  email?: string
  /** Department name */
  department?: string
  /** Job position/title */
  position?: string
  /** Additional notes */
  notes?: string
  /** Pinyin for Chinese name search */
  pinyin?: string
}

/**
 * Input data for updating an existing contact
 * All fields are optional - only provided fields will be updated
 */
export interface UpdateContactInput {
  /** Display name */
  name?: string
  /** Custom nickname */
  nickname?: string
  /** Avatar URL or path */
  avatar?: string
  /** Phone number */
  phone?: string
  /** Email address */
  email?: string
  /** Department name */
  department?: string
  /** Job position/title */
  position?: string
  /** Additional notes */
  notes?: string
  /** Pinyin for Chinese name search */
  pinyin?: string
  /** Whether marked as favorite */
  isFavorite?: boolean
}

/**
 * Input data for creating a new contact group
 */
export interface CreateContactGroupInput {
  /** Group name (required) */
  name: string
  /** Color code for UI display */
  color?: string
  /** Icon name or identifier */
  icon?: string
  /** Sort order for display */
  sortOrder?: number
}

/**
 * Input data for updating an existing contact group
 * All fields are optional - only provided fields will be updated
 */
export interface UpdateContactGroupInput {
  /** Group name */
  name?: string
  /** Color code for UI display */
  color?: string
  /** Icon name or identifier */
  icon?: string
  /** Sort order for display */
  sortOrder?: number
}

/**
 * Batch operation result
 */
export interface BatchOperationResult {
  /** Number of successful operations */
  success: number
  /** Number of failed operations */
  failed: number
  /** Error messages for failed operations */
  errors: Array<{ id: number; error: string }>
}

/**
 * Contacts view state for UI components
 */
export interface ContactsViewState {
  /** Current view mode */
  viewMode: ContactView
  /** Current sort field */
  sortBy: ContactSortBy
  /** Current sort order */
  sortOrder: ContactSortOrder
  /** Active filters */
  filters: ContactFilters
  /** Selected contact IDs */
  selectedIds: Set<number>
  /** Expanded group IDs (for groups view mode) */
  expandedGroups: Set<number>
}

/**
 * Contact list item for display purposes
 * Combines contact with computed properties
 */
export interface ContactListItem extends Contact {
  /** Display text (uses nickname if available, otherwise name) */
  displayName: string
  /** Status indicator text */
  statusText: string
  /** Whether is currently online (computed with idle threshold) */
  isCurrentlyOnline: boolean
}

/**
 * Contact with group membership info
 */
export interface ContactWithGroups extends Contact {
  /** Full group objects this contact belongs to */
  groupDetails: ContactGroup[]
}

/**
 * Group with contacts pre-loaded
 */
export interface GroupWithContacts extends ContactGroup {
  /** Contacts in this group */
  contacts: Contact[]
}
