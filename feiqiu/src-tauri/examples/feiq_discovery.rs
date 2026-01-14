// NeoLan 与飞秋（FeiQ）交互示例
//
// 此示例展示如何使用 NeoLan 的网络模块与飞秋进行交互：
// - 发送上线广播（兼容飞秋/IPMsg 协议）
// - 接收飞秋用户的上线消息
// - 实时显示在线用户列表
//
// 编译运行：
// cargo run --example feiq_discovery
//
// 使用说明：
// 1. 确保飞秋在同一局域网运行
// 2. 启动此程序后会自动发现飞秋用户
// 3. 按 'l' 键显示在线用户列表
// 4. 按 'r' 键重新广播上线
// 5. 按 'q' 或 Ctrl+C 退出

use std::collections::HashMap;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 引入 NeoLan 协议模块
use feiqiu::network::protocol::{self, ProtocolMessage, msg_type};
use feiqiu::utils;

/// 飞秋用户信息
#[derive(Debug, Clone)]
struct FeiqUser {
    ip: IpAddr,
    port: u16,
    username: String,
    hostname: String,
    first_seen: std::time::Instant,
}

impl FeiqUser {
    fn new(ip: IpAddr, port: u16, username: String, hostname: String) -> Self {
        Self {
            ip,
            port,
            username,
            hostname,
            first_seen: std::time::Instant::now(),
        }
    }

    fn display_name(&self) -> String {
        format!("{}@{}", self.username, self.hostname)
    }
}

/// 创建 IPMsg 协议消息（使用 protocol.rs 中的序列化函数）
fn create_ipmsg_message(
    packet_id: u64,
    sender_name: &str,
    sender_host: &str,
    command: u32,
    extra: &str,
) -> Vec<u8> {
    let msg = ProtocolMessage {
        version: protocol::PROTOCOL_VERSION,
        packet_id,
        sender_name: sender_name.to_string(),
        sender_host: sender_host.to_string(),
        msg_type: command,
        content: extra.to_string(),
    };
    // 使用 protocol.rs 中的序列化函数
    protocol::serialize_message(&msg).unwrap_or_else(|e| {
        eprintln!("序列化消息失败: {:?}", e);
        Vec::new()
    })
}

/// 解析 IPMsg 协议消息（使用 protocol.rs 中的解析函数）
fn parse_ipmsg_message(data: &[u8]) -> Result<(u32, String, String, String), Box<dyn std::error::Error>> {
    let msg = protocol::parse_message(data)?;
    let mode = msg_type::get_mode(msg.msg_type) as u32;
    Ok((mode, msg.sender_name, msg.sender_host, msg.content))
}

/// 飞秋发现器
struct FeiqDiscovery {
    socket: UdpSocket,
    users: Arc<Mutex<HashMap<IpAddr, FeiqUser>>>,
    packet_id: u64,
    local_username: String,
    local_hostname: String,
}

impl FeiqDiscovery {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // 绑定到默认端口 2425
        let socket = UdpSocket::bind(("0.0.0.0", msg_type::IPMSG_DEFAULT_PORT))?;
        socket.set_broadcast(true)?;

        // 获取本地用户名和主机名
        let local_username = whoami::username();
        let local_hostname = whoami::fallible::hostname()
            .unwrap_or_else(|_| "localhost".to_string());

        // 获取本机 IP 地址
        let local_ip = socket.local_addr()?.ip();

        println!("📡 飞秋发现器启动");
        println!("📍 本机: {}@{}", local_username, local_hostname);
        println!("🌐 本机 IP: {}", local_ip);
        println!("🔌 绑定端口: {}", msg_type::IPMSG_DEFAULT_PORT);
        println!("📡 广播地址: {}", format!("255.255.255.255:{}", msg_type::IPMSG_DEFAULT_PORT));
        println!();

