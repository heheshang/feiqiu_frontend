'use client'

import { useState, useEffect } from 'react'
import { BasicSettingsProps, UserStatus, NetworkConfig, User, NetworkStatus } from '../../lib/types/basic-settings'
import { NetworkStatusCard } from './NetworkStatusCard'
import { cn } from '../../lib/utils'
import { Upload, AlertTriangle } from 'lucide-react'
import { getConfig, setConfig } from '../../lib/api/config'
import type { Config } from '../../lib/converters'

// Default values for when config loading fails
const defaultUser: User = {
  id: 'local',
  name: '',
  signature: '',
  status: 'online',
  department: '',
}

const defaultNetworkConfig: NetworkConfig = {
  id: 'network',
  udpPort: 2425,
  bindAddress: '0.0.0.0',
  broadcastAddress: '255.255.255.255',
  maxRetries: 3,
  timeout: 5000,
}

const defaultNetworkStatus: NetworkStatus = {
  ipAddress: '未知',
  macAddress: '',
  connectionStatus: 'disconnected',
  listeningPort: 0,
  lastSeen: new Date().toISOString(),
  onlineUsers: 0,
}

// Helper function to validate IPv4 address
function isValidIPv4(ip: string): boolean {
  if (ip === '0.0.0.0') return true
  const regex = /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/
  const match = ip.match(regex)
  if (!match) return false
  return match.slice(1, 5).every((octet) => {
    const num = parseInt(octet, 10)
    return num >= 0 && num <= 255
  })
}

// Convert backend Config to UI User type
function configToUser(config: Config): User {
  return {
    id: 'local',
    name: config.username || '',
    avatarUrl: config.avatar || undefined,
    signature: '', // Not stored in backend config yet
    status: (config.status as UserStatus) || 'online',
    department: '', // Not stored in backend config yet
  }
}

// Convert backend Config to UI NetworkConfig type
function configToNetworkConfig(config: Config): NetworkConfig {
  return {
    id: 'network',
    udpPort: config.udpPort || 2425,
    bindAddress: config.bindIp || '0.0.0.0',
    broadcastAddress: '255.255.255.255', // Default, not in backend yet
    maxRetries: 3, // Default, not in backend yet
    timeout: 5000, // Default, not in backend yet
  }
}

// Convert UI User to partial backend Config
function userToConfig(user: User): Partial<Config> {
  return {
    username: user.name,
    status: user.status,
  }
}

// Convert UI NetworkConfig to partial backend Config
function networkConfigToConfig(networkConfig: NetworkConfig): Partial<Config> {
  return {
    udpPort: networkConfig.udpPort,
    bindIp: networkConfig.bindAddress,
  }
}

