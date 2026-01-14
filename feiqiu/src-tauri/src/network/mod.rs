// Network communication layer - UDP/TCP sockets, packet parsing/sending

pub mod protocol;
pub mod tcp;
pub mod udp;

// Re-export commonly used types
pub use protocol::{
    get_message_type_name, msg_type, parse_message, serialize_message, FileSendRequest,
    FileSendResponse, ProtocolMessage, PROTOCOL_VERSION,
};

pub use udp::{UdpTransport, DEFAULT_UDP_PORT};

pub use tcp::{TcpTransport, DEFAULT_BUFFER_SIZE, PORT_RANGE_END, PORT_RANGE_START};
