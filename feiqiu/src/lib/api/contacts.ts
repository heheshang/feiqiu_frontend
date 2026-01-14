/**
 * Contacts API
 *
 * Provides type-safe wrappers for all contacts-related IPC commands.
 */

import { invokeCommand } from './base'
import type {
  Contact,
  ContactGroup,
  ContactFilters,
  ContactStats,
  CreateContactInput,
  UpdateContactInput,
  CreateContactGroupInput,
  UpdateContactGroupInput,
  BatchOperationResult,
} from '../types/contacts'

// Backend DTO types (must match Rust backend exactly)
interface ContactDto {
  id: number
  peerId?: number
  name: string
  nickname?: string
  avatar?: string
  phone?: string
  email?: string
  department?: string
  position?: string
  notes?: string
  isFavorite: boolean
  pinyin?: string
  isOnline: boolean
  lastSeen?: number
  createdAt: number
  updatedAt?: number
  groups: string[]
}

interface ContactGroupDto {
  id: number
  name: string
  color?: string
  icon?: string
  sortOrder: number
  createdAt: number
  memberCount: number
}

interface ContactStatsDto {
  total: number
  online: number
  offline: number
  favorites: number
  byDepartment: Record<string, number>
}

/**
 * Converts backend DTO to frontend Contact type
 */
function toFrontendContact(dto: ContactDto): Contact {
  return {
    id: dto.id,
    peerId: dto.peerId,
    name: dto.name,
    nickname: dto.nickname,
    avatar: dto.avatar,
    phone: dto.phone,
    email: dto.email,
    department: dto.department,
    position: dto.position,
    notes: dto.notes,
    isFavorite: dto.isFavorite,
    pinyin: dto.pinyin,
    isOnline: dto.isOnline,
    lastSeen: dto.lastSeen,
    createdAt: dto.createdAt,
    updatedAt: dto.updatedAt,
    groups: dto.groups,
  }
}

/**
 * Converts backend DTO to frontend ContactGroup type
 */
function toFrontendContactGroup(dto: ContactGroupDto): ContactGroup {
  return {
    id: dto.id,
    name: dto.name,
    color: dto.color,
    icon: dto.icon,
    sortOrder: dto.sortOrder,
    createdAt: dto.createdAt,
    memberCount: dto.memberCount,
  }
}

/**
 * Converts backend DTO to frontend ContactStats type
 */
function toFrontendContactStats(dto: ContactStatsDto): ContactStats {
  return {
    total: dto.total,
    online: dto.online,
    offline: dto.offline,
    favorites: dto.favorites,
    byDepartment: dto.byDepartment,
  }
}

// ==================== Contact CRUD ====================

/**
 * Gets all contacts with optional filters
 *
 * @param filters - Optional filters to apply
 * @returns Array of contacts
 */
export async function getContacts(filters?: ContactFilters): Promise<Contact[]> {
  const result = await invokeCommand<ContactDto[]>('get_contacts', { filters })
  return result.map(toFrontendContact)
}

/**
 * Gets a single contact by ID
 *
 * @param id - Contact ID
 * @returns Contact or null if not found
 */
export async function getContact(id: number): Promise<Contact | null> {
  const result = await invokeCommand<ContactDto | null>('get_contact', { id })
  return result ? toFrontendContact(result) : null
}

/**
 * Creates a new contact
 *
 * @param contact - Contact data to create
 * @returns Created contact with generated ID
 */
export async function createContact(contact: CreateContactInput): Promise<Contact> {
  const result = await invokeCommand<ContactDto>('create_contact', { contact: contact as unknown as Record<string, unknown> })
  return toFrontendContact(result)
}

/**
 * Updates an existing contact
 *
 * @param id - Contact ID to update
 * @param contact - Partial contact data to update
 * @returns Updated contact
 */
export async function updateContact(
  id: number,
  contact: UpdateContactInput
): Promise<Contact> {
  const result = await invokeCommand<ContactDto>('update_contact', { id, contact })
  return toFrontendContact(result)
}

/**
 * Deletes a contact
 *
 * @param id - Contact ID to delete
 */
export async function deleteContact(id: number): Promise<void> {
  await invokeCommand<void>('delete_contact', { id })
}

