import { useState, useEffect, useMemo, useCallback } from 'react'
import { AppShell } from './components/shell'
import { BasicSettings } from './components/basic-settings'
import { Messaging } from './components/messaging'
import { FileTransfer } from './components/file-transfer'
import { CollaborationTools } from './components/collaboration'
import { OrganizationChart } from './components/organization'
import { Contacts } from './components/contacts'
import { NavTab, UserStatus } from './lib/types/shell'
import { useDarkMode } from './hooks/useDarkMode'
import { User as SettingsUser, NetworkConfig, NetworkStatus } from './lib/types/basic-settings'
import { Conversation, User as MessagingUser } from './lib/types/messaging'
import { FileTransfer as FileTransferType, User as FTUser } from './lib/types/file-transfer'
import { Screenshot, User as ColUser } from './lib/types/collaboration'
import { Department, User as OrgUser } from './lib/types/organization'
import { contactsApi } from './lib/api'
import { onEvent } from './lib/events'
import type { MessageReceivedEvent } from './lib/events'
import type { CreateContactInput } from './lib/types/contacts'

// New hooks for real data
import { usePeers } from './hooks/usePeers'
import { useMessages } from './hooks/useMessages'
import { useConfig } from './hooks/useConfig'
import { useFileTransfers } from './hooks/useFileTransfers'
import { useContacts } from './hooks/useContacts'
import { toMessagingUser, toSettingsUser, type Peer, type Message, type Config } from './lib/converters'

interface AppUser extends SettingsUser {
  avatar: string
}

// Convert DTO Message to Messaging Message
function toMessagingMessage(msg: Message): import('./lib/types/messaging').Message {
  return {
    id: msg.id,
    type: msg.type,
    content: msg.content,
    timestamp: msg.timestamp,
    senderId: msg.senderId,
    senderName: msg.senderName,
    status: msg.status,
    reactions: [],
  }
}

// Convert peers to conversations (only show peers with message history)
function peersToConversations(peers: Peer[], myIp: string, messages: Message[]): Conversation[] {
  // Group messages by peer IP
  const messagesByPeer = new Map<string, Message[]>()
  messages.forEach(msg => {
    const peerIp = msg.senderIp === myIp ? msg.receiverIp : msg.senderIp
    if (!messagesByPeer.has(peerIp)) {
      messagesByPeer.set(peerIp, [])
    }
    messagesByPeer.get(peerIp)!.push(msg)
  })

  // Create conversations from peers (only those with messages)
  return peers
    .filter(peer => messagesByPeer.has(peer.ip)) // Only include peers with message history
    .map(peer => {
      const peerMessages = messagesByPeer.get(peer.ip) || []
      const lastMessage = peerMessages[peerMessages.length - 1]

      return {
        id: peer.ip,
        type: 'single' as const,
        pinned: false,
        unreadCount: peerMessages.filter(m => m.status === 'unread').length,
        lastMessage: lastMessage ? {
          id: lastMessage.id,
          content: lastMessage.content,
          type: lastMessage.type,
          timestamp: lastMessage.timestamp,
          senderId: lastMessage.senderId,
          senderName: lastMessage.senderName,
        } : undefined,
        participant: toMessagingUser(peer),
        messages: peerMessages.map(toMessagingMessage),
      }
    })
}

