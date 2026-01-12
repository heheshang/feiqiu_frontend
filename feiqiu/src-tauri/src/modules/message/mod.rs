// Message handling module
//
// This module handles all message-related functionality:
// - Protocol parsing and serialization
// - Message encryption/decryption
// - Message routing and delivery

pub mod handler;
pub mod types;

pub use handler::MessageHandler;

