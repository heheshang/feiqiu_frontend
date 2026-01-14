# Change: 添加通讯录（联系人）功能

## Why

当前飞秋应用的"通讯录"功能尚未实现，用户无法：
- 查看所有联系人（包括在线和历史记录）
- 组织联系人到自定义分组
- 添加手动联系人条目
- 设置联系人备注和收藏
- 搜索和过滤联系人
- 批量操作联系人

现有的 `peers` 系统仅跟踪当前可见的局域网对等方，缺乏持久化的联系人管理能力。

## What Changes

- **新增通讯录功能**：实现完整的联系人管理系统
- **联系人列表管理** (`contacts-list`)：显示所有联系人（在线+历史），实时状态更新
- **联系人分组** (`contact-groups`)：创建自定义分组，将联系人添加/移除分组
- **联系人 CRUD 操作** (`contact-crud`)：添加、编辑、删除联系人记录
- **联系人搜索过滤** (`contact-search`)：按姓名、部门、职位、拼音、IP 搜索
- **批量操作** (`contact-batch`)：多选界面，批量删除、移动分组、导出

- **后端支持**：
  - 新增数据库表：`contacts`、`contact_groups`、`contact_group_members`
  - 新增 IPC 命令：`get_contacts`、`create_contact`、`update_contact`、`delete_contact`

- **前端 UI**：
  - 新增主导航标签："通讯录"
  - 新增组件：`src/components/contacts/`
  - 新增类型：`src/lib/types/contacts.ts`

## Impact

- **Affected specs**:
  - 新增：`contacts-list` - 联系人列表管理规范
  - 新增：`contact-groups` - 联系人分组规范
  - 新增：`contact-crud` - 联系人 CRUD 操作规范
  - 新增：`contact-search` - 联系人搜索过滤规范
  - 新增：`contact-batch` - 批量操作规范

- **Affected code**:
  - `feiqiu/src/components/contacts/` - 新增通讯录组件
  - `feiqiu/src/lib/types/contacts.ts` - 新增联系人类型
  - `feiqiu/src-tauri/src/commands/contacts.rs` - 新增联系人命令
  - `feiqiu/src-tauri/src/storage/entities/contacts.rs` - 新增联系人实体
  - `feiqiu/src-tauri/src/storage/contact_repo.rs` - 新增联系人仓储

- **Dependencies**:
  - 依赖现有的 `peers` 系统
  - 需要数据库迁移支持新表
