'use client'

import { useState } from 'react'
import { BasicSettingsProps, UserStatus } from '../../lib/types/basic-settings'
import { NetworkStatusCard } from './NetworkStatusCard'
import { cn } from '../../lib/utils'
import { Upload, AlertTriangle } from 'lucide-react'

export function BasicSettings({
  user,
  networkConfig,
  networkStatus,
  activeTab = 'profile',
  onTabChange,
  onUpdateUser,
  onUploadAvatar,
  onStatusChange,
  onSaveNetworkConfig,
  onCancelNetworkConfig,
}: BasicSettingsProps) {
  const [editedUser, setEditedUser] = useState(user)
  const [editedConfig, setEditedConfig] = useState(networkConfig)
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false)
  const [showAvatarUpload, setShowAvatarUpload] = useState(false)

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

  const handleSaveUser = () => {
    onUpdateUser?.(editedUser)
    setHasUnsavedChanges(false)
  }

  const handleSaveNetwork = () => {
    if (editedConfig.udpPort < 1024 || editedConfig.udpPort > 65535) {
      alert('UDP 端口必须在 1024-65535 之间')
      return
    }
    onSaveNetworkConfig?.(editedConfig)
    setHasUnsavedChanges(false)
  }

  const handleCancelNetwork = () => {
    setEditedConfig(networkConfig)
    setHasUnsavedChanges(false)
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
          {activeTab === 'profile' && (
            <div className="space-y-6">
              <NetworkStatusCard networkStatus={networkStatus} />

              <div className="bg-white dark:bg-slate-800 rounded-xl shadow-sm border border-slate-200 dark:border-slate-700">
                <div className="p-6 space-y-6">
                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      头像
                    </label>
                    <div className="flex items-center gap-4">
                      {user.avatarUrl ? (
                        <img
                          src={user.avatarUrl}
                          alt={user.name}
                          className="w-20 h-20 rounded-full object-cover ring-2 ring-slate-200 dark:ring-slate-700"
                        />
                      ) : (
                        <div className="w-20 h-20 rounded-full bg-gradient-to-br from-emerald-400 to-emerald-600 flex items-center justify-center text-white text-2xl font-semibold">
                          {user.name.charAt(0)}
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
                            onStatusChange?.(status.value)
                            setEditedUser({ ...editedUser, status: status.value })
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
                      {user.department}
                    </div>
                  </div>
                </div>

                {hasUnsavedChanges && (
                  <div className="px-6 py-4 bg-slate-50 dark:bg-slate-900 border-t border-slate-200 dark:border-slate-700 flex justify-end gap-3">
                    <button
                      onClick={() => {
                        setEditedUser(user)
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

              {hasUnsavedChanges && (
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
