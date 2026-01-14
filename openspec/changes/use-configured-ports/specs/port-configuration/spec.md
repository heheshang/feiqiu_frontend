# 端口配置使用规范

## ADDED Requirements

### Requirement: UDP 端口配置加载与使用
系统 SHALL 从数据库加载 UDP 端口配置，并在服务初始化时使用该端口。

#### Scenario: 首次启动使用默认 UDP 端口
- **GIVEN** 数据库中没有配置（首次启动）
- **WHEN** 应用启动并初始化网络服务
- **THEN** 系统 SHALL 使用默认 UDP 端口 2425
- **AND** 记录信息日志："Using default UDP port: 2425"
- **AND** 记录信息日志："UDP receive transport bound to port 2425"

#### Scenario: 使用数据库配置的 UDP 端口
- **GIVEN** 数据库中存在 UDP 端口配置（例如 2500）
- **WHEN** 应用启动并初始化网络服务
- **THEN** 系统 SHALL 使用配置的 UDP 端口 2500
- **AND** 记录信息日志："Using configured UDP port from database: 2500"
- **AND** 记录信息日志："UDP receive transport bound to port 2500"

#### Scenario: UDP 端口配置来源标识
- **WHEN** 配置加载完成
- **THEN** 系统 SHALL 在日志中标识配置来源
- **AND** 如果使用默认值，记录："Using default UDP port: {port}"
- **AND** 如果从数据库加载，记录："Using configured UDP port from database: {port}"

### Requirement: TCP 端口范围配置
系统 SHALL 从数据库加载 TCP 端口范围配置（tcp_port_start, tcp_port_end）并传递给文件传输模块。

#### Scenario: TCP 端口范围日志
- **WHEN** 应用启动并加载配置
- **THEN** 系统 SHALL 记录 TCP 端口范围信息
- **AND** 日志格式："TCP port range configured: {start}-{end} ({count} ports available)"
- **AND** 示例："TCP port range configured: 8000-9000 (1001 ports available)"

#### Scenario: TCP 端口范围传递给文件传输模块
- **GIVEN** 配置的 TCP 端口范围为 8000-9000
- **WHEN** 创建 `FileTransferManager`
- **THEN** 系统 SHALL 传递 `tcp_port_start` 和 `tcp_port_end` 给管理器
- **AND** 记录日志："Creating FileTransferManager with TCP port range: 8000-9000"

#### Scenario: 默认 TCP 端口范围
- **GIVEN** 数据库中没有 TCP 端口配置
- **WHEN** 应用启动
- **THEN** 系统 SHALL 使用默认 TCP 端口范围 8000-9000
- **AND** 记录日志："TCP port range configured: 8000-9000 (1001 ports available)"

### Requirement: 端口配置验证
系统 SHALL 在服务启动后验证实际绑定的端口与配置一致。

#### Scenario: UDP 端口绑定验证
- **WHEN** UDP 传输绑定到端口
- **THEN** 系统 SHALL 记录实际绑定的端口号
- **AND** 日志格式："UDP receive transport bound to port {actual_port}"
- **AND** 实际端口应与配置的 UDP 端口一致

#### Scenario: 端口配置一致性检查
- **GIVEN** 配置的 UDP 端口为 2500
- **WHEN** 服务启动完成
- **THEN** 实际绑定的端口 SHALL 为 2500
- **AND** 如果不一致，记录错误日志

### Requirement: 配置加载可观测性
系统 SHALL 提供清晰的日志以便诊断端口配置问题。

#### Scenario: 配置加载过程日志
- **WHEN** 应用启动
- **THEN** 系统 SHALL 记录配置加载过程
- **AND** 记录 "Loading configuration from database..."
- **AND** 成功时记录 "Configuration loaded successfully"
- **AND** 失败时记录警告日志并说明使用默认值

#### Scenario: 端口配置摘要日志
- **WHEN** 网络服务初始化完成
- **THEN** 系统 SHALL 记录端口配置摘要
- **AND** 包含 UDP 端口和 TCP 端口范围
- **AND** 包含配置来源（数据库或默认）
