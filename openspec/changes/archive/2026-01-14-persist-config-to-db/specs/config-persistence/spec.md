# 配置持久化功能规范

## ADDED Requirements

### Requirement: 配置初始化加载
系统 SHALL 在应用启动时从数据库加载配置，如果配置不存在则使用默认配置并保存到数据库。

#### Scenario: 首次启动加载默认配置
- **WHEN** 应用首次启动且数据库中没有配置
- **THEN** 系统 SHALL 使用 `AppConfig::default()` 作为初始配置
- **AND** 将默认配置保存到 `settings` 表（key = "app_config"）
- **AND** 记录信息日志："使用默认配置初始化"

#### Scenario: 后续启动加载已保存配置
- **WHEN** 应用启动且数据库中已存在配置
- **THEN** 系统 SHALL 从 `settings` 表加载配置（WHERE key = "app_config"）
- **AND** 解析 JSON 配置值到 `AppConfig` 结构
- **AND** 将加载的配置存储到 `AppState.config`

#### Scenario: 配置加载失败降级
- **WHEN** 配置加载失败（数据库错误、JSON 解析错误等）
- **THEN** 系统 SHALL 使用 `AppConfig::default()` 作为降级配置
- **AND** 记录警告日志："配置加载失败，使用默认值: {错误原因}"
- **AND** 应用正常启动，不阻塞

### Requirement: 配置变更持久化
系统 SHALL 在配置变更时自动保存配置到数据库，确保配置不丢失。

#### Scenario: 完整配置变更保存
- **WHEN** 调用 `set_config(new_config)` 更新完整配置
- **THEN** 系统 SHALL 验证新配置（`validate()`）
- **AND** 将配置序列化为 JSON
- **AND** UPSERT 到 `settings` 表（key = "app_config", value = JSON）
- **AND** 更新 `updated_at` 时间戳
- **AND** 异步执行保存操作，不阻塞调用线程

#### Scenario: 部分配置字段更新保存
- **WHEN** 调用 `update_config(|c| c.field = value)` 更新部分字段
- **THEN** 系统 SHALL 在内存中更新配置字段
- **AND** 将完整配置保存到数据库
- **AND** 异步执行保存操作
- **AND** 发送 `ConfigChanged` 事件

#### Scenario: 配置保存失败处理
- **WHEN** 配置保存到数据库失败
- **THEN** 系统 SHALL 记录错误日志："配置保存失败: {错误原因}"
- **AND** 内存配置保持不变
- **AND** 不影响用户界面操作
- **AND** 下次启动时使用内存中的配置

### Requirement: 配置存储格式
系统 SHALL 使用 JSON 格式将配置存储在数据库 `settings` 表中。

#### Scenario: 配置序列化为 JSON
- **GIVEN** `AppConfig` 包含所有配置字段
- **WHEN** 保存配置到数据库
- **THEN** 配置 SHALL 序列化为 JSON 字符串
- **AND** JSON 包含所有非默认值字段
- **AND** 布尔值、数字、字符串、可选值正确序列化

#### Scenario: 配置从 JSON 反序列化
- **GIVEN** 数据库 `settings` 表存储 JSON 配置
- **WHEN** 加载配置
- **THEN** JSON SHALL 反序列化为 `AppConfig` 结构
- **AND** 所有字段正确映射
- **AND** 缺失字段使用 serde `#[serde(default)]` 特性的默认值

#### Scenario: 存储 KV 结构
- **GIVEN** 配置存储在 `settings` 表
- **WHEN** 查询配置
- **THEN** 使用 `key = "app_config"` 查询
- **AND** `value` 字段包含 JSON 配置字符串
- **AND** `updated_at` 反映最后更新时间

### Requirement: 配置仓库访问
系统 SHALL 通过 `ConfigRepository` 提供线程安全的配置存储访问。

#### Scenario: 获取配置仓储实例
- **GIVEN** 数据库已初始化
- **WHEN** 调用 `get_config_repo()`
- **THEN** 返回 `Some(ConfigRepository)` 实例
- **AND** 实例可跨线程共享（`Arc<Mutex<>>` 包装）

#### Scenario: 数据库未初始化时访问
- **GIVEN** 数据库未初始化
- **WHEN** 调用 `get_config_repo()`
- **THEN** 返回 `None`
- **AND** 配置操作跳过数据库访问

#### Scenario: 并发配置访问
- **GIVEN** 多个线程同时访问配置
- **WHEN** 一个线程保存配置，其他线程读取配置
- **THEN** 操作 SHALL 线程安全
- **AND** 不会导致数据竞争或死锁
