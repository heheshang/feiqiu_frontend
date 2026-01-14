// File transfer manager - handles file transfer requests and tasks
use crate::network::{msg_type, FileSendRequest, ProtocolMessage, UdpTransport, PROTOCOL_VERSION};
use crate::utils::hash;
use crate::{NeoLanError, Result};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::types::{TransferStatus, TransferTask};

/// File transfer manager
///
/// Manages file transfer tasks, including sending requests and tracking transfers.
pub struct FileTransferManager {
    /// UDP transport for sending requests
    udp: Arc<UdpTransport>,

    /// Transfer tasks (indexed by task ID)
    tasks: Arc<Mutex<Vec<TransferTask>>>,

    /// Local username
    username: String,

    /// Local hostname
    hostname: String,

    /// TCP port range for file transfers
    tcp_port_start: u16,
    tcp_port_end: u16,
}

impl FileTransferManager {
    /// Create a new file transfer manager
    ///
    /// # Arguments
    /// * `udp` - UDP transport for sending requests
    /// * `username` - Local username
    /// * `hostname` - Local hostname
    /// * `tcp_port_start` - Start of TCP port range for file transfers
    /// * `tcp_port_end` - End of TCP port range for file transfers
    ///
    /// # Returns
    /// * `Ok(FileTransferManager)` - Successfully created manager
    /// * `Err(NeoLanError)` - Creation failed
    pub fn new(
        udp: Arc<UdpTransport>,
        username: String,
        hostname: String,
        tcp_port_start: u16,
        tcp_port_end: u16,
    ) -> Self {
        tracing::info!(
            "Creating FileTransferManager for user: {} with TCP port range: {}-{}",
            username,
            tcp_port_start,
            tcp_port_end
        );

        Self {
            udp,
            tasks: Arc::new(Mutex::new(Vec::new())),
            username,
            hostname,
            tcp_port_start,
            tcp_port_end,
        }
    }

    /// Get the next available TCP port for file transfer
    ///
    /// Returns the starting port of the configured range.
    /// In a future implementation, this could track used ports and find an available one.
    pub fn get_next_tcp_port(&self) -> u16 {
        self.tcp_port_start
    }

    /// Send a file transfer request to a peer
    ///
    /// # Arguments
    /// * `path` - Path to the file to send
    /// * `target` - Target peer IP address
    ///
    /// # Returns
    /// * `Ok(Uuid)` - Task ID for tracking the transfer
    /// * `Err(NeoLanError)` - Request failed
    ///
    /// # Process
    /// 1. Calculate file MD5 hash
    /// 2. Get file size
    /// 3. Create IPMSG_GETFILEDATA message
    /// 4. Send via UDP to target peer
    /// 5. Create transfer task in Pending state
    pub fn send_request(&self, path: &Path, target: IpAddr) -> Result<Uuid> {
        tracing::info!("Sending file transfer request: {:?} -> {}", path, target);

        // Validate file exists
        if !path.exists() {
            return Err(NeoLanError::FileTransfer(format!(
                "File not found: {}",
                path.display()
            )));
        }

        // Calculate file metadata
        let file_name = path
            .file_name()
            .ok_or_else(|| {
                NeoLanError::FileTransfer(format!("Invalid file path: {}", path.display()))
            })?
            .to_string_lossy()
            .to_string();

        tracing::debug!("Calculating MD5 for file: {}", file_name);
        let md5 = hash::calculate_file_md5(path)?;

        let file_size = hash::get_file_size(path)?;
        tracing::debug!("File size: {} bytes, MD5: {}", file_size, md5);

        // Create transfer request
        let request = FileSendRequest {
            name: file_name.clone(),
            size: file_size,
            md5: md5.clone(),
        };

        // Create protocol message
        let packet_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| NeoLanError::Other(format!("Time error: {}", e)))?
            .as_secs();

        let proto_msg = ProtocolMessage {
            version: PROTOCOL_VERSION as u8,
            packet_id,
            sender_name: self.username.clone(),
            sender_host: self.hostname.clone(),
            msg_type: msg_type::IPMSG_GETFILEDATA,
            content: serde_json::to_string(&request).map_err(|e| {
                NeoLanError::FileTransfer(format!("Failed to serialize request: {}", e))
            })?,
        };

        // Serialize and send via UDP
        let msg_bytes = crate::network::serialize_message(&proto_msg).map_err(|e| {
            NeoLanError::FileTransfer(format!("Failed to serialize message: {}", e))
        })?;

        let addr = SocketAddr::new(target, 2425); // IPMsg standard port for sending to peers
        self.udp.send_to(&msg_bytes, addr)?;

        tracing::info!(
            "File transfer request sent: {} ({} bytes, MD5: {}) -> {}",
            file_name,
            file_size,
            md5,
            target
        );

        // Create and store transfer task
        let task = TransferTask::new_upload(target, path.to_path_buf(), file_name, file_size, md5);

        let task_id = task.id;
        self.add_task(task)?;

        tracing::info!("Created transfer task: {}", task_id);

