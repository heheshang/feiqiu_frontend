//! Protocol tests for IPMsg/FeiQ message parsing and serialization

use feiqiu::network::msg_type::{
    self, get_mode, has_opt, make_command, IPMSG_ENCRYPTOPT, IPMSG_FILEATTACHOPT,
    IPMSG_SENDCHECKOPT, IPMSG_SENDMSG,
};
use feiqiu::network::protocol::{
    get_message_type_name, parse_message, serialize_message, FileSendRequest, FileSendResponse,
    ProtocolMessage,
};

// Max content size (1MB) - matches protocol.rs private constant
const MAX_CONTENT_SIZE: usize = 1024 * 1024;

#[test]
fn test_mode_opt_helpers() {
    let cmd = make_command(IPMSG_SENDMSG, IPMSG_FILEATTACHOPT | IPMSG_SENDCHECKOPT);
    assert_eq!(get_mode(cmd), IPMSG_SENDMSG as u8);
    assert!(has_opt(cmd, IPMSG_FILEATTACHOPT));
    assert!(has_opt(cmd, IPMSG_SENDCHECKOPT));
    assert!(!has_opt(cmd, IPMSG_ENCRYPTOPT));
}

#[test]
fn test_parse_text_message() {
    let data = b"1:123:Alice:alice-pc:32:Hello World"; // 32 = IPMSG_SENDMSG
    let msg = parse_message(data).unwrap();

    assert_eq!(msg.version, 1);
    assert_eq!(msg.packet_id, 123);
    assert_eq!(msg.sender_name, "Alice");
    assert_eq!(msg.sender_host, "alice-pc");
    assert_eq!(msg.msg_type, msg_type::IPMSG_SENDMSG);
    assert_eq!(msg.content, "Hello World");
}

#[test]
fn test_serialize_and_parse() {
    let original = ProtocolMessage {
        version: 1,
        packet_id: 456,
        user_id: "T0170006".to_string(),
        sender_name: "Bob".to_string(),
        sender_host: "bob-pc".to_string(),
        msg_type: msg_type::IPMSG_BR_ENTRY,
        content: "".to_string(),
    };

    let bytes = serialize_message(&original).unwrap();
    let parsed = parse_message(&bytes).unwrap();

    assert_eq!(parsed.version, original.version);
    assert_eq!(parsed.packet_id, original.packet_id);
    assert_eq!(parsed.sender_name, original.sender_name);
    assert_eq!(parsed.sender_host, original.sender_host);
    assert_eq!(parsed.msg_type, original.msg_type);
    assert_eq!(parsed.content, original.content);
}

#[test]
fn test_parse_empty_content() {
    let data = b"1:1:Alice:alice-pc:1:"; // 1 = IPMSG_BR_ENTRY
    let msg = parse_message(data).unwrap();

    assert_eq!(msg.version, 1);
    assert_eq!(msg.msg_type, msg_type::IPMSG_BR_ENTRY);
    assert_eq!(msg.content, "");
}

#[test]
fn test_serialize_empty_content() {
    let msg = ProtocolMessage {
        version: 1,
        packet_id: 1,
        user_id: "T0170006".to_string(),
        sender_name: "Test".to_string(),
        sender_host: "test-pc".to_string(),
        msg_type: msg_type::IPMSG_BR_ENTRY,
        content: "".to_string(),
    };

    let bytes = serialize_message(&msg).unwrap();
    let parsed = parse_message(&bytes).unwrap();

    assert_eq!(parsed.content, "");
}

