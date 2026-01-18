// IPMsg-compatible protocol parser and serializer
//
// Protocol format:
// version:packet_id:sender_name:sender_host:msg_type:content[:ext_fields]
//
// Encoding: UTF-8 (standard), GBK (FeiQ compatibility)
// Default UDP port: 2425

use crate::{NeoLanError, Result};
use encoding_rs::GBK;
use serde::{Deserialize, Serialize};

/// Message type constants (compatible with IPMsg protocol)
pub mod msg_type {
    // IP Messenger (IPMSG) 常量与工具 (Rust)
    // 基于原始 ipmsg.h / IPMSG.h 的权威映射（mode = 低 8 位，options = 高 24 位）
    #![allow(non_upper_case_globals)]

    /// helpers: 从 command 中取出 mode (低 8 位) 与 opts (高 24 位)
    #[inline]
    pub const fn get_mode(command: u32) -> u8 {
        (command & 0x000000ff) as u8
    }
    #[inline]
    pub const fn get_opt(command: u32) -> u32 {
        command & 0xffffff00
    }
    #[inline]
    pub const fn has_opt(command: u32, flag: u32) -> bool {
        (get_opt(command) & flag) != 0
    }

    /// 协议头 / 版本 / 端口
    pub const IPMSG_VERSION: u16 = 0x0001; // 协议版本
    /// IPMsg 标准默认端口 (re-exported from AppConfig for protocol compatibility)
    pub const IPMSG_DEFAULT_PORT: u16 = 0x0979; // 2425 (standard IPMsg port)

    /// command (mode) — 低 8 位
    pub const IPMSG_NOOPERATION: u32 = 0x00000000; // 0 无操作
    pub const IPMSG_BR_ENTRY: u32 = 0x00000001; // 1 广播上线（entry）
    pub const IPMSG_BR_EXIT: u32 = 0x00000002; // 2 广播下线（exit）
    pub const IPMSG_ANSENTRY: u32 = 0x00000003; // 3 对 BR_ENTRY 的应答
    pub const IPMSG_BR_ABSENCE: u32 = 0x00000004; // 4 广播缺席

    pub const IPMSG_BR_ISGETLIST: u32 = 0x00000010; // 16 请求是否需要列表（is-getlist）
    pub const IPMSG_OKGETLIST: u32 = 0x00000011; // 17 同意发送列表（ok-getlist）
    pub const IPMSG_GETLIST: u32 = 0x00000012; // 18 请求列表（getlist）
    pub const IPMSG_ANSLIST: u32 = 0x00000013; // 19 返回列表（anslist）
    pub const IPMSG_BR_ISGETLIST2: u32 = 0x00000018; // 24 请求扩展列表（is-getlist2）

    pub const IPMSG_SENDMSG: u32 = 0x00000020; // 32 发送消息
    pub const IPMSG_RECVMSG: u32 = 0x00000021; // 33 接收确认
    pub const IPMSG_READMSG: u32 = 0x00000030; // 48 消息已读
    pub const IPMSG_DELMSG: u32 = 0x00000031; // 49 删除消息
    pub const IPMSG_ANSREADMSG: u32 = 0x00000032; // 50 对已读的应答

    pub const IPMSG_GETINFO: u32 = 0x00000040; // 64 请求用户信息
    pub const IPMSG_SENDINFO: u32 = 0x00000041; // 65 发送用户信息

    pub const IPMSG_GETABSENCEINFO: u32 = 0x00000050; // 80 请求缺席信息
    pub const IPMSG_SENDABSENCEINFO: u32 = 0x00000051; // 81 发送缺席信息

    pub const IPMSG_GETFILEDATA: u32 = 0x00000060; // 96 请求文件数据（文件传输）
    pub const IPMSG_RELEASEFILES: u32 = 0x00000061; // 97 释放文件资源
    pub const IPMSG_GETDIRFILES: u32 = 0x00000062; // 98 请求目录文件列表

    pub const IPMSG_GETPUBKEY: u32 = 0x00000072; // 114 请求公钥
    pub const IPMSG_ANSPUBKEY: u32 = 0x00000073; // 115 应答公钥

