// src-tauri/src/error.rs
use thiserror::Error;

/// NeoLan 统一错误类型
#[derive(Error, Debug)]
pub enum NeoLanError {
    /// 网络相关错误
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    /// 协议解析错误
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// 存储错误（数据库）
    #[error("Storage error: {0}")]
    Storage(String),

    /// 加密错误
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// 配置错误
    #[error("Config error: {0}")]
    Config(String),

    /// JSON 序列化错误
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// 节点未找到
    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    /// 文件传输错误
    #[error("File transfer error: {0}")]
    FileTransfer(String),

    /// 超时错误
    #[error("Timeout error: {0}")]
    Timeout(String),

    /// 验证错误
    #[error("Validation error: {0}")]
    Validation(String),

    /// 其他错误
    #[error("Other error: {0}")]
    Other(String),
}

impl NeoLanError {
    /// Add context to an error, creating a new error with additional information
    ///
    /// This is a non-breaking way to add context to errors without changing the error variants.
    ///
    /// # Arguments
    /// * `context` - Additional context string to prepend to the error message
    ///
    /// # Returns
    /// A new NeoLanError with the context added
    ///
    /// # Examples
    /// ```ignore
    /// let err = NeoLanError::Network(io_err);
    /// let err_with_context = err.with_context("failed to connect to peer");
    /// ```
    pub fn with_context<C: Into<String>>(self, context: C) -> Self {
        let context = context.into();
        match self {
            NeoLanError::Network(e) => NeoLanError::Storage(format!("{}: {}", context, e)),
            NeoLanError::Protocol(s) => NeoLanError::Protocol(format!("{}: {}", context, s)),
            NeoLanError::Storage(s) => NeoLanError::Storage(format!("{}: {}", context, s)),
            NeoLanError::Crypto(s) => NeoLanError::Crypto(format!("{}: {}", context, s)),
            NeoLanError::Config(s) => NeoLanError::Config(format!("{}: {}", context, s)),
            NeoLanError::Json(e) => NeoLanError::Storage(format!("{}: {}", context, e)),
            NeoLanError::PeerNotFound(s) => {
                NeoLanError::PeerNotFound(format!("{}: {}", context, s))
            }
            NeoLanError::FileTransfer(s) => {
                NeoLanError::FileTransfer(format!("{}: {}", context, s))
            }
            NeoLanError::Timeout(s) => NeoLanError::Timeout(format!("{}: {}", context, s)),
            NeoLanError::Validation(s) => NeoLanError::Validation(format!("{}: {}", context, s)),
            NeoLanError::Other(s) => NeoLanError::Other(format!("{}: {}", context, s)),
        }
    }

    /// Create a storage error with context
    ///
    /// Convenience method for creating storage errors with context.
    pub fn storage_context<C: Into<String>>(context: C) -> Self {
        NeoLanError::Storage(context.into())
    }

    /// Create a network error with context
    ///
    /// Convenience method for creating network errors with context.
    pub fn network_context<C: Into<String> + std::fmt::Display>(context: C) -> Self {
        NeoLanError::Other(format!("Network error: {}", context))
    }

    /// Create a protocol error with context
    ///
    /// Convenience method for creating protocol errors with context.
    pub fn protocol_context<C: Into<String>>(context: C) -> Self {
        NeoLanError::Protocol(context.into())
    }
}

/// 类型别名，简化 Result 使用
pub type Result<T> = std::result::Result<T, NeoLanError>;

// Implement Tauri 2.x IPC error conversion
impl From<NeoLanError> for tauri::ipc::InvokeError {
    fn from(err: NeoLanError) -> Self {
        tauri::ipc::InvokeError::from(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = NeoLanError::Protocol("invalid message format".to_string());
        assert_eq!(format!("{}", err), "Protocol error: invalid message format");
    }

    #[test]
    fn test_peer_not_found() {
        let err = NeoLanError::PeerNotFound("192.168.1.100".to_string());
        assert!(err.to_string().contains("192.168.1.100"));
    }

    #[test]
    fn test_network_error_from_io() {
        let io_err =
            std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "connection refused");
        let err: NeoLanError = io_err.into();
        assert!(matches!(err, NeoLanError::Network(_)));
    }

    #[test]
    fn test_json_error_from() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let err: NeoLanError = json_err.into();
        assert!(matches!(err, NeoLanError::Json(_)));
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_ok() -> Result<String> {
            Ok("success".to_string())
        }

        fn returns_err() -> Result<String> {
            Err(NeoLanError::Config("invalid config".to_string()))
        }

        assert!(returns_ok().is_ok());
        assert!(returns_err().is_err());
    }

    #[test]
    fn test_with_context_storage() {
        let err = NeoLanError::Storage("database locked".to_string());
        let enriched = err.with_context("while saving message");
        assert_eq!(
            format!("{}", enriched),
            "Storage error: while saving message: database locked"
        );
    }

    #[test]
    fn test_with_context_protocol() {
        let err = NeoLanError::Protocol("invalid packet".to_string());
        let enriched = err.with_context("parsing broadcast");
        assert_eq!(
            format!("{}", enriched),
            "Protocol error: parsing broadcast: invalid packet"
        );
    }

    #[test]
    fn test_with_context_network() {
        let io_err =
            std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "connection refused");
        let err: NeoLanError = io_err.into();
        let enriched = err.with_context("connecting to peer 192.168.1.100");
        // Network errors get converted to Storage with context
        assert!(format!("{}", enriched).contains("connecting to peer 192.168.1.100"));
    }

    #[test]
    fn test_storage_context() {
        let err = NeoLanError::storage_context("database connection failed");
        assert_eq!(
            format!("{}", err),
            "Storage error: database connection failed"
        );
    }

    #[test]
    fn test_network_context() {
        let err = NeoLanError::network_context("UDP send failed");
        assert_eq!(
            format!("{}", err),
            "Other error: Network error: UDP send failed"
        );
    }

    #[test]
    fn test_protocol_context() {
        let err = NeoLanError::protocol_context("malformed message");
        assert_eq!(format!("{}", err), "Protocol error: malformed message");
    }
}
