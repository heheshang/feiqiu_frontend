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

/// Represents the detected message format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MessageFormat {
    /// Standard IPMsg format: version:packet_id:sender_name:sender_host:msg_type:content
    Standard,
    /// FeiQ format: version:timestamp:user_id:hostname:msg_type:content
    FeiQ,
}

/// Intermediate parsing context
#[derive(Debug)]
struct ParseContext<'a> {
    /// Raw message fields
    fields: Vec<&'a str>,
    /// Detected format type
    format: MessageFormat,
}

impl<'a> ParseContext<'a> {
    /// Create a new parse context from raw fields
    fn new(fields: Vec<&'a str>, format: MessageFormat, _original: &'a str) -> Self {
        Self {
            fields,
            format,
        }
    }

    /// Get a field by index, returning error if missing
    fn get_field(&self, index: usize) -> Result<&'a str> {
        self.fields
            .get(index)
            .copied()
            .ok_or_else(|| NeoLanError::Protocol(format!("Missing field at index {}", index)))
    }

    /// Check if this is FeiQ format
    fn is_feiq(&self) -> bool {
        self.format == MessageFormat::FeiQ
    }
}

/// Detect message format from field layout
///
/// FeiQ format has a 10-digit timestamp at fields[1]
/// Standard IPMsg has packet_id at fields[1]
fn detect_message_format(fields: &[&str]) -> MessageFormat {
    // FeiQ format detection:
    // - Must have at least 6 fields
    // - Second field (fields[1]) must be exactly 10 digits (Unix timestamp)
    if fields.len() >= 6
        && fields[1].len() == 10
        && fields[1].chars().all(|c| c.is_ascii_digit())
    {
        tracing::debug!("Detected FeiQ format message (timestamp field at index 1)");
        MessageFormat::FeiQ
    } else {
        tracing::debug!("Detected standard IPMsg format");
        MessageFormat::Standard
    }
}

/// Extract IPMsg-compatible section from message
///
/// Handles FeiQ hybrid format by extracting everything after the last '#'
/// For standard IPMsg, returns the message as-is
fn extract_ipmsg_section(message_str: &str) -> &str {
    if let Some(last_hash_pos) = message_str.rfind('#') {
        &message_str[last_hash_pos + 1..]
    } else {
        message_str
    }
}

/// Parse version field
fn parse_version(field: &str, format: MessageFormat) -> Result<u8> {
    match format {
        MessageFormat::Standard => field
            .parse()
            .map_err(|_| NeoLanError::Protocol(format!("Invalid version: {}", field))),
        MessageFormat::FeiQ => {
            // FeiQ uses version 1 by default
            tracing::debug!("Using default version for FeiQ format");
            Ok(PROTOCOL_VERSION)
        }
    }
}

/// Parse packet_id field
///
/// For standard IPMsg: packet_id is numeric
/// For FeiQ: may be non-numeric, use timestamp-based fallback
fn parse_packet_id(field: &str, format: MessageFormat) -> Result<u64> {
    use std::time::SystemTime;

    match format {
        MessageFormat::Standard => field
            .parse()
            .map_err(|_| NeoLanError::Protocol(format!("Invalid packet_id: {}", field))),
        MessageFormat::FeiQ => {
            // Try numeric parse first, fallback to timestamp
            Ok(field.parse::<u64>().unwrap_or_else(|_| {
                tracing::warn!(
                    "FeiQ packet_id '{}' is not numeric, using timestamp-based fallback",
                    field
                );
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(1)
            }))
        }
    }
}

/// Parse sender_name based on format
///
/// For standard IPMsg: sender_name is at fields[2]
/// For FeiQ: sender_name is fields[2] (converted to uppercase, like user_id)
fn parse_sender_name(ctx: &ParseContext) -> Result<String> {
    match ctx.format {
        MessageFormat::Standard => Ok(ctx.get_field(2)?.to_string()),
        MessageFormat::FeiQ => {
            // For FeiQ, get user_id from fields[2] and convert to uppercase
            let field2 = ctx.get_field(2)?;
            Ok(field2.to_string().to_uppercase())
        }
    }
}

/// Parse sender_host based on format
///
/// For standard IPMsg: sender_host is at fields[3]
/// For FeiQ: hostname is at fields[3]
fn parse_sender_host(ctx: &ParseContext) -> Result<String> {
    let field_index = match ctx.format {
        MessageFormat::Standard => 3,
        MessageFormat::FeiQ => 3,
    };
    Ok(ctx.get_field(field_index)?.to_string())
}