    /// option / flags（通用 / 全局）
    pub const IPMSG_ABSENCEOPT: u32 = 0x00000100; // 256 缺席标志（全局）
    pub const IPMSG_SERVEROPT: u32 = 0x00000200; // 512 服务器标志（全局）
    pub const IPMSG_DIALUPOPT: u32 = 0x00010000; // 65536 拨号连接标志
    pub const IPMSG_FILEATTACHOPT: u32 = 0x00200000; // 2097152 文件附加标志
    pub const IPMSG_ENCRYPTOPT: u32 = 0x00400000; // 4194304 加密标志
    pub const IPMSG_UTF8OPT: u32 = 0x00800000; // 8388608 UTF-8 编码标志

    /// option for send command（发送上下文特有）
    // 注意：有些值与上面的“通用”标志数值相同 —— 解释时应基于 mode（即先 GET_MODE）
    pub const IPMSG_SENDCHECKOPT: u32 = 0x00000100; // 256 发送确认（send 上下文）
    pub const IPMSG_SECRETOPT: u32 = 0x00000200; // 512 私密发送（send 上下文）
    pub const IPMSG_BROADCASTOPT: u32 = 0x00000400; // 1024 广播发送（send 上下文）
    pub const IPMSG_MULTICASTOPT: u32 = 0x00000800; // 2048 多播发送
    pub const IPMSG_NOPOPUPOPT: u32 = 0x00001000; // 4096 不弹出（接收端）
    pub const IPMSG_AUTORETOPT: u32 = 0x00002000; // 8192 自动回复请求
    pub const IPMSG_RETRYOPT: u32 = 0x00004000; // 16384 重试选项
    pub const IPMSG_PASSWORDOPT: u32 = 0x00008000; // 32768 带密码发送
    pub const IPMSG_NOLOGOPT: u32 = 0x00020000; // 131072 不记录日志

    // 下面给出一些常用组合构造函数作为参考：
    #[inline]
    pub const fn make_command(mode: u32, opts: u32) -> u32 {
        (mode & 0x000000ff) | (opts & 0xffffff00)
    }
}

/// IPMsg protocol version (NeoLan uses version 1)
pub const PROTOCOL_VERSION: u8 = 1;

/// Protocol delimiter
const PROTOCOL_DELIMITER: &str = ":";

/// Minimum number of protocol fields
const MIN_FIELD_COUNT: usize = 6;

/// Maximum packet ID value
const MAX_PACKET_ID: u64 = u32::MAX as u64;

/// Maximum message content size (1MB)
const MAX_CONTENT_SIZE: usize = 1024 * 1024;

/// Escape delimiter characters in a string for protocol serialization
///
/// This prevents content with colons from corrupting the protocol format.
/// Escaped characters: `:` → `\:` and `\` → `\\`
fn escape_delimiters(s: &str) -> String {
    s.replace('\\', "\\\\").replace(':', "\\:")
}

/// Unescape delimiter escape sequences in a parsed string
///
/// Reverses the escaping done by `escape_delimiters`.
/// Unescapes: `\:` → `:` and `\\` → `\`
fn unescape_delimiters(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(&next) = chars.peek() {
                if next == ':' || next == '\\' {
                    result.push(next);
                    chars.next();
                    continue;
                }
            }
        }
        result.push(c);
    }

    result
}

/// Decode bytes to string with encoding auto-detection
/// Tries UTF-8 first (standard IPMsg), then GBK (FeiQ compatibility)
fn decode_message_bytes(data: &[u8]) -> String {
    // Try UTF-8 first (standard IPMsg)
    if let Ok(utf8_str) = std::str::from_utf8(data) {
        return utf8_str.to_string();
    }

    // Try GBK encoding (FeiQ uses GBK for Chinese)
    let (cow, _, _) = GBK.decode(data);
    let gbk_str = cow.to_string();

    // Log encoding detection for debugging
    tracing::debug!("GBK encoding detected for message (non-UTF8 bytes)");

    gbk_str
}

