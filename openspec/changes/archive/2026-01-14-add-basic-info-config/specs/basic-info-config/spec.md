# 基本信息配置功能规范

## ADDED Requirements

### Requirement: 配置加载
系统 SHALL 在组件挂载时从后端加载当前配置并显示在用户界面。

#### Scenario: 首次加载配置
- **WHEN** 用户打开基础设置页面
- **THEN** 系统 SHALL 通过 `getConfig()` API 加载当前配置
- **AND** 显示当前昵称、在线状态、UDP 端口和绑定地址
- **AND** 如果加载失败，显示错误提示并使用默认值

#### Scenario: 配置持久化
- **WHEN** 用户关闭并重新打开应用
- **THEN** 系统 SHALL 显示之前保存的配置值

### Requirement: 个人信息编辑
系统 SHALL 允许用户编辑个人信息（昵称、在线状态）并保存到后端。

#### Scenario: 保存昵称变更
- **WHEN** 用户修改昵称并点击"保存更改"
- **AND** 昵称非空且长度不超过 50 字符
- **THEN** 系统 SHALL 通过 `setConfig()` API 保存配置
- **AND** 显示"保存成功"提示
- **AND** 清除未保存更改状态

#### Scenario: 昵称验证失败
- **WHEN** 用户输入空昵称或超过 50 字符
- **THEN** 系统 SHALL 显示验证错误提示
- **AND** 禁用保存按钮或阻止保存操作

#### Scenario: 变更在线状态
- **WHEN** 用户点击在线状态按钮（在线/离开/忙碌/离线）
- **THEN** 系统 SHALL 立即更新 UI 显示
- **AND** 通过 `setConfig()` API 保存新状态

#### Scenario: 保存失败处理
- **WHEN** 保存操作因后端错误失败
- **THEN** 系统 SHALL 显示错误提示
- **AND** 保留用户编辑内容（不清除表单）

### Requirement: 网络配置编辑
系统 SHALL 允许用户编辑网络配置（UDP 端口、绑定地址）并保存到后端。

#### Scenario: 保存网络配置
- **WHEN** 用户修改 UDP 端口或绑定地址并点击"保存配置"
- **AND** UDP 端口在 1024-65535 范围内
- **AND** 绑定地址是有效的 IPv4 地址或 0.0.0.0
- **THEN** 系统 SHALL 通过 `setConfig()` API 保存配置
- **AND** 显示"保存成功"提示
- **AND** 显示警告："网络配置更改需要重启服务才能生效"

#### Scenario: UDP 端口验证
- **WHEN** 用户输入的 UDP 端口小于 1024 或大于 65535
- **THEN** 系统 SHALL 显示错误提示："UDP 端口必须在 1024-65535 之间"
- **AND** 阻止保存操作

#### Scenario: IP 地址格式验证
- **WHEN** 用户输入的绑定地址不符合 IPv4 格式
- **THEN** 系统 SHALL 显示错误提示："请输入有效的 IP 地址"
- **AND** 阻止保存操作

#### Scenario: 取消网络配置更改
- **WHEN** 用户点击"取消"按钮
- **THEN** 系统 SHALL 恢复为原始配置值
- **AND** 清除未保存更改状态

### Requirement: 配置类型转换
系统 SHALL 正确转换前端 UI 类型和后端配置类型。

#### Scenario: 用户类型映射
- **GIVEN** 后端 `Config` 包含 `username` 和 `status`
- **WHEN** 加载配置到 UI
- **THEN** `Config.username` 映射到 `User.name`
- **AND** `Config.status` 映射到 `User.status`（类型：online | away | busy | offline）

#### Scenario: 网络配置映射
- **GIVEN** 后端 `Config` 包含 `bindIp` 和 `udpPort`
- **WHEN** 加载配置到 UI
- **THEN** `Config.bindIp` 映射到 `NetworkConfig.bindAddress`
- **AND** `Config.udpPort` 映射到 `NetworkConfig.udpPort`

### Requirement: 用户反馈
系统 SHALL 在配置操作期间提供适当的用户反馈。

#### Scenario: 保存成功反馈
- **WHEN** 配置成功保存到后端
- **THEN** 系统 SHALL 显示成功提示
- **AND** 2 秒后自动隐藏提示

#### Scenario: 加载错误反馈
- **WHEN** 配置加载失败
- **THEN** 系统 SHALL 显示错误提示："加载配置失败，请重试"
- **AND** 使用默认值填充表单

#### Scenario: 网络配置警告
- **WHEN** 用户保存网络配置更改
- **THEN** 系统 SHALL 显示警告提示框
- **AND** 提示内容："保存网络配置后，飞秋服务需要重启才能使更改生效"
