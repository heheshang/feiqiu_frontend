# Design: 会话持久化系统设计

## Context

当前系统的会话是通过 `peersToConversations()` 函数从在线节点和消息历史动态派生的。这种设计存在以下问题：

1. **无持久化**：会话元数据（置顶、归档、未读计数）无法保存
2. **状态不一致**：前端和后端对会话的理解可能不同步
3. **性能问题**：每次需要扫描所有消息来计算会话列表
4. **功能限制**：无法支持"空会话"（点击通讯录但还没发消息的会话）

### Stakeholders

- **用户**：需要会话列表在应用重启后保持一致
- **开发者**：需要一个清晰的会话管理抽象层
- **系统**：需要支持未来扩展到群聊功能

## Goals / Non-Goals

### Goals

- 持久化会话元数据（创建时间、更新时间、置顶状态、归档状态）
- 支持单聊会话（peer-to-peer）
- 支持通讯录点击启动会话
- 应用重启后自动加载会话记录
- 保持向后兼容现有消息数据

### Non-Goals

- 群聊功能（Phase 2）- 本设计仅考虑单聊
- 会话搜索功能
- 会话导入/导出
- 会话加密
- 多端同步

## Decisions

### 决策 1：数据库表设计

**选择**：分离 `conversations` 和 `conversation_participants` 两张表

**原因**：
- 支持未来扩展到群聊（Phase 2）
- 单聊和群聊使用统一的数据模型
- 参与者表支持更复杂的权限控制（未来）

**表结构**：

```sql
-- conversations 表
CREATE TABLE conversations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,              -- 'single' 或 'group' (Phase 2)
    created_at BIGINT NOT NULL,       -- 创建时间
    updated_at BIGINT NOT NULL,       -- 更新时间
    is_pinned BOOLEAN DEFAULT 0,      -- 是否置顶
    is_archived BOOLEAN DEFAULT 0,    -- 是否归档
    is_muted BOOLEAN DEFAULT 0,       -- 是否静音 (Phase 2)
    unread_count INTEGER DEFAULT 0,   -- 未读消息数
    last_message_id INTEGER,          -- 最后一条消息 ID
    last_message_at BIGINT,           -- 最后消息时间
    last_message_content TEXT,        -- 最后消息内容（冗余，提升查询性能）
    last_message_type TEXT            -- 最后消息类型
);

-- conversation_participants 表
CREATE TABLE conversation_participants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_id INTEGER NOT NULL, -- 关联 conversations.id
    peer_ip TEXT NOT NULL,            -- 参与者 IP（单聊时唯一标识）
    joined_at BIGINT NOT NULL,        -- 加入时间
    left_at BIGINT,                   -- 离开时间（NULL 表示仍在会话中）
    role TEXT DEFAULT 'member',       -- 角色：'owner', 'admin', 'member' (Phase 2)
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
);

-- 索引
CREATE INDEX idx_conversations_type ON conversations(type);
CREATE INDEX idx_conversations_updated_at ON conversations(updated_at DESC);
CREATE INDEX idx_conversation_participants_conversation ON conversation_participants(conversation_id);
CREATE INDEX idx_conversation_participants_peer_ip ON conversation_participants(peer_ip);
```

**Alternatives considered**:
- **单表设计**：将参与者直接存在 conversations 表中（`peer_ip` 字段）
  - 优点：简单，查询更快
  - 缺点：无法扩展到群聊，需要重构

- **JSON 存储参与者**：将参与者列表存为 JSON 字段
  - 优点：灵活
  - 缺点：无法建立索引，查询效率低，不支持复杂查询

### 决策 2：会话标识策略

**选择**：对于单聊，使用 `peer_ip` 作为会话的唯一标识

**原因**：
- IP 地址在 LAN 环境中是稳定的
- 与现有 peer 管理系统一致
- 简化查询逻辑

**查找单聊会话的逻辑**：
```rust
pub async fn find_single_conversation_by_peer(
    &self,
    my_ip: &str,
    peer_ip: &str,
) -> Result<Option<ConversationModel>> {
    // 查找同时包含我对方 IP 的会话
    // 对于单聊，participants 表中应该只有 2 条记录
}
```

### 决策 2.5：get_peers 与 get_conversations 的分离

**选择**：保留 `get_peers` 命令用于其他场景，但会话列表不再依赖它派生

**原因**：
- `get_peers` 和 `get_conversations` 服务于不同的目的：
  - `get_peers` - 网络层概念，获取所有发现的设备（用于网络状态、在线用户列表等）
  - `get_conversations` - 应用层概念，获取用户的会话列表（用于聊天界面）
- 当前 `App.tsx` 中使用 `peersToConversations(peers, messages)` 派生会话的方式存在问题：
  - 会话依赖 peers 存在，peer 离线会话可能消失
  - 无法持久化会话元数据
  - 重启后必须重新扫描 peers 才能重建会话列表