/// Explain message type with its flags for debugging
/// Returns a human-readable description of the message type
pub fn explain_message_type(msg_type: u32) -> String {
    let mode = msg_type::get_mode(msg_type) as u32;
    let _opts = msg_type::get_opt(msg_type);

    let mode_name = get_message_type_name(msg_type);

    let mut flags = Vec::new();

    // Check common option flags
    if msg_type::has_opt(msg_type, msg_type::IPMSG_FILEATTACHOPT) {
        flags.push("FILEATTACH".to_string());
    }
    if msg_type::has_opt(msg_type, msg_type::IPMSG_UTF8OPT) {
        flags.push("UTF8".to_string());
    }
    if msg_type::has_opt(msg_type, msg_type::IPMSG_ENCRYPTOPT) {
        flags.push("ENCRYPT".to_string());
    }
    if msg_type::has_opt(msg_type, msg_type::IPMSG_ABSENCEOPT) {
        flags.push("ABSENCE".to_string());
    }
    if msg_type::has_opt(msg_type, msg_type::IPMSG_SENDCHECKOPT) {
        flags.push("SENDCHECK".to_string());
    }
    if msg_type::has_opt(msg_type, msg_type::IPMSG_SECRETOPT) {
        flags.push("SECRET".to_string());
    }
    if msg_type::has_opt(msg_type, msg_type::IPMSG_BROADCASTOPT) {
        flags.push("BROADCAST".to_string());
    }

    let flags_str = if flags.is_empty() {
        String::new()
    } else {
        format!(" | [{}]", flags.join(" | "))
    };

    format!(
        "{} (0x{:08X} = mode: 0x{:02X}{})",
        mode_name, msg_type, mode, flags_str
    )
}

/// IPMsg-compatible protocol message
///
/// This structure represents a single message in the IPMsg protocol format.
/// All text fields are encoded in UTF-8.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolMessage {
    /// Protocol version (NeoLan uses 1)
    pub version: u8,

    /// Packet ID (monotonically increasing)
    pub packet_id: u64,

    /// User unique ID (user_id) - for LAN identification
    pub user_id: String,

    /// Sender's username (display name)
    pub sender_name: String,

    /// Sender's hostname
    pub sender_host: String,

    /// Message type (see msg_type constants)
    pub msg_type: u32,

    /// Message content (format depends on msg_type)
    pub content: String,
}

/// File transfer request (JSON content for FILE_SEND_REQ)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileSendRequest {
    /// File name
    pub name: String,

    /// File size in bytes
    pub size: u64,

    /// MD5 hash (hex string)
    pub md5: String,
}

/// File transfer response (JSON content for FILE_SEND_RSP)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileSendResponse {
    /// true = accept, false = reject
    pub accept: bool,

    /// TCP port for data transfer (only if accept = true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

