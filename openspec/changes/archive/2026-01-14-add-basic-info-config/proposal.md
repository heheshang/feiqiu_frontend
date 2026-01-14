# Change: 实现基本信息配置功能

## Why

当前项目已具备后端配置管理命令（`get_config`, `set_config`）和前端配置 API，但缺少完整的基本信息配置 UI 功能。用户需要能够：

1. 查看和编辑个人信息（昵称、在线状态）
2. 配置网络设置（UDP 端口、绑定地址）
3. 保存并持久化这些配置

前端已存在 `BasicSettings.tsx` 组件但未连接到后端 API，需要完成集成以实现完整的配置功能。

## What Changes

- **前端集成**：将 `BasicSettings.tsx` 组件连接到后端配置 API
- **状态管理**：实现配置状态管理和实时更新
- **表单验证**：添加网络端口范围验证、IP 地址格式验证
- **持久化**：确保配置正确保存到后端并在应用重启后恢复
- **用户反馈**：添加保存成功/失败提示，网络配置变更提示

## Impact

- **Affected specs**:
  - 新增：`basic-info-config` - 基本信息配置能力规范
- **Affected code**:
  - `feiqiu/src/components/basic-settings/BasicSettings.tsx` - 集成配置 API
  - `feiqiu/src/lib/api/config.ts` - 已存在，无需修改
  - `feiqiu/src-tauri/src/commands/config.rs` - 已存在，无需修改
- **Dependencies**:
  - 需要添加 toast/通知库用于用户反馈（或使用 Tauri API）
