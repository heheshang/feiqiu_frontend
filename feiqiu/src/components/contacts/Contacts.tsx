/**
 * Contacts Component
 *
 * Main contacts page component with tabs for contacts list and group management.
 * Includes all CRUD operations for contacts.
 */

import { useState } from 'react'
import { ContactList } from './ContactList'
import { GroupManagement } from './GroupManagement'
import { CreateContactDialog, EditContactDialog, ContactDetailDialog } from './ContactDialogs'
import { useContacts } from '@/hooks/useContacts'
import type { Contact, ContactGroup, CreateContactInput, UpdateContactInput } from '@/lib/types/contacts'
import { cn } from '@/lib/utils'

type Tab = 'contacts' | 'groups'

export function Contacts() {
  const {
    contacts,
    groups,
    isLoading,
    error,
    stats,
    refresh,
    toggleFavorite,
    deleteContact,
    createContact,
    updateContact,
    createGroup,
    updateGroup,
    deleteGroup,
    addToGroup,
    removeFromGroup,
  } = useContacts({
    enabled: true,
    refreshInterval: 0,
  })

  const [activeTab, setActiveTab] = useState<Tab>('contacts')

  // Dialog states
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [showEditDialog, setShowEditDialog] = useState(false)
  const [showDetailDialog, setShowDetailDialog] = useState(false)
  const [selectedContact, setSelectedContact] = useState<Contact | null>(null)

  const handleContactClick = (contact: Contact) => {
    setSelectedContact(contact)
    setShowDetailDialog(true)
  }

  const handleToggleFavorite = async (contact: Contact | number) => {
    try {
      const id = typeof contact === 'number' ? contact : contact.id
      await toggleFavorite(id)
    } catch (err) {
      console.error('Failed to toggle favorite:', err)
    }
  }

  const handleDeleteContact = async (contactId: number) => {
    try {
      await deleteContact(contactId)
    } catch (err) {
      console.error('Failed to delete contact:', err)
    }
  }

  // Create contact handler
  const handleCreateContact = async (data: CreateContactInput) => {
    await createContact(data)
  }

  // Edit contact handlers
  const handleEditContact = (contact: Contact) => {
    setSelectedContact(contact)
    setShowEditDialog(true)
  }

  const handleUpdateContact = async (id: number, data: UpdateContactInput) => {
    await updateContact(id, data)
    setShowEditDialog(false)
    setSelectedContact(null)
  }

  // Message handler
  const handleSendMessage = (contact: Contact) => {
    // TODO: Implement navigate to chat with this contact
    console.log('Send message to:', contact)
    setShowDetailDialog(false)
  }

  // Group management handlers
  const handleCreateGroup = async (data: { name: string; color?: string; icon?: string }) => {
    await createGroup(data)
  }

  const handleUpdateGroup = async (id: number, updates: { name?: string; color?: string; icon?: string }) => {
    await updateGroup(id, updates)
  }

  const handleDeleteGroup = async (id: number) => {
    await deleteGroup(id)
  }

  const handleAddContacts = async (groupId: number, contactIds: number[]) => {
    await addToGroup(groupId, contactIds)
  }

  const handleRemoveContacts = async (groupId: number, contactIds: number[]) => {
    await removeFromGroup(groupId, contactIds)
  }

  // Batch operation handlers
  const handleBatchDelete = async (contactIds: number[]) => {
    for (const id of contactIds) {
      await deleteContact(id)
    }
  }

  const handleBatchMoveToGroup = async (contactIds: number[], groupId: number) => {
    await addToGroup(groupId, contactIds)
  }

  const handleBatchExport = (contactIds: number[]) => {
    const selectedContacts = contacts.filter(c => contactIds.includes(c.id))
    const csvContent = [
      ['姓名', '昵称', '电话', '邮箱', '部门', '职位', '分组'].join(','),
      ...selectedContacts.map(c => [
        c.name,
        c.nickname || '',
        c.phone || '',
        c.email || '',
        c.department || '',
        c.position || '',
        c.groups.join('; ')
      ].join(','))
    ].join('\n')

    const blob = new Blob(['\ufeff' + csvContent], { type: 'text/csv;charset=utf-8;' })
    const link = document.createElement('a')
    const url = URL.createObjectURL(blob)
    link.setAttribute('href', url)
    link.setAttribute('download', `contacts_${new Date().toISOString().slice(0, 10)}.csv`)
    link.style.visibility = 'hidden'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="flex items-center justify-between px-6 py-4 bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-slate-100">通讯录</h1>
          {stats && (
            <p className="text-sm text-slate-500 dark:text-slate-400 mt-1">
              共 {stats.total} 位联系人，{stats.online} 位在线，{groups.length} 个分组
            </p>
          )}
        </div>

        <div className="flex items-center gap-2">
          {/* Tab switcher */}
          <div className="flex items-center bg-slate-100 dark:bg-slate-800 rounded-lg p-1">
            <button
              onClick={() => setActiveTab('contacts')}
              className={cn(
                'px-4 py-2 text-sm font-medium rounded-md transition-colors',
                activeTab === 'contacts'
                  ? 'bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 shadow-sm'
                  : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'
              )}
            >
              联系人
            </button>
            <button
              onClick={() => setActiveTab('groups')}
              className={cn(
                'px-4 py-2 text-sm font-medium rounded-md transition-colors',
                activeTab === 'groups'
                  ? 'bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 shadow-sm'
                  : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'
              )}
            >
              分组
            </button>
          </div>

          {/* Refresh button */}
          <button
            onClick={refresh}
            className="p-2 text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors"
            title="刷新"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
          </button>

          {/* Add contact button (only show on contacts tab) */}
          {activeTab === 'contacts' && (
            <button
              onClick={() => setShowCreateDialog(true)}
              className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-emerald-600 dark:bg-emerald-500 rounded-lg hover:bg-emerald-700 dark:hover:bg-emerald-600 transition-colors"
            >
              <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
              </svg>
              新建联系人
            </button>
          )}
        </div>
      </div>

      {/* Content */}
      {activeTab === 'contacts' ? (
        <ContactList
          contacts={contacts}
          groups={groups}
          isLoading={isLoading}
          error={error}
          onContactClick={handleContactClick}
          onToggleFavorite={handleToggleFavorite}
          onDeleteContact={handleDeleteContact}
          onCreateContact={() => setShowCreateDialog(true)}
          onBatchDelete={handleBatchDelete}
          onBatchMoveToGroup={handleBatchMoveToGroup}
          onBatchExport={handleBatchExport}
          className="flex-1"
        />
      ) : (
        <GroupManagement
          groups={groups}
          contacts={contacts}
          onCreateGroup={handleCreateGroup}
          onUpdateGroup={handleUpdateGroup}
          onDeleteGroup={handleDeleteGroup}
          onAddContacts={handleAddContacts}
          onRemoveContacts={handleRemoveContacts}
          isLoading={isLoading}
          className="flex-1"
        />
      )}

      {/* Dialogs */}
      <CreateContactDialog
        isOpen={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
        onSubmit={handleCreateContact}
        groups={groups}
        isLoading={isLoading}
      />

      <EditContactDialog
        isOpen={showEditDialog}
        onClose={() => {
          setShowEditDialog(false)
          setSelectedContact(null)
        }}
        contact={selectedContact}
        onSubmit={handleUpdateContact}
        groups={groups}
        isLoading={isLoading}
      />

      <ContactDetailDialog
        isOpen={showDetailDialog}
        onClose={() => {
          setShowDetailDialog(false)
          setSelectedContact(null)
        }}
        contact={selectedContact}
        onEdit={handleEditContact}
        onMessage={handleSendMessage}
        onToggleFavorite={handleToggleFavorite}
        isLoading={isLoading}
      />
    </div>
  )
}