**实现变更**：
```typescript
// 当前实现（需要移除）
const { peers } = usePeers({ enabled: true })
const conversations = useMemo(() => {
  return peersToConversations(peers, myIp, messages)
}, [peers, messages])

// 新实现
const { conversations } = useConversations({ enabled: true })
```

**保留 get_peers 的用途**：
- 网络状态监控（显示在线/离线用户数量）
- 通讯录中匹配联系人对应的 peer
- 系统信息面板显示网络拓扑

### 决策 3：自动创建会话时机

**选择**：在以下场景自动创建会话
1. 发送消息时（如果会话不存在）
2. 接收消息时（如果会话不存在）
3. 点击通讯录启动会话时

**原因**：
- 用户无需手动管理会话创建
- 与微信、钉钉等主流 IM 体验一致
- 支持"空会话"场景

**实现位置**：
- 发送消息：`commands/message.rs::send_message()`
- 接收消息：`modules/message/handler.rs::handle_incoming_message()`
- 启动会话：`commands/conversation.rs::get_or_create_conversation()`

### 决策 4：未读计数更新策略

**选择**：在接收消息时增加未读计数，用户打开会话时清零

**原因**：
- 简单可靠
- 与主流 IM 一致

**实现细节**：
```rust
// 接收消息时
conversation_repo.increment_unread(conversation_id).await?;

// 用户打开会话时
conversation_repo.mark_as_read(conversation_id).await?;
```

### 决策 5：数据迁移策略

**选择**：应用启动时自动扫描现有消息，为有消息历史的 peer 创建会话

**原因**：
- 用户无感知
- 保证数据完整性
- 一次性迁移成本

**迁移逻辑**：
```rust
pub async fn migrate_existing_messages_to_conversations(&self) -> Result<()> {
    // 1. 获取所有消息
    let all_messages = message_repo.find_all(u64::MAX).await?;

    // 2. 按 (sender_ip, receiver_ip) 分组
    let mut peer_pairs: HashSet<(String, String)> = HashSet::new();
    for msg in &all_messages {
        peer_pairs.insert((msg.sender_ip.clone(), msg.receiver_ip.clone()));
    }

    // 3. 为每个 peer pair 创建会话
    for (my_ip, peer_ip) in peer_pairs {
        let existing = conversation_repo.find_single_conversation_by_peer(&my_ip, &peer_ip).await?;
        if existing.is_none() {
            conversation_repo.insert(&ConversationModel {
                type: "single".to_string(),
                // ...
            }).await?;
        }
    }

    Ok(())
}
```

## Risks / Trade-offs

### Risk 1: 会话与消息不一致

**风险**：会话记录可能被删除，但消息仍然存在

**缓解措施**：
- 删除会话时提供选项：仅删除会话记录 or 同时删除消息
- 如果消息存在但会话不存在，自动重建会话（`find_or_create` 模式）

### Risk 2: 并发更新冲突

**风险**：同时接收多条消息时，未读计数可能不准确

**缓解措施**：
- 使用数据库原子操作（`unread_count + 1`）
- 使用 `Arc<Mutex<>>` 保护共享状态

### Risk 3: IP 地址变化

**风险**：用户重启后 IP 地址可能变化（DHCP）

**缓解措施**：
- 使用 `user_id` 作为主要标识，IP 仅作为网络路由标识
- 定期刷新 peer 信息，更新会话参与者

### Risk 4: 性能影响

**风险**：每次收发消息都需要更新会话记录

**缓解措施**：
- 使用数据库索引加速查询
- 批量更新会话（如果短时间内多条消息）
- 考虑使用缓存（Redis/内存）

## Migration Plan

### Phase 1: 数据库迁移（应用启动时执行）

```sql
-- 1. 创建新表
-- 2. 扫描现有消息
-- 3. 为每个有消息历史的 peer 创建会话
-- 4. 验证迁移完整性
```

### Phase 2: 代码迁移

1. 后端：添加 conversation 模块
2. 前端：添加 conversation store 和 hook
3. UI：逐步替换派生会话为持久化会话

### Phase 3: 验证

1. 单元测试
2. 集成测试
3. 手动测试：发送消息、重启应用、验证会话恢复

### Rollback

如果迁移失败：
1. 回滚数据库到迁移前状态
2. 删除新增的 conversation 相关代码
3. 恢复原有的 `peersToConversations()` 派生逻辑

## Open Questions

1. **Q**: 是否需要支持会话搜索？
   **A**: Phase 1 不支持，Phase 2 考虑添加

2. **Q**: 会话数量是否有上限？
   **A**: 暂无限制，后续可考虑添加归档机制

3. **Q**: 如何处理群聊？
   **A**: Phase 2 添加群聊支持，当前设计已经预留了 `conversation_participants` 表