#[test]
fn test_content_with_colon() {
    // Content containing ":" should be preserved by joining fields 5+
    let data = b"1:1:Alice:alice-pc:32:Time: 12:30:45"; // 32 = IPMSG_SENDMSG
    let msg = parse_message(data).unwrap();

    // Fields 5+ are joined with ":"
    assert_eq!(msg.content, "Time: 12:30:45");

    // JSON encoding also works with colons
    let json_content = serde_json::json!({"time": "12:30:45"});
    let msg = ProtocolMessage {
        version: 1,
        packet_id: 1,
        user_id: "T0170006".to_string(),
        sender_name: "Alice".to_string(),
        sender_host: "alice-pc".to_string(),
        msg_type: msg_type::IPMSG_SENDMSG,
        content: json_content.to_string(),
    };

    let bytes = serialize_message(&msg).unwrap();
    let parsed = parse_message(&bytes).unwrap();

    assert_eq!(parsed.content, r#"{"time":"12:30:45"}"#);
}

#[test]
fn test_serialize_file_request() {
    let request = FileSendRequest {
        name: "document.pdf".to_string(),
        size: 1024000,
        md5: "d41d8cd98f00b204e9800998ecf8427e".to_string(),
    };

    let msg = ProtocolMessage {
        version: 1,
        packet_id: 1,
        user_id: "T0170006".to_string(),
        sender_name: "Alice".to_string(),
        sender_host: "alice-pc".to_string(),
        msg_type: msg_type::IPMSG_GETFILEDATA,
        content: serde_json::to_string(&request).unwrap(),
    };

    let bytes = serialize_message(&msg).unwrap();
    let parsed = parse_message(&bytes).unwrap();

    assert_eq!(parsed.msg_type, msg_type::IPMSG_GETFILEDATA);

    let parsed_request: FileSendRequest = serde_json::from_str(&parsed.content).unwrap();
    assert_eq!(parsed_request.name, "document.pdf");
    assert_eq!(parsed_request.size, 1024000);
}

#[test]
fn test_serialize_file_response_accept() {
    let response = FileSendResponse {
        accept: true,
        port: Some(8001),
    };

    let content = serde_json::to_string(&response).unwrap();

    assert!(content.contains(r#""accept":true"#));
    assert!(content.contains(r#""port":8001"#));
}

#[test]
fn test_serialize_file_response_reject() {
    let response = FileSendResponse {
        accept: false,
        port: None,
    };

    let content = serde_json::to_string(&response).unwrap();

    assert!(content.contains(r#""accept":false"#));
    assert!(!content.contains("port"));
}

#[test]
fn test_invalid_utf8() {
    let data = &[0xFF, 0xFF, 0xFF]; // Invalid UTF-8
    let result = parse_message(data);

    assert!(result.is_err());
}

#[test]
fn test_invalid_version() {
    // For compatibility with FeiQ and other IPMsg implementations,
    // parse_message accepts different versions (warns but doesn't error)
    let data = b"2:1:Alice:alice-pc:32:test"; // 32 = IPMSG_SENDMSG
    let result = parse_message(data);

    // Should parse successfully (version 2 is accepted for compatibility)
    assert!(result.is_ok());
    let msg = result.unwrap();
    assert_eq!(msg.version, 2);

    // Version 1 is also accepted
    let data = b"1:1:Alice:alice-pc:32:test";
    let result = parse_message(data);
    assert!(result.is_ok());
    let msg = result.unwrap();
    assert_eq!(msg.version, 1);
}

#[test]
fn test_serialize_invalid_version() {
    let msg = ProtocolMessage {
        version: 2,
        packet_id: 1,
        user_id: "T0170006".to_string(),
        sender_name: "Test".to_string(),
        sender_host: "test-pc".to_string(),
        msg_type: msg_type::IPMSG_SENDMSG,
        content: "test".to_string(),
    };

    let result = serialize_message(&msg);
    assert!(result.is_err());
}

#[test]
fn test_empty_sender_name() {
    let data = b"1:1::alice-pc:32:test"; // 32 = IPMSG_SENDMSG
    let result = parse_message(data);

    assert!(result.is_err());
}

#[test]
fn test_serialize_empty_sender_name() {
    let msg = ProtocolMessage {
        version: 1,
        packet_id: 1,
        user_id: "T0170006".to_string(),
        sender_name: "".to_string(),
        sender_host: "test-pc".to_string(),
        msg_type: msg_type::IPMSG_SENDMSG,
        content: "test".to_string(),
    };

    let result = serialize_message(&msg);
    assert!(result.is_err());
}

#[test]
fn test_get_message_type_name() {
    assert_eq!(
        get_message_type_name(msg_type::IPMSG_SENDMSG),
        "IPMSG_SENDMSG"
    );
    assert_eq!(
        get_message_type_name(msg_type::IPMSG_BR_ENTRY),
        "IPMSG_BR_ENTRY"
    );
    assert_eq!(get_message_type_name(0xFFFFFFFF), "UNKNOWN");
}

#[test]
fn test_serialize_feiq_br_entry() {
    // Test BR_ENTRY message in FeiQ format
    let msg = ProtocolMessage {
        version: 1,
        packet_id: 12345,
        user_id: "T0170006".to_string(),
        sender_name: "TestUser".to_string(),
        sender_host: "test-pc".to_string(),
        msg_type: msg_type::IPMSG_BR_ENTRY | msg_type::IPMSG_UTF8OPT,
        content: "TestUser".to_string(), // FeiQ puts username in content
    };

    use feiqiu::network::protocol::serialize_message_for_feiq;
    let bytes = serialize_message_for_feiq(&msg, "5C60BA7361C6", 2425).unwrap();
    let result = String::from_utf8_lossy(&bytes);

    // Verify FeiQ header starts with correct format
    assert!(result.starts_with("1_lbt6_0#128#5C60BA7361C6#2425#0#12345:"));
    assert!(result.contains(":test-pc:"));

    println!("FeiQ BR_ENTRY: {}", result);
}

#[test]
fn test_serialize_feiq_send_msg() {
    // Test SENDMSG message in FeiQ format
    let msg = ProtocolMessage {
        version: 1,
        packet_id: 67890,
        user_id: "T0170006".to_string(),
        sender_name: "Alice".to_string(),
        sender_host: "alice-pc".to_string(),
        msg_type: msg_type::IPMSG_SENDMSG | msg_type::IPMSG_SENDCHECKOPT | msg_type::IPMSG_UTF8OPT,
        content: "Hello, FeiQ!".to_string(),
    };

    use feiqiu::network::protocol::serialize_message_for_feiq;
    let bytes = serialize_message_for_feiq(&msg, "AABBCCDDEEFF", 2425).unwrap();
    let result = String::from_utf8_lossy(&bytes);

    assert!(result.starts_with("1_lbt6_0#128#AABBCCDDEEFF#2425#0#67890:"));
    assert!(result.contains("Hello, FeiQ!"));

    println!("FeiQ SENDMSG: {}", result);
}

#[test]
fn test_get_local_mac_address() {
    // Test MAC address generation
    use feiqiu::network::protocol::get_local_mac_address;
    let mac = get_local_mac_address().unwrap();

    // Should be hex string (could be 12 chars for real MAC or longer for hash-based)
    assert!(!mac.is_empty());
    assert!(mac.chars().all(|c| c.is_ascii_hexdigit()));

    println!("Generated MAC: {}", mac);
}

#[test]
fn test_parse_feiq_format_roundtrip() {
    use feiqiu::network::protocol::serialize_message_for_feiq;

    // Create a BR_ENTRY message (username in content)
    let original = ProtocolMessage {
        version: 1,
        packet_id: 99999,
        user_id: "T0170006".to_string(),
        sender_name: "Bob".to_string(),
        sender_host: "bob-pc".to_string(),
        msg_type: msg_type::IPMSG_BR_ENTRY,
        content: "Bob".to_string(), // BR_ENTRY: username in content
    };

    // Serialize as FeiQ
    let feiq_bytes = serialize_message_for_feiq(&original, "112233445566", 2425).unwrap();

    // Parse it back (should detect FeiQ format)
    let parsed = parse_message(&feiq_bytes).unwrap();

    // For BR_ENTRY, username is extracted from content field
    assert_eq!(parsed.sender_name, "Bob");
    assert_eq!(parsed.sender_host, original.sender_host);
    assert_eq!(parsed.msg_type, original.msg_type);
}

#[test]
fn test_packet_id_overflow() {
    let msg = ProtocolMessage {
        version: 1,
        packet_id: u64::MAX,
        user_id: "T0170006".to_string(),
        sender_name: "Test".to_string(),
        sender_host: "test-pc".to_string(),
        msg_type: msg_type::IPMSG_SENDMSG,
        content: "test".to_string(),
    };

    let result = serialize_message(&msg);
    assert!(result.is_err());
}

#[test]
fn test_content_too_large() {
    let large_content = "x".repeat(MAX_CONTENT_SIZE + 1);

    let msg = ProtocolMessage {
        version: 1,
        packet_id: 1,
        user_id: "T0170006".to_string(),
        sender_name: "Test".to_string(),
        sender_host: "test-pc".to_string(),
        msg_type: msg_type::IPMSG_SENDMSG,
        content: large_content,
    };

    let result = serialize_message(&msg);
    assert!(result.is_err());
}

#[test]
fn test_parse_feiq_chinese_username() {
    // FeiQ format with Chinese username - using actual UTF-8 string
    let message_str = "1_lbt4_41#128#24F5AAD7C96A#0#0#0#311c#9:1767153479:t0250254:DESKTOP-IOHG15K:6291459:陈俞辛";
    let data = message_str.as_bytes();

    let msg = parse_message(data).expect("Failed to parse FeiQ message");

    // Verify the username is extracted from content field
    assert_eq!(msg.sender_name, "陈俞辛");
    assert_eq!(msg.sender_host, "DESKTOP-IOHG15K");
    // 6291459 = 0x600003 = IPMSG_ANSENTRY (0x03) with some options
    assert_eq!(
        msg_type::get_mode(msg.msg_type) as u32,
        msg_type::IPMSG_ANSENTRY
    );
}

#[test]
fn test_parse_feiq_with_regular_username() {
    // Test parsing FeiQ format with regular ASCII username
    let message_str =
        "1_lbt4_6#128#112233445566#0#0#0#311c#9:1761386707:test-user:test-pc:6291459:TestUser";
    let data = message_str.as_bytes();

    let msg = parse_message(data).expect("Failed to parse FeiQ message");

    // Verify the username is extracted from content field
    assert_eq!(msg.sender_name, "TestUser");
    assert_eq!(msg.sender_host, "test-pc");
    // 6291459 = 0x600003 = IPMSG_ANSENTRY (0x03) with some options
    assert_eq!(
        msg_type::get_mode(msg.msg_type) as u32,
        msg_type::IPMSG_ANSENTRY
    );
}
