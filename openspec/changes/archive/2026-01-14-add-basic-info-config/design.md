# Design: 基本信息配置功能

## Context

当前项目状态：
- **后端**：已完成配置管理命令（`get_config`, `set_config`, `reset_config`）在 `feiqiu/src-tauri/src/commands/config.rs`
- **前端 API**：已提供类型安全的配置 API 在 `feiqiu/src/lib/api/config.ts`
- **UI 组件**：`BasicSettings.tsx` 组件已存在但使用 mock 数据和回调函数
- **数据流**：需要建立 UI → API → 后端的完整数据流

## Goals / Non-Goals

### Goals
- 实现个人信息配置（昵称、在线状态）的读取和保存
- 实现网络配置（UDP 端口、绑定地址）的读取和保存
- 提供表单验证和用户反馈
- 确保配置持久化

### Non-Goals
- 不包含头像上传功能（已存在于 UI 但后端 API 待实现）
- 不包含高级网络设置（TCP 端口范围、加密等）
- 不包含配置导入/导出功能

## Decisions

### 1. 组件状态管理策略
**选择**：在 `BasicSettings` 组件内使用 React `useState` 管理编辑状态，通过 API 直接保存到后端

**原因**：
- 配置数据量小，不需要全局状态管理
- 简单直接，避免过度设计
- 后端已提供完整的持久化机制

**替代方案**：使用 Zustand store - 被拒绝因为增加了不必要的复杂性

### 2. 配置加载时机
**选择**：组件 `useEffect` 中加载初始配置

**原因**：
- 确保组件挂载时显示最新配置
- 避免配置过时问题

### 3. 网络配置变更处理
**选择**：保存网络配置后显示警告提示，告知用户需要重启服务

**原因**：
- 后端网络服务需要重启才能应用新的端口和绑定地址
- 避免用户困惑为什么配置没有立即生效

### 4. 用户反馈机制
**选择**：使用简单的 `alert()` 和内联状态提示

**原因**：
- 快速实现，无需额外依赖
- 后续可升级为 toast 组件

**替代方案**：使用 react-hot-toast 或 Tauri dialog API - 考虑后续升级

## Data Flow

```
┌─────────────────┐
│  BasicSettings  │
│    Component    │
└────────┬────────┘
         │
         ├─ useEffect → getConfig() ──────────┐
         │                                    │
         ├─ onSaveUser → setConfig() ────────┤
         │                                    │
         └─ onSaveNetwork → setConfig() ──────┤
                                              │
         ┌────────────────────────────────────┘
         ▼
┌─────────────────┐     ┌────────────────┐
│  config.ts      │────▶│  Tauri IPC     │
│  (Frontend API) │     │  invoke()      │
└─────────────────┘     └────────┬───────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │ config.rs       │
                        │ (Rust Command)  │
                        └────────┬────────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │  AppState       │
                        │  (In-Memory +   │
                        │   File Persist) │
                        └─────────────────┘
```

## Type Mapping

### Backend (Rust) → Frontend (TypeScript)

| Backend ConfigDto | Frontend Config | UI Component |
|-------------------|-----------------|--------------|
| `username` | `username` | `editedUser.name` |
| `hostname` | `hostname` | (显示用，不可编辑) |
| `bindIp` | `bindIp` | `editedConfig.bindAddress` |
| `udpPort` | `udpPort` | `editedConfig.udpPort` |
| `status` | `status` | `editedUser.status` |

### UI Type → Backend Type Conversion

```typescript
// UI: BasicSettingsProps.user
interface User {
  name: string        // → Config.username
  status: UserStatus  // → Config.status
}

// UI: BasicSettingsProps.networkConfig
interface NetworkConfig {
  udpPort: number      // → Config.udpPort
  bindAddress: string  // → Config.bindIp
}

// Backend: ConfigDto
interface ConfigDto {
  username: string
  bindIp: string
  udpPort: number
  status?: string
  ...
}
```

## Validation Rules

### 个人信息
- **昵称**：非空，最大长度 50 字符
- **在线状态**：枚举值 `online | away | busy | offline`

### 网络配置
- **UDP 端口**：1024-65535，数字类型
- **绑定地址**：有效 IPv4 地址或 `0.0.0.0`
  - 正则：`^(\d{1,3}\.){3}\d{1,3}$` 或 `0.0.0.0`
  - 每个八位组：0-255

## Error Handling

| 错误场景 | 处理方式 |
|---------|---------|
| 配置加载失败 | 显示错误提示，使用默认值 |
| 端口格式无效 | 前端验证拦截，显示错误消息 |
| IP 地址格式无效 | 前端验证拦截，显示错误消息 |
| 后端保存失败 | 显示错误提示，保留编辑状态 |

## Migration Plan

无需迁移 - 这是新增功能。

## Open Questions

1. **Q**: 是否需要实现"恢复默认配置"功能？
   **A**: 后端已提供 `reset_config` 命令，可在后续版本添加 UI

2. **Q**: 网络配置保存后是否自动重启服务？
   **A**: 否，需要用户手动重启。自动重启可能中断正在进行的通信

3. **Q**: 头像上传是否包含在此功能中？
   **A**: 后端配置 API 已预留 `avatar` 字段，但文件上传逻辑待后续实现