function App() {
  const { theme, toggleTheme } = useDarkMode()
  const [activeTab, setActiveTab] = useState<NavTab>('chat')
  const [showSettings, setShowSettings] = useState(false)
  const [settingsTab, setSettingsTab] = useState<'profile' | 'network'>('profile')

  // Real data hooks
  const { peers, isLoading: peersLoading } = usePeers({ enabled: true })
  const { messages, sendMessage: sendBackendMessage } = useMessages({ enabled: true })
  const { config, updateConfig } = useConfig({ enabled: true })
  const { transfers, acceptTransfer, rejectTransfer, cancelTransfer } = useFileTransfers({ enabled: true })
  const { contacts } = useContacts({ enabled: true, refreshInterval: 0 })

  // Local state
  const [user, setUser] = useState<AppUser>({
    id: 'local',
    name: config?.username || '用户',
    avatar: config?.avatar || '',
    avatarUrl: config?.avatar || '',
    signature: '',
    status: (config?.status as UserStatus) || 'online',
    department: '',
  })

  const [activeConversationId, setActiveConversationId] = useState<string | null>(null)
  const [manuallyAddedConversations, setManuallyAddedConversations] = useState<Set<string>>(new Set())

  // Update user when config changes
  useEffect(() => {
    if (config) {
      setUser(prev => ({
        ...prev,
        name: config.username,
        avatar: config.avatar || '',
        avatarUrl: config.avatar || '',
        status: config.status as UserStatus,
      }))
    }
  }, [config])

  // Auto-add peer to contacts when receiving message from new peer
  useEffect(() => {
    const messageReceivedUnsub = onEvent<MessageReceivedEvent>('message_received', async (event) => {
      const senderIp = event.message.sender_ip
      const senderName = event.message.sender_name

      // Check if sender is already in contacts by name or ip
      const existingContact = contacts.find(c =>
        c.name === senderName || c.name.includes(senderIp)
      )

      if (!existingContact) {
        // Find the peer to get additional info
        const peer = peers.find(p => p.ip === senderIp)
        if (peer) {
          try {
            // Create new contact from peer (note: peer.id is string IP address, don't use as peerId)
            const newContact: CreateContactInput = {
              name: senderName,
              nickname: peer.nickname || undefined,
              avatar: peer.avatar || undefined,
              // peerId not set because it requires database ID, not IP string
            }

            await contactsApi.createContact(newContact)
            console.log(`[Auto-add] Added contact: ${senderName} (${senderIp})`)
          } catch (error) {
            console.error(`[Auto-add] Failed to add contact: ${senderName}`, error)
          }
        }
      }
    })

    return () => {
      messageReceivedUnsub.remove()
    }
  }, [contacts, peers])

  // Auto-select first conversation on mount
  useEffect(() => {
    if (peers.length > 0 && !activeConversationId) {
      setActiveConversationId(peers[0].ip)
    }
  }, [peers, activeConversationId])

  // Derive conversations from peers and messages
  const conversations = useMemo(() => {
    const myIp = config?.bindIp || '0.0.0.0'
    const peerConversations = peersToConversations(peers, myIp, messages)

    // Add manually added conversations from contacts
    const manuallyAddedConversationsList: Conversation[] = []
    for (const conversationId of manuallyAddedConversations) {
      const peer = peers.find(p => p.ip === conversationId)
      if (peer) {
        manuallyAddedConversationsList.push({
          id: peer.ip,
          type: 'single',
          pinned: false,
          unreadCount: 0,
          lastMessage: undefined,
          participant: toMessagingUser(peer),
          messages: [],
        })
      }
    }

    // Merge conversations (avoid duplicates)
    const existingIds = new Set(peerConversations.map(c => c.id))
    const uniqueManualConversations = manuallyAddedConversationsList.filter(conv => !existingIds.has(conv.id))

    return [...peerConversations, ...uniqueManualConversations]
  }, [peers, messages, config?.bindIp, manuallyAddedConversations])

  // Current user for messaging
  const currentUser: MessagingUser = useMemo(() => ({
    id: 'local',
    name: user.name,
    avatar: user.avatarUrl || `https://api.dicebear.com/7.x/avataaars/svg?seed=${user.name}`,
    status: user.status,
  }), [user.name, user.avatarUrl, user.status])

  // Network config from real config
  const networkConfig: NetworkConfig = useMemo(() => ({
    id: 'net-config-001',
    udpPort: config?.udpPort || 2425,
    bindAddress: config?.bindIp || '0.0.0.0',
    broadcastAddress: '255.255.255.255',
    maxRetries: 3,
    timeout: 5000,
  }), [config])

  // Network status (derived)
  const networkStatus: NetworkStatus = useMemo(() => ({
    ipAddress: config?.bindIp || '0.0.0.0',
    macAddress: '',
    connectionStatus: peers.some(p => p.status === 'online') ? 'connected' : 'disconnected',
    listeningPort: config?.udpPort || 2425,
    lastSeen: new Date().toISOString(),
    onlineUsers: peers.filter(p => p.status === 'online').length,
  }), [config, peers])

  // File transfers converted to frontend types
  const fileTransfers: FileTransferType[] = transfers

  const handleSettings = () => {
    setShowSettings(true)
  }

  const handleNetworkConfig = () => {
    setShowSettings(true)
    setSettingsTab('network')
  }

  const handleLogout = () => {
    console.log('Logging out...')
  }

  const handleUpdateUser = async (updatedUser: Partial<AppUser>) => {
    setUser((prev) => ({ ...prev, ...updatedUser }))
    if (updatedUser.status) {
      await updateConfig({ status: updatedUser.status })
    }
    if (updatedUser.name) {
      await updateConfig({ username: updatedUser.name })
    }
  }

  const handleStatusChange = async (status: UserStatus) => {
    setUser((prev) => ({ ...prev, status }))
    await updateConfig({ status })
  }

  const handleSendMessage = async (conversationId: string, content: string) => {
    await sendBackendMessage(content, conversationId)
  }

  const handleStartConversation = (contactId: string) => {
    // Add to manually added conversations
    setManuallyAddedConversations(prev => new Set([...prev, contactId]))
    // Switch to chat tab
    setActiveTab('chat')
    // Select the conversation
    setActiveConversationId(contactId)
  }

  const handleSendImage = (conversationId: string, file: File) => {
    console.log('Send image:', { conversationId, fileName: file.name })
    // TODO: Implement file/image sending
  }

  const handleFileTransferPause = (id: string) => {
    cancelTransfer(id)
  }

  const handleFileTransferResume = (id: string) => {
    // TODO: Implement resume (may need backend support)
    console.log('Resume transfer:', id)
  }

  const handleFileTransferCancel = (id: string) => {
    cancelTransfer(id)
  }

  const handleFileTransferRetry = (id: string) => {
    // TODO: Implement retry (may need backend support)
    console.log('Retry transfer:', id)
  }

  const handleSendFile = (files: File[]) => {
    console.log('Send files:', files.map(f => f.name))
    // TODO: Implement file sending
  }

  const handleAcceptTransfer = (id: string) => {
    acceptTransfer(id)
  }

  const handleRejectTransfer = (id: string) => {
    rejectTransfer(id)
  }

  const handleScreenshot = (type: string) => {
    console.log('Screenshot type:', type)
  }

  const renderContent = () => {
    if (showSettings) {
      return (
        <BasicSettings
          user={user}
          networkConfig={networkConfig}
          networkStatus={networkStatus}
          activeTab={settingsTab}
          onTabChange={setSettingsTab}
          onUpdateUser={handleUpdateUser}
          onStatusChange={handleStatusChange}
          onSaveNetworkConfig={async (config) => {
            await updateConfig({
              bindIp: config.bindAddress,
              udpPort: config.udpPort,
            })
          }}
          onCancelNetworkConfig={() => setShowSettings(false)}
        />
      )
    }

    switch (activeTab) {
      case 'chat':
        return (
          <Messaging
            conversations={conversations}
            currentUser={currentUser}
            activeConversationId={activeConversationId}
            onConversationSelect={setActiveConversationId}
            onSendMessage={handleSendMessage}
            onSendImage={handleSendImage}
            onMessageReply={(id) => console.log('Reply to:', id)}
            onMessageReact={(id, emoji) => console.log('React:', id, emoji)}
            onMessageRetract={(id) => console.log('Retract:', id)}
          />
        )
      case 'contacts':
        return <Contacts onStartConversation={handleStartConversation} />
      case 'files':
        return (
          <FileTransfer
            currentUser ={currentUser}
            fileTransfers={fileTransfers}
            users={{}}
            onPause={handleFileTransferPause}
            onResume={handleFileTransferResume}
            onCancel={handleFileTransferCancel}
            onRetry={handleFileTransferRetry}
            onSendFile={handleSendFile}
            onAcceptTransfer={handleAcceptTransfer}
            onRejectTransfer={handleRejectTransfer}
          />
        )
      case 'organization':
        // TODO: Implement organization chart with real data
        return (
          <div className="flex-1 flex items-center justify-center h-full">
            <p className="text-slate-400 dark:text-slate-500">组织架构功能开发中</p>
          </div>
        )
      default:
        return null
    }
  }

  return (
    <AppShell
      mainNav={{
        activeTab,
        onTabChange: (tab) => {
          setActiveTab(tab)
          setShowSettings(false)
        },
        user: {
          name: user.name,
          avatar: user.avatar,
          status: user.status,
        },
        onUserProfile: handleSettings,
      }}
      userMenu={{
        user: {
          name: user.name,
          avatar: user.avatar,
          status: user.status,
        },
        menuItems: [
          { id: 'settings', label: '个人设置', icon: 'settings', action: handleSettings },
          { id: 'network', label: '网络设置', icon: 'network', action: handleNetworkConfig },
          { id: 'logout', label: '退出登录', icon: 'logout', action: handleLogout },
        ],
      }}
    >
      {renderContent()}
    </AppShell>
  )
}

export default App
