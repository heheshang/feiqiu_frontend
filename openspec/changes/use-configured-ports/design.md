# Design: 确保使用配置中的 UDP/TCP 端口

## Context

### 当前实现状态

**UDP 端口**（已在 `lib.rs` 中使用配置）：
```rust
// lib.rs:175-176
let config = app_state_for_setup.get_config();
let udp_port = config.udp_port;  // ✅ 从配置读取

// lib.rs:179
let udp_recv = match UdpTransport::bind_with_retry(udp_port, 10) {
```

**TCP 端口范围**（定义在配置中，但未使用）：
```rust
// config/app.rs
pub struct AppConfig {
    pub tcp_port_start: u16,  // 默认 8000
    pub tcp_port_end: u16,    // 默认 9000
}
```

### 潜在问题

1. **缺少日志验证**：无法确认配置是否正确加载
2. **TCP 端口未传递**：文件传输模块可能未使用配置的端口范围
3. **配置来源不明确**：日志中无法区分使用的是默认值还是数据库配置

## Goals / Non-Goals

### Goals
- 添加详细的端口配置日志
- 验证服务实际使用的端口与配置一致
- 确保 TCP 端口范围传递给文件传输模块
- 便于调试和诊断端口配置问题

### Non-Goals
- 修改端口配置的存储方式
- 修改端口绑定逻辑
- 实现端口热重载（需要重启服务）

## Decisions

### 1. 日志增强
**选择**：在关键位置添加日志显示实际使用的端口

**原因**：
- 便于用户和开发者诊断问题
- 无需修改核心逻辑
- 零性能影响

### 2. TCP 端口范围传递
**选择**：将 `tcp_port_start` 和 `tcp_port_end` 传递给 `FileTransferManager`

**原因**：
- 文传传输模块需要使用配置的端口范围
- 当前可能使用硬编码的默认值

### 3. 配置来源标识
**选择**：在日志中标识配置来源（"from database" 或 "using defaults"）

**原因**：
- 清楚显示配置是否成功从数据库加载
- 便于诊断配置加载问题

## Implementation Details

### 1. 增强端口配置日志

**位置**：`lib.rs` 在初始化 PeerManager 后

```rust
// Get UDP port from config (now loaded from database)
let config = app_state_for_setup.get_config();
let udp_port = config.udp_port;

// NEW: Log the port source
if udp_port == AppConfig::DEFAULT_UDP_PORT {
    tracing::info!("Using default UDP port: {}", udp_port);
} else {
    tracing::info!("Using configured UDP port from database: {}", udp_port);
}

// Bind UDP transport for receiving (PeerManager) with retry
let udp_recv = match UdpTransport::bind_with_retry(udp_port, 10) {
    Ok(u) => {
        // NEW: Log actual bound port
        tracing::info!("UDP receive transport bound to port {}", u.port());
        u
    }
    Err(e) => {
        tracing::error!("Failed to bind UDP transport after retries: {}", e);
        return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>);
    }
};
```

### 2. TCP 端口范围日志

**位置**：`lib.rs` 在初始化后

```rust
// NEW: Log TCP port range configuration
tracing::info!(
    "TCP port range configured: {}-{} ({} ports available)",
    config.tcp_port_start,
    config.tcp_port_end,
    config.tcp_port_end - config.tcp_port_start + 1
);
```

### 3. 传递 TCP 端口给文件传输模块

**修改 `FileTransferManager`**：

```rust
pub struct FileTransferManager {
    udp: Arc<UdpTransport>,
    tasks: Arc<Mutex<Vec<TransferTask>>>,
    username: String,
    hostname: String,
    // NEW: TCP port range for file transfers
    tcp_port_start: u16,
    tcp_port_end: u16,
}

impl FileTransferManager {
    pub fn new(
        udp: Arc<UdpTransport>,
        username: String,
        hostname: String,
        tcp_port_start: u16,
        tcp_port_end: u16,
    ) -> Self {
        tracing::info!("Creating FileTransferManager with TCP port range: {}-{}",
            tcp_port_start, tcp_port_end);
        // ...
    }

    // NEW: Get next available TCP port
    pub fn get_next_tcp_port(&self) -> Result<u16> {
        // Find available port in range
        Ok(self.tcp_port_start)
    }
}
```

### 4. 添加配置验证命令（可选）

```rust
#[tauri::command]
fn get_active_ports(state: tauri::State<AppState>) -> Result<ActivePortsDto> {
    let config = state.get_config();
    Ok(ActivePortsDto {
        udp_port: config.udp_port,
        tcp_port_start: config.tcp_port_start,
        tcp_port_end: config.tcp_port_end,
        source: "database", // or "default"
    })
}
```

## Data Flow

```
┌─────────────────────────────────────────────────────────┐
│                  Application Startup                      │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              init_database()                            │
│  - Establish SQLite connection                          │
│  - Initialize ConfigRepository                         │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              init_config()                              │
│  - Load config from database                           │
│  - Log: "Config loaded from database" or "Using defaults"│
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              get_config()                               │
│  - Returns loaded config (with correct ports)           │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│         Extract and log port configuration               │
│  - Log: "Using configured UDP port: 2425"              │
│  - Log: "TCP port range configured: 8000-9000"          │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│         Initialize services with loaded ports            │
│  - UdpTransport::bind_with_retry(config.udp_port)      │
│  - Log: "UDP receive transport bound to port 2425"      │
│  - FileTransferManager::new(..., tcp_port_start, ...)   │
└─────────────────────────────────────────────────────────┘
```

## Testing Strategy

### 测试场景

1. **默认端口验证**
   - 删除数据库文件
   - 启动应用
   - 检查日志显示 "Using default UDP port: 2425"

2. **配置端口验证**
   - 修改 UDP 端口为 2500
   - 重启应用
   - 检查日志显示 "Using configured UDP port from database: 2500"

3. **端口绑定验证**
   - 检查日志 "UDP receive transport bound to port X"
   - 验证实际绑定端口与配置一致

### 日志输出示例

**首次启动（无数据库配置）**：
```
INFO Loading configuration from database...
WARN Config not found in database, using defaults
INFO Using default UDP port: 2425
INFO TCP port range configured: 8000-9000 (1001 ports available)
INFO UDP receive transport bound to port 2425
```

**后续启动（有数据库配置）**：
```
INFO Loading configuration from database...
INFO Configuration loaded successfully
INFO Using configured UDP port from database: 2500
INFO TCP port range configured: 8000-9000 (1001 ports available)
INFO UDP receive transport bound to port 2500
```

## Open Questions

1. **Q**: 是否需要实现端口热重载（不重启服务）？
   **A**: 不在此次实现范围内。当前需要重启应用才能使端口配置生效。

2. **Q**: 如果端口绑定失败是否应该回退到默认端口？
   **A**: 当前实现会直接失败。可以考虑在后续版本中添加回退逻辑。

3. **Q**: TCP 端口范围如何与实际文件传输关联？
   **A**: 文件传输模块需要在配置的端口范围内选择可用端口。具体实现需要在文件传输逻辑中使用。
