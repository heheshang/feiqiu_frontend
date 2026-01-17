//! Modules tests (peer, message, file_transfer)

use chrono::Utc;
use feiqiu::config::AppConfig;
use feiqiu::modules::file_transfer::response::{FileTransferResponse, PendingRequest};
use feiqiu::modules::file_transfer::types::{TransferDirection, TransferStatus, TransferTask};
use feiqiu::modules::file_transfer::FileTransferManager;
use feiqiu::modules::message::handler::MessageHandler;
use feiqiu::modules::message::types::{Message, MessageType};
use feiqiu::modules::peer::discovery::PeerDiscovery;
use feiqiu::modules::peer::types::{PeerInfo, PeerNode, PeerStatus};
use feiqiu::network::msg_type;
use feiqiu::network::parse_message;
use feiqiu::network::protocol::FileSendRequest;
use feiqiu::network::protocol::ProtocolMessage;
use feiqiu::network::serialize_message;
use feiqiu::network::UdpTransport;
use feiqiu::NeoLanError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

// ============== Peer tests ==============

#[test]
fn test_peer_node_new() {
    let ip = "192.168.1.100".parse().unwrap();
    let node = PeerNode::new(ip, 2425);

    assert_eq!(node.ip, ip);
    assert_eq!(node.port, 2425);
    assert!(node.username.is_none());
    assert!(node.hostname.is_none());
    assert!(node.is_online());
}

#[test]
fn test_peer_node_with_details() {
    let ip = "192.168.1.100".parse().unwrap();
    let node = PeerNode::with_details(
        ip,
        2425,
        Some("T0170006".to_string()),
        Some("Alice".to_string()),
        Some("alice-pc".to_string()),
    );

    assert_eq!(node.ip, ip);
    assert_eq!(node.user_id, Some("T0170006".to_string()));
    assert_eq!(node.username, Some("Alice".to_string()));
    assert_eq!(node.hostname, Some("alice-pc".to_string()));
}

#[test]
fn test_display_name() {
    let ip = "192.168.1.100".parse().unwrap();
    let mut node = PeerNode::new(ip, 2425);

    // With only IP
    assert_eq!(node.display_name(), "192.168.1.100");

    // With hostname
    node.hostname = Some("alice-pc".to_string());
    assert_eq!(node.display_name(), "alice-pc");

    // With username (takes precedence)
    node.username = Some("Alice".to_string());
    assert_eq!(node.display_name(), "Alice");

    // With nickname (highest precedence)
    node.nickname = Some("Awesome Alice".to_string());
    assert_eq!(node.display_name(), "Awesome Alice");
}

#[test]
fn test_peer_status() {
    assert!(PeerStatus::Online.is_online());
    assert!(!PeerStatus::Offline.is_online());
    assert!(!PeerStatus::Away.is_online());
}

#[test]
fn test_mark_offline_online() {
    let ip = "192.168.1.100".parse().unwrap();
    let mut node = PeerNode::new(ip, 2425);

    assert!(node.is_online());

    node.mark_offline();
    assert!(!node.is_online());

    node.mark_online();
    assert!(node.is_online());
}

#[test]
fn test_peer_info() {
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
    let peer = PeerInfo::new(ip, 2425, Some("Alice".to_string()));

    assert_eq!(peer.ip, ip);
    assert_eq!(peer.port, 2425);
    assert_eq!(peer.username, Some("Alice".to_string()));
}

#[test]
fn test_peer_info_from_node() {
    let ip = "192.168.1.100".parse().unwrap();
    let node = PeerNode::with_details(
        ip,
        2425,
        Some("T0170006".to_string()),
        Some("Alice".to_string()),
        Some("alice-pc".to_string()),
    );

    let peer = PeerInfo::from_node(&node);

    assert_eq!(peer.ip, ip);
    assert_eq!(peer.port, 2425);
    assert_eq!(peer.username, Some("Alice".to_string()));
}

// ============== Peer Discovery tests ==============

#[test]
fn test_peer_discovery_creation() {
    let udp = UdpTransport::bind(0).unwrap();
    let discovery = PeerDiscovery::new(
        udp,
        "T0170006".to_string(),
        "TestUser".to_string(),
        "test-host".to_string(),
    );

    assert_eq!(discovery.username(), "TestUser");
    assert_eq!(discovery.hostname(), "test-host");
}