/// Extract and parse content field
///
/// For standard IPMsg: content is at fields[5+], needs unescaping
/// For FeiQ: content is at fields[5+]
fn extract_content(ctx: &ParseContext) -> Result<String> {
    let content = if ctx.is_feiq() {
        // For FeiQ, content is fields[5+]
        if ctx.fields.len() > 6 {
            ctx.fields[5..].join(PROTOCOL_DELIMITER)
        } else {
            ctx.get_field(5)?.to_string()
        }
    } else {
        // Standard IPMsg: content is fields[5+], needs unescaping
        let raw_content = if ctx.fields.len() > 6 {
            ctx.fields[5..].join(PROTOCOL_DELIMITER)
        } else {
            ctx.get_field(5)?.to_string()
        };
        unescape_delimiters(&raw_content)
    };

    Ok(content)
}

/// Extract user_id based on format
///
/// For standard IPMsg: user_id is not present (returns empty string)
/// For FeiQ: user_id is at fields[2] if it looks like a user_id (e.g., "T0170006")
fn extract_user_id(ctx: &ParseContext) -> String {
    if !ctx.is_feiq() {
        return String::new();
    }

    // For FeiQ, try to extract user_id from fields[2]
    // User IDs typically start with 'T' followed by digits (e.g., "T0170006")
    if let Ok(field2) = ctx.get_field(2) {
        // if field2.starts_with('T') && field2.len() >= 8 && field2.chars().skip(1).all(|c| c.is_ascii_digit()) {
            return field2.to_string().to_uppercase();
        // }
    }

    String::new()
}

/// Validate packet ID is within acceptable range
fn validate_packet_id(packet_id: u64) -> Result<()> {
    if packet_id > MAX_PACKET_ID {
        return Err(NeoLanError::Protocol(format!(
            "Packet ID out of range: {} (max {})",
            packet_id, MAX_PACKET_ID
        )));
    }
    Ok(())
}

/// Validate sender name is not empty
fn validate_sender_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(NeoLanError::Protocol(
            "Sender name cannot be empty".to_string(),
        ));
    }
    Ok(())
}

/// Validate sender host is not empty
fn validate_sender_host(host: &str) -> Result<()> {
    if host.is_empty() {
        return Err(NeoLanError::Protocol(
            "Sender host cannot be empty".to_string(),
        ));
    }
    Ok(())
}

