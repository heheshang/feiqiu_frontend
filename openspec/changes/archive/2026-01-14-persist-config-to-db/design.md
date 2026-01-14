# Design: 配置信息持久化到数据库

## Context

### 当前状态
- **内存配置**：`AppState` 中的 `config: Arc<Mutex<AppConfig>>` 仅在内存中存储配置
- **未持久化**：应用重启后配置丢失，恢复为默认值
- **已存在的仓储**：`ConfigRepository` 已实现在 `src-tauri/src/config/app.rs`，提供：
  - `load_app_config()` - 从 `settings` 表加载配置
  - `save_app_config()` - 保存配置到 `settings` 表
  - `reset_to_default()` - 重置为默认配置
  - 但被标记为 `#[allow(dead_code)]` 且未使用

### 数据库表结构
`settings` 表已存在（`storage/entities/settings.rs`）：
```rust
pub struct Model {
    pub id: i32,
    pub key: String,      // 配置键名（如 "app_config"）
    pub value: String,    // JSON 序列化的配置值
    pub updated_at: DateTime,
}
```

## Goals / Non-Goals

### Goals
- 配置在应用启动时从数据库自动加载
- 配置变更时自动保存到数据库
- 首次启动使用默认配置并保存
- 配置加载失败时降级为默认配置

### Non-Goals
- 配置版本控制或迁移（假设配置结构向后兼容）
- 多用户配置（单用户桌面应用）
- 配置导入/导出功能

## Decisions

### 1. ConfigRepository 集成方式
**选择**：在 `AppState` 中添加 `ConfigRepository` 实例，在数据库初始化后加载配置

**原因**：
- `ConfigRepository` 已存在且实现完整，直接复用
- 在 `init_database()` 后立即加载配置，确保数据库连接可用
- 使用 `Arc<Mutex<>>` 包装以支持并发访问

**实现位置**：
```rust
pub struct AppState {
    // 新增字段
    config_repo: Arc<Mutex<Option<ConfigRepository>>>,
    // 现有字段保持不变
    config: Arc<Mutex<AppConfig>>,
    // ...
}
```

### 2. 配置加载时机
**选择**：在 `init_database()` 方法返回前加载配置

**原因**：
- 数据库连接已建立，`ConfigRepository` 可用
- 配置在应用早期阶段可用，后续模块可依赖
- 避免异步初始化问题

### 3. 配置保存时机
**选择**：修改 `set_config()` 和 `update_config()` 方法，每次配置变更都保存

**原因**：
- 简单直接，确保配置不丢失
- 配置变更频率低，性能影响可忽略
- 用户期望配置立即保存

### 4. 错误处理策略
**选择**：配置加载失败时使用默认配置，记录警告日志

**原因**：
- 应用应能正常启动，即使配置损坏或缺失
- 用户可以重新配置
- 避免因配置问题导致应用无法使用

## Data Flow

### 启动时加载配置
```
┌─────────────────────────────────────────────────────────┐
│                    Application Start                     │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              AppState.init_database()                    │
│  - 建立 SQLite 连接                                     │
│  - 运行数据库迁移                                        │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│        ConfigRepository::load_app_config()              │
│  - 查询 settings 表 WHERE key = "app_config"           │
│  - 如果存在：解析 JSON 并返回                           │
│  - 如果不存在：返回 Ok(AppConfig::default())           │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              AppState.config = loaded_config            │
│  - 存储到内存                                           │
│  - 应用启动完成                                         │
└─────────────────────────────────────────────────────────┘
```

### 配置变更保存
```
┌─────────────────────────────────────────────────────────┐
│            Frontend calls set_config()                  │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              AppState.set_config()                       │
│  - 验证配置（validate()）                               │
│  - 更新内存配置                                         │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│        ConfigRepository::save_app_config()              │
│  - 序列化配置为 JSON                                    │
│  - UPSERT 到 settings 表                                │
│  - key = "app_config", value = JSON                     │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                   配置已持久化                          │
└─────────────────────────────────────────────────────────┘
```

## Type Mapping

### Rust Struct ↔ JSON (stored in database)

