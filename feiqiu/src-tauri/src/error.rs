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
