# Implementation Tasks: 确保使用配置中的 UDP/TCP 端口

## 1. 增强端口配置日志
- [x] 1.1 在 `lib.rs` 中添加 UDP 端口来源日志（"from database" 或 "using defaults"）
- [x] 1.2 添加 UDP 端口实际绑定日志
- [x] 1.3 添加 TCP 端口范围配置日志
- [x] 1.4 在 `init_config()` 中添加配置来源标识日志

## 2. TCP 端口范围传递
- [x] 2.1 在 `FileTransferManager` 添加 `tcp_port_start` 和 `tcp_port_end` 字段
- [x] 2.2 修改 `FileTransferManager::new()` 接受 TCP 端口范围参数
- [x] 2.3 在 `lib.rs` 中传递 TCP 端口范围给 `FileTransferManager`
- [x] 2.4 添加 TCP 端口范围使用日志

## 3. 配置验证（可选）
- [ ] 3.1 添加 `get_active_ports` 命令返回实际使用的端口
- [ ] 3.2 添加端口配置验证函数
- [ ] 3.3 在前端显示当前端口配置

## 4. 测试与验证
- [ ] 4.1 测试首次启动（使用默认端口）的日志输出
- [ ] 4.2 测试配置端口后的日志输出
- [ ] 4.3 验证实际绑定的端口与配置一致
- [ ] 4.4 测试端口配置持久化（重启后保持）

## Dependencies

- 必须先完成配置持久化功能（已完成）
- `ConfigRepository` 已集成（已完成）

## Parallelizable Work

- 任务 1 和 2 可以并行开发
- 任务 3 可以在实现过程中同步进行

## Implementation Notes

### 关键文件
- `feiqiu/src-tauri/src/lib.rs` - 主要修改文件，添加日志和传递 TCP 端口
- `feiqiu/src-tauri/src/modules/file_transfer/manager.rs` - 添加 TCP 端口范围字段
- `feiqiu/src-tauri/src/modules/file_transfer/response.rs` - 修复测试代码

### 已完成的修改

1. **UDP 端口日志增强** (`feiqiu/src-tauri/src/lib.rs`)
   - 添加 UDP 端口来源判断日志：
     - 默认端口（2425）："Using default UDP port: 2425"
     - 数据库配置："Using configured UDP port from database: {port}"
   - 添加 UDP 绑定成功日志："UDP receive transport bound to port {port}"

2. **TCP 端口范围日志** (`feiqiu/src-tauri/src/lib.rs`)
   - 添加 TCP 端口范围配置日志：
     - "TCP port range configured: {start}-{end} ({count} ports available)"

3. **FileTransferManager 结构更新** (`feiqiu/src-tauri/src/modules/file_transfer/manager.rs`)
   - 添加 `tcp_port_start: u16` 和 `tcp_port_end: u16` 字段
   - 更新 `new()` 构造函数接受 TCP 端口范围参数
   - 添加 `get_next_tcp_port()` 方法返回下一个可用端口
   - 添加创建日志："Creating FileTransferManager for user: {user} with TCP port range: {start}-{end}"

4. **测试代码修复**
   - 更新所有测试代码中的 `FileTransferManager::new()` 调用
   - 传递默认 TCP 端口范围 (8000-9000)

### 注意事项

- **FileTransferManager 尚未在运行时代码中实例化**：当前检查显示 `FileTransferManager` 仅在测试代码中使用，运行时代码中尚未直接实例化。当运行时实现时，需要从 `config` 获取 TCP 端口范围并传递给管理器。

### 预期日志输出

**使用默认端口**：
```
INFO Loading configuration from database...
INFO Configuration loaded successfully
INFO Using default UDP port: 2425
INFO TCP port range configured: 8000-9000 (1001 ports available)
INFO UDP receive transport bound to port 2425
```

**使用数据库配置（非默认端口）**：
```
INFO Loading configuration from database...
INFO Configuration loaded successfully
INFO Using configured UDP port from database: 2500
INFO TCP port range configured: 8000-9000 (1001 ports available)
INFO UDP receive transport bound to port 2500
```