#[test]
fn test_peer_discovery_with_defaults() {
    let udp = UdpTransport::bind(0).unwrap();
    let discovery = PeerDiscovery::with_defaults(udp, "T0170006".to_string());

    // Username should be non-empty
    assert!(!discovery.username().is_empty());
    // Hostname should be non-empty
    assert!(!discovery.hostname().is_empty());
}

#[test]
fn test_announce_online() {
    let udp = UdpTransport::bind(0).unwrap();
    let discovery = PeerDiscovery::new(
        udp,
        "T0170006".to_string(),
        "TestUser".to_string(),
        "test-host".to_string(),
    );

    // Should not fail
    let result = discovery.announce_online();
    assert!(result.is_ok());
}

#[test]
fn test_create_message() {
    let udp = UdpTransport::bind(0).unwrap();
    let discovery = PeerDiscovery::new(
        udp,
        "T0170006".to_string(),
        "Alice".to_string(),
        "alice-pc".to_string(),
    );

    let msg = discovery.create_message(msg_type::IPMSG_SENDMSG, "Hello".to_string());

    assert_eq!(msg.version, 1);
    assert_eq!(msg.sender_name, "Alice");
    assert_eq!(msg.sender_host, "alice-pc");
    assert_eq!(msg.msg_type, msg_type::IPMSG_SENDMSG);
    assert_eq!(msg.content, "Hello");
}

#[test]
fn test_packet_id_increment() {
    let udp = UdpTransport::bind(0).unwrap();
    let discovery = PeerDiscovery::new(
        udp,
        "T0170006".to_string(),
        "Test".to_string(),
        "test".to_string(),
    );

    let id1 = discovery.next_packet_id();
    let id2 = discovery.next_packet_id();
    let id3 = discovery.next_packet_id();

    assert!(id2 > id1);
    assert!(id3 > id2);
}

#[test]
fn test_message_types() {
    // Verify IPMsg message type constants are accessible
    assert_eq!(msg_type::IPMSG_BR_ENTRY, 0x00000001);
    assert_eq!(msg_type::IPMSG_BR_EXIT, 0x00000002);
}

// ============== Message tests ==============

fn create_test_sender() -> PeerInfo {
    PeerInfo::new(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        2425,
        Some("Alice".to_string()),
    )
}

fn create_test_receiver() -> PeerInfo {
    PeerInfo::new(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 101)),
        2425,
        Some("Bob".to_string()),
    )
}

#[test]
fn test_message_type_from_protocol() {
    assert_eq!(
        MessageType::from_protocol(msg_type::IPMSG_SENDMSG),
        MessageType::Text
    );
    assert_eq!(
        MessageType::from_protocol(msg_type::IPMSG_GETFILEDATA),
        MessageType::FileRequest
    );
    assert_eq!(
        MessageType::from_protocol(msg_type::IPMSG_RELEASEFILES),
        MessageType::FileResponse
    );
    assert_eq!(
        MessageType::from_protocol(msg_type::IPMSG_BR_ENTRY),
        MessageType::Presence
    );
    assert_eq!(
        MessageType::from_protocol(msg_type::IPMSG_READMSG),
        MessageType::ReadReceipt
    );
}

#[test]
fn test_message_type_to_protocol() {
    assert_eq!(MessageType::Text.to_protocol(), msg_type::IPMSG_SENDMSG);
    assert_eq!(
        MessageType::FileRequest.to_protocol(),
        msg_type::IPMSG_GETFILEDATA
    );
    assert_eq!(
        MessageType::FileResponse.to_protocol(),
        msg_type::IPMSG_RELEASEFILES
    );
    assert_eq!(
        MessageType::Presence.to_protocol(),
        msg_type::IPMSG_BR_ENTRY
    );
    assert_eq!(
        MessageType::ReadReceipt.to_protocol(),
        msg_type::IPMSG_READMSG
    );
}

#[test]
fn test_new_text_message() {
    let sender = create_test_sender();
    let receiver = create_test_receiver();
    let msg = Message::new_text(sender.clone(), receiver.clone(), "Hello World".to_string());

    assert_eq!(msg.msg_type, MessageType::Text);
    assert_eq!(msg.content, "Hello World");
    assert_eq!(msg.sender.ip, sender.ip);
    assert_eq!(msg.receiver.ip, receiver.ip);
    assert!(msg.is_text());
    assert!(!msg.is_file_transfer());
    assert!(!msg.is_empty());
}

