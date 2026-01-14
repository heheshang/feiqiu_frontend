# Implementation Tasks: 配置信息持久化到数据库

## 1. ConfigRepository 集成
- [x] 1.1 在 `AppState` 添加 `config_repo: Arc<Mutex<Option<ConfigRepository>>>` 字段
- [x] 1.2 在 `init_database()` 中初始化 `ConfigRepository` 实例
- [x] 1.3 添加 `get_config_repo()` 辅助方法
- [x] 1.4 从 `config/app.rs` 移除 `ConfigRepository` 的 `#[allow(dead_code)]` 标记

## 2. 配置加载
- [x] 2.1 在 `AppState` 添加 `init_config()` 方法
- [x] 2.2 实现配置加载失败时的降级逻辑（使用默认配置）
- [x] 2.3 在 `lib.rs` 的应用启动流程中调用 `init_config()`
- [x] 2.4 添加配置加载的日志记录

## 3. 配置保存
- [x] 3.1 修改 `set_config()` 方法，添加配置持久化逻辑
- [x] 3.2 修改 `update_config()` 方法，添加配置持久化逻辑
- [x] 3.3 实现异步保存以避免阻塞
- [x] 3.4 添加保存失败的错误处理和日志记录

## 4. 测试与验证
- [ ] 4.1 测试首次启动（无数据库配置）使用默认值
- [ ] 4.2 测试配置修改后重启应用，验证配置持久化
- [ ] 4.3 测试数据库连接失败时的降级行为
- [ ] 4.4 测试配置 JSON 损坏时的恢复能力
- [ ] 4.5 验证前端配置 UI 修改后正确保存

## Dependencies

- 必须先完成数据库初始化（已完成）
- `settings` 表已存在（已完成）
- `ConfigRepository` 已实现（已完成，待集成）

## Parallelizable Work

- 任务 1 和 2 可以并行开发（添加字段和加载逻辑）
- 任务 4 可以在实现过程中同步进行

## Implementation Notes

### 关键文件
- `feiqiu/src-tauri/src/state/app_state.rs` - 主要修改文件
- `feiqiu/src-tauri/src/config/app.rs` - 移除 dead_code 标记
- `feiqiu/src-tauri/src/lib.rs` - 添加 init_config 调用
- `feiqiu/src/hooks/useConfig.ts` - 更新 `updateConfig` 返回类型

### 数据库表
- `settings` 表已存在，无需创建

### 已完成的修改

1. **AppState 修改** (`feiqiu/src-tauri/src/state/app_state.rs`)
   - 添加 `config_repo: Arc<Mutex<Option<ConfigRepository>>>` 字段
   - 在 `init_database()` 中初始化 `ConfigRepository`
   - 添加 `get_config_repo()` 方法
   - 添加 `init_config()` 方法，加载配置并处理错误
   - 修改 `set_config()` 和 `update_config()` 方法，异步保存配置

2. **ConfigRepository 激活** (`feiqiu/src-tauri/src/config/app.rs`)
   - 移除 `ConfigRepository` 及其方法的 `#[allow(dead_code)]` 标记

3. **应用启动集成** (`feiqiu/src-tauri/src/lib.rs`)
   - 在数据库迁移完成后调用 `init_config()`
   - 配置加载完成后用于初始化 PeerManager

4. **前端适配** (`feiqiu/src/hooks/useConfig.ts`)
   - 更新 `updateConfig` 方法返回类型为 `Promise<void>`
   - 更新 `UseConfigResult` 接口

### 测试场景
1. **首次启动**：删除数据库文件，启动应用，验证使用默认配置
2. **配置持久化**：修改昵称/端口，重启应用，验证配置保留
3. **错误恢复**：手动损坏 settings.value JSON，启动应用，验证使用默认配置