/// Parse a byte stream into a ProtocolMessage
///
/// # Arguments
/// * `data` - Raw bytes received from UDP socket
///
/// # Returns
/// * `Ok(ProtocolMessage)` - Successfully parsed message
/// * `Err(NeoLanError)` - Parse error
///
/// # Protocol Format
/// Standard: `version:packet_id:sender_name:sender_host:msg_type:content[:ext_fields]`
/// FeiQ: `1_lbt4_6#128#...#packet_id:timestamp:sender_name:sender_host:msg_type:content`
///
/// # Examples
/// ```no_run
/// # use feiqiu::network::parse_message;
/// # use feiqiu::NeoLanError;
/// let data = b"1:123:Alice:alice-pc:4:Hello World";
/// let msg = parse_message(data)?;
/// # Ok::<(), NeoLanError>(())
/// ```
pub fn parse_message(data: &[u8]) -> Result<ProtocolMessage> {
    // Decode bytes with auto-detection (UTF-8 or GBK for FeiQ)
    let message_str = decode_message_bytes(data);
    tracing::debug!("Received message: {}", message_str);

    // Handle FeiQ hybrid format: contains '#' followed by IPMsg-compatible section
    // FeiQ format: 1_lbt4_6#128#C81F663237C8#0#0#0#311c#9:1761386707:cgc:DESKTOP-IOHG15K:6291459:...
    // The actual IPMsg section starts after the last '#'
    let parse_section = if message_str.contains('#') {
        // Find the last '#' and extract the IPMsg section after it
        if let Some(last_hash_pos) = message_str.rfind('#') {
            &message_str[last_hash_pos + 1..]
        } else {
            &message_str
        }
    } else {
        &message_str
    };

    // Split by delimiter
    let fields: Vec<&str> = parse_section.split(PROTOCOL_DELIMITER).collect();

    // Validate minimum field count
    if fields.len() < MIN_FIELD_COUNT {
        return Err(NeoLanError::Protocol(format!(
            "Invalid message format: expected at least {} fields, got {}. Data: {}",
            MIN_FIELD_COUNT,
            fields.len(),
            parse_section
        )));
    }
    // 1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0170006:LINLINDONG-N:6291459:董琳琳DT-DTG4
    // Detect FeiQ format: version:timestamp:packet_id:hostname:msg_type:content
    // vs standard: version:packet_id:sender_name:sender_host:msg_type:content
    // FeiQ timestamp is typically a 10-digit Unix timestamp (e.g., 1761386707) at fields[1]
    // NOTE: In FeiQ BR_ENTRY messages, the username is in the CONTENT field, not sender_name!
    let (version, packet_id, sender_name, sender_host, is_feiq) = {
        // Try to detect if fields[1] is a timestamp (10-digit number)
        // FeiQ format after stripping last '#': version:timestamp:packet_id:hostname:msg_type:content
        if fields.len() >= 6
            && fields[1].len() == 10
            && fields[1].chars().all(|c| c.is_ascii_digit())
        {
            // FeiQ format detected
            // fields[0]=version, fields[1]=timestamp, fields[2]=packet_id, fields[3]=hostname, fields[4]=msg_type, fields[5]=content
            // For FeiQ BR_ENTRY, the username is in the content field, not sender_name
            eprintln!("Detected FeiQ format message (with timestamp field)");
            // Extract username from content field (fields[5+])
            let content = if fields.len() > 6 {
                fields[5..].join(PROTOCOL_DELIMITER)
            } else {
                fields[5].to_string()
            };
            (
                PROTOCOL_VERSION,
                fields[1],             // Packet ID field
                content,               // Content field (contains username for BR_ENTRY)
                fields[3].to_string(), // Hostname field
                true,                  // Mark as FeiQ format
            )
        } else {
            // Standard IPMsg format
            let v: u8 = fields[0]
                .parse()
                .map_err(|_| NeoLanError::Protocol(format!("Invalid version: {}", fields[0])))?;
            (
                v,
                fields[1],
                fields[2].to_string(),
                fields[3].to_string(),
                false,
            )
        }
    };

    // Validate version (FeiQ compatibility: only warn, don't error)
    if version != PROTOCOL_VERSION {
        tracing::warn!(
            "Protocol version {} differs from expected {}, accepting for compatibility",
            version,
            PROTOCOL_VERSION
        );
    }

    // Parse packet ID - for standard IPMsg it's numeric, for FeiQ it might not be
    let packet_id: u64 = if !is_feiq {
        // Standard IPMsg: packet_id is numeric
        packet_id
            .parse()
            .map_err(|_| NeoLanError::Protocol(format!("Invalid packet_id: {}", packet_id)))?
    } else {
        // FeiQ: packet_id might not be numeric, use timestamp-based fallback
        use std::time::SystemTime;
        packet_id.parse::<u64>().unwrap_or_else(|_| {
            tracing::warn!(
                "FeiQ packet_id '{}' is not numeric, using timestamp-based fallback",
                packet_id
            );
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(1)
        })
    };

    // Validate packet ID range
    if packet_id > MAX_PACKET_ID {
        return Err(NeoLanError::Protocol(format!(
            "Packet ID out of range: {} (max {})",
            packet_id, MAX_PACKET_ID
        )));
    }

    // Validate sender name is not empty
    if sender_name.is_empty() {
        return Err(NeoLanError::Protocol(
            "Sender name cannot be empty".to_string(),
        ));
    }

    // Validate sender host is not empty
    if sender_host.is_empty() {
        return Err(NeoLanError::Protocol(
            "Sender host cannot be empty".to_string(),
        ));
    }

    // Debug log for FeiQ format detection
    if is_feiq {
        eprintln!("DEBUG: FeiQ format detected");
        fields
            .iter()
            .enumerate()
            .for_each(|(i, field)| eprintln!("DEBUG: fields[{}] = '{}'", i, field));
        eprintln!(
            "DEBUG: sender_name={}, sender_host={} ",
            sender_name, sender_host
        );
    } else {
        eprintln!("DEBUG: Standard IPMsg format");
        eprintln!("DEBUG: fields[0]={}", fields[0]);
        eprintln!("DEBUG: fields[1]={}", fields[1]);
        eprintln!("DEBUG: fields[2]={}", fields[2]);
        eprintln!("DEBUG: fields[3]={}", fields[3]);
        eprintln!(
            "DEBUG: sender_name={}, sender_host={}",
            sender_name, sender_host
        );
    }

    // Parse message type
    // For FeiQ format: msg_type is at fields[4]
    // For standard IPMsg format: msg_type is at fields[4]
    let msg_type: u32 = fields[4]
        .parse()
        .map_err(|_| NeoLanError::Protocol(format!("Invalid msg_type: {}", fields[4])))?;

    // Log message type with explanation
    tracing::debug!("Message type: {}", explain_message_type(msg_type));

    // Extract content
    // For FeiQ format: content was already extracted above as sender_name
    // For standard IPMsg format: content is at fields[5+]
    let content = if is_feiq {
        // For FeiQ, the content field was already used as sender_name
        // Set content empty since it's been consumed as the username
        String::new()
    } else {
        // Standard IPMsg format: content is at fields[5+]
        let raw_content = if fields.len() > 6 {
            fields[5..].join(PROTOCOL_DELIMITER)
        } else {
            fields[5].to_string()
        };
        // Unescape content to restore original text
        unescape_delimiters(&raw_content)
    };

    // Validate content size
    if content.len() > MAX_CONTENT_SIZE {
        return Err(NeoLanError::Protocol(format!(
            "Content too large: {} bytes (max {})",
            content.len(),
            MAX_CONTENT_SIZE
        )));
    }

    Ok(ProtocolMessage {
        version,
        packet_id,
        user_id: String::new(),
        sender_name,
        sender_host,
        msg_type,
        content,
    })
}