export function BasicSettings({
  user: initialUser = defaultUser,
  networkConfig: initialNetworkConfig = defaultNetworkConfig,
  networkStatus: initialNetworkStatus = defaultNetworkStatus,
  activeTab = 'profile',
  onTabChange,
  onUpdateUser,
  onUploadAvatar,
  onStatusChange,
  onSaveNetworkConfig,
  onCancelNetworkConfig,
}: BasicSettingsProps) {
  const [editedUser, setEditedUser] = useState<User>(initialUser)
  const [editedConfig, setEditedConfig] = useState<NetworkConfig>(initialNetworkConfig)
  const [currentNetworkStatus, setCurrentNetworkStatus] = useState<NetworkStatus>(initialNetworkStatus)
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false)
  const [showAvatarUpload, setShowAvatarUpload] = useState(false)
  const [isLoading, setIsLoading] = useState(true)
  const [networkConfigChanged, setNetworkConfigChanged] = useState(false)

  // Load config on component mount
  useEffect(() => {
    async function loadConfig() {
      try {
        setIsLoading(true)
        const config = await getConfig()
        setEditedUser(configToUser(config))
        setEditedConfig(configToNetworkConfig(config))
        // Update network status with loaded config
        setCurrentNetworkStatus({
          ...initialNetworkStatus,
          ipAddress: config.bindIp || '未知',
          listeningPort: config.udpPort || 0,
        })
      } catch (error) {
        console.error('Failed to load config:', error)
        alert('加载配置失败，使用默认值')
      } finally {
        setIsLoading(false)
      }
    }
    loadConfig()
  }, [])

  // Reload props when they change
  useEffect(() => {
    if (initialUser.name) {
      setEditedUser(initialUser)
    }
  }, [initialUser])

  useEffect(() => {
    if (initialNetworkConfig.udpPort) {
      setEditedConfig(initialNetworkConfig)
    }
  }, [initialNetworkConfig])

  const handleAvatarUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (file) {
      if (file.size > 2 * 1024 * 1024) {
        alert('头像文件大小不能超过 2MB')
        return
      }
      if (!['image/jpeg', 'image/png', 'image/gif'].includes(file.type)) {
        alert('仅支持 JPG、PNG、GIF 格式的图片')
        return
      }
      onUploadAvatar?.(file)
      setShowAvatarUpload(false)
    }
  }

  const handleSaveUser = async () => {
    // Validate nickname
    if (!editedUser.name.trim()) {
      alert('昵称不能为空')
      return
    }
    if (editedUser.name.length > 50) {
      alert('昵称不能超过 50 个字符')
      return
    }

    try {
      // Get current config and merge with user changes
      const currentConfig = await getConfig()
      const configUpdate = {
        ...currentConfig,
        username: editedUser.name,
        status: editedUser.status,
      }
      await setConfig(configUpdate)
      onUpdateUser?.(editedUser)
      setHasUnsavedChanges(false)
      alert('保存成功')
      // Call optional status change callback
      onStatusChange?.(editedUser.status)
    } catch (error) {
      console.error('Failed to save user config:', error)
      alert('保存失败，请重试')
    }
  }

  const handleSaveNetwork = async () => {
    // Validate UDP port
    if (editedConfig.udpPort < 1024 || editedConfig.udpPort > 65535) {
      alert('UDP 端口必须在 1024-65535 之间')
      return
    }

    // Validate bind address
    if (!isValidIPv4(editedConfig.bindAddress)) {
      alert('请输入有效的 IP 地址')
      return
    }

    try {
      // Get current config and merge with network changes
      const currentConfig = await getConfig()
      const configUpdate = {
        ...currentConfig,
        udpPort: editedConfig.udpPort,
        bindIp: editedConfig.bindAddress,
      }
      await setConfig(configUpdate)
      onSaveNetworkConfig?.(editedConfig)
      setHasUnsavedChanges(false)
      setNetworkConfigChanged(true)
      alert('保存成功')
      // Reset the network config changed flag after 5 seconds
      setTimeout(() => setNetworkConfigChanged(false), 5000)
    } catch (error) {
      console.error('Failed to save network config:', error)
      alert('保存失败，请重试')
    }
  }

  const handleCancelNetwork = () => {
    // Reload original config
    async function reloadConfig() {
      try {
        const config = await getConfig()
        setEditedConfig(configToNetworkConfig(config))
      } catch (error) {
        console.error('Failed to reload config:', error)
      }
    }
    reloadConfig()
    setHasUnsavedChanges(false)
    setNetworkConfigChanged(false)
    onCancelNetworkConfig?.()
  }

  const statusOptions: { value: UserStatus; label: string; color: string }[] = [
    { value: 'online', label: '在线', color: 'bg-emerald-500' },
    { value: 'away', label: '离开', color: 'bg-amber-500' },
    { value: 'busy', label: '忙碌', color: 'bg-amber-500' },
    { value: 'offline', label: '离线', color: 'bg-slate-400' },
  ]

  return (
    <div className="h-full flex flex-col bg-slate-50 dark:bg-slate-900">
      <div className="bg-white dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
        <div className="max-w-4xl mx-auto px-6 py-4">
          <h1 className="text-2xl font-semibold text-slate-800 dark:text-slate-200">
            基础设置
          </h1>
        </div>
      </div>

      <div className="bg-white dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
        <div className="max-w-4xl mx-auto px-6">
          <div className="flex gap-8">
            <button
              onClick={() => onTabChange?.('profile')}
              className={cn(
                'py-4 px-1 border-b-2 font-medium transition-colors',
                activeTab === 'profile'
                  ? 'border-emerald-500 text-emerald-600 dark:text-emerald-400'
                  : 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200'
              )}
            >
              个人信息
            </button>
            <button
              onClick={() => onTabChange?.('network')}
              className={cn(
                'py-4 px-1 border-b-2 font-medium transition-colors',
                activeTab === 'network'
                  ? 'border-emerald-500 text-emerald-600 dark:text-emerald-400'
                  : 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200'
              )}
            >
              网络设置
            </button>
          </div>
        </div>
      </div>

      <div className="flex-1 overflow-auto">
        <div className="max-w-4xl mx-auto px-6 py-8">
          {isLoading ? (
            <div className="flex items-center justify-center py-20">
              <div className="text-slate-600 dark:text-slate-400">加载配置中...</div>
            </div>
          ) : activeTab === 'profile' && (
            <div className="space-y-6">
              <NetworkStatusCard networkStatus={currentNetworkStatus} />

              <div className="bg-white dark:bg-slate-800 rounded-xl shadow-sm border border-slate-200 dark:border-slate-700">
                <div className="p-6 space-y-6">
                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      头像
                    </label>
                    <div className="flex items-center gap-4">
                      {editedUser.avatarUrl ? (
                        <img
                          src={editedUser.avatarUrl}
                          alt={editedUser.name}
                          className="w-20 h-20 rounded-full object-cover ring-2 ring-slate-200 dark:ring-slate-700"
                        />
                      ) : (
                        <div className="w-20 h-20 rounded-full bg-gradient-to-br from-emerald-400 to-emerald-600 flex items-center justify-center text-white text-2xl font-semibold">
                          {editedUser.name.charAt(0)}
                        </div>
                      )}
                      <div>
                        <input
                          type="file"
                          id="avatar-upload"
                          className="hidden"
                          accept="image/jpeg,image/png,image/gif"
                          onChange={handleAvatarUpload}
                        />
                        <label
                          htmlFor="avatar-upload"
                          className="inline-flex items-center gap-2 px-4 py-2 bg-emerald-500 hover:bg-emerald-600 text-white rounded-lg font-medium transition-colors cursor-pointer"
                        >
                          <Upload className="w-4 h-4" />
                          上传头像
                        </label>
                        <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                          支持 JPG、PNG、GIF，最大 2MB
                        </p>
                      </div>
                    </div>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      昵称
                    </label>
                    <input
                      type="text"
                      value={editedUser.name}
                      onChange={(e) => {
                        setEditedUser({ ...editedUser, name: e.target.value })
                        setHasUnsavedChanges(true)
                      }}
                      className="w-full px-4 py-2.5 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-800 dark:text-slate-200 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors"
                      placeholder="输入您的昵称"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      个性签名
                    </label>
                    <textarea
                      value={editedUser.signature}
                      onChange={(e) => {
                        setEditedUser({ ...editedUser, signature: e.target.value })
                        setHasUnsavedChanges(true)
                      }}
                      rows={3}
                      className="w-full px-4 py-2.5 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-800 dark:text-slate-200 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors resize-none"
                      placeholder="写下您的个性签名..."
                      maxLength={100}
                    />
                    <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                      {editedUser.signature.length}/100
                    </p>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      在线状态
                    </label>
                    <div className="flex flex-wrap gap-3">
                      {statusOptions.map((status) => (
                        <button
                          key={status.value}
                          onClick={() => {
                            setEditedUser({ ...editedUser, status: status.value })
                            setHasUnsavedChanges(true)
                          }}
                          className={cn(
                            'inline-flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-all',
                            editedUser.status === status.value
                              ? `${status.color} text-white shadow-md`
                              : 'bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-600'
                          )}
                        >
                          <span className={`w-2 h-2 rounded-full ${status.color}`} />
                          {status.label}
                        </button>
                      ))}
                    </div>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      所属部门
                    </label>
                    <div className="px-4 py-2.5 bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-600 rounded-lg text-slate-600 dark:text-slate-400">
                      {editedUser.department}
                    </div>
                  </div>
                </div>

                {hasUnsavedChanges && (
                  <div className="px-6 py-4 bg-slate-50 dark:bg-slate-900 border-t border-slate-200 dark:border-slate-700 flex justify-end gap-3">
                    <button
                      onClick={async () => {
                        // Reload original config
                        try {
                          const config = await getConfig()
                          setEditedUser(configToUser(config))
                        } catch (error) {
                          console.error('Failed to reload user config:', error)
                        }
                        setHasUnsavedChanges(false)
                      }}
                      className="px-6 py-2 text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 font-medium transition-colors"
                    >
                      取消
                    </button>
                    <button
                      onClick={handleSaveUser}
                      className="px-6 py-2 bg-emerald-500 hover:bg-emerald-600 text-white rounded-lg font-medium transition-colors shadow-sm"
                    >
                      保存更改
                    </button>
                  </div>
                )}
              </div>
            </div>
          )}

          {activeTab === 'network' && (
            <div className="space-y-6">
              <div className="bg-white dark:bg-slate-800 rounded-xl shadow-sm border border-slate-200 dark:border-slate-700">
                <div className="p-6 space-y-6">
                  <div>
                    <h3 className="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4">
                      网络配置
                    </h3>
                    <p className="text-sm text-slate-600 dark:text-slate-400 mb-6">
                      修改网络配置后需要重启服务才能生效。
                    </p>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      UDP 监听端口
                    </label>
                    <input
                      type="number"
                      value={editedConfig.udpPort}
                      onChange={(e) => {
                        const port = parseInt(e.target.value) || 0
                        setEditedConfig({ ...editedConfig, udpPort: port })
                        setHasUnsavedChanges(true)
                      }}
                      className="w-full px-4 py-2.5 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-800 dark:text-slate-200 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors"
                      placeholder="1024-65535"
                      min={1024}
                      max={65535}
                    />
                    <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                      端口范围：1024-65535（默认：2425）
                    </p>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      绑定地址
                    </label>
                    <input
                      type="text"
                      value={editedConfig.bindAddress}
                      onChange={(e) => {
                        setEditedConfig({ ...editedConfig, bindAddress: e.target.value })
                        setHasUnsavedChanges(true)
                      }}
                      className="w-full px-4 py-2.5 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-800 dark:text-slate-200 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors"
                      placeholder="0.0.0.0"
                    />
                    <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                      0.0.0.0 表示监听所有网卡
                    </p>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      广播地址
                    </label>
                    <input
                      type="text"
                      value={editedConfig.broadcastAddress}
                      onChange={(e) => {
                        setEditedConfig({ ...editedConfig, broadcastAddress: e.target.value })
                        setHasUnsavedChanges(true)
                      }}
                      className="w-full px-4 py-2.5 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-800 dark:text-slate-200 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors"
                      placeholder="255.255.255.255"
                    />
                  </div>

                  <div className="pt-6 border-t border-slate-200 dark:border-slate-700">
                    <h4 className="text-sm font-medium text-slate-700 dark:text-slate-300 mb-4">
                      高级设置
                    </h4>

                    <div className="mb-4">
                      <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        最大重试次数
                      </label>
                      <input
                        type="number"
                        value={editedConfig.maxRetries}
                        onChange={(e) => {
                          const retries = parseInt(e.target.value) || 0
                          setEditedConfig({ ...editedConfig, maxRetries: retries })
                          setHasUnsavedChanges(true)
                        }}
                        className="w-full px-4 py-2.5 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-800 dark:text-slate-200 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors"
                        min={0}
                        max={10}
                      />
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        超时时间（毫秒）
                      </label>
                      <input
                        type="number"
                        value={editedConfig.timeout}
                        onChange={(e) => {
                          const timeout = parseInt(e.target.value) || 0
                          setEditedConfig({ ...editedConfig, timeout })
                          setHasUnsavedChanges(true)
                        }}
                        className="w-full px-4 py-2.5 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-800 dark:text-slate-200 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors"
                        min={1000}
                        max={30000}
                        step={1000}
                      />
                    </div>
                  </div>
                </div>

                {hasUnsavedChanges && (
                  <div className="px-6 py-4 bg-slate-50 dark:bg-slate-900 border-t border-slate-200 dark:border-slate-700 flex justify-end gap-3">
                    <button
                      onClick={handleCancelNetwork}
                      className="px-6 py-2 text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 font-medium transition-colors"
                    >
                      取消
                    </button>
                    <button
                      onClick={handleSaveNetwork}
                      className="px-6 py-2 bg-emerald-500 hover:bg-emerald-600 text-white rounded-lg font-medium transition-colors shadow-sm"
                    >
                      保存配置
                    </button>
                  </div>
                )}
              </div>

              {networkConfigChanged && (
                <div className="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-xl p-4 flex gap-3">
                  <AlertTriangle className="w-5 h-5 text-amber-600 dark:text-amber-400 flex-shrink-0 mt-0.5" />
                  <div>
                    <h4 className="font-medium text-amber-800 dark:text-amber-200">
                      需要重启服务
                    </h4>
                    <p className="text-sm text-amber-700 dark:text-amber-300 mt-1">
                      保存网络配置后，飞秋服务需要重启才能使更改生效。
                    </p>
                  </div>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