/**
 * Batch deletes multiple contacts
 *
 * @param ids - Array of contact IDs to delete
 * @returns Batch operation result
 */
export async function deleteContacts(ids: number[]): Promise<BatchOperationResult> {
  const result: BatchOperationResult = {
    success: 0,
    failed: 0,
    errors: [],
  }

  for (const id of ids) {
    try {
      await deleteContact(id)
      result.success++
    } catch (error) {
      result.failed++
      result.errors.push({
        id,
        error: error instanceof Error ? error.message : String(error),
      })
    }
  }

  return result
}

// ==================== Contact Groups ====================

/**
 * Gets all contact groups
 *
 * @returns Array of contact groups
 */
export async function getContactGroups(): Promise<ContactGroup[]> {
  const result = await invokeCommand<ContactGroupDto[]>('get_contact_groups', {})
  return result.map(toFrontendContactGroup)
}

/**
 * Creates a new contact group
 *
 * @param group - Group data to create
 * @returns Created group with generated ID
 */
export async function createContactGroup(group: CreateContactGroupInput): Promise<ContactGroup> {
  const result = await invokeCommand<ContactGroupDto>('create_contact_group', { group: group as unknown as Record<string, unknown> })
  return toFrontendContactGroup(result)
}

/**
 * Updates an existing contact group
 *
 * @param id - Group ID to update
 * @param group - Partial group data to update
 * @returns Updated group
 */
export async function updateContactGroup(
  id: number,
  group: UpdateContactGroupInput
): Promise<ContactGroup> {
  const result = await invokeCommand<ContactGroupDto>('update_contact_group', { id, group })
  return toFrontendContactGroup(result)
}

/**
 * Deletes a contact group
 *
 * @param id - Group ID to delete
 */
export async function deleteContactGroup(id: number): Promise<void> {
  await invokeCommand<void>('delete_contact_group', { id })
}

// ==================== Group Membership ====================

/**
 * Adds contacts to a group
 *
 * @param groupId - Group ID
 * @param contactIds - Array of contact IDs to add
 */
export async function addContactsToGroup(
  groupId: number,
  contactIds: number[]
): Promise<void> {
  await invokeCommand<void>('add_contacts_to_group', {
    groupId,
    contactIds,
  })
}

/**
 * Removes contacts from a group
 *
 * @param groupId - Group ID
 * @param contactIds - Array of contact IDs to remove
 */
export async function removeContactsFromGroup(
  groupId: number,
  contactIds: number[]
): Promise<void> {
  await invokeCommand<void>('remove_contacts_from_group', {
    groupId,
    contactIds,
  })
}

/**
 * Sets the groups for a contact (replaces existing memberships)
 *
 * @param contactId - Contact ID
 * @param groupIds - Array of group IDs
 */
export async function setContactGroups(
  contactId: number,
  groupIds: number[]
): Promise<void> {
  // Get current groups for this contact
  const contact = await getContact(contactId)
  if (!contact) {
    throw new Error(`Contact with ID ${contactId} not found`)
  }

  // Get all groups to find current memberships
  const allGroups = await getContactGroups()
  const currentGroupIds: number[] = []

  // For each group, check if contact is a member
  for (const group of allGroups) {
    const groupContacts = await getContactsByGroup(group.id)
    if (groupContacts.some(c => c.id === contactId)) {
      currentGroupIds.push(group.id)
    }
  }

  // Calculate differences
  const toAdd = groupIds.filter(id => !currentGroupIds.includes(id))
  const toRemove = currentGroupIds.filter(id => !groupIds.includes(id))

  // Apply changes
  if (toAdd.length > 0) {
    await addContactsToGroup(contactId, toAdd)
  }
  if (toRemove.length > 0) {
    await removeContactsFromGroup(contactId, toRemove)
  }
}

// ==================== Search & Filter ====================

/**
 * Searches contacts by query string
 *
 * @param query - Search query string
 * @returns Array of matching contacts
 */
export async function searchContacts(query: string): Promise<Contact[]> {
  const result = await invokeCommand<ContactDto[]>('search_contacts', { query })
  return result.map(toFrontendContact)
}

/**
 * Gets contacts filtered by online status
 *
 * @param isOnline - Online status to filter by
 * @returns Array of contacts matching the online status
 */