/// Serialize a ProtocolMessage into a byte stream
///
/// # Arguments
/// * `msg` - Protocol message to serialize
///
/// # Returns
/// * `Ok(Vec<u8>)` - Serialized bytes ready for UDP transmission
/// * `Err(NeoLanError)` - Serialization error
///
/// # Protocol Format
/// `version:packet_id:sender_name:sender_host:msg_type:content`
///
/// # Examples
/// ```no_run
/// # use feiqiu::network::{ProtocolMessage, serialize_message, msg_type};
/// # use feiqiu::NeoLanError;
/// let msg = ProtocolMessage {
///     version: 1,
///     packet_id: 123,
///     user_id: "T0170006".to_string(),
///     sender_name: "Alice".to_string(),
///     sender_host: "alice-pc".to_string(),
///     msg_type: msg_type::IPMSG_SENDMSG,
///     content: "Hello World".to_string(),
/// };
/// let bytes = serialize_message(&msg)?;
/// # Ok::<(), NeoLanError>(())
/// ```
pub fn serialize_message(msg: &ProtocolMessage) -> Result<Vec<u8>> {
    // Validate version
    if msg.version != PROTOCOL_VERSION {
        return Err(NeoLanError::Protocol(format!(
            "Unsupported protocol version: {} (expected {})",
            msg.version, PROTOCOL_VERSION
        )));
    }

    // Validate packet ID range
    if msg.packet_id > MAX_PACKET_ID {
        return Err(NeoLanError::Protocol(format!(
            "Packet ID out of range: {} (max {})",
            msg.packet_id, MAX_PACKET_ID
        )));
    }

    // Validate sender name
    if msg.sender_name.is_empty() {
        return Err(NeoLanError::Protocol(
            "Sender name cannot be empty".to_string(),
        ));
    }

    // Check for delimiter in sender name
    if msg.sender_name.contains(PROTOCOL_DELIMITER) {
        return Err(NeoLanError::Protocol(
            "Sender name cannot contain ':'".to_string(),
        ));
    }

    // Validate sender host
    if msg.sender_host.is_empty() {
        return Err(NeoLanError::Protocol(
            "Sender host cannot be empty".to_string(),
        ));
    }

    // Check for delimiter in sender host
    if msg.sender_host.contains(PROTOCOL_DELIMITER) {
        return Err(NeoLanError::Protocol(
            "Sender host cannot contain ':'".to_string(),
        ));
    }

    // Validate content size
    if msg.content.len() > MAX_CONTENT_SIZE {
        return Err(NeoLanError::Protocol(format!(
            "Content too large: {} bytes (max {})",
            msg.content.len(),
            MAX_CONTENT_SIZE
        )));
    }

    // Escape content to prevent protocol injection
    let escaped_content = escape_delimiters(&msg.content);

    // Build protocol string
    let protocol_string = format!(
        "{}:{}:{}:{}:{}:{}",
        msg.version, msg.packet_id, msg.sender_name, msg.sender_host, msg.msg_type, escaped_content
    );

    // Convert to bytes (UTF-8)
    Ok(protocol_string.into_bytes())
}

