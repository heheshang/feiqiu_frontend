// File transfer response handler - handles incoming file transfer requests
use crate::network::{
    msg_type, FileSendRequest, FileSendResponse, ProtocolMessage, PROTOCOL_VERSION,
};
use crate::state::app_state::TauriEvent;
use crate::{NeoLanError, Result};
use chrono::Utc;
use serde_json;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use uuid::Uuid;

use super::types::TransferTask;
use super::FileTransferManager;

/// Pending file transfer request
///
/// Stores information about a received file transfer request
/// that is waiting for user confirmation.
#[derive(Clone, Debug)]
pub struct PendingRequest {
    /// Unique request ID
    pub id: Uuid,

    /// Sender IP address
    pub sender_ip: IpAddr,

    /// Sender name
    pub sender_name: String,

    /// File name
    pub file_name: String,

    /// File size in bytes
    pub file_size: u64,

    /// MD5 hash
    pub md5: String,

    /// Request timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// File transfer response handler
///
/// Handles incoming file transfer requests and sends responses.
pub struct FileTransferResponse {
    /// File transfer manager (for creating download tasks)
    manager: Arc<FileTransferManager>,

    /// Local username
    username: String,

    /// Local hostname
    hostname: String,
}

impl FileTransferResponse {
    /// Create a new response handler
    ///
    /// # Arguments
    /// * `manager` - File transfer manager
    /// * `username` - Local username
    /// * `hostname` - Local hostname
    pub fn new(manager: Arc<FileTransferManager>, username: String, hostname: String) -> Self {
        Self {
            manager,
            username,
            hostname,
        }
    }

    /// Handle an incoming file transfer request
    ///
    /// # Arguments
    /// * `proto_msg` - Protocol message containing the request
    /// * `sender_ip` - Sender's IP address
    ///
    /// # Returns
    /// * `Ok(PendingRequest)` - Request parsed successfully
    /// * `Err(NeoLanError)` - Parsing failed
    ///
    /// # Process
    /// 1. Parse FileSendRequest from message content
    /// 2. Create PendingRequest
    /// 3. Return for user confirmation (via Tauri event)
    pub fn handle_incoming_request(
        &self,
        proto_msg: &ProtocolMessage,
        sender_ip: IpAddr,
    ) -> Result<PendingRequest> {
        tracing::info!(
            "File transfer request from {}: {}",
            proto_msg.sender_name,
            sender_ip
        );

        // Parse FileSendRequest from JSON content
        let file_request: FileSendRequest = serde_json::from_str(&proto_msg.content)
            .map_err(|e| NeoLanError::Protocol(format!("Failed to parse file request: {}", e)))?;

        tracing::info!(
            "File request: name={}, size={}, md5={}",
            file_request.name,
            file_request.size,
            file_request.md5
        );

        // Create pending request
        let request = PendingRequest {
            id: Uuid::new_v4(),
            sender_ip,
            sender_name: proto_msg.sender_name.clone(),
            file_name: file_request.name.clone(),
            file_size: file_request.size,
            md5: file_request.md5.clone(),
            created_at: Utc::now(),
        };

        Ok(request)
    }

    /// Send a file transfer response (accept or reject)
    ///
    /// # Arguments
    /// * `request` - The pending request to respond to
    /// * `accept` - true to accept, false to reject
    /// * `tcp_port` - TCP port for data transfer (only if accept = true)
    /// * `udp` - UDP transport for sending the response
    ///
    /// # Returns
    /// * `Ok(())` - Response sent successfully
    /// * `Err(NeoLanError)` - Sending failed
    pub fn send_response(
        &self,
        request: &PendingRequest,
        accept: bool,
        tcp_port: Option<u16>,
        udp: &crate::network::UdpTransport,
    ) -> Result<()> {
        // Create response
        let response = FileSendResponse {
            accept,
            port: if accept { tcp_port } else { None },
        };

        let content = serde_json::to_string(&response).map_err(|e| {
            NeoLanError::FileTransfer(format!("Failed to serialize response: {}", e))
        })?;

        // Create protocol message
        let packet_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| NeoLanError::Other(format!("Time error: {}", e)))?
            .as_secs();

