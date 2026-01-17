// Message types - core data structures for message handling
//
// This module defines the core message data structures:
// - Message: Application layer message representation
// - MessageType: Enum for different message types
// - Conversion functions between network and storage layers

use crate::modules::peer::types::PeerInfo;
use crate::network::msg_type;
use crate::network::ProtocolMessage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Message type enumeration
///
/// Represents different types of messages in the application layer.
/// Maps to IPMsg protocol message types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    /// Text message (IPMSG_SENDMSG)
    Text,

    /// File transfer request (IPMSG_GETFILEDATA)
    FileRequest,

    /// File transfer response (IPMSG_RELEASEFILES)
    FileResponse,

    /// Presence notification (BR_ENTRY, BR_EXIT, ANSENTRY)
    Presence,

    /// Read receipt (IPMSG_READMSG)
    ReadReceipt,

    /// Receive acknowledgment (IPMSG_RECVMSG)
    RecvAck,

    /// Unknown message type
    Unknown,
}

impl MessageType {
    /// Convert from IPMsg protocol message type (u32)
    pub fn from_protocol(msg_type: u32) -> Self {
        let mode = msg_type::get_mode(msg_type) as u32;

        match mode {
            msg_type::IPMSG_SENDMSG => Self::Text,
            msg_type::IPMSG_GETFILEDATA => Self::FileRequest,
            msg_type::IPMSG_RELEASEFILES => Self::FileResponse,
            msg_type::IPMSG_BR_ENTRY | msg_type::IPMSG_BR_EXIT | msg_type::IPMSG_ANSENTRY => {
                Self::Presence
            }
            msg_type::IPMSG_READMSG => Self::ReadReceipt,
            msg_type::IPMSG_RECVMSG => Self::RecvAck,
            _ => Self::Unknown,
        }
    }

    /// Convert to IPMsg protocol message type (u32)
    pub fn to_protocol(&self) -> u32 {
        match self {
            Self::Text => msg_type::IPMSG_SENDMSG,
            Self::FileRequest => msg_type::IPMSG_GETFILEDATA,
            Self::FileResponse => msg_type::IPMSG_RELEASEFILES,
            Self::Presence => msg_type::IPMSG_BR_ENTRY, // Default to BR_ENTRY
            Self::ReadReceipt => msg_type::IPMSG_READMSG,
            Self::RecvAck => msg_type::IPMSG_RECVMSG,
            Self::Unknown => msg_type::IPMSG_NOOPERATION,
        }
    }
}

/// Application layer message
///
/// This represents a complete message in the application layer.
/// It bridges between network protocol messages and storage models.
///
/// # Fields
/// - `id`: Unique message identifier (UUID)
/// - `packet_id`: Protocol layer packet ID (from ProtocolMessage)
/// - `sender`: Sender information
/// - `receiver`: Receiver information
/// - `msg_type`: Message type
/// - `content`: Message content (format depends on msg_type)
/// - `timestamp`: Message creation time
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier (UUID)
    pub id: Uuid,

    /// Protocol layer packet ID
    pub packet_id: String,

    /// Sender information
    pub sender: PeerInfo,

    /// Receiver information
    pub receiver: PeerInfo,

    /// Message type
    pub msg_type: MessageType,

    /// Message content
    pub content: String,

    /// Message timestamp
    pub timestamp: DateTime<Utc>,
}