/// Get message type name for debugging
/// Uses IPMsg protocol standard message type names
pub fn get_message_type_name(msg_type: u32) -> &'static str {
    // Extract mode (low 8 bits) for comparison
    let mode = msg_type::get_mode(msg_type) as u32;

    match mode {
        msg_type::IPMSG_NOOPERATION => "IPMSG_NOOPERATION",
        msg_type::IPMSG_BR_ENTRY => "IPMSG_BR_ENTRY",
        msg_type::IPMSG_BR_EXIT => "IPMSG_BR_EXIT",
        msg_type::IPMSG_ANSENTRY => "IPMSG_ANSENTRY",
        msg_type::IPMSG_BR_ABSENCE => "IPMSG_BR_ABSENCE",
        msg_type::IPMSG_BR_ISGETLIST => "IPMSG_BR_ISGETLIST",
        msg_type::IPMSG_OKGETLIST => "IPMSG_OKGETLIST",
        msg_type::IPMSG_GETLIST => "IPMSG_GETLIST",
        msg_type::IPMSG_ANSLIST => "IPMSG_ANSLIST",
        msg_type::IPMSG_BR_ISGETLIST2 => "IPMSG_BR_ISGETLIST2",
        msg_type::IPMSG_SENDMSG => "IPMSG_SENDMSG",
        msg_type::IPMSG_RECVMSG => "IPMSG_RECVMSG",
        msg_type::IPMSG_READMSG => "IPMSG_READMSG",
        msg_type::IPMSG_DELMSG => "IPMSG_DELMSG",
        msg_type::IPMSG_ANSREADMSG => "IPMSG_ANSREADMSG",
        msg_type::IPMSG_GETINFO => "IPMSG_GETINFO",
        msg_type::IPMSG_SENDINFO => "IPMSG_SENDINFO",
        msg_type::IPMSG_GETABSENCEINFO => "IPMSG_GETABSENCEINFO",
        msg_type::IPMSG_SENDABSENCEINFO => "IPMSG_SENDABSENCEINFO",
        msg_type::IPMSG_GETFILEDATA => "IPMSG_GETFILEDATA",
        msg_type::IPMSG_RELEASEFILES => "IPMSG_RELEASEFILES",
        msg_type::IPMSG_GETDIRFILES => "IPMSG_GETDIRFILES",
        msg_type::IPMSG_GETPUBKEY => "IPMSG_GETPUBKEY",
        msg_type::IPMSG_ANSPUBKEY => "IPMSG_ANSPUBKEY",
        _ => "UNKNOWN",
    }
}

/// Get the local MAC address for FeiQ protocol
/// Returns a hex string representing the MAC address
pub fn get_local_mac_address() -> Result<String> {
    use std::net::UdpSocket;

    // Try to get MAC from network interface
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(NeoLanError::Network)?;
    let addr = "8.8.8.8:80";

    // Connect to a known address (doesn't actually send)
    socket.connect(addr).map_err(NeoLanError::Network)?;

    // Get the local address
    let local_addr = socket.local_addr().map_err(NeoLanError::Network)?;

    // Use IP address to generate a consistent MAC-like identifier
    // This is a fallback - real implementations would query the interface
    let ip_bytes = match local_addr.ip() {
        std::net::IpAddr::V4(ip) => ip.octets(),
        std::net::IpAddr::V6(ip) => {
            let bytes = ip.octets();
            [bytes[0], bytes[1], bytes[2], bytes[3]]
        }
    };
    let mac = format!(
        "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        ip_bytes[0] ^ 0x02, // Set local bit
        ip_bytes[1],
        ip_bytes[2],
        ip_bytes[3],
        (local_addr.port() >> 8) as u8,
        local_addr.port() & 0xFF,
    );

    Ok(mac)
}