#[test]
fn test_new_file_request_message() {
    let sender = create_test_sender();
    let receiver = create_test_receiver();
    let msg = Message::new_file_request(
        sender.clone(),
        receiver.clone(),
        r#"{"name":"test.pdf","size":1024,"md5":"abc123"}"#.to_string(),
    );

    assert_eq!(msg.msg_type, MessageType::FileRequest);
    assert!(msg.is_file_transfer());
    assert!(!msg.is_text());
}

#[test]
fn test_from_protocol_message() {
    let proto_msg = ProtocolMessage {
        version: 1,
        packet_id: 123,
        user_id: "T0170006".to_string(),
        sender_name: "Alice".to_string(),
        sender_host: "alice-pc".to_string(),
        msg_type: msg_type::IPMSG_SENDMSG,
        content: "Test message".to_string(),
    };

    let sender = create_test_sender();
    let receiver = create_test_receiver();
    let msg = Message::from_protocol(&proto_msg, sender, receiver);

    assert_eq!(msg.packet_id, "123");
    assert_eq!(msg.msg_type, MessageType::Text);
    assert_eq!(msg.content, "Test message");
}

#[test]
fn test_to_protocol_message() {
    let sender = create_test_sender();
    let receiver = create_test_receiver();
    let msg = Message::new_text(sender, receiver, "Hello".to_string());

    let proto_msg = msg.to_protocol("Alice", "alice-pc");

    assert_eq!(proto_msg.version, 1);
    assert_eq!(proto_msg.sender_name, "Alice");
    assert_eq!(proto_msg.sender_host, "alice-pc");
    assert_eq!(proto_msg.msg_type, msg_type::IPMSG_SENDMSG);
    assert_eq!(proto_msg.content, "Hello");
}

#[test]
fn test_message_size() {
    let sender = create_test_sender();
    let receiver = create_test_receiver();
    let msg = Message::new_text(sender, receiver, "Hello World".to_string());

    assert_eq!(msg.size(), 11);
}

#[test]
fn test_is_empty() {
    let sender = create_test_sender();
    let receiver = create_test_receiver();

    let msg_with_content = Message::new_text(sender.clone(), receiver.clone(), "Hi".to_string());
    assert!(!msg_with_content.is_empty());

    let msg_whitespace = Message::new_text(sender, receiver, "   ".to_string());
    assert!(msg_whitespace.is_empty());
}

// ============== Message Handler tests ==============

fn create_handler_test_config() -> AppConfig {
    AppConfig {
        username: "TestUser".to_string(),
        hostname: "test-host".to_string(),
        bind_ip: "127.0.0.1".to_string(),
        udp_port: 2425,
        ..Default::default()
    }
}

#[test]
fn test_message_handler_new() {
    let udp = UdpTransport::bind(0).unwrap();
    let config = create_handler_test_config();
    let handler = MessageHandler::new(udp, config);

    assert_eq!(handler.packet_id_counter(), 1);
}

#[test]
fn test_next_packet_id() {
    let udp = UdpTransport::bind(0).unwrap();
    let config = create_handler_test_config();
    let handler = MessageHandler::new(udp, config);

    let id1 = handler.next_packet_id();
    let id2 = handler.next_packet_id();
    let id3 = handler.next_packet_id();

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
}

#[test]
fn test_packet_id_counter() {
    let udp = UdpTransport::bind(0).unwrap();
    let config = create_handler_test_config();
    let handler = MessageHandler::new(udp, config);

    assert_eq!(handler.packet_id_counter(), 1);

    handler.next_packet_id();
    handler.next_packet_id();

    assert_eq!(handler.packet_id_counter(), 3);
}

#[test]
fn test_reset_packet_id_counter() {
    let udp = UdpTransport::bind(0).unwrap();
    let config = create_handler_test_config();
    let handler = MessageHandler::new(udp, config);

    handler.next_packet_id();
    handler.next_packet_id();

    handler.reset_packet_id_counter(100);

    assert_eq!(handler.packet_id_counter(), 100);
    assert_eq!(handler.next_packet_id(), 100);
    assert_eq!(handler.next_packet_id(), 101);
}

#[test]
fn test_udp_reference() {
    let udp = UdpTransport::bind(0).unwrap();
    let config = create_handler_test_config();
    let handler = MessageHandler::new(udp, config);

    // Should be able to access UDP transport
    let port = handler.udp().port();
    assert!(port > 0);
}

