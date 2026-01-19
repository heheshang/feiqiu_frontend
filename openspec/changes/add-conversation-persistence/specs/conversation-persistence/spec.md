# Capability: 会话持久化 (Conversation Persistence)

会话持久化功能为飞秋应用提供完整的会话生命周期管理，包括会话记录的创建、更新、删除和查询。用户可以从通讯录点击启动会话，发送消息时自动创建会话记录，应用重启后自动恢复会话列表。

## ADDED Requirements

### Requirement: 会话数据模型

The system SHALL provide a conversation data model for persisting conversation metadata.

#### Scenario: 会话实体包含必要字段
- **WHEN** 系统创建会话实体
- **THEN** 会话必须包含以下字段：
  - `id`: 唯一标识符
  - `type`: 会话类型（'single' 单聊 或 'group' 群聊）
  - `created_at`: 会话创建时间
  - `updated_at`: 会话最后更新时间
  - `is_pinned`: 是否置顶
  - `is_archived`: 是否归档
  - `is_muted`: 是否静音
  - `unread_count`: 未读消息数量
  - `last_message_id`: 最后一条消息的 ID（可选）
  - `last_message_at`: 最后一条消息的时间（可选）
  - `last_message_content`: 最后一条消息的内容（可选，冗余存储以提升性能）
  - `last_message_type`: 最后一条消息的类型（可选）

#### Scenario: 会话参与者实体包含必要字段
- **WHEN** 系统创建会话参与者实体
- **THEN** 参与者必须包含以下字段：
  - `id`: 唯一标识符
  - `conversation_id`: 关联的会话 ID
  - `peer_ip`: 参与者的 IP 地址
  - `joined_at`: 加入会话的时间
  - `left_at`: 离开会话的时间（NULL 表示仍在会话中）
  - `role`: 角色（'owner', 'admin', 'member'，默认 'member'）

### Requirement: 会话创建

The system SHALL automatically create conversation records in the following scenarios.

#### Scenario: 发送消息时自动创建会话
- **WHEN** 用户向某个 peer 发送第一条消息
- **THEN** 系统必须自动创建单聊会话
- **AND** 会话参与者包含发送方和接收方
- **AND** 发送的消息成为会话的最后消息

#### Scenario: 接收消息时自动创建会话
- **WHEN** 系统接收到某个 peer 的第一条消息
- **THEN** 系统必须自动创建单聊会话
- **AND** 会话未读计数设置为 1
- **AND** 接收的消息成为会话的最后消息

#### Scenario: 从通讯录启动会话
- **WHEN** 用户在通讯录中点击某个联系人并选择"发消息"
- **THEN** 系统必须创建或获取该联系人的会话
- **AND** 切换到聊天界面并选中该会话
- **AND** 如果会话无消息历史，显示空会话状态

#### Scenario: 避免重复创建会话
- **WHEN** 尝试为已存在会话的两个 peer 创建会话
- **THEN** 系统必须返回现有会话而不是创建新会话
- **AND** 判断依据是：两个 peer 的 IP 是否已存在于同一个会话的参与者列表中

### Requirement: 会话查询

The system MUST provide the capability to query conversation lists.

#### Scenario: 获取所有会话列表
- **WHEN** 应用启动或用户刷新会话列表
- **THEN** 系统必须返回所有会话
- **AND** 会话按最后消息时间倒序排列
- **AND** 置顶会话排在最前面
- **AND** 包含每个会话的参与者信息和最后消息预览

#### Scenario: 根据 peer IP 查询单聊会话
- **WHEN** 需要查找与特定 peer 的会话
- **THEN** 系统必须返回包含该 peer 的单聊会话
- **AND** 如果不存在则返回 None

#### Scenario: 会话列表包含未读计数
- **WHEN** 查询会话列表
- **THEN** 每个会话必须包含准确的未读消息数量
- **AND** 未读计数为 0 时不显示未读标识

### Requirement: 会话更新

The system MUST support updating conversation metadata.

#### Scenario: 更新会话最后消息
- **WHEN** 收到或发送新消息
- **THEN** 系统必须更新会话的 `last_message_id`, `last_message_at`, `last_message_content`, `last_message_type` 字段
- **AND** 更新会话的 `updated_at` 为当前时间
- **AND** 如果是接收消息，增加 `unread_count`

#### Scenario: 标记会话为已读
- **WHEN** 用户打开或切换到某个会话
- **THEN** 系统必须将 `unread_count` 设置为 0
- **AND** 发送 `conversation-updated` 事件到前端

#### Scenario: 置顶会话
- **WHEN** 用户置顶某个会话
- **THEN** 系统必须将 `is_pinned` 设置为 true
- **AND** 会话在列表中排在最前面

#### Scenario: 取消置顶会话
- **WHEN** 用户取消置顶某个会话
- **THEN** 系统必须将 `is_pinned` 设置为 false
- **AND** 会话按正常排序规则显示

#### Scenario: 归档会话
- **WHEN** 用户归档某个会话
- **THEN** 系统必须将 `is_archived` 设置为 true
- **AND** 会话从主列表中移除（但仍然存在于数据库中）

#### Scenario: 取消归档会话
- **WHEN** 用户取消归档某个会话
- **THEN** 系统必须将 `is_archived` 设置为 false
- **AND** 会话重新出现在主列表中