        // 测试广播发送
        println!("💡 提示：如果看不到其他用户，请检查：");
        println!("   1. 飞秋是否正在运行（端口 {}）", msg_type::IPMSG_DEFAULT_PORT);
        println!("   2. 防火墙是否允许 UDP {} 端口", msg_type::IPMSG_DEFAULT_PORT);
        println!("   3. 是否在同一局域网内");
        println!();

        Ok(Self {
            socket,
            users: Arc::new(Mutex::new(HashMap::new())),
            packet_id: 1,
            local_username,
            local_hostname,
        })
    }

    /// 发送上线广播（兼容飞秋）
    fn broadcast_online(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let message = create_ipmsg_message(
            self.packet_id,
            &self.local_username,
            &self.local_hostname,
            msg_type::IPMSG_BR_ENTRY,
            "",  // extra 字段为空
        );

        let addr = format!("255.255.255.255:{}", msg_type::IPMSG_DEFAULT_PORT).parse::<SocketAddr>()?;
        self.socket.send_to(&message, addr)?;

        println!("📢 已发送上线广播 (packet_id: {})", self.packet_id);
        self.packet_id += 1;

        Ok(())
    }

    /// 处理接收到的消息
    fn handle_message(&mut self, data: &[u8], sender: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let (command, username, hostname, extra) = parse_ipmsg_message(data)?;
        let ip = sender.ip();

        match command {
            msg_type::IPMSG_BR_ENTRY => {
                // 收到飞秋用户的上线广播
                // 检查是否是重复消息
                if self.users.lock().unwrap().contains_key(&ip) {
                    return Ok(());
                }

                // 添加到用户列表
                let user = FeiqUser::new(ip, sender.port(), username.clone(), hostname.clone());
                println!("👤 发现用户: {} ({})", user.display_name(), ip);
                println!("   主机名: {}", hostname);
                println!("   端口: {}", sender.port());

                self.users.lock().unwrap().insert(ip, user.clone());

                // 发送应答消息（与 Python RecvData.py 逻辑一致）
                self.send_answer(&user)?;

                // 显示当前用户总数
                println!();
                println!("📊 当前在线用户: {} 人", self.users.lock().unwrap().len());
                println!();
            }
            msg_type::IPMSG_ANSENTRY => {
                // 收到入场应答（与 Python RecvData.py 逻辑一致）
                if !self.users.lock().unwrap().contains_key(&ip) {
                    let user = FeiqUser::new(ip, sender.port(), username.clone(), hostname);
                    self.users.lock().unwrap().insert(ip, user);
                    println!("✅ {} 已在线 ({})", username, ip);
                }
            }
            msg_type::IPMSG_BR_EXIT => {
                // 收到下线广播（与 Python RecvData.py 逻辑一致）
                println!("👋 {} 下线 ({})", username, ip);
                self.users.lock().unwrap().remove(&ip);
                println!();
                println!("📊 当前在线用户: {} 人", self.users.lock().unwrap().len());
                println!();
            }
            msg_type::IPMSG_SENDMSG => {
                // 收到消息（与 Python RecvData.py 逻辑一致）
                println!("💬 收到消息: {} ({}) >> {}", username, ip, extra);

                // 自动回复已收到（IPMSG_RECVMSG）
                let recv_msg = create_ipmsg_message(
                    self.packet_id,
                    &self.local_username,
                    &self.local_hostname,
                    msg_type::IPMSG_RECVMSG,
                    "",
                );
                let addr = SocketAddr::new(ip, sender.port());
                self.socket.send_to(&recv_msg, addr)?;
                self.packet_id += 1;
            }
            _ => {
                // 其他消息类型
                println!("📩 收到消息 (类型: 0x{:08X}) 来自: {}", command, sender.ip());
            }
        }

        Ok(())
    }

    /// 发送入场应答
    fn send_answer(&mut self, user: &FeiqUser) -> Result<(), Box<dyn std::error::Error>> {
        let message = create_ipmsg_message(
            self.packet_id,
            &self.local_username,
            &self.local_hostname,
            msg_type::IPMSG_ANSENTRY,
            "",
        );

        let addr = SocketAddr::new(user.ip, user.port);
        self.socket.send_to(&message, addr)?;

        self.packet_id += 1;
        Ok(())
    }

    /// 显示用户列表
    fn print_users(&self) {
        let users = self.users.lock().unwrap();
        if users.is_empty() {
            println!("📭 暂无在线用户");
            return;
        }

        println!("╔═══════════════════════════════════════════════════════════════╗");
        println!("║                    在线用户列表 ({:02} 人)                      ║", users.len());
        println!("╠═══════════════════════════════════════════════════════════════╣");
        println!("║ {:<20} │ {:<15} │ {:<8} ║", "用户名", "IP 地址", "时长");
        println!("╠═══════════════════════════════════════════════════════════════╣");

        for user in users.values() {
            let duration = user.first_seen.elapsed().as_secs();
            let duration_str = if duration < 60 {
                format!("{}秒", duration)
            } else if duration < 3600 {
                format!("{}分", duration / 60)
            } else {
                format!("{}时", duration / 3600)
            };

            println!("║ {:<20} │ {:<15} │ {:<8} ║",
                user.username,
                user.ip.to_string(),
                duration_str
            );
        }

        println!("╚═══════════════════════════════════════════════════════════════╝");
    }

    /// 运行发现循环
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 65535];
        let mut last_broadcast = std::time::Instant::now();
        const BROADCAST_INTERVAL: Duration = Duration::from_secs(30);

        println!("🔍 正在搜索飞秋用户...");
        println!("💡 提示：确保飞秋在同一局域网且端口 2425 未被防火墙阻止");
        println!();

        loop {
            // 定期发送上线广播
            if last_broadcast.elapsed() >= BROADCAST_INTERVAL {
                self.broadcast_online()?;
                last_broadcast = std::time::Instant::now();
            }

            // 接收消息（非阻塞）
            self.socket.set_read_timeout(Some(Duration::from_millis(100)))?;

            match self.socket.recv_from(&mut buffer) {
                Ok((len, sender)) => {
                    if let Err(e) = self.handle_message(&buffer[..len], sender) {
                        eprintln!("处理消息错误: {:?}", e);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut => {
                    // 超时是正常的（Windows 返回 TimedOut，Unix 返回 WouldBlock）
                    // 继续循环
                }
                Err(e) => {
                    eprintln!("接收错误: {:?}", e);
                }
            }
        }
    }
}