```rust
// AppConfig struct (in memory)
pub struct AppConfig {
    pub username: String,
    pub hostname: String,
    pub bind_ip: String,
    pub udp_port: u16,
    pub tcp_port_start: u16,
    pub tcp_port_end: u16,
    pub heartbeat_interval: u64,
    pub peer_timeout: u64,
    pub encryption_enabled: bool,
    pub encryption_key: Option<String>,
    pub offline_message_retention_days: u32,
    pub auto_accept_files: bool,
    pub file_save_dir: String,
    pub log_level: String,
}

// Serialized to JSON (stored in settings.value)
{
  "username": "Alice",
  "hostname": "DESKTOP-ABC",
  "bind_ip": "0.0.0.0",
  "udp_port": 2425,
  "tcp_port_start": 8000,
  "tcp_port_end": 9000,
  "heartbeat_interval": 60,
  "peer_timeout": 180,
  "encryption_enabled": false,
  "encryption_key": null,
  "offline_message_retention_days": 30,
  "auto_accept_files": false,
  "file_save_dir": "C:\\Users\\Alice\\Downloads",
  "log_level": "info"
}
```

## Implementation Details

### AppState 新增方法

```rust
impl AppState {
    /// 初始化配置（在数据库初始化后调用）
    async fn init_config(&self) -> Result<()> {
        if let Some(repo) = self.get_config_repo() {
            let config = repo.load_app_config().await
                .unwrap_or_else(|e| {
                    tracing::warn!("Failed to load config from DB: {}, using defaults", e);
                    AppConfig::default()
                });
            *self.config.lock().unwrap() = config;
        }
        Ok(())
    }

    /// 获取配置仓储
    fn get_config_repo(&self) -> Option<ConfigRepository> {
        self.config_repo.lock().unwrap().as_ref().cloned()
    }

    /// 保存配置（在配置变更时调用）
    async fn persist_config(&self, config: &AppConfig) -> Result<()> {
        if let Some(repo) = self.get_config_repo() {
            repo.save_app_config(config).await?;
        }
        Ok(())
    }
}
```

### 修改现有方法

```rust
impl AppState {
    /// Set the configuration（修改为持久化）
    pub fn set_config(&self, config: AppConfig) {
        // 先保存到数据库
        let repo = self.get_config_repo();
        let config_clone = config.clone();

        // 在后台线程中保存，避免阻塞
        if let Some(repo) = repo {
            tokio::spawn(async move {
                if let Err(e) = repo.save_app_config(&config_clone).await {
                    tracing::error!("Failed to save config: {}", e);
                }
            });
        }

        // 更新内存配置
        *self.config.lock().unwrap() = config;
        self.emit_event(super::events::AppEvent::ConfigChanged);
    }

    /// Update configuration fields（修改为持久化）
    pub fn update_config<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.config.lock().unwrap();
        updater(&mut config);
        let config_clone = config.clone();
        drop(config);

        // 保存到数据库
        let repo = self.get_config_repo();
        if let Some(repo) = repo {
            let repo_clone = repo.clone();
            tokio::spawn(async move {
                if let Err(e) = repo_clone.save_app_config(&config_clone).await {
                    tracing::error!("Failed to save config: {}", e);
                }
            });
        }

        self.emit_event(super::events::AppEvent::ConfigChanged);
        Ok(())
    }
}
```

## Migration Plan

### 阶段 1：集成 ConfigRepository
1. 在 `AppState` 添加 `config_repo` 字段
2. 在 `init_database()` 中初始化 `ConfigRepository`
3. 添加 `get_config_repo()` 辅助方法

### 阶段 2：配置加载
1. 添加 `init_config()` 方法
2. 在 `lib.rs` 的 `setup()` 函数中调用 `init_config()`
3. 添加错误处理和降级逻辑

### 阶段 3：配置保存
1. 修改 `set_config()` 方法
2. 修改 `update_config()` 方法
3. 添加保存失败的日志记录

### 阶段 4：测试
1. 首次启动验证（使用默认配置）
2. 配置修改验证（修改后重启检查是否持久化）
3. 配置损坏恢复验证（手动损坏 JSON 后检查降级行为）

## Risks / Trade-offs

| 风险 | 缓解措施 |
|------|----------|
| 数据库初始化失败导致无法加载配置 | 捕获错误，使用默认配置，记录警告日志 |
| 配置保存失败导致配置不同步 | 异步保存，记录错误，下次启动时使用内存配置 |
| JSON 序列化/反序列化失败 | 捕获 serde_json 错误，降级为默认配置 |
| 并发配置更新冲突 | 使用 `Arc<Mutex<>>` 保护配置，单线程写入 |

## Open Questions

1. **Q**: 配置保存是否应该同步还是异步？
   **A**: 异步保存，避免阻塞 UI。失败时记录日志但不影响用户体验。

2. **Q**: 是否需要配置备份/回滚功能？
   **A**: 不在此次实现范围内。如需要可后续添加 `settings` 表的历史记录功能。

3. **Q**: 配置字段变更时如何处理迁移？
   **A**: 使用 `serde_json` 的 `#[serde(default)]` 特性确保新字段有默认值，旧配置仍可加载。