#### Scenario: 静音会话
- **WHEN** 用户静音某个会话
- **THEN** 系统必须将 `is_muted` 设置为 true
- **AND** 该会话的新消息不发送通知

### Requirement: 会话删除

The system MUST support deleting conversation records.

#### Scenario: 删除会话但保留消息
- **WHEN** 用户选择"删除会话"操作
- **THEN** 系统必须删除会话记录和参与者记录
- **AND** 消息历史仍然保留在数据库中
- **AND** 如果后续收到该 peer 的新消息，自动创建新会话

#### Scenario: 删除会话和消息
- **WHEN** 用户选择"删除聊天记录"操作
- **THEN** 系统必须删除会话记录、参与者记录
- **AND** 删除该会话相关的所有消息

### Requirement: 会话持久化

The system MUST restore conversation lists after application restart.

#### Scenario: 应用启动时自动加载会话
- **WHEN** 应用启动
- **THEN** 系统必须从数据库加载所有会话
- **AND** 会话列表在聊天界面显示
- **AND** 用户无需手动恢复

#### Scenario: 数据迁移：为现有消息创建会话
- **WHEN** 系统首次升级到支持会话持久化的版本
- **THEN** 系统必须扫描现有消息历史
- **AND** 为每个有消息历史的 peer 自动创建会话记录
- **AND** 迁移过程对用户透明

### Requirement: 会话事件通知

The system MUST send events to the frontend when conversation state changes.

#### Scenario: 会话创建事件
- **WHEN** 创建新会话
- **THEN** 系统必须发送 `conversation-created` 事件
- **AND** 事件包含完整的会话信息
- **AND** 前端自动添加会话到列表

#### Scenario: 会话更新事件
- **WHEN** 会话元数据发生变化（最后消息、未读计数、置顶等）
- **THEN** 系统必须发送 `conversation-updated` 事件
- **AND** 事件包含更新后的会话信息
- **AND** 前端自动更新会话列表显示

#### Scenario: 未读计数变化事件
- **WHEN** 会话未读计数发生变化
- **THEN** 系统必须通过 `conversation-updated` 事件通知前端
- **AND** 前端更新未读标识

### Requirement: 后端 IPC 命令

The system MUST provide the following backend IPC commands for frontend invocation.

#### Scenario: get_conversations 命令
- **WHEN** 前端调用 `get_conversations()`
- **THEN** 系统必须返回所有会话列表
- **AND** 每个会话包含参与者信息和最后消息预览
- **AND** 返回结果按更新时间倒序排列，置顶会话排在前面

#### Scenario: get_or_create_conversation 命令
- **WHEN** 前端调用 `get_or_create_conversation(peerIp)`
- **THEN** 系统必须查找与该 peer 的现有会话
- **AND** 如果不存在则创建新会话
- **AND** 返回会话信息

#### Scenario: update_conversation 命令
- **WHEN** 前端调用 `update_conversation(id, updates)`
- **THEN** 系统必须更新会话元数据
- **AND** 发送 `conversation-updated` 事件
- **AND** 返回更新后的会话信息

#### Scenario: mark_conversation_read 命令
- **WHEN** 前端调用 `mark_conversation_read(conversationId)`
- **THEN** 系统必须将未读计数设置为 0
- **AND** 发送 `conversation-updated` 事件

#### Scenario: delete_conversation 命令
- **WHEN** 前端调用 `delete_conversation(conversationId, deleteMessages)`
- **THEN** 系统必须删除会话记录
- **AND** 如果 `deleteMessages` 为 true，同时删除相关消息
- **AND** 发送 `conversation-deleted` 事件

### Requirement: 前端状态管理

The frontend MUST provide conversation state management mechanisms.

#### Scenario: conversationsStore Zustand Store
- **WHEN** 前端需要管理会话状态
- **THEN** 系统必须提供 `conversationsStore` Zustand store
- **AND** store 包含 `conversations`, `activeConversationId`, `isLoading`, `error` 状态
- **AND** 提供 `setConversations`, `setActiveConversation`, `updateConversation`, `markAsRead`, `deleteConversation` 操作

#### Scenario: useConversations 自定义 Hook
- **WHEN** 组件需要访问会话数据
- **THEN** 系统必须提供 `useConversations` hook
- **AND** hook 自动加载会话列表
- **AND** hook 自动监听 `conversation-created`, `conversation-updated`, `conversation-deleted` 事件
- **AND** hook 提供 `{ conversations, activeConversationId, isLoading, error, refresh, setActiveConversation, markAsRead, deleteConversation }` 返回值

#### Scenario: 会话列表实时更新
- **WHEN** 后端发送会话事件
- **THEN** 前端必须自动更新会话列表
- **AND** UI 立即反映最新状态
- **AND** 无需手动刷新

### Requirement: 通讯录集成

The contacts feature MUST integrate seamlessly with the conversation system.

#### Scenario: 从通讯录启动会话
- **WHEN** 用户在通讯录中点击"发消息"
- **THEN** 系统必须调用 `get_or_create_conversation` IPC 命令
- **AND** 切换到聊天界面
- **AND** 选中并显示该会话

#### Scenario: 通讯录显示会话状态
- **WHEN** 用户在通讯录中查看联系人
- **THEN** 如果存在与该联系人的会话，显示最后消息预览
- **AND** 如果有未读消息，显示未读计数
