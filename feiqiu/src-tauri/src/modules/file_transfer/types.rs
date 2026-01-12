// File transfer types - transfer task, status, and direction
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;
use uuid::Uuid;

/// File transfer task
///
/// Represents an active file transfer (upload or download).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferTask {
    /// Unique task ID (UUID)
    pub id: Uuid,

    /// Transfer direction
    pub direction: TransferDirection,

    /// Peer IP address
    pub peer_ip: IpAddr,

    /// File path (local)
    pub file_path: PathBuf,

    /// File name
    pub file_name: String,

    /// File size in bytes
    pub file_size: u64,

    /// MD5 hash (hex string)
    pub md5: String,

    /// Current transfer status
    pub status: TransferStatus,

    /// Bytes transferred
    pub transferred_bytes: u64,

    /// TCP port for data transfer
    pub port: Option<u16>,

    /// Creation time
    pub created_at: DateTime<Utc>,

    /// Last update time
    pub updated_at: DateTime<Utc>,

    /// Error message (if failed)
    pub error: Option<String>,
}

/// Transfer direction
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransferDirection {
    /// Upload (sending file to peer)
    Upload,
    /// Download (receiving file from peer)
    Download,
}

/// Transfer status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransferStatus {
    /// Pending acceptance
    Pending,
    /// Active transfer
    Active,
    /// Paused
    Paused,
    /// Completed successfully
    Completed,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
}

impl TransferTask {
    /// Create a new upload task
    pub fn new_upload(
        peer_ip: IpAddr,
        file_path: PathBuf,
        file_name: String,
        file_size: u64,
        md5: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            direction: TransferDirection::Upload,
            peer_ip,
            file_path,
            file_name,
            file_size,
            md5,
            status: TransferStatus::Pending,
            transferred_bytes: 0,
            port: None,
            created_at: now,
            updated_at: now,
            error: None,
        }
    }

    /// Create a new download task
    pub fn new_download(
        peer_ip: IpAddr,
        file_name: String,
        file_size: u64,
        md5: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            direction: TransferDirection::Download,
            peer_ip,
            file_path: PathBuf::new(), // Will be set when accepted
            file_name,
            file_size,
            md5,
            status: TransferStatus::Pending,
            transferred_bytes: 0,
            port: None,
            created_at: now,
            updated_at: now,
            error: None,
        }
    }

    /// Get transfer progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.file_size == 0 {
            0.0
        } else {
            self.transferred_bytes as f64 / self.file_size as f64
        }
    }

    /// Get transfer progress percentage (0 to 100)
    pub fn progress_percent(&self) -> u8 {
        (self.progress() * 100.0) as u8
    }

    /// Update transferred bytes
    pub fn update_progress(&mut self, bytes: u64) {
        self.transferred_bytes = bytes.min(self.file_size);
        self.updated_at = Utc::now();
    }

    /// Mark as active
    pub fn mark_active(&mut self, port: u16) {
        self.status = TransferStatus::Active;
        self.port = Some(port);
        self.updated_at = Utc::now();
    }

    /// Mark as completed
    pub fn mark_completed(&mut self) {
        self.status = TransferStatus::Completed;
        self.transferred_bytes = self.file_size;
        self.updated_at = Utc::now();
    }

    /// Mark as failed
    pub fn mark_failed(&mut self, error: String) {
        self.status = TransferStatus::Failed;
        self.error = Some(error);
        self.updated_at = Utc::now();
    }

    /// Mark as cancelled
    pub fn mark_cancelled(&mut self) {
        self.status = TransferStatus::Cancelled;
        self.updated_at = Utc::now();
    }

    /// Pause transfer
    pub fn pause(&mut self) {
        if self.status == TransferStatus::Active {
            self.status = TransferStatus::Paused;
            self.updated_at = Utc::now();
        }
    }

    /// Resume transfer
    pub fn resume(&mut self) {
        if self.status == TransferStatus::Paused {
            self.status = TransferStatus::Active;
            self.updated_at = Utc::now();
        }
    }

    /// Check if transfer is finished (completed, failed, or cancelled)
    pub fn is_finished(&self) -> bool {
        matches!(
            self.status,
            TransferStatus::Completed | TransferStatus::Failed | TransferStatus::Cancelled
        )
    }

    /// Check if transfer is active (not paused and not finished)
    pub fn is_active(&self) -> bool {
        self.status == TransferStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

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
        assert_eq!(task.error, Some("Connection lost".to_string()));
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

    #[test]
    fn test_is_active() {
        let mut task = TransferTask::new_upload(
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            PathBuf::from("/test/file.txt"),
            "file.txt".to_string(),
            1024,
            "abc123".to_string(),
        );

        assert!(!task.is_active());

        task.mark_active(8001);
        assert!(task.is_active());

        task.pause();
        assert!(!task.is_active());
    }
}