/// Serialize a ProtocolMessage in FeiQ format
/// FeiQ format: 1_lbt6_0#128#MAC#port#0#packet_id:timestamp:username:hostname:msg_type:content
pub fn serialize_message_for_feiq(
    msg: &ProtocolMessage,
    mac_address: &str,
    port: u16,
) -> Result<Vec<u8>> {
    use std::time::SystemTime;

    // Get timestamp
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());

    // Get username from content (FeiQ puts username in content for BR_ENTRY)
    let username = if msg.msg_type & msg_type::IPMSG_BR_ENTRY != 0 {
        &msg.content
    } else {
        &msg.sender_name
    };

    // Build FeiQ header: 1_lbt6_0#128#MAC#port#0#packet_id:
    let feiq_header = format!("1_lbt6_0#128#{}#{}#0#{}:", mac_address, port, msg.packet_id);

    // Build IPMsg body: timestamp:username:hostname:msg_type:content
    let ipmsg_body = format!(
        "{}:{}:{}:{}:{}",
        timestamp, username, msg.sender_host, msg.msg_type, msg.content
    );

    // Combine
    let feiq_message = format!("{}{}", feiq_header, ipmsg_body);

    Ok(feiq_message.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_delimiters_basic() {
        assert_eq!(escape_delimiters("hello"), "hello");
        assert_eq!(escape_delimiters("hello:world"), "hello\\:world");
        assert_eq!(escape_delimiters("hello\\world"), "hello\\\\world");
    }

    #[test]
    fn test_escape_delimiters_multiple() {
        assert_eq!(escape_delimiters("a:b:c"), "a\\:b\\:c");
        assert_eq!(escape_delimiters("a\\b\\c"), "a\\\\b\\\\c");
        assert_eq!(escape_delimiters("a:b\\c"), "a\\:b\\\\c");
    }

    #[test]
    fn test_unescape_delimiters_basic() {
        assert_eq!(unescape_delimiters("hello"), "hello");
        assert_eq!(unescape_delimiters("hello\\:world"), "hello:world");
        assert_eq!(unescape_delimiters("hello\\\\world"), "hello\\world");
    }

    #[test]
    fn test_unescape_delimiters_multiple() {
        assert_eq!(unescape_delimiters("a\\:b\\:c"), "a:b:c");
        assert_eq!(unescape_delimiters("a\\\\b\\\\c"), "a\\b\\c");
    }

    #[test]
    fn test_escape_unescape_roundtrip() {
        let original = "Hello: World\\ Test";
        let escaped = escape_delimiters(original);
        let unescaped = unescape_delimiters(&escaped);
        assert_eq!(original, unescaped);
    }

    #[test]
    fn test_message_with_colons_in_content() {
        let msg = ProtocolMessage {
            version: 1,
            packet_id: 123,
            user_id: "T0170006".to_string(),
            sender_name: "Alice".to_string(),
            sender_host: "alice-pc".to_string(),
            msg_type: msg_type::IPMSG_SENDMSG,
            content: "Hello: World".to_string(),
        };

        let serialized = serialize_message(&msg).unwrap();
        let parsed = parse_message(&serialized).unwrap();

        assert_eq!(parsed.content, "Hello: World");
    }

    #[test]
    fn test_message_with_backslashes_in_content() {
        let msg = ProtocolMessage {
            version: 1,
            packet_id: 123,
            user_id: "T0170006".to_string(),
            sender_name: "Alice".to_string(),
            sender_host: "alice-pc".to_string(),
            msg_type: msg_type::IPMSG_SENDMSG,
            content: "Path: C:\\Users\\Test".to_string(),
        };

        let serialized = serialize_message(&msg).unwrap();
        let parsed = parse_message(&serialized).unwrap();

        assert_eq!(parsed.content, "Path: C:\\Users\\Test");
    }
}
