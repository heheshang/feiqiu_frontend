/**
 * useContacts Hook
 *
 * Custom hook for fetching and managing contact data from the backend.
 * Provides CRUD operations, filtering, and real-time updates when contacts change.
 */

import { useState, useEffect, useCallback, useRef } from 'react'
import { contactsApi } from '@/lib/api'
import type {
  Contact,
  ContactGroup,
  ContactFilters,
  ContactStats,
  ContactView,
  ContactSortBy,
  ContactSortOrder,
  CreateContactInput,
  UpdateContactInput,
  CreateContactGroupInput,
  UpdateContactGroupInput,
  BatchOperationResult,
} from '@/lib/types/contacts'

/**
 * Hook return value
 */
export interface UseContactsResult {
  /** Array of contacts */
  contacts: Contact[]
  /** Array of contact groups */
  groups: ContactGroup[]
  /** Loading state */
  isLoading: boolean
  /** Error state */
  error: Error | null
  /** Contact statistics */
  stats: ContactStats | null
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
  /** Function to manually refresh data */
  refresh: () => Promise<void>
  /** Function to set view mode */
  setViewMode: (view: ContactView) => void
  /** Function to set sort field */
  setSortBy: (sortBy: ContactSortBy) => void
  /** Function to set sort order */
  setSortOrder: (order: ContactSortOrder) => void
  /** Function to set filters */
  setFilters: (filters: ContactFilters) => void
  /** Function to clear filters */
  clearFilters: () => void
  /** Function to select/deselect contacts */
  toggleSelection: (id: number) => void
  /** Function to select all contacts */
  selectAll: () => void
  /** Function to clear selection */
  clearSelection: () => void
  /** Function to create a new contact */
  createContact: (contact: CreateContactInput) => Promise<Contact>
  /** Function to update a contact */
  updateContact: (id: number, contact: UpdateContactInput) => Promise<Contact>
  /** Function to delete a contact */
  deleteContact: (id: number) => Promise<void>
  /** Function to delete multiple contacts */
  deleteContacts: (ids: number[]) => Promise<BatchOperationResult>
  /** Function to create a new group */
  createGroup: (group: CreateContactGroupInput) => Promise<ContactGroup>
  /** Function to update a group */
  updateGroup: (id: number, group: UpdateContactGroupInput) => Promise<ContactGroup>
  /** Function to delete a group */
  deleteGroup: (id: number) => Promise<void>
  /** Function to add contacts to a group */
  addToGroup: (groupId: number, contactIds: number[]) => Promise<void>
  /** Function to remove contacts from a group */
  removeFromGroup: (groupId: number, contactIds: number[]) => Promise<void>
  /** Function to search contacts */
  search: (query: string) => Promise<Contact[]>
  /** Function to toggle favorite status */
  toggleFavorite: (id: number) => Promise<void>
}

/**
 * Options for the useContacts hook
 */
export interface UseContactsOptions {
  /** Initial filters to apply */
  initialFilters?: ContactFilters
  /** Initial view mode */
  initialViewMode?: ContactView
  /** Initial sort field */
  initialSortBy?: ContactSortBy
  /** Initial sort order */
  initialSortOrder?: ContactSortOrder
  /** Whether to fetch contacts on mount (default: true) */
  enabled?: boolean
  /** Polling interval in milliseconds (default: 0 - no polling) */
  refreshInterval?: number
}

/**
 * Default state values
 */
const DEFAULT_VIEW_MODE: ContactView = 'list'
const DEFAULT_SORT_BY: ContactSortBy = 'name'
const DEFAULT_SORT_ORDER: ContactSortOrder = 'asc'

/**
 * Hook for fetching and managing contact data
 *
 * @param options - Hook options
 * @returns Contact data and management functions
 *
 * @example
 * ```tsx
 * function ContactList() {
 *   const { contacts, groups, isLoading, createContact, deleteContact } = useContacts()
 *
 *   if (isLoading) return <div>Loading...</div>
 *
 *   return (
 *     <div>
 *       <button onClick={() => createContact({ name: 'John Doe' })}>
 *         Add Contact
 *       </button>
 *       <ul>
 *         {contacts.map(contact => (
 *           <li key={contact.id}>
 *             {contact.name}
 *             <button onClick={() => deleteContact(contact.id)}>Delete</button>
 *           </li>
 *         ))}
 *       </ul>
 *     </div>
 *   )
 * }
 * ```
 */
