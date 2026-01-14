# Change: 确保使用配置中的 UDP/TCP 端口

## Why

当前代码已经从配置中读取 UDP 端口（`lib.rs:176`），但用户报告配置的端口未生效。可能的原因：

1. **缺少日志**：无法确认配置是否正确加载
2. **缺少验证**：没有验证服务实际使用的端口是否与配置一致
3. **初始化顺序问题**：可能存在竞态条件或配置读取时机问题

需要增强端口配置的可观测性和验证，确保配置的端口真正被使用。

## What Changes

- **增强日志**：添加配置加载时的详细日志，显示实际使用的 UDP/TCP 端口
- **端口验证**：在服务初始化后记录实际绑定的端口，便于调试
- **配置显示**：在启动日志中明确显示使用的端口来源（默认 vs 数据库）
- **TCP 端口范围传递**：确保 `tcp_port_start` 和 `tcp_port_end` 传递给文件传输管理器

## Impact

- **Affected specs**:
  - 新增：`port-configuration` - 端口配置使用规范

- **Affected code**:
  - `feiqiu/src-tauri/src/lib.rs` - 增强端口配置相关日志
  - `feiqiu/src-tauri/src/modules/file_transfer/` - 传递 TCP 端口范围配置
  - `feiqiu/src-tauri/src/commands/config.rs` - 添加获取实际使用端口的命令（可选）

- **Dependencies**:
  - 依赖现有的配置持久化功能（已实现）