fn main() {
        // Initialize logging system first
    utils::logger::init_logger();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║              NeoLan - 飞秋（FeiQ）交互示例                    ║");
    println!("║                                                                ║");
    println!("║  此程序展示如何与飞秋进行 IPMsg 协议通信                   ║");
    println!("║  • 自动发现局域网内的飞秋用户                                 ║");
    println!("║  • 实时显示在线用户列表                                       ║");
    println!("║  • 兼容 IPMsg 协议（飞秋、飞鸽传书等）                        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // 创建发现器
    let mut discovery = match FeiqDiscovery::new() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ 启动失败: {}", e);
            eprintln!("💡 提示：端口 2425 可能被占用，请关闭飞秋或其他 IPMsg 软件");
            return;
        }
    };

    // 发送初始上线广播
    if let Err(e) = discovery.broadcast_online() {
        eprintln!("❌ 发送广播失败: {}", e);
        return;
    }

    println!("💡 等待飞秋用户响应... (每 30 秒重新广播一次)");
    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("  命令列表:");
    println!("    l - 显示在线用户列表");
    println!("    r - 重新广播上线");
    println!("    i - 获取用户信息 (输入 IP 地址)");
    println!("    q - 退出程序");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // 设置运行标志
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    let running_clone = running.clone();

    // 设置 Ctrl+C 处理
    ctrlc::set_handler(move || {
        println!();
        println!("🛑 收到退出信号...");
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    }).expect("无法设置 Ctrl+C 处理器");

    // 在后台线程运行发现器
    let socket_clone = discovery.socket.try_clone().unwrap();
    let username_clone = discovery.local_username.clone();
    let hostname_clone = discovery.local_hostname.clone();
    let users_arc_clone = Arc::clone(&discovery.users);

    thread::spawn(move || {
        let mut buffer = [0u8; 65535];
        let mut last_broadcast = std::time::Instant::now();
        const BROADCAST_INTERVAL: Duration = Duration::from_secs(30);

        loop {
            // 检查是否应该退出
            if !running_clone.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }

            // 定期发送上线广播
            if last_broadcast.elapsed() >= BROADCAST_INTERVAL {
                let message = create_ipmsg_message(
                    1,
                    &username_clone,
                    &hostname_clone,
                    msg_type::IPMSG_BR_ENTRY,
                    "",
                );

                if let Ok(addr) = format!("255.255.255.255:{}", msg_type::IPMSG_DEFAULT_PORT).parse::<SocketAddr>() {
                    let _ = socket_clone.send_to(&message, addr);
                }

                last_broadcast = std::time::Instant::now();
            }

            // 接收消息（非阻塞）
            socket_clone.set_read_timeout(Some(Duration::from_millis(100))).unwrap();

            match socket_clone.recv_from(&mut buffer) {
                Ok((len, sender)) => {
                    // 调试：显示接收到的原始数据
                    if let Ok(msg_str) = std::str::from_utf8(&buffer[..len]) {
                        if msg_str.len() < 200 {  // 只显示较短的消息
                            println!("📨 [DEBUG] 收到数据: {} 来自: {}", msg_str, sender.ip());
                        }
                    }

                    if let Ok((command, username, hostname, extra)) = parse_ipmsg_message(&buffer[..len]) {
                        let ip = sender.ip();
                        
                        match command {
                            msg_type::IPMSG_GETINFO => {
                                // 收到获取用户信息请求，回复 SENDINFO
                                println!("ℹ️  收到用户信息请求: {} ({})", username, ip);
                                
                                // 构造用户信息回复（格式：用户名\0主机名\0其他信息）
                                let user_info = format!(
                                    "{}\0{}\0NeoLan v1.0 - Rust IPMsg Client",
                                    username_clone,
                                    hostname_clone
                                );
                                
                                let info_msg = create_ipmsg_message(
                                    1,
                                    &username_clone,
                                    &hostname_clone,
                                    msg_type::IPMSG_SENDINFO,
                                    &user_info,
                                );
                                let _ = socket_clone.send_to(&info_msg, sender);
                                println!("📤 已回复用户信息给 {} ({})", username, ip);
                            }
                            msg_type::IPMSG_BR_ENTRY => {
                                let mut users = users_arc_clone.lock().unwrap();
                                if !users.contains_key(&ip) {
                                    let user = FeiqUser::new(ip, sender.port(), username.clone(), hostname.clone());
                                    println!("👤 发现用户: {} ({})", user.display_name(), ip);
                                    println!("   主机名: {}", hostname);
                                    println!("   端口: {}", sender.port());
                                    println!();
                                    println!("📊 当前在线用户: {} 人", users.len() + 1);
                                    println!();
                                    users.insert(ip, user);

                                    // 发送入场应答（与 Python RecvData.py 一致）
                                    drop(users); // 释放锁
                                    let answer_msg = create_ipmsg_message(
                                        1,
                                        &username_clone,
                                        &hostname_clone,
                                        msg_type::IPMSG_ANSENTRY,
                                        "",
                                    );
                                    let _ = socket_clone.send_to(&answer_msg, sender);
                                }
                            }
                            msg_type::IPMSG_ANSENTRY => {
                                let mut users = users_arc_clone.lock().unwrap();
                                if !users.contains_key(&ip) {
                                    let user = FeiqUser::new(ip, sender.port(), username.clone(), hostname);
                                    println!("✅ {} 已在线 ({})", username, ip);
                                    users.insert(ip, user);
                                }
                            }
                            msg_type::IPMSG_BR_EXIT => {
                                let mut users = users_arc_clone.lock().unwrap();
                                println!("👋 {} 下线 ({})", username, ip);
                                users.remove(&ip);
                                println!();
                                println!("📊 当前在线用户: {} 人", users.len());
                                println!();
                            }
                            msg_type::IPMSG_SENDMSG => {
                                println!("💬 收到消息: {} ({}) >> {}", username, ip, extra);
                                // 自动回复已收到
                                let recv_msg = create_ipmsg_message(
                                    1,
                                    &username_clone,
                                    &hostname_clone,
                                    msg_type::IPMSG_RECVMSG,
                                    "",
                                );
                                let _ = socket_clone.send_to(&recv_msg, sender);
                            }
                            msg_type::IPMSG_SENDINFO => {
                                // 收到用户信息回复
                                println!("ℹ️  收到用户信息回复: {} ({})", username, ip);
                                // 解析用户信息（格式：用户名\0主机名\0其他信息）
                                let info_parts: Vec<&str> = extra.split('\0').collect();
                                if !info_parts.is_empty() {
                                    println!("   用户名: {}", info_parts.get(0).unwrap_or(&""));
                                    println!("   主机名: {}", info_parts.get(1).unwrap_or(&""));
                                    println!("   附加信息: {}", info_parts.get(2).unwrap_or(&""));
                                }
                            }
                            _ => {
                                println!("📩 [DEBUG] 收到其他消息类型: 0x{:08X} 来自: {}", command, sender.ip());
                            }
                        }
                    } else {
                        println!("⚠️  [DEBUG] 解析失败，原始数据: {:?}", &buffer[..len]);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut => {
                    // 超时是正常的
                }
                Err(e) => {
                    eprintln!("❌ [DEBUG] 接收错误: {:?}", e);
                }
            }
        }
    });

    // 主线程处理用户输入
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "l" => {
                println!();
                discovery.print_users();
                println!();
            }
            "r" => {
                println!();
                if let Err(e) = discovery.broadcast_online() {
                    eprintln!("❌ 广播失败: {:?}", e);
                } else {
                    println!("✅ 已重新广播上线");
                }
                println!();
            }
            "q" => {
                println!();
                println!("👋 正在退出...");
                running.store(false, std::sync::atomic::Ordering::SeqCst);
                break;
            }
            "i" => {
                // 获取用户信息
                println!();
                print!("请输入目标 IP 地址: ");
                io::stdout().flush().unwrap();
                
                let mut ip_input = String::new();
                io::stdin().read_line(&mut ip_input).unwrap();
                let ip_input = ip_input.trim();
                
                if let Ok(target_ip) = ip_input.parse::<IpAddr>() {
                    // 发送 IPMSG_GETINFO 请求
                    let getinfo_msg = create_ipmsg_message(
                        discovery.packet_id,
                        &discovery.local_username,
                        &discovery.local_hostname,
                        msg_type::IPMSG_GETINFO,
                        "",
                    );
                    
                    let target_addr = SocketAddr::new(target_ip, msg_type::IPMSG_DEFAULT_PORT);
                    match discovery.socket.send_to(&getinfo_msg, target_addr) {
                        Ok(_) => {
                            println!("📤 已发送用户信息请求到 {}", target_ip);
                            discovery.packet_id += 1;
                        }
                        Err(e) => {
                            eprintln!("❌ 发送失败: {:?}", e);
                        }
                    }
                } else {
                    println!("❌ 无效的 IP 地址: {}", ip_input);
                }
                println!();
            }
            "" => {
                // 空输入，继续
            }
            _ => {
                println!();
                println!("❓ 未知命令: {}", input);
                println!("   可用命令: l (列表), r (广播), i (获取用户信息), q (退出)");
                println!();
            }
        }
    }

    // 显示最终用户列表
    println!();
    discovery.print_users();

    println!();
    println!("👋 程序退出");
}
