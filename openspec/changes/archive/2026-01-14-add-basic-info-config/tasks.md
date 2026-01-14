# Implementation Tasks: 基本信息配置功能

## 1. 前端 API 集成
- [x] 1.1 在 `BasicSettings.tsx` 中导入 `configApi`
- [x] 1.2 添加 `useEffect` hook 加载初始配置
- [x] 1.3 将后端 `Config` 映射到组件 `User` 和 `NetworkConfig` 类型
- [x] 1.4 实现配置加载错误处理

## 2. 个人信息配置
- [x] 2.1 实现 `handleSaveUser` 调用 `setConfig` API
- [x] 2.2 添加昵称验证（非空、最大长度）
- [x] 2.3 添加在线状态变更同步到配置
- [x] 2.4 实现保存成功/失败用户反馈

## 3. 网络配置
- [x] 3.1 实现 `handleSaveNetwork` 调用 `setConfig` API
- [x] 3.2 添加 UDP 端口范围验证（1024-65535）
- [x] 3.3 添加 IP 地址格式验证
- [x] 3.4 显示网络配置变更警告（需重启服务）

## 4. 组件连接
- [x] 4.1 更新组件 Props 保留向后兼容的回调（支持父组件传入 props）
- [ ] 4.2 连接到应用路由（如需要）
- [ ] 4.3 测试配置持久化（应用重启后保持）

## 5. 测试与验证
- [ ] 5.1 测试个人信息保存和加载
- [ ] 5.2 测试网络配置保存和加载
- [ ] 5.3 测试表单验证（无效端口、IP 地址）
- [ ] 5.4 测试错误处理场景

## Dependencies

- 必须先完成后端配置命令（已完成）
- 前端配置 API 已存在（已完成）

## Parallelizable Work

- 任务 2 和 3 可并行开发（不同的配置类型）
- 任务 5 可在实现过程中同步进行

## Implementation Notes

### Completed Changes

1. **前端 API 集成** (`feiqiu/src/components/basic-settings/BasicSettings.tsx`)
   - 导入 `getConfig`, `setConfig` API
   - 添加 `useEffect` 在组件挂载时加载配置
   - 添加 `isLoading` 状态显示加载状态
   - 实现类型转换函数：`configToUser`, `configToNetworkConfig`, `userToConfig`, `networkConfigToConfig`
   - 添加 `isValidIPv4` 验证函数

2. **个人信息配置**
   - `handleSaveUser`: 验证昵称（非空、最大 50 字符），调用 `setConfig` API
   - 在线状态变更立即更新 UI 并标记为未保存状态
   - 保存成功/失败时显示 alert 提示

3. **网络配置**
   - `handleSaveNetwork`: 验证端口范围（1024-65535）和 IP 地址格式
   - 保存成功后显示警告提示（需重启服务）
   - `handleCancelNetwork`: 重新加载后端配置

4. **向后兼容**
   - 保留所有原有 props 支持父组件传入
   - 添加默认值确保组件可独立使用