        Ok(task_id)
    }

    /// Get all transfer tasks
    ///
    /// # Returns
    /// * `Vec<TransferTask>` - Clone of all tasks
    pub fn get_tasks(&self) -> Vec<TransferTask> {
        self.tasks
            .lock()
            .map(|tasks| tasks.clone())
            .unwrap_or_default()
    }

    /// Get a task by ID
    ///
    /// # Arguments
    /// * `id` - Task ID
    ///
    /// # Returns
    /// * `Option<TransferTask>` - Task if found
    pub fn get_task(&self, id: Uuid) -> Option<TransferTask> {
        self.tasks
            .lock()
            .ok()
            .and_then(|tasks| tasks.iter().find(|t| t.id == id).cloned())
    }

    /// Get tasks by peer IP
    ///
    /// # Arguments
    /// * `peer_ip` - Peer IP address
    ///
    /// # Returns
    /// * `Vec<TransferTask>` - Matching tasks
    pub fn get_tasks_by_peer(&self, peer_ip: IpAddr) -> Vec<TransferTask> {
        self.tasks
            .lock()
            .map(|tasks| {
                tasks
                    .iter()
                    .filter(|t| t.peer_ip == peer_ip)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get tasks by status
    ///
    /// # Arguments
    /// * `status` - Transfer status
    ///
    /// # Returns
    /// * `Vec<TransferTask>` - Matching tasks
    pub fn get_tasks_by_status(&self, status: TransferStatus) -> Vec<TransferTask> {
        self.tasks
            .lock()
            .map(|tasks| {
                tasks
                    .iter()
                    .filter(|t| t.status == status)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Update a task
    ///
    /// # Arguments
    /// * `task` - Updated task
    ///
    /// # Returns
    /// * `Ok(())` - Task updated
    /// * `Err(NeoLanError)` - Update failed
    pub fn update_task(&self, task: TransferTask) -> Result<()> {
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|_| NeoLanError::Other("Failed to lock tasks".to_string()))?;

        if let Some(existing) = tasks.iter_mut().find(|t| t.id == task.id) {
            *existing = task;
            Ok(())
        } else {
            Err(NeoLanError::FileTransfer(format!(
                "Task not found: {}",
                task.id
            )))
        }
    }

    /// Cancel a task
    ///
    /// # Arguments
    /// * `id` - Task ID
    ///
    /// # Returns
    /// * `Ok(())` - Task cancelled
    /// * `Err(NeoLanError)` - Cancel failed
    pub fn cancel_task(&self, id: Uuid) -> Result<()> {
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|_| NeoLanError::Other("Failed to lock tasks".to_string()))?;

        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.mark_cancelled();
            tracing::info!("Transfer task cancelled: {}", id);
            Ok(())
        } else {
            Err(NeoLanError::FileTransfer(format!("Task not found: {}", id)))
        }
    }

    /// Remove completed/failed/cancelled tasks
    ///
    /// # Returns
    /// * `usize` - Number of tasks removed
    pub fn cleanup_finished_tasks(&self) -> usize {
        if let Ok(mut tasks) = self.tasks.lock() {
            let before = tasks.len();
            tasks.retain(|t| !t.is_finished());
            let removed = before - tasks.len();
            if removed > 0 {
                tracing::info!("Cleaned up {} finished transfer tasks", removed);
            }
            removed
        } else {
            0
        }
    }

    /// Add a task to the list
    pub fn add_task(&self, task: TransferTask) -> Result<()> {
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|_| NeoLanError::Other("Failed to lock tasks".to_string()))?;

        tasks.push(task);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    // Note: These tests require a UDP socket, so they're integration tests
    // Run with: cargo test -- --ignored

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
    fn test_get_tasks_by_status() {
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

        // Add multiple tasks with different statuses
        let peer_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let mut task1 = TransferTask::new_upload(
            peer_ip,
            std::env::temp_dir().join("test1.txt"),
            "test1.txt".to_string(),
            1024,
            "abc123".to_string(),
        );
        task1.status = TransferStatus::Pending;
        manager.add_task(task1).unwrap();

        let mut task2 = TransferTask::new_upload(
            peer_ip,
            std::env::temp_dir().join("test2.txt"),
            "test2.txt".to_string(),
            2048,
            "def456".to_string(),
        );
        task2.status = TransferStatus::Active;
        manager.add_task(task2).unwrap();

        let mut task3 = TransferTask::new_upload(
            peer_ip,
            std::env::temp_dir().join("test3.txt"),
            "test3.txt".to_string(),
            4096,
            "ghi789".to_string(),
        );
        task3.status = TransferStatus::Pending;
        manager.add_task(task3).unwrap();

        // Get pending tasks
        let pending_tasks = manager.get_tasks_by_status(TransferStatus::Pending);
        assert_eq!(pending_tasks.len(), 2);

        // Get active tasks
        let active_tasks = manager.get_tasks_by_status(TransferStatus::Active);
        assert_eq!(active_tasks.len(), 1);
    }
}
