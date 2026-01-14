# Change: 将配置信息存储到后端数据库

## Why

当前应用的配置（`AppConfig`）仅存储在内存（`AppState.config: Arc<Mutex<AppConfig>>`）中，未持久化到数据库。这导致：

1. **配置丢失**：应用重启后所有配置恢复为默认值，用户设置的昵称、端口、绑定地址等无法保存
2. **重复配置**：用户每次启动应用都需要重新配置个人信息和网络设置
3. **已存在但未集成**：`ConfigRepository` 已实现在 `src-tauri/src/config/app.rs` 中，但标记为 `#[allow(dead_code)]` 且未在 `AppState` 中使用

## What Changes

- **集成 ConfigRepository**：在 `AppState` 初始化时从数据库加载配置，配置变更时同步保存到数据库
- **应用启动时加载配置**：首次启动使用默认配置并保存到数据库，后续启动从数据库加载
- **配置变更持久化**：修改 `set_config()` 和 `update_config()` 方法，在更新内存配置的同时保存到数据库
- **错误处理**：配置加载失败时使用默认值并记录日志

## Impact

- **Affected specs**:
  - 新增：`config-persistence` - 配置持久化能力规范

- **Affected code**:
  - `feiqiu/src-tauri/src/state/app_state.rs` - 集成 `ConfigRepository`，修改初始化和配置更新逻辑
  - `feiqiu/src-tauri/src/config/app.rs` - 移除 `#[allow(dead_code)]`，使用 `ConfigRepository`
  - `feiqiu/src-tauri/src/storage/entities/settings.rs` - 确保设置表存在（已存在）
  - `feiqiu/src-tauri/src/lib.rs` - 在应用启动时初始化配置加载

- **Dependencies**:
  - `sea-orm` 和 `settings` 表已存在，无需新增依赖
  - 需要确保数据库迁移包含 `settings` 表
