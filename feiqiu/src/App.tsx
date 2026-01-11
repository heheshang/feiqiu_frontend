import { useState } from 'react'
import { AppShell } from './components/shell'
import { BasicSettings } from './components/basic-settings'
import { Messaging } from './components/messaging'
import { FileTransfer } from './components/file-transfer'
import { CollaborationTools } from './components/collaboration'
import { OrganizationChart } from './components/organization'
import { NavTab, UserStatus } from './lib/types/shell'
import { useDarkMode } from './hooks/useDarkMode'
import { User as SettingsUser, NetworkConfig, NetworkStatus } from './lib/types/basic-settings'
import { Conversation, User as MessagingUser } from './lib/types/messaging'
import { FileTransfer as FileTransferType, User as FTUser } from './lib/types/file-transfer'
import { Screenshot, User as ColUser } from './lib/types/collaboration'
import { Department, User as OrgUser } from './lib/types/organization'

interface AppUser extends SettingsUser {
  avatar: string
}

const sampleUser: AppUser = {
  id: 'user-001',
  name: '张伟',
  avatar: '',
  avatarUrl: '',
  signature: '工作努力，生活快乐！',
  status: 'online',
  department: '前端组',
}

const sampleNetworkConfig: NetworkConfig = {
  id: 'net-config-001',
  udpPort: 2425,
  bindAddress: '0.0.0.0',
  broadcastAddress: '255.255.255.255',
  maxRetries: 3,
  timeout: 5000,
}

const sampleNetworkStatus: NetworkStatus = {
  ipAddress: '192.168.1.100',
  macAddress: '00:1A:2B:3C:4D:5E',
  connectionStatus: 'connected',
  listeningPort: 2425,
  lastSeen: new Date().toISOString(),
  onlineUsers: 23,
}