/// Validate content size
fn validate_content_size(content: &str) -> Result<()> {
    if content.len() > MAX_CONTENT_SIZE {
        return Err(NeoLanError::Protocol(format!(
            "Content too large: {} bytes (max {})",
            content.len(),
            MAX_CONTENT_SIZE
        )));
    }
    Ok(())
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
/// let data = b"1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0170006:LINLINDONG-N:6291459:董琳琳DT-DTG4";
/// let msg = parse_message(data)?;
/// # Ok::<(), NeoLanError>(())
/// ```
pub fn parse_message(data: &[u8]) -> Result<ProtocolMessage> {
    // Step 1: Decode bytes with auto-detection (UTF-8 or GBK for FeiQ)
    let message_str = decode_message_bytes(data);
    tracing::debug!("Received message: {}", message_str);

    // Step 2: Extract IPMsg-compatible section (handle FeiQ hybrid format)
    let parse_section = extract_ipmsg_section(&message_str);

    // Step 3: Split by delimiter
    let fields: Vec<&str> = parse_section.split(PROTOCOL_DELIMITER).collect();

    // Step 4: Validate minimum field count
    if fields.len() < MIN_FIELD_COUNT {
        return Err(NeoLanError::Protocol(format!(
            "Invalid message format: expected at least {} fields, got {}. Data: {}",
            MIN_FIELD_COUNT,
            fields.len(),
            parse_section
        )));
    }

    // Step 5: Detect message format (FeiQ vs Standard IPMsg)
    let format = detect_message_format(&fields);

    // Step 6: Create parse context
    let ctx = ParseContext::new(fields, format, parse_section);

    // Step 7: Log detailed field information for debugging
    tracing::debug!(
        "Parsing {} format message with {} fields",
        if ctx.is_feiq() { "FeiQ" } else { "standard IPMsg" },
        ctx.fields.len()
    );
    for (i, field) in ctx.fields.iter().enumerate() {
        tracing::trace!("  fields[{}] = '{}'", i, field);
    }

    // Step 8: Parse version field
    let version = parse_version(ctx.get_field(0)?, ctx.format)?;
    if version != PROTOCOL_VERSION {
        tracing::warn!(
            "Protocol version {} differs from expected {}, accepting for compatibility",
            version,
            PROTOCOL_VERSION
        );
    }

    // Step 9: Parse packet_id field
    let packet_id = parse_packet_id(ctx.get_field(1)?, ctx.format)?;
    validate_packet_id(packet_id)?;

    // Step 10: Parse sender_name and sender_host (format-dependent)
    let sender_name = parse_sender_name(&ctx)?;
    let sender_host = parse_sender_host(&ctx)?;

    // Step 11: Validate sender fields
    validate_sender_name(&sender_name)?;
    validate_sender_host(&sender_host)?;

    tracing::debug!(
        "Parsed: sender_name='{}', sender_host='{}'",
        sender_name,
        sender_host
    );

    // Step 12: Parse message type
    let msg_type_field = ctx.get_field(4)?;
    let msg_type: u32 = msg_type_field
        .parse()
        .map_err(|_| NeoLanError::Protocol(format!("Invalid msg_type: {}", msg_type_field)))?;
    tracing::debug!("Message type: {}", explain_message_type(msg_type));

    // Step 13: Extract content and user_id (format-dependent)
    let content = extract_content(&ctx)?;
    validate_content_size(&content)?;

    let user_id = extract_user_id(&ctx);
    if !user_id.is_empty() {
        tracing::debug!("Extracted user_id: {}", user_id);
    }

    Ok(ProtocolMessage {
        version,
        packet_id,
        user_id,
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

    // ===== Escape/Unescape Tests =====

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

    // ===== Format Detection Tests =====

    #[test]
    fn test_detect_format_standard_ipmsg() {
        // Standard IPMsg: version:packet_id:sender_name:sender_host:msg_type:content
        let fields = vec!["1", "12345", "Alice", "alice-pc", "32", "Hello"];
        assert_eq!(detect_message_format(&fields), MessageFormat::Standard);
    }

    #[test]
    fn test_detect_format_feiq_with_timestamp() {
        // FeiQ: version:timestamp:packet_id:hostname:msg_type:content
        let fields = vec!["1", "1765442982", "12345", "DESKTOP-ABC", "32", "Hello"];
        assert_eq!(detect_message_format(&fields), MessageFormat::FeiQ);
    }

    #[test]
    fn test_detect_format_insufficient_fields() {
        // Less than 6 fields should be detected as standard (not FeiQ)
        let fields = vec!["1", "12345", "Alice"];
        assert_eq!(detect_message_format(&fields), MessageFormat::Standard);
    }

    #[test]
    fn test_detect_format_non_digit_timestamp() {
        // Timestamp field not all digits
        let fields = vec!["1", "abcdefghij", "12345", "DESKTOP-ABC", "32", "Hello"];
        assert_eq!(detect_message_format(&fields), MessageFormat::Standard);
    }

    #[test]
    fn test_detect_format_wrong_timestamp_length() {
        // Wrong timestamp length (9 digits instead of 10)
        let fields = vec!["1", "123456789", "12345", "DESKTOP-ABC", "32", "Hello"];
        assert_eq!(detect_message_format(&fields), MessageFormat::Standard);
    }

    // ===== IPMsg Section Extraction Tests =====

    #[test]
    fn test_extract_ipmsg_section_standard() {
        // Standard IPMsg format without '#'
        let message = "1:12345:Alice:alice-pc:32:Hello";
        assert_eq!(extract_ipmsg_section(message), message);
    }

    #[test]
    fn test_extract_ipmsg_section_feiq() {
        // FeiQ format with '#': 1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0170006:LINLINDONG-N:6291459
        // The IPMsg section after last '#' is: 9:1765442982:T0170006:LINLINDONG-N:6291459
        // This includes a "9" field before the timestamp (part of FeiQ's extended format)
        let message = "1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0170006:LINLINDONG-N:6291459";
        assert_eq!(
            extract_ipmsg_section(message),
            "9:1765442982:T0170006:LINLINDONG-N:6291459"
        );
    }

    #[test]
    fn test_extract_ipmsg_section_multiple_hashes() {
        // Multiple '#' characters
        let message = "header#more#data:1:12345:Alice:alice-pc:32:Hello";
        // The last '#' is before "data:", so we get "data:1:12345:Alice:alice-pc:32:Hello"
        assert_eq!(
            extract_ipmsg_section(message),
            "data:1:12345:Alice:alice-pc:32:Hello"
        );
    }

    // ===== Field Parser Tests =====

    #[test]
    fn test_parse_version_standard() {
        let result = parse_version("1", MessageFormat::Standard).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_parse_version_standard_invalid() {
        let result = parse_version("abc", MessageFormat::Standard);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_version_feiq() {
        let result = parse_version("ignored", MessageFormat::FeiQ).unwrap();
        assert_eq!(result, PROTOCOL_VERSION);
    }

    #[test]
    fn test_parse_packet_id_standard() {
        let result = parse_packet_id("12345", MessageFormat::Standard).unwrap();
        assert_eq!(result, 12345);
    }

    #[test]
    fn test_parse_packet_id_feiq_numeric() {
        let result = parse_packet_id("12345", MessageFormat::FeiQ).unwrap();
        assert_eq!(result, 12345);
    }

    #[test]
    fn test_parse_packet_id_standard_invalid() {
        let result = parse_packet_id("abc", MessageFormat::Standard);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_packet_id_feiq_fallback() {
        // FeiQ non-numeric should use timestamp fallback (returns current time)
        let result = parse_packet_id("abc", MessageFormat::FeiQ).unwrap();
        // Should be a reasonable timestamp (between 2020 and 2030)
        assert!(result > 1577836800); // 2020-01-01
        assert!(result < 1893456000); // 2030-01-01
    }

    // ===== Validation Tests =====

    #[test]
    fn test_validate_packet_id_valid() {
        assert!(validate_packet_id(12345).is_ok());
        assert!(validate_packet_id(MAX_PACKET_ID).is_ok());
    }

    #[test]
    fn test_validate_packet_id_out_of_range() {
        assert!(validate_packet_id(MAX_PACKET_ID + 1).is_err());
    }

    #[test]
    fn test_validate_sender_name_valid() {
        assert!(validate_sender_name("Alice").is_ok());
        assert!(validate_sender_name("张三").is_ok());
    }

    #[test]
    fn test_validate_sender_name_empty() {
        assert!(validate_sender_name("").is_err());
    }

    #[test]
    fn test_validate_sender_host_valid() {
        assert!(validate_sender_host("localhost").is_ok());
        assert!(validate_sender_host("DESKTOP-ABC").is_ok());
    }

    #[test]
    fn test_validate_sender_host_empty() {
        assert!(validate_sender_host("").is_err());
    }

    #[test]
    fn test_validate_content_size_valid() {
        let small_content = "a".repeat(100);
        assert!(validate_content_size(&small_content).is_ok());
    }

    #[test]
    fn test_validate_content_size_too_large() {
        let large_content = "a".repeat(MAX_CONTENT_SIZE + 1);
        assert!(validate_content_size(&large_content).is_err());
    }

    #[test]
    fn test_validate_content_size_exact_max() {
        let max_content = "a".repeat(MAX_CONTENT_SIZE);
        assert!(validate_content_size(&max_content).is_ok());
    }

    // ===== ParseContext Tests =====

    #[test]
    fn test_parse_context_get_field_valid() {
        let fields = vec!["1", "12345", "Alice"];
        let ctx = ParseContext::new(fields, MessageFormat::Standard, "test");
        assert_eq!(ctx.get_field(0).unwrap(), "1");
        assert_eq!(ctx.get_field(2).unwrap(), "Alice");
    }

    #[test]
    fn test_parse_context_get_field_invalid() {
        let fields = vec!["1", "12345", "Alice"];
        let ctx = ParseContext::new(fields, MessageFormat::Standard, "test");
        assert!(ctx.get_field(10).is_err());
    }

    #[test]
    fn test_parse_context_is_feiq() {
        let fields = vec!["1", "12345", "Alice"];
        let ctx_feiq = ParseContext::new(fields.clone(), MessageFormat::FeiQ, "test");
        let ctx_std = ParseContext::new(fields, MessageFormat::Standard, "test");

        assert!(ctx_feiq.is_feiq());
        assert!(!ctx_std.is_feiq());
    }

    // ===== Integration Tests =====

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

    #[test]
    fn test_parse_standard_ipmsg_message() {
        let data = b"1:12345:Alice:alice-pc:32:Hello World";
        let msg = parse_message(data).unwrap();

        assert_eq!(msg.version, 1);
        assert_eq!(msg.packet_id, 12345);
        assert_eq!(msg.sender_name, "Alice");
        assert_eq!(msg.sender_host, "alice-pc");
        assert_eq!(msg.msg_type, 32);
        assert_eq!(msg.content, "Hello World");
    }

    #[test]
    fn test_parse_feiq_format_message() {
        // FeiQ format with timestamp at fields[1]
        // After stripping last '#': flags:timestamp:user_id:hostname:msg_type:content
        // fields[0]=flags, fields[1]=timestamp(10 digits), fields[2]=user_id (e.g., "T0170006"), fields[3]=hostname, fields[4]=msg_type, fields[5]=content
        let data = b"1_lbt6_0#128#5C60BA7361C6#2425#0#4001#9:1765442982:T0170006:DESKTOP-ABC:32:Hello";
        let msg = parse_message(data).unwrap();

        assert_eq!(msg.version, 1);
        // For FeiQ with 10-digit timestamp, the timestamp field becomes packet_id
        assert_eq!(msg.packet_id, 1765442982);
        assert_eq!(msg.sender_host, "DESKTOP-ABC");
        assert_eq!(msg.msg_type, 32);
        // For FeiQ BR_ENTRY, sender_name comes from user_id field (fields[2])
        assert_eq!(msg.sender_name, "T0170006");
        // user_id should be extracted from fields[2]
        assert_eq!(msg.user_id, "T0170006");
        // For FeiQ, content is fields[5+]
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_parse_message_with_too_few_fields() {
        let data = b"1:12345:Alice";
        let result = parse_message(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_message_with_invalid_version() {
        let data = b"abc:12345:Alice:alice-pc:32:Hello";
        let result = parse_message(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_message_with_invalid_packet_id() {
        let data = b"1:abc:Alice:alice-pc:32:Hello";
        let result = parse_message(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_message_with_invalid_msg_type() {
        let data = b"1:12345:Alice:alice-pc:abc:Hello";
        let result = parse_message(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_and_parse_roundtrip() {
        let original = ProtocolMessage {
            version: 1,
            packet_id: 999,
            user_id: "user123".to_string(),
            sender_name: "Bob".to_string(),
            sender_host: "bob-desktop".to_string(),
            msg_type: msg_type::IPMSG_SENDMSG,
            content: "Test message with special chars: \\:".to_string(),
        };

        let serialized = serialize_message(&original).unwrap();
        let parsed = parse_message(&serialized).unwrap();

        assert_eq!(parsed.version, original.version);
        assert_eq!(parsed.packet_id, original.packet_id);
        assert_eq!(parsed.sender_name, original.sender_name);
        assert_eq!(parsed.sender_host, original.sender_host);
        assert_eq!(parsed.msg_type, original.msg_type);
        assert_eq!(parsed.content, original.content);
        // Note: user_id is not preserved in standard IPMsg serialization (it's FeiQ-specific)
    }

    #[test]
    fn test_user_id_extraction_feiq() {
        // Test that user_id is correctly extracted from FeiQ format
        let data = b"1_lbt6_0#128#5C60BA7361C6#2425#0#4001#9:1765442982:T0170006:DESKTOP-ABC:32:Hello";
        let msg = parse_message(data).unwrap();

        assert_eq!(msg.user_id, "T0170006");
        assert_eq!(msg.sender_name, "T0170006"); // sender_name should match user_id for FeiQ
    }

    #[test]
    fn test_user_id_empty_standard_ipmsg() {
        // Test that user_id is empty for standard IPMsg format
        let data = b"1:12345:Alice:alice-pc:32:Hello World";
        let msg = parse_message(data).unwrap();

        assert!(msg.user_id.is_empty());
        assert_eq!(msg.sender_name, "Alice");
    }

    #[test]
    fn test_user_id_empty_feiq_without_valid_id() {
        // Test that user_id is extracted from fields[2] and converted to uppercase
        // (validation logic removed - user_id is always fields[2] uppercased for FeiQ)
        let data = b"1_lbt6_0#128#5C60BA7361C6#2425#0#4001#9:1765442982:not-valid-id:DESKTOP-ABC:32:Hello";
        let msg = parse_message(data).unwrap();

        // user_id should be uppercase version of fields[2]
        assert_eq!(msg.user_id, "NOT-VALID-ID");
        // sender_name should match user_id (from fields[2])
        assert_eq!(msg.sender_name, "NOT-VALID-ID");
    }
}