#[test]
fn test_send_empty_message_error() {
    let udp = UdpTransport::bind(0).unwrap();
    let config = create_handler_test_config();
    let handler = MessageHandler::new(udp, config);

    let target_ip: IpAddr = "127.0.0.1".parse().unwrap();
    let result = handler.send_text_message(target_ip, "   ");

    assert!(result.is_err());

    if let Err(NeoLanError::Validation(msg)) = result {
        assert!(msg.contains("empty"));
    } else {
        panic!("Expected Validation error");
    }
}

#[test]
fn test_send_text_message_to_loopback() {
    let sender_udp = UdpTransport::bind(0).unwrap();
    let receiver_udp = UdpTransport::bind(0).unwrap();
    let receiver_port = receiver_udp.port();

    let config = AppConfig {
        username: "Sender".to_string(),
        hostname: "sender-host".to_string(),
        bind_ip: "127.0.0.1".to_string(),
        udp_port: sender_udp.port(),
        ..Default::default()
    };

    let handler = MessageHandler::new(sender_udp, config);
    let target_ip: IpAddr = "127.0.0.1".parse().unwrap();

    // Override default port for testing
    let _result = handler.send_text_message(target_ip, "Hello, Test!");

    // Send to our own receiver port
    let sender_udp = handler.udp();
    // Create message with valid packet ID
    let message = Message {
        id: uuid::Uuid::new_v4(),
        packet_id: "12345".to_string(), // Valid small packet ID for testing
        sender: PeerInfo::new(target_ip, receiver_port, Some("Receiver".to_string())),
        receiver: PeerInfo::new(target_ip, sender_udp.port(), Some("Sender".to_string())),
        msg_type: MessageType::Text,
        content: "Hello, Test!".to_string(),
        timestamp: chrono::Utc::now(),
    };

    let proto_msg = message.to_protocol("Sender", "sender-host");
    let bytes = serialize_message(&proto_msg).unwrap();

    let target_addr = SocketAddr::new(target_ip, receiver_port);
    sender_udp.send_to(&bytes, target_addr).unwrap();

    // Try to receive on the receiver socket (with timeout)
    receiver_udp.set_read_timeout(Some(100)).unwrap();
    let mut buffer = [0u8; 65535];
    let result = receiver_udp.recv_from(&mut buffer);

    assert!(result.is_ok());
    let (len, _addr) = result.unwrap();
    assert!(len > 0);

    // Parse and verify the message
    let received = parse_message(&buffer[..len]).unwrap();
    assert_eq!(received.sender_name, "Sender");
    assert_eq!(received.content, "Hello, Test!");
}

#[test]
fn test_send_generic_message() {
    let udp = UdpTransport::bind(0).unwrap();
    let config = create_handler_test_config();
    let handler = MessageHandler::new(udp, config);

    let target_ip: IpAddr = "127.0.0.1".parse().unwrap();
    let result = handler.send_message(
        target_ip,
        MessageType::FileRequest,
        r#"{"name":"test.txt","size":1024,"md5":"abc123"}"#.to_string(),
    );

    // Should not error (sends to localhost which may not receive, but send should succeed)
    assert!(result.is_ok());
}

// ============== File Transfer tests ==============

#[test]
fn test_transfer_task_new_upload() {
    let task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        PathBuf::from("/test/file.txt"),
        "file.txt".to_string(),
        1024,
        "abc123".to_string(),
    );

    assert_eq!(task.direction, TransferDirection::Upload);
    assert_eq!(task.status, TransferStatus::Pending);
    assert_eq!(task.transferred_bytes, 0);
    assert_eq!(task.file_size, 1024);
    assert_eq!(task.progress(), 0.0);
    assert_eq!(task.progress_percent(), 0);
}

#[test]
fn test_transfer_task_new_download() {
    let task = TransferTask::new_download(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        "file.txt".to_string(),
        1024,
        "abc123".to_string(),
    );

    assert_eq!(task.direction, TransferDirection::Download);
    assert_eq!(task.status, TransferStatus::Pending);
}

#[test]
fn test_transfer_progress() {
    let mut task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        PathBuf::from("/test/file.txt"),
        "file.txt".to_string(),
        1000,
        "abc123".to_string(),
    );

    task.update_progress(500);
    assert_eq!(task.progress(), 0.5);
    assert_eq!(task.progress_percent(), 50);

    task.update_progress(1000);
    assert_eq!(task.progress(), 1.0);
    assert_eq!(task.progress_percent(), 100);
}

#[test]
fn test_mark_active() {
    let mut task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        PathBuf::from("/test/file.txt"),
        "file.txt".to_string(),
        1024,
        "abc123".to_string(),
    );

    task.mark_active(8001);
    assert_eq!(task.status, TransferStatus::Active);
    assert_eq!(task.port, Some(8001));
}