export function useContacts(options: UseContactsOptions = {}): UseContactsResult {
  const {
    initialFilters,
    initialViewMode = DEFAULT_VIEW_MODE,
    initialSortBy = DEFAULT_SORT_BY,
    initialSortOrder = DEFAULT_SORT_ORDER,
    enabled = true,
    refreshInterval = 0,
  } = options

  const [contacts, setContacts] = useState<Contact[]>([])
  const [groups, setGroups] = useState<ContactGroup[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<Error | null>(null)
  const [stats, setStats] = useState<ContactStats | null>(null)
  const [viewMode, setViewModeState] = useState<ContactView>(initialViewMode)
  const [sortBy, setSortByState] = useState<ContactSortBy>(initialSortBy)
  const [sortOrder, setSortOrderState] = useState<ContactSortOrder>(initialSortOrder)
  const [filters, setFiltersState] = useState<ContactFilters>(initialFilters || {})
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set())

  // Use ref to track if component is mounted
  const isMountedRef = useRef(true)
  const pollingIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null)

  /**
   * Apply sorting to contacts array
   */
  const sortContacts = useCallback(
    (contactsToSort: Contact[]): Contact[] => {
      const sorted = [...contactsToSort].sort((a, b) => {
        let comparison = 0

        switch (sortBy) {
          case 'name':
            comparison = a.name.localeCompare(b.name, 'zh-CN')
            break
          case 'department':
            if (!a.department && !b.department) comparison = 0
            else if (!a.department) comparison = 1
            else if (!b.department) comparison = -1
            else comparison = a.department.localeCompare(b.department, 'zh-CN')
            break
          case 'lastSeen':
            if (!a.lastSeen && !b.lastSeen) comparison = 0
            else if (!a.lastSeen) comparison = 1
            else if (!b.lastSeen) comparison = -1
            else comparison = a.lastSeen - b.lastSeen
            break
          case 'created':
            comparison = a.createdAt - b.createdAt
            break
          case 'favorites':
            // Favorites first, then by name
            if (a.isFavorite && !b.isFavorite) comparison = -1
            else if (!a.isFavorite && b.isFavorite) comparison = 1
            else comparison = a.name.localeCompare(b.name, 'zh-CN')
            break
        }

        return sortOrder === 'asc' ? comparison : -comparison
      })

      return sorted
    },
    [sortBy, sortOrder]
  )

  /**
   * Fetch contacts and groups from backend
   */
  const fetchData = useCallback(async () => {
    if (!enabled || !isMountedRef.current) {
      return
    }

    setIsLoading(true)
    setError(null)

    try {
      const [contactsData, groupsData, statsData] = await Promise.all([
        contactsApi.getContacts(filters),
        contactsApi.getContactGroups(),
        contactsApi.getContactStats(),
      ])

      if (isMountedRef.current) {
        setContacts(sortContacts(contactsData))
        setGroups(groupsData)
        setStats(statsData)
      }
    } catch (err) {
      if (isMountedRef.current) {
        setError(err instanceof Error ? err : new Error(String(err)))
        console.error('[useContacts] Failed to fetch contacts:', err)
      }
    } finally {
      if (isMountedRef.current) {
        setIsLoading(false)
      }
    }
  }, [enabled, filters, sortContacts])

  /**
   * Manually refresh data
   */
  const refresh = useCallback(async () => {
    await fetchData()
  }, [fetchData])

  /**
   * Create a new contact
   */
  const createContact = useCallback(async (contact: CreateContactInput): Promise<Contact> => {
    try {
      const newContact = await contactsApi.createContact(contact)

      if (isMountedRef.current) {
        setContacts((prev) => sortContacts([...prev, newContact]))
      }

      return newContact
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [sortContacts])

  /**
   * Update an existing contact
   */
  const updateContact = useCallback(async (id: number, contact: UpdateContactInput): Promise<Contact> => {
    try {
      const updatedContact = await contactsApi.updateContact(id, contact)

      if (isMountedRef.current) {
        setContacts((prev) =>
          sortContacts(prev.map((c) => (c.id === id ? updatedContact : c)))
        )
      }

      return updatedContact
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [sortContacts])

  /**
   * Delete a contact
   */
  const deleteContact = useCallback(async (id: number): Promise<void> => {
    try {
      await contactsApi.deleteContact(id)

      if (isMountedRef.current) {
        setContacts((prev) => prev.filter((c) => c.id !== id))
        setSelectedIds((prev) => {
          const next = new Set(prev)
          next.delete(id)
          return next
        })
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [])

  /**
   * Delete multiple contacts
   */
  const deleteContacts = useCallback(async (ids: number[]): Promise<BatchOperationResult> => {
    try {
      const result = await contactsApi.deleteContacts(ids)

      if (isMountedRef.current) {
        setContacts((prev) => prev.filter((c) => !ids.includes(c.id)))
        setSelectedIds((prev) => {
          const next = new Set(prev)
          ids.forEach(id => next.delete(id))
          return next
        })
      }

      return result
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [])

  /**
   * Create a new group
   */
  const createGroup = useCallback(async (group: CreateContactGroupInput): Promise<ContactGroup> => {
    try {
      const newGroup = await contactsApi.createContactGroup(group)

      if (isMountedRef.current) {
        setGroups((prev) => [...prev, newGroup])
      }

      return newGroup
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [])

  /**
   * Update a group
   */
  const updateGroup = useCallback(async (id: number, group: UpdateContactGroupInput): Promise<ContactGroup> => {
    try {
      const updatedGroup = await contactsApi.updateContactGroup(id, group)

      if (isMountedRef.current) {
        setGroups((prev) =>
          prev.map((g) => (g.id === id ? updatedGroup : g))
        )
      }

      return updatedGroup
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [])

  /**
   * Delete a group
   */
  const deleteGroup = useCallback(async (id: number): Promise<void> => {
    try {
      await contactsApi.deleteContactGroup(id)

      if (isMountedRef.current) {
        setGroups((prev) => prev.filter((g) => g.id !== id))
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [])

  /**
   * Add contacts to a group
   */
  const addToGroup = useCallback(async (groupId: number, contactIds: number[]): Promise<void> => {
    try {
      await contactsApi.addContactsToGroup(groupId, contactIds)

      // Refresh to get updated data
      await refresh()
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [refresh])

  /**
   * Remove contacts from a group
   */
  const removeFromGroup = useCallback(async (groupId: number, contactIds: number[]): Promise<void> => {
    try {
      await contactsApi.removeContactsFromGroup(groupId, contactIds)

      // Refresh to get updated data
      await refresh()
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [refresh])

  /**
   * Search contacts
   */
  const search = useCallback(async (query: string): Promise<Contact[]> => {
    try {
      const results = await contactsApi.searchContacts(query)
      return sortContacts(results)
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [sortContacts])

  /**
   * Toggle favorite status for a contact
   */
  const toggleFavorite = useCallback(async (id: number): Promise<void> => {
    try {
      const contact = contacts.find(c => c.id === id)
      if (!contact) {
        throw new Error(`Contact with ID ${id} not found`)
      }

      await contactsApi.setFavoriteStatus(id, !contact.isFavorite)

      if (isMountedRef.current) {
        setContacts((prev) =>
          sortContacts(prev.map((c) =>
            c.id === id ? { ...c, isFavorite: !c.isFavorite } : c
          ))
        )
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [contacts, sortContacts])

  /**
   * Set view mode
   */
  const setViewMode = useCallback((view: ContactView) => {
    setViewModeState(view)
  }, [])

  /**
   * Set sort field
   */
  const setSortBy = useCallback((newSortBy: ContactSortBy) => {
    setSortByState(newSortBy)
  }, [])

  /**
   * Set sort order
   */
  const setSortOrder = useCallback((order: ContactSortOrder) => {
    setSortOrderState(order)
  }, [])

  /**
   * Set filters
   */
  const setFilters = useCallback((newFilters: ContactFilters) => {
    setFiltersState(newFilters)
  }, [])

  /**
   * Clear filters
   */
  const clearFilters = useCallback(() => {
    setFiltersState({})
  }, [])

  /**
   * Toggle contact selection
   */
  const toggleSelection = useCallback((id: number) => {
    setSelectedIds((prev) => {
      const next = new Set(prev)
      if (next.has(id)) {
        next.delete(id)
      } else {
        next.add(id)
      }
      return next
    })
  }, [])

  /**
   * Select all visible contacts
   */
  const selectAll = useCallback(() => {
    setSelectedIds(new Set(contacts.map(c => c.id)))
  }, [contacts])

  /**
   * Clear selection
   */
  const clearSelection = useCallback(() => {
    setSelectedIds(new Set())
  }, [])

  /**
   * Fetch data on mount and when filters/sort change
   */
  useEffect(() => {
    fetchData()
  }, [fetchData])

  /**
   * Setup polling for periodic refresh
   */
  useEffect(() => {
    if (!enabled || refreshInterval <= 0) {
      return
    }

    pollingIntervalRef.current = setInterval(() => {
      fetchData()
    }, refreshInterval)

    return () => {
      if (pollingIntervalRef.current) {
        clearInterval(pollingIntervalRef.current)
        pollingIntervalRef.current = null
      }
    }
  }, [enabled, refreshInterval, fetchData])

  /**
   * Cleanup on unmount
   */
  useEffect(() => {
    isMountedRef.current = true

    return () => {
      isMountedRef.current = false
    }
  }, [])

  return {
    contacts,
    groups,
    isLoading,
    error,
    stats,
    viewMode,
    sortBy,
    sortOrder,
    filters,
    selectedIds,
    refresh,
    setViewMode,
    setSortBy,
    setSortOrder,
    setFilters,
    clearFilters,
    toggleSelection,
    selectAll,
    clearSelection,
    createContact,
    updateContact,
    deleteContact,
    deleteContacts,
    createGroup,
    updateGroup,
    deleteGroup,
    addToGroup,
    removeFromGroup,
    search,
    toggleFavorite,
  }
}

/**
 * Hook for getting favorite contacts
 * Convenience wrapper around useContacts
 *
 * @param options - Additional hook options
 * @returns Contact data filtered to favorites only
 */
export function useFavoriteContacts(
  options?: Omit<UseContactsOptions, 'initialFilters'>
): UseContactsResult {
  return useContacts({
    ...options,
    initialFilters: { isFavorite: true },
  })
}

/**
 * Hook for getting online contacts
 * Convenience wrapper around useContacts
 *
 * @param options - Additional hook options
 * @returns Contact data filtered to online contacts only
 */
export function useOnlineContacts(
  options?: Omit<UseContactsOptions, 'initialFilters'>
): UseContactsResult {
  return useContacts({
    ...options,
    initialFilters: { isOnline: true },
  })
}

/**
 * Hook for getting contacts by department
 * Convenience wrapper around useContacts
 *
 * @param department - Department name to filter by
 * @param options - Additional hook options
 * @returns Contact data filtered to department
 */
export function useContactsByDepartment(
  department: string,
  options?: Omit<UseContactsOptions, 'initialFilters'>
): UseContactsResult {
  return useContacts({
    ...options,
    initialFilters: { department },
  })
}

/**
 * Hook for getting contacts in a specific group
 * Convenience wrapper around useContacts
 *
 * @param groupId - Group ID to filter by
 * @param options - Additional hook options
 * @returns Contact data filtered to group
 */
export function useContactsByGroup(
  groupId: number,
  options?: Omit<UseContactsOptions, 'initialFilters'>
): UseContactsResult {
  return useContacts({
    ...options,
    initialFilters: { groupId },
  })
}
