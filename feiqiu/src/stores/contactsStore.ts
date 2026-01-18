import { create } from 'zustand'
import type {
  Contact,
  ContactGroup,
  ContactStats,
  ContactView,
  ContactSortBy,
  ContactSortOrder,
  ContactFilters,
} from '@/lib/types/contacts'

export interface ContactsState {
  contacts: Contact[]
  groups: ContactGroup[]
  isLoading: boolean
  error: Error | null
  stats: ContactStats | null
  viewMode: ContactView
  sortBy: ContactSortBy
  sortOrder: ContactSortOrder
  filters: ContactFilters
  selectedIds: Set<number>

  setContacts: (contacts: Contact[]) => void
  setGroups: (groups: ContactGroup[]) => void
  setContactsLoading: (isLoading: boolean) => void
  setContactsError: (error: Error | null) => void
  setContactsStats: (stats: ContactStats) => void
  setViewMode: (view: ContactView) => void
  setSortBy: (sortBy: ContactSortBy) => void
  setSortOrder: (order: ContactSortOrder) => void
  setFilters: (filters: ContactFilters) => void
  clearFilters: () => void
  toggleSelection: (id: number) => void
  selectAll: () => void
  clearSelection: () => void
  addContact: (contact: Contact) => void
  updateContact: (id: number, updates: Partial<Contact>) => void
  removeContact: (id: number) => void
  addGroup: (group: ContactGroup) => void
  updateGroup: (id: number, updates: Partial<ContactGroup>) => void
  removeGroup: (id: number) => void
  resetContacts: () => void
}

export const useContactsStore = create<ContactsState>((set, get) => ({
  contacts: [],
  groups: [],
  isLoading: false,
  error: null,
  stats: null,
  viewMode: 'list',
  sortBy: 'name',
  sortOrder: 'asc',
  filters: {},
  selectedIds: new Set(),

  setContacts: (contacts) => set({ contacts }),

  setGroups: (groups) => set({ groups }),

  setContactsLoading: (isLoading) => set({ isLoading }),

  setContactsError: (error) => set({ error }),

  setContactsStats: (stats) => set({ stats }),

  setViewMode: (viewMode) => set({ viewMode }),

  setSortBy: (sortBy) => set({ sortBy }),

  setSortOrder: (sortOrder) => set({ sortOrder }),

  setFilters: (filters) => set({ filters }),

  clearFilters: () => set({ filters: {} }),

  toggleSelection: (id) =>
    set((state) => {
      const newSelected = new Set(state.selectedIds)
      if (newSelected.has(id)) {
        newSelected.delete(id)
      } else {
        newSelected.add(id)
      }
      return { selectedIds: newSelected }
    }),

  selectAll: () =>
    set((state) => ({
      selectedIds: new Set(state.contacts.map((c) => c.id)),
    })),

  clearSelection: () => set({ selectedIds: new Set() }),

  addContact: (contact) =>
    set((state) => {
      if (state.contacts.some((c) => c.id === contact.id)) {
        return state
      }
      return { contacts: [...state.contacts, contact] }
    }),

  updateContact: (id, updates) =>
    set((state) => ({
      contacts: state.contacts.map((contact) =>
        contact.id === id ? { ...contact, ...updates } : contact
      ),
    })),

  removeContact: (id) =>
    set((state) => ({
      contacts: state.contacts.filter((contact) => contact.id !== id),
    })),

  addGroup: (group) =>
    set((state) => {
      if (state.groups.some((g) => g.id === group.id)) {
        return state
      }
      return { groups: [...state.groups, group] }
    }),

  updateGroup: (id, updates) =>
    set((state) => ({
      groups: state.groups.map((group) =>
        group.id === id ? { ...group, ...updates } : group
      ),
    })),

  removeGroup: (id) =>
    set((state) => ({
      groups: state.groups.filter((group) => group.id !== id),
    })),

  resetContacts: () =>
    set({
      contacts: [],
      groups: [],
      isLoading: false,
      error: null,
      stats: null,
      selectedIds: new Set(),
    }),
}))