export async function getContactsByOnlineStatus(isOnline: boolean): Promise<Contact[]> {
  return getContacts({ isOnline })
}

/**
 * Gets contacts filtered by favorite status
 *
 * @param isFavorite - Favorite status to filter by
 * @returns Array of favorite contacts
 */
export async function getFavoriteContacts(): Promise<Contact[]> {
  return getContacts({ isFavorite: true })
}

/**
 * Gets contacts in a specific department
 *
 * @param department - Department name
 * @returns Array of contacts in the department
 */
export async function getContactsByDepartment(department: string): Promise<Contact[]> {
  return getContacts({ department })
}

/**
 * Gets contacts in a specific group
 *
 * @param groupId - Group ID
 * @returns Array of contacts in the group
 */
export async function getContactsByGroup(groupId: number): Promise<Contact[]> {
  return getContacts({ groupId })
}

// ==================== Statistics ====================

/**
 * Gets contact statistics
 *
 * @returns Contact statistics
 */
export async function getContactStats(): Promise<ContactStats> {
  const result = await invokeCommand<ContactStatsDto>('get_contact_stats', {})
  return toFrontendContactStats(result)
}

/**
 * Gets all unique department names from contacts
 *
 * @returns Array of unique department names
 */
export async function getDepartments(): Promise<string[]> {
  const stats = await getContactStats()
  return Object.keys(stats.byDepartment).sort()
}

// ==================== Favorites ====================

/**
 * Toggles favorite status for a contact
 *
 * @param id - Contact ID
 * @param isFavorite - New favorite status
 */
export async function setFavoriteStatus(id: number, isFavorite: boolean): Promise<void> {
  await updateContact(id, { isFavorite })
}

/**
 * Adds a contact to favorites
 *
 * @param id - Contact ID
 */
export async function addToFavorites(id: number): Promise<void> {
  await setFavoriteStatus(id, true)
}

/**
 * Removes a contact from favorites
 *
 * @param id - Contact ID
 */
export async function removeFromFavorites(id: number): Promise<void> {
  await setFavoriteStatus(id, false)
}

// ==================== Convenience Functions ====================

/**
 * Gets recently active contacts (online or recently seen)
 *
 * @param thresholdMs - Activity threshold in milliseconds (default: 1 hour)
 * @returns Array of recently active contacts
 */
export async function getRecentContacts(thresholdMs: number = 3600000): Promise<Contact[]> {
  const contacts = await getContacts()
  const now = Date.now()

  return contacts.filter(
    c =>
      c.isOnline ||
      (c.lastSeen && now - c.lastSeen < thresholdMs)
  )
}

/**
 * Gets display name for a contact (nickname or name)
 *
 * @param contact - Contact object
 * @returns Display name
 */
export function getContactDisplayName(contact: Contact): string {
  return contact.nickname || contact.name
}

/**
 * Gets initials from a contact name for avatar display
 *
 * @param contact - Contact object
 * @returns Initials (up to 2 characters)
 */
export function getContactInitials(contact: Contact): string {
  const name = getContactDisplayName(contact)
  const parts = name.trim().split(/\s+/)

  if (parts.length >= 2) {
    return (parts[0][0] + parts[1][0]).toUpperCase()
  }
  return name.slice(0, 2).toUpperCase()
}

// ==================== API Object ====================

/**
 * Contacts API object
 * Provides all contacts-related API methods in a single object
 */
export const contactsApi = {
  // Contact CRUD
  getContacts,
  getContact,
  createContact,
  updateContact,
  deleteContact,
  deleteContacts,

  // Contact Groups
  getContactGroups,
  createContactGroup,
  updateContactGroup,
  deleteContactGroup,

  // Group Membership
  addContactsToGroup,
  removeContactsFromGroup,
  setContactGroups,

  // Search & Filter
  searchContacts,
  getContactsByOnlineStatus,
  getFavoriteContacts,
  getContactsByDepartment,
  getContactsByGroup,

  // Statistics
  getContactStats,
  getDepartments,

  // Favorites
  setFavoriteStatus,
  addToFavorites,
  removeFromFavorites,

  // Convenience
  getRecentContacts,
  getContactDisplayName,
  getContactInitials,
}
