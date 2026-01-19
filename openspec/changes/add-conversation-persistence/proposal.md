# Change: 添加会话记录持久化与展示功能

## Why

当前系统中会话列表是从在线节点和消息历史动态派生的，存在以下问题：

1. **会话不持久化**：当与某个节点的所有消息被删除后，会话记录随之消失
2. **无法记录空会话**：用户点击通讯录发送消息时，如果还没有消息历史，无法在会话列表中显示该会话
3. **会话元数据丢失**：置顶状态、未读计数、最后活跃时间等会话元数据无法持久化
4. **重启后状态丢失**：应用重启后需要重新扫描消息才能重建会话列表，用户体验差

## What Changes

- **添加 `conversations` 数据库表**：存储会话元数据（创建时间、更新时间、置顶状态、归档状态、最后消息时间等）
- **添加 `conversation_participants` 数据库表**：支持单聊和群聊的参与者管理
- **新增后端 IPC 命令**：
  - `get_conversations()` - 获取会话列表
  - `get_or_create_conversation()` - 获取或创建会话
  - `update_conversation()` - 更新会话元数据（置顶、归档等）
  - `mark_conversation_read()` - 标记会话为已读
  - `delete_conversation()` - 删除会话
- **新增后端事件**：
  - `conversation-created` - 会话创建事件
  - `conversation-updated` - 会话更新事件
- **新增前端 Store**：`conversationsStore` 用于管理会话状态
- **新增前端 Hook**：`useConversations` 用于自动同步会话数据
- **修改消息发送流程**：发送消息时自动创建或更新会话记录
- **[重要] 架构变更**：移除 `App.tsx` 中的 `peersToConversations()` 派生逻辑，会话列表直接从 `get_conversations` 获取，不再依赖 `get_peers`

## Impact

- **影响的功能模块**：
  - `messaging` - 消息和会话管理
  - `contacts` - 通讯录启动会话功能
- **影响的代码文件**：
  - 后端: `src-tauri/src/storage/entities/*`, `src-tauri/src/storage/*_repo.rs`, `src-tauri/src/commands/*`
  - 前端: `src/lib/types/messaging.ts`, `src/hooks/useConversations.ts`, `src/components/messaging/*`
  - 数据库: 新增 migrations
- **破坏性变更**：无（向后兼容现有消息数据）
- **数据迁移**：需要为现有消息历史自动创建会话记录