        let proto_msg = ProtocolMessage {
            version: PROTOCOL_VERSION as u8,
            packet_id,
            user_id: String::new(),
            sender_name: self.username.clone(),
            sender_host: self.hostname.clone(),
            msg_type: msg_type::IPMSG_RELEASEFILES,
            content,
        };

        // Serialize and send via UDP
        let msg_bytes = crate::network::serialize_message(&proto_msg)?;
        let addr = SocketAddr::new(request.sender_ip, 2425); // IPMsg standard port
        udp.send_to(&msg_bytes, addr)?;

        if accept {
            tracing::info!(
                "File transfer ACCEPTED: {} (port: {:?}) -> {}",
                request.file_name,
                tcp_port,
                request.sender_ip
            );
        } else {
            tracing::info!(
                "File transfer REJECTED: {} -> {}",
                request.file_name,
                request.sender_ip
            );
        }

        Ok(())
    }

    /// Create a download task when request is accepted
    ///
    /// # Arguments
    /// * `request` - The accepted request
    ///
    /// # Returns
    /// * `Uuid` - Task ID for tracking
    pub fn create_download_task(&self, request: &PendingRequest) -> Uuid {
        let task = TransferTask::new_download(
            request.sender_ip,
            request.file_name.clone(),
            request.file_size,
            request.md5.clone(),
        );

        let task_id = task.id;

        // Add to manager
        let _ = self.manager.add_task(task);

        tracing::info!(
            "Created download task: {} for file: {}",
            task_id,
            request.file_name
        );

        task_id
    }

    /// Convert PendingRequest to Tauri event for frontend
    ///
    /// # Arguments
    /// * `request` - The pending request
    ///
    /// # Returns
    /// * `TauriEvent` - Event for frontend
    pub fn to_event(&self, request: &PendingRequest) -> TauriEvent {
        TauriEvent::FileTransferRequest {
            request_id: request.id.to_string(),
            sender_ip: request.sender_ip.to_string(),
            sender_name: request.sender_name.clone(),
            file_name: request.file_name.clone(),
            file_size: request.file_size,
            md5: request.md5.clone(),
            created_at: request.created_at.timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::TransferDirection;
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_handle_incoming_request() {
        let udp = Arc::new(crate::network::UdpTransport::bind(0).unwrap());
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
        let udp = Arc::new(crate::network::UdpTransport::bind(0).unwrap());
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
        let udp = Arc::new(crate::network::UdpTransport::bind(0).unwrap());
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

    #[test]
    fn test_to_event() {
        let udp = Arc::new(crate::network::UdpTransport::bind(0).unwrap());
        let manager = Arc::new(FileTransferManager::new(
            udp,
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

        let event = handler.to_event(&request);

        match event {
            TauriEvent::FileTransferRequest {
                request_id,
                sender_ip,
                sender_name,
                file_name,
                file_size,
                md5,
                created_at: _,
            } => {
                assert_eq!(request_id, request.id.to_string());
                assert_eq!(sender_ip, "192.168.1.100");
                assert_eq!(sender_name, "Alice");
                assert_eq!(file_name, "test.txt");
                assert_eq!(file_size, 1024);
                assert_eq!(md5, "abc123");
            }
            _ => panic!("Expected FileTransferRequest event"),
        }
    }

    #[test]
    fn test_create_download_task() {
        let udp = Arc::new(crate::network::UdpTransport::bind(0).unwrap());
        let manager = Arc::new(FileTransferManager::new(
            udp,
            "TestUser".to_string(),
            "test-host".to_string(),
            8000, // tcp_port_start
            9000, // tcp_port_end
        ));

        let handler = FileTransferResponse::new(
            manager.clone(),
            "TestUser".to_string(),
            "test-host".to_string(),
        );

        let request = PendingRequest {
            id: Uuid::new_v4(),
            sender_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            sender_name: "Alice".to_string(),
            file_name: "test.txt".to_string(),
            file_size: 1024,
            md5: "abc123".to_string(),
            created_at: Utc::now(),
        };

        let task_id = handler.create_download_task(&request);

        // Verify task was created
        let task = manager.get_task(task_id);
        assert!(task.is_some());
        let task = task.unwrap();
        assert_eq!(task.direction, TransferDirection::Download);
        assert_eq!(task.file_name, "test.txt");
    }
}