4. **Q**: 会话删除后，消息是否删除？
   **A**: 提供"删除会话"和"删除聊天记录"两个选项

5. **Q**: 最后消息内容是否需要冗余存储？
   **A**: 是的，提升查询性能，避免每次都 JOIN messages 表

## 数据流图

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Frontend (React)                             │
│                                                                     │
│  ┌──────────────────┐          ┌──────────────────────────────┐   │
│  │   usePeers Hook  │          │  useConversations Hook       │   │
│  │  - get_peers()   │          │  - get_conversations()       │   │
│  │  - 网络状态监控   │          │  - 会话列表管理              │   │
│  │  - 在线用户列表   │          │  - 自动事件监听              │   │
│  └──────────────────┘          └───────────┬──────────────────┘   │
│         ▼                                     │                     │
│  ┌──────────────────┐                        ▼                     │
│  │   peersStore     │          ┌──────────────────────────────┐   │
│  │  (网络层状态)    │          │  conversationsStore          │   │
│  └──────────────────┘          │  (应用层状态)                 │   │
│                                │  - conversations              │   │
│                                │  - activeConversationId       │   │
│                                │  - isLoading, error           │   │
│                                └────────────┬─────────────────┘   │
└─────────────────────────────────────────────┼─────────────────────┘
                                              │
                    ┌─────────────────────────┼─────────────────────┐
                    │                         │                     │
                    │ Tauri IPC              │ Tauri IPC           │
                    ▼                         ▼                     │
┌─────────────────────────────────────────────────────────────────────┐
│                       Backend (Rust)                                │
│                                                                     │
│  ┌─────────────────────┐    ┌──────────────────────────────────┐   │
│  │   peer.rs           │    │   conversation.rs (NEW)          │   │
│  │  - get_peers()      │    │  - get_conversations()           │   │
│  │  - get_peer_stats() │    │  - get_or_create_conversation()  │   │
│  └─────────────────────┘    │  - update_conversation()         │   │
│                             │  - mark_conversation_read()      │   │
│                             │  - delete_conversation()         │   │
│                             └────────────┬─────────────────────┘   │
│                                          │                         │
│                             ┌────────────▼─────────────────────┐   │
│                             │  ConversationRepository (NEW)    │   │
│                             │  - CRUD operations               │   │
│                             │  - Unread count tracking         │   │
│                             │  - Last message update           │   │
│                             └────────────┬─────────────────────┘   │
└──────────────────────────────────────────┼─────────────────────────┘
                                           │
                                   ┌───────▼────────┐
                                   │  Sea-ORM Query │
                                   └───────┬────────┘
                                           │
┌──────────────────────────────────────────▼─────────────────────────┐
│                        SQLite Database                               │
│  ┌──────────────────┐  ┌──────────────────────┐  ┌─────────────┐  │
│  │      peers       │  │   conversations (NEW)│  │  messages   │  │
│  │  (网络节点)      │  │  conversation_        │  │  (消息)    │  │
│  │                  │  │   participants (NEW)  │  │             │  │
│  └──────────────────┘  └──────────────────────┘  └─────────────┘  │
└─────────────────────────────────────────────────────────────────────┘

关键架构变更：
1. peersStore 和 conversationsStore 完全分离
2. usePeers 仅用于网络层状态（网络状态、在线用户）
3. useConversations 专门管理应用层会话列表
4. Messaging 组件直接使用 conversationsStore，不再依赖 peersStore
```

## 实体关系图

```
┌──────────────────┐       ┌─────────────────────────┐
│     peers        │       │   conversations          │
│  ─────────────   │       │  ────────────────        │
│  id (PK)         │       │  id (PK)                 │
│  ip (unique)     │◄──────│  type ('single'|'group')  │
│  username        │       │  is_pinned               │
│  nickname        │       │  is_archived             │
│  ...             │       │  unread_count            │
└──────────────────┘       │  last_message_at         │
                           │  created_at              │
                           │  updated_at              │
                           └───────────┬─────────────┘
                                       │
                                       │ 1
                                       │
                                       │ N
┌──────────────────────────────────────┴───────────────────┐
│            conversation_participants                      │
│           ────────────────────────                        │
│  id (PK)                                                  │
│  conversation_id (FK) ──► conversations.id                │
│  peer_ip                                                  │
│  joined_at                                                │
│  left_at                                                  │
│  role                                                     │
└──────────────────────────────────────────────────────────┘
                                       │
                                       │ N (via last_message_id)
                                       │ 1
┌──────────────────────────────────────┴───────────────────┐
│                  messages                                 │
│              ──────────────                                │
│  id (PK)                                                  │
│  msg_id (unique)                                          │
│  sender_ip                                                │
│  receiver_ip                                              │
│  content                                                  │
│  sent_at                                                  │
│  ...                                                      │
└──────────────────────────────────────────────────────────┘
```