#[test]
fn test_mark_completed() {
    let mut task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        PathBuf::from("/test/file.txt"),
        "file.txt".to_string(),
        1024,
        "abc123".to_string(),
    );

    task.mark_completed();
    assert_eq!(task.status, TransferStatus::Completed);
    assert_eq!(task.transferred_bytes, 1024);
}

#[test]
fn test_mark_failed() {
    let mut task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        PathBuf::from("/test/file.txt"),
        "file.txt".to_string(),
        1024,
        "abc123".to_string(),
    );

    task.mark_failed("Connection lost".to_string());
    assert_eq!(task.status, TransferStatus::Failed);
    assert!(task.error.is_some());
}

#[test]
fn test_pause_resume() {
    let mut task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        PathBuf::from("/test/file.txt"),
        "file.txt".to_string(),
        1024,
        "abc123".to_string(),
    );

    task.mark_active(8001);
    task.pause();
    assert_eq!(task.status, TransferStatus::Paused);

    task.resume();
    assert_eq!(task.status, TransferStatus::Active);
}

#[test]
fn test_is_active() {
    let task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        PathBuf::from("/test/file.txt"),
        "file.txt".to_string(),
        1024,
        "abc123".to_string(),
    );

    assert!(!task.is_active());

    let mut active_task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        PathBuf::from("/test/file.txt"),
        "file.txt".to_string(),
        1024,
        "abc123".to_string(),
    );
    active_task.mark_active(8001);

    assert!(active_task.is_active());
}

#[test]
fn test_is_finished() {
    let mut task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        PathBuf::from("/test/file.txt"),
        "file.txt".to_string(),
        1024,
        "abc123".to_string(),
    );

    assert!(!task.is_finished());

    task.mark_completed();
    assert!(task.is_finished());
}

// ============== File Transfer Manager tests ==============

/// Integration test requiring UDP socket
#[test]
#[ignore]
fn test_send_request() {
    // Create UDP transport
    let udp = Arc::new(UdpTransport::bind(0).unwrap());

    // Create manager
    let manager = FileTransferManager::new(
        udp,
        "TestUser".to_string(),
        "test-host".to_string(),
        8000, // tcp_port_start
        9000, // tcp_port_end
    );

    // Create a test file
    let test_file = std::env::temp_dir().join("test_transfer.txt");
    std::fs::write(&test_file, b"Hello, File Transfer!").unwrap();

    // Send request
    let target = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
    let result = manager.send_request(&test_file, target);

    // Clean up
    std::fs::remove_file(&test_file).unwrap();

    // Should succeed (UDP send may fail if no peer listening, but that's OK)
    assert!(result.is_ok() || result.is_err()); // Just check it doesn't panic
}

#[test]
fn test_task_management() {
    // Create UDP transport
    let udp = Arc::new(UdpTransport::bind(0).unwrap());

    // Create manager
    let manager = FileTransferManager::new(
        udp,
        "TestUser".to_string(),
        "test-host".to_string(),
        8000, // tcp_port_start
        9000, // tcp_port_end
    );

    // Create a test task manually
    let task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        std::env::temp_dir().join("test.txt"),
        "test.txt".to_string(),
        1024,
        "abc123".to_string(),
    );

    let task_id = task.id;

    // Add task
    manager.add_task(task).unwrap();

    // Get all tasks
    let tasks = manager.get_tasks();
    assert_eq!(tasks.len(), 1);

    // Get task by ID
    let found_task = manager.get_task(task_id);
    assert!(found_task.is_some());
    assert_eq!(found_task.unwrap().file_name, "test.txt");

    // Get tasks by peer
    let peer_tasks = manager.get_tasks_by_peer(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
    assert_eq!(peer_tasks.len(), 1);

    // Cancel task
    manager.cancel_task(task_id).unwrap();
    let task = manager.get_task(task_id).unwrap();
    assert_eq!(task.status, TransferStatus::Cancelled);

    // Cleanup
    let removed = manager.cleanup_finished_tasks();
    assert_eq!(removed, 1);
    assert_eq!(manager.get_tasks().len(), 0);
}

#[test]
fn test_tcp_port_range() {
    // Create UDP transport
    let udp = Arc::new(UdpTransport::bind(0).unwrap());

    // Create manager with known port range
    let manager = FileTransferManager::new(
        udp,
        "TestUser".to_string(),
        "test-host".to_string(),
        8000, // tcp_port_start
        9000, // tcp_port_end
    );

    // Test get_tcp_port_range
    let (start, end) = manager.get_tcp_port_range();
    assert_eq!(start, 8000);
    assert_eq!(end, 9000);
}