impl Message {
    /// Create a new text message
    ///
    /// # Arguments
    /// * `sender` - Sender peer information
    /// * `receiver` - Receiver peer information
    /// * `content` - Text content
    ///
    /// # Examples
    /// ```
    /// # use feiqiu::modules::message::types::{Message, MessageType};
    /// # use feiqiu::modules::peer::types::PeerInfo;
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// let sender = PeerInfo::new(
    ///     IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
    ///     2425,
    ///     Some("Alice".to_string())
    /// );
    /// let receiver = PeerInfo::new(
    ///     IpAddr::V4(Ipv4Addr::new(192, 168, 1, 101)),
    ///     2425,
    ///     Some("Bob".to_string())
    /// );
    /// let msg = Message::new_text(sender, receiver, "Hello World".to_string());
    /// ```
    #[allow(dead_code)]
    pub fn new_text(sender: PeerInfo, receiver: PeerInfo, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            packet_id: Uuid::new_v4().to_string(),
            sender,
            receiver,
            msg_type: MessageType::Text,
            content,
            timestamp: Utc::now(),
        }
    }

    /// Create a new file transfer request message
    #[allow(dead_code)]
    pub fn new_file_request(sender: PeerInfo, receiver: PeerInfo, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            packet_id: Uuid::new_v4().to_string(),
            sender,
            receiver,
            msg_type: MessageType::FileRequest,
            content,
            timestamp: Utc::now(),
        }
    }

    /// Create a message from a protocol message
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message from network layer
    /// * `sender` - Sender peer information
    /// * `receiver` - Receiver peer information (local peer)
    ///
    /// # Returns
    /// A new Message instance with data from the protocol message
    #[allow(dead_code)]
    pub fn from_protocol(
        proto_msg: &ProtocolMessage,
        sender: PeerInfo,
        receiver: PeerInfo,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            packet_id: proto_msg.packet_id.to_string(),
            sender,
            receiver,
            msg_type: MessageType::from_protocol(proto_msg.msg_type),
            content: proto_msg.content.clone(),
            timestamp: Utc::now(),
        }
    }

    /// Convert to protocol message for network transmission
    ///
    /// # Returns
    /// A ProtocolMessage ready for UDP transmission
    ///
    /// # Note
    /// This method requires the sender's username and hostname
    /// to be set before calling. Those should come from the app config.
    pub fn to_protocol(&self, sender_name: &str, sender_host: &str) -> ProtocolMessage {
        self.to_protocol_with_options(sender_name, sender_host, 0)
    }

    /// A ProtocolMessage with options ready for UDP transmission
    ///
    /// # Arguments
    /// * `sender_name` - Sender's username
    /// * `sender_host` - Sender's hostname
    /// * `options` - Protocol options (e.g., IPMSG_SENDCHECKOPT)
    pub fn to_protocol_with_options(
        &self,
        sender_name: &str,
        sender_host: &str,
        options: u32,
    ) -> ProtocolMessage {
        // Combine mode and options using make_command
        let mode = self.msg_type.to_protocol();
        let msg_type = crate::network::msg_type::make_command(mode, options);

        ProtocolMessage {
            version: 1,
            packet_id: self
                .packet_id
                .parse()
                .unwrap_or_else(|_| Uuid::new_v4().as_u128() as u64),
            user_id: String::new(),
            sender_name: sender_name.to_string(),
            sender_host: sender_host.to_string(),
            msg_type,
            content: self.content.clone(),
        }
    }

    /// Get message size in bytes
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.content.len()
    }

    /// Check if this is a text message
    #[allow(dead_code)]
    pub fn is_text(&self) -> bool {
        self.msg_type == MessageType::Text
    }

    /// Check if this is a file transfer message
    #[allow(dead_code)]
    pub fn is_file_transfer(&self) -> bool {
        matches!(
            self.msg_type,
            MessageType::FileRequest | MessageType::FileResponse
        )
    }

    /// Check if message is empty
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

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

        let msg_with_content =
            Message::new_text(sender.clone(), receiver.clone(), "Hi".to_string());
        assert!(!msg_with_content.is_empty());

        let msg_whitespace = Message::new_text(sender, receiver, "   ".to_string());
        assert!(msg_whitespace.is_empty());
    }

    #[test]
    fn test_message_serialization() {
        let sender = create_test_sender();
        let receiver = create_test_receiver();
        let msg = Message::new_text(sender, receiver, "Test".to_string());

        // Test JSON serialization
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("Test"));

        // Test deserialization
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.content, "Test");
        assert_eq!(deserialized.msg_type, MessageType::Text);
    }
}