const sampleConversations: Conversation[] = [
  {
    id: 'conv-1',
    type: 'single',
    pinned: true,
    unreadCount: 2,
    lastMessage: {
      id: 'msg-15',
      content: '好的，下午三点会议室见',
      type: 'text',
      timestamp: new Date(Date.now() - 30 * 60000).toISOString(),
      senderId: 'user-2',
      senderName: '李明',
    },
    participant: {
      id: 'user-2',
      name: '李明',
      avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Liming',
      status: 'online',
      department: '产品部',
    },
    messages: [
      {
        id: 'msg-10',
        type: 'text',
        content: '你好，请问今天有时间过一下需求吗？',
        timestamp: new Date(Date.now() - 4 * 3600000).toISOString(),
        senderId: 'user-1',
        senderName: '我',
        status: 'read',
        reactions: [],
      },
      {
        id: 'msg-11',
        type: 'text',
        content: '有的，下午2点之后都可以',
        timestamp: new Date(Date.now() - 3.5 * 3600000).toISOString(),
        senderId: 'user-2',
        senderName: '李明',
        status: 'read',
        reactions: [{ emoji: '👍', users: [{ id: 'user-1', name: '我' }] }],
      },
      {
        id: 'msg-12',
        type: 'text',
        content: '太好了，我整理了一下新的需求文档',
        timestamp: new Date(Date.now() - 3 * 3600000).toISOString(),
        senderId: 'user-1',
        senderName: '我',
        status: 'read',
        reactions: [],
      },
      {
        id: 'msg-13',
        type: 'text',
        content: '好的，发给我看看',
        timestamp: new Date(Date.now() - 2.5 * 3600000).toISOString(),
        senderId: 'user-2',
        senderName: '李明',
        status: 'read',
        reactions: [],
      },
      {
        id: 'msg-14',
        type: 'text',
        content: '文档已经发到你的邮箱了',
        timestamp: new Date(Date.now() - 2 * 3600000).toISOString(),
        senderId: 'user-1',
        senderName: '我',
        status: 'read',
        reactions: [],
        quote: {
          messageId: 'msg-13',
          content: '好的，发给我看看',
          senderName: '李明',
        },
      },
      {
        id: 'msg-15',
        type: 'text',
        content: '好的，下午三点会议室见',
        timestamp: new Date(Date.now() - 30 * 60000).toISOString(),
        senderId: 'user-2',
        senderName: '李明',
        status: 'unread',
        reactions: [],
      },
    ],
  },
  {
    id: 'conv-2',
    type: 'single',
    pinned: false,
    unreadCount: 0,
    lastMessage: {
      id: 'msg-12',
      content: '[图片]',
      type: 'image',
      timestamp: new Date(Date.now() - 24 * 3600000).toISOString(),
      senderId: 'user-3',
      senderName: '王芳',
    },
    participant: {
      id: 'user-3',
      name: '王芳',
      avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Wangfang',
      status: 'offline',
      department: '设计部',
    },
  },
  {
    id: 'conv-3',
    type: 'group',
    pinned: true,
    unreadCount: 5,
    lastMessage: {
      id: 'msg-25',
      content: '@张伟 请看一下这个设计稿',
      type: 'text',
      timestamp: new Date(Date.now() - 15 * 60000).toISOString(),
      senderId: 'user-4',
      senderName: '赵强',
    },
    group: {
      id: 'group-1',
      name: '产品研发群',
      avatar: 'https://api.dicebear.com/7.x/identicon/svg?seed=product',
      memberCount: 12,
      members: [
        { id: 'user-1', name: '张伟', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Zhangwei' },
        { id: 'user-2', name: '李明', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Liming' },
        { id: 'user-4', name: '赵强', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Zhaoqiang' },
        { id: 'user-5', name: '刘洋', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=LiuYang' },
      ],
    },
  },
]

const currentUser: MessagingUser = {
  id: 'user-1',
  name: '张伟',
  avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Zhangwei',
  status: 'online',
}

const fileTransferUsers: Record<string, FTUser> = {
  'user-1': { id: 'user-1', name: '张伟', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Zhangwei' },
  'user-2': { id: 'user-2', name: '李明', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Liming' },
  'user-3': { id: 'user-3', name: '王芳', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Wangfang' },
  'user-4': { id: 'user-4', name: '孙磊', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Sunlei' },
  'user-5': { id: 'user-5', name: '陈静', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Chenjing' },
}

const sampleFileTransfers: FileTransferType[] = [
  {
    id: 'transfer-1',
    direction: 'send',
    senderId: 'user-1',
    receiverId: 'user-2',
    fileName: '产品设计规范文档.pdf',
    fileSize: 15728640,
    fileType: 'pdf',
    status: 'transferring',
    progress: 67,
    transferSpeed: 15728640,
    remainingTime: 5,
    startTime: new Date(Date.now() - 30 * 60000).toISOString(),
    endTime: null,
  },
  {
    id: 'transfer-2',
    direction: 'receive',
    senderId: 'user-3',
    receiverId: 'user-1',
    fileName: 'UI设计稿v2.0.fig',
    fileSize: 52428800,
    fileType: 'fig',
    status: 'transferring',
    progress: 42,
    transferSpeed: 20971520,
    remainingTime: 12,
    startTime: new Date(Date.now() - 45 * 60000).toISOString(),
    endTime: null,
  },
  {
    id: 'transfer-3',
    direction: 'send',
    senderId: 'user-1',
    receiverId: 'user-4',
    fileName: '项目计划表.xlsx',
    fileSize: 1048576,
    fileType: 'xlsx',
    status: 'completed',
    progress: 100,
    transferSpeed: 0,
    remainingTime: 0,
    startTime: new Date(Date.now() - 24 * 3600000).toISOString(),
    endTime: new Date(Date.now() - 24 * 3600000 + 60000).toISOString(),
  },
  {
    id: 'transfer-4',
    direction: 'receive',
    senderId: 'user-2',
    receiverId: 'user-1',
    fileName: '会议记录.docx',
    fileSize: 524288,
    fileType: 'docx',
    status: 'paused',
    progress: 35,
    transferSpeed: 0,
    remainingTime: 0,
    startTime: new Date(Date.now() - 2 * 3600000).toISOString(),
    endTime: null,
  },
  {
    id: 'transfer-5',
    direction: 'send',
    senderId: 'user-1',
    receiverId: 'user-5',
    fileName: '产品需求文档.docx',
    fileSize: 2097152,
    fileType: 'docx',
    status: 'waiting',
    progress: 0,
    transferSpeed: 0,
    remainingTime: 0,
    startTime: new Date().toISOString(),
    endTime: null,
  },
  {
    id: 'transfer-6',
    direction: 'receive',
    senderId: 'user-4',
    receiverId: 'user-1',
    fileName: '系统架构图.png',
    fileSize: 3145728,
    fileType: 'png',
    status: 'completed',
    progress: 100,
    transferSpeed: 0,
    remainingTime: 0,
    startTime: new Date(Date.now() - 48 * 3600000).toISOString(),
    endTime: new Date(Date.now() - 48 * 3600000 + 60000).toISOString(),
  },
  {
    id: 'transfer-7',
    direction: 'send',
    senderId: 'user-1',
    receiverId: 'user-3',
    fileName: '测试报告.pdf',
    fileSize: 8388608,
    fileType: 'pdf',
    status: 'failed',
    progress: 78,
    transferSpeed: 0,
    remainingTime: 0,
    startTime: new Date(Date.now() - 12 * 3600000).toISOString(),
    endTime: new Date(Date.now() - 12 * 3600000 + 2 * 60000).toISOString(),
    errorMessage: '网络连接中断',
  },
]

const collaborationUsers: Record<string, ColUser> = {
  'user-1': { id: 'user-1', name: '张伟', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Zhangwei' },
  'user-2': { id: 'user-2', name: '李明', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Liming' },
  'user-3': { id: 'user-3', name: '王芳', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Wangfang' },
}

const sampleScreenshots: Screenshot[] = [
  {
    id: 'screenshot-1',
    type: 'region',
    title: '界面问题反馈',
    imageUrl: 'https://placehold.co/800x600/e2e8f0/64748b?text=Screenshot+1',
    thumbnailUrl: 'https://placehold.co/200x150/e2e8f0/64748b?text=Preview',
    createdAt: new Date(Date.now() - 2 * 3600000).toISOString(),
    createdBy: 'user-1',
    annotations: [
      {
        id: 'anno-1',
        type: 'rectangle',
        x: 100,
        y: 80,
        width: 200,
        height: 100,
        color: '#ef4444',
        lineWidth: 3
      },
      {
        id: 'anno-2',
        type: 'text',
        x: 110,
        y: 60,
        content: '这里需要修改',
        color: '#ef4444',
        fontSize: 16
      }
    ],
    status: 'draft'
  },
  {
    id: 'screenshot-2',
    type: 'fullscreen',
    title: '产品界面设计稿',
    imageUrl: 'https://placehold.co/1920x1080/f1f5f9/475569?text=Fullscreen+Screenshot',
    thumbnailUrl: 'https://placehold.co/200x150/f1f5f9/475569?text=Preview',
    createdAt: new Date(Date.now() - 24 * 3600000).toISOString(),
    createdBy: 'user-1',
    annotations: [
      {
        id: 'anno-4',
        type: 'arrow',
        startX: 500,
        startY: 300,
        endX: 700,
        endY: 400,
        color: '#22c55e',
        lineWidth: 4
      }
    ],
    status: 'sent',
    sentTo: 'user-2'
  },
  {
    id: 'screenshot-3',
    type: 'window',
    title: '操作流程演示',
    imageUrl: 'https://placehold.co/600x400/ede9fe/6b21a8?text=Window+Screenshot',
    thumbnailUrl: 'https://placehold.co/200x150/ede9fe/6b21a8?text=Preview',
    createdAt: new Date(Date.now() - 48 * 3600000).toISOString(),
    createdBy: 'user-1',
    annotations: [],
    status: 'saved'
  }
]

const sampleDepartments: Department[] = [
  { id: 'dept-1', name: '公司总部', parentId: null, level: 0, memberCount: 45 },
  { id: 'dept-1-1', name: '技术部', parentId: 'dept-1', level: 1, memberCount: 20 },
  { id: 'dept-1-1-1', name: '前端组', parentId: 'dept-1-1', level: 2, memberCount: 8 },
  { id: 'dept-1-1-2', name: '后端组', parentId: 'dept-1-1', level: 2, memberCount: 7 },
  { id: 'dept-1-1-3', name: '运维组', parentId: 'dept-1-1', level: 2, memberCount: 5 },
  { id: 'dept-1-2', name: '产品部', parentId: 'dept-1', level: 1, memberCount: 10 },
  { id: 'dept-1-3', name: '设计部', parentId: 'dept-1', level: 1, memberCount: 8 },
  { id: 'dept-1-4', name: '市场部', parentId: 'dept-1', level: 1, memberCount: 7 },
]

const sampleOrgUsers: OrgUser[] = [
  { id: 'user-1', name: '张伟', pinyin: 'zhangwei', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Zhangwei', position: '高级前端工程师', department: '前端组', departmentId: 'dept-1-1-1', status: 'online', email: 'zhangwei@company.com', phone: '138****0001' },
  { id: 'user-2', name: '李明', pinyin: 'liming', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Liming', position: '产品经理', department: '产品部', departmentId: 'dept-1-2', status: 'online', email: 'liming@company.com', phone: '138****0002' },
  { id: 'user-3', name: '王芳', pinyin: 'wangfang', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Wangfang', position: 'UI设计师', department: '设计部', departmentId: 'dept-1-3', status: 'offline', email: 'wangfang@company.com', phone: '138****0003' },
  { id: 'user-4', name: '赵强', pinyin: 'zhaoqiang', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Zhaoqiang', position: '后端工程师', department: '后端组', departmentId: 'dept-1-1-2', status: 'online', email: 'zhaoqiang@company.com', phone: '138****0004' },
  { id: 'user-5', name: '刘洋', pinyin: 'liuyang', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=LiuYang', position: '测试工程师', department: '运维组', departmentId: 'dept-1-1-3', status: 'away', email: 'liuyang@company.com', phone: '138****0005' },
  { id: 'user-6', name: '陈静', pinyin: 'chenjing', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Chenjing', position: '产品总监', department: '产品部', departmentId: 'dept-1-2', status: 'online', email: 'chenjing@company.com', phone: '138****0006' },
  { id: 'user-7', name: '孙磊', pinyin: 'sunlei', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Sunlei', position: '前端工程师', department: '前端组', departmentId: 'dept-1-1-1', status: 'online', email: 'sunlei@company.com', phone: '138****0007' },
  { id: 'user-8', name: '周敏', pinyin: 'zhoumin', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Zhoumin', position: '市场经理', department: '市场部', departmentId: 'dept-1-4', status: 'offline', email: 'zhoumin@company.com', phone: '138****0008' },
  { id: 'user-9', name: '吴超', pinyin: 'wuchao', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Wuchao', position: '架构师', department: '后端组', departmentId: 'dept-1-1-2', status: 'online', email: 'wuchao@company.com', phone: '138****0009' },
  { id: 'user-10', name: '郑雪', pinyin: 'zhengxue', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Zhengxue', position: 'UX设计师', department: '设计部', departmentId: 'dept-1-3', status: 'away', email: 'zhengxue@company.com', phone: '138****0010' },
]

function App() {
  const { theme, toggleTheme } = useDarkMode()
  const [activeTab, setActiveTab] = useState<NavTab>('chat')
  const [showSettings, setShowSettings] = useState(false)
  const [settingsTab, setSettingsTab] = useState<'profile' | 'network'>('profile')
  const [user, setUser] = useState<AppUser>(sampleUser)
  const [activeConversationId, setActiveConversationId] = useState<string | null>('conv-1')
  const [fileTransfers, setFileTransfers] = useState<FileTransferType[]>(sampleFileTransfers)

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

  const handleUpdateUser = (updatedUser: Partial<AppUser>) => {
    setUser((prev) => ({ ...prev, ...updatedUser }))
    console.log('Update user:', updatedUser)
  }

  const handleStatusChange = (status: UserStatus) => {
    setUser((prev) => ({ ...prev, status }))
    console.log('Status changed:', status)
  }

  const handleSendMessage = (conversationId: string, content: string) => {
    console.log('Send message:', { conversationId, content })
  }

  const handleSendImage = (conversationId: string, file: File) => {
    console.log('Send image:', { conversationId, fileName: file.name })
  }

  const handleFileTransferPause = (id: string) => {
    setFileTransfers(prev => prev.map(t => t.id === id ? { ...t, status: 'paused' as const } : t))
    console.log('Pause transfer:', id)
  }

  const handleFileTransferResume = (id: string) => {
    setFileTransfers(prev => prev.map(t => t.id === id ? { ...t, status: 'transferring' as const } : t))
    console.log('Resume transfer:', id)
  }

  const handleFileTransferCancel = (id: string) => {
    setFileTransfers(prev => prev.filter(t => t.id !== id))
    console.log('Cancel transfer:', id)
  }

  const handleFileTransferRetry = (id: string) => {
    setFileTransfers(prev => prev.map(t => t.id === id ? { ...t, status: 'transferring' as const, progress: 0 } : t))
    console.log('Retry transfer:', id)
  }

  const handleSendFile = (files: File[]) => {
    console.log('Send files:', files.map(f => f.name))
  }

  const handleScreenshot = (type: string) => {
    console.log('Screenshot type:', type)
  }

  const renderContent = () => {
    if (showSettings) {
      return (
        <BasicSettings
          user={user}
          networkConfig={sampleNetworkConfig}
          networkStatus={sampleNetworkStatus}
          activeTab={settingsTab}
          onTabChange={setSettingsTab}
          onUpdateUser={handleUpdateUser}
          onStatusChange={handleStatusChange}
          onSaveNetworkConfig={(config) => console.log('Save network config:', config)}
          onCancelNetworkConfig={() => console.log('Cancel network config')}
        />
      )
    }

    switch (activeTab) {
      case 'chat':
        return (
          <Messaging
            conversations={sampleConversations}
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
        return (
          <div className="flex-1 flex items-center justify-center h-full">
            <p className="text-slate-400 dark:text-slate-500">通讯录功能开发中</p>
          </div>
        )
      case 'organization':
        return (
          <OrganizationChart
            currentUser={sampleOrgUsers[0]}
            departments={sampleDepartments}
            users={sampleOrgUsers}
            onDepartmentSelect={(id) => console.log('Select department:', id)}
            onStartChat={(userId) => console.log('Start chat:', userId)}
            onViewDetails={(userId) => console.log('View details:', userId)}
            onSearch={(query) => console.log('Search:', query)}
          />
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