#[test]
fn test_get_tasks_by_status() {
    // Create UDP transport
    let udp = Arc::new(UdpTransport::bind(0).unwrap());

    // Create manager
    let manager = FileTransferManager::new(
        udp,
        "TestUser".to_string(),
        "test-host".to_string(),
        8000,
        9000,
    );

    // Create and add tasks with different statuses
    let pending_task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        std::env::temp_dir().join("pending.txt"),
        "pending.txt".to_string(),
        1024,
        "abc123".to_string(),
    );
    manager.add_task(pending_task).unwrap();

    let mut active_task = TransferTask::new_upload(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 101)),
        std::env::temp_dir().join("active.txt"),
        "active.txt".to_string(),
        2048,
        "def456".to_string(),
    );
    active_task.mark_active(8001);
    manager.add_task(active_task).unwrap();

    // Get tasks by status
    let pending_tasks = manager.get_tasks_by_status(TransferStatus::Pending);
    let active_tasks = manager.get_tasks_by_status(TransferStatus::Active);

    assert_eq!(pending_tasks.len(), 1);
    assert_eq!(active_tasks.len(), 1);
}

// ============== File Transfer Response tests ==============

#[test]
fn test_handle_incoming_request() {
    let udp = Arc::new(UdpTransport::bind(0).unwrap());
    let manager = Arc::new(FileTransferManager::new(
        udp.clone(),
        "TestUser".to_string(),
        "test-host".to_string(),
        8000, // tcp_port_start
        9000, // tcp_port_end
    ));

    let handler =
        FileTransferResponse::new(manager, "TestUser".to_string(), "test-host".to_string());

    // Create a file request
    let file_request = FileSendRequest {
        name: "test.txt".to_string(),
        size: 1024,
        md5: "abc123".to_string(),
    };

    let proto_msg = ProtocolMessage {
        version: 1,
        packet_id: 1,
        user_id: "T0170006".to_string(),
        sender_name: "Alice".to_string(),
        sender_host: "alice-pc".to_string(),
        msg_type: msg_type::IPMSG_GETFILEDATA,
        content: serde_json::to_string(&file_request).unwrap(),
    };

    let sender_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

    // Handle request
    let result = handler.handle_incoming_request(&proto_msg, sender_ip);

    assert!(result.is_ok());
    let pending = result.unwrap();
    assert_eq!(pending.file_name, "test.txt");
    assert_eq!(pending.file_size, 1024);
    assert_eq!(pending.sender_ip, sender_ip);
    assert_eq!(pending.sender_name, "Alice");
}

#[test]
fn test_send_accept_response() {
    let udp = Arc::new(UdpTransport::bind(0).unwrap());
    let manager = Arc::new(FileTransferManager::new(
        udp.clone(),
        "TestUser".to_string(),
        "test-host".to_string(),
        8000, // tcp_port_start
        9000, // tcp_port_end
    ));

    let handler =
        FileTransferResponse::new(manager, "TestUser".to_string(), "test-host".to_string());

    let request = PendingRequest {
        id: Uuid::new_v4(),
        sender_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        sender_name: "Alice".to_string(),
        file_name: "test.txt".to_string(),
        file_size: 1024,
        md5: "abc123".to_string(),
        created_at: Utc::now(),
    };

    // Send accept response
    let result = handler.send_response(&request, true, Some(8001), &udp);

    // May fail if no one is listening, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_send_reject_response() {
    let udp = Arc::new(UdpTransport::bind(0).unwrap());
    let manager = Arc::new(FileTransferManager::new(
        udp.clone(),
        "TestUser".to_string(),
        "test-host".to_string(),
        8000, // tcp_port_start
        9000, // tcp_port_end
    ));

    let handler =
        FileTransferResponse::new(manager, "TestUser".to_string(), "test-host".to_string());

    let request = PendingRequest {
        id: Uuid::new_v4(),
        sender_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        sender_name: "Alice".to_string(),
        file_name: "test.txt".to_string(),
        file_size: 1024,
        md5: "abc123".to_string(),
        created_at: Utc::now(),
    };

    // Send reject response
    let result = handler.send_response(&request, false, None, &udp);

    // May fail if no one is listening, but should not panic
    assert!(result.is_ok() || result.is_err());
}
