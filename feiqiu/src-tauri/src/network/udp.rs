// UDP transport module - handles UDP socket for control messages
//
// This module provides UDP socket functionality for:
// - Broadcasting to LAN (peer discovery)
// - Unicast messaging (direct peer communication)
// - Receiving incoming messages

use crate::config::AppConfig;
use crate::{NeoLanError, Result};
use std::net::{SocketAddr, UdpSocket};

/// Default UDP port for IPMsg protocol (re-exported from AppConfig)
pub const DEFAULT_UDP_PORT: u16 = AppConfig::DEFAULT_UDP_PORT;

/// Broadcast address for LAN (re-exported from AppConfig)
pub const BROADCAST_ADDR: &str = AppConfig::BROADCAST_ADDR;

/// Default receive buffer size (re-exported from AppConfig)
pub const DEFAULT_BUFFER_SIZE: usize = AppConfig::UDP_BUFFER_SIZE;

/// UDP transport wrapper
///
/// Provides a high-level interface for UDP socket operations.
/// Supports broadcasting and unicast messaging.
pub struct UdpTransport {
    /// Inner UdpSocket
    socket: UdpSocket,

    /// Bound port
    port: u16,
}

impl UdpTransport {
    /// Bind to a specific port
    ///
    /// # Arguments
    /// * `port` - UDP port to bind to (use 0 for OS-assigned port)
    ///
    /// # Returns
    /// * `Ok(UdpTransport)` - Successfully bound transport
    /// * `Err(NeoLanError)` - Binding failed
    ///
    /// # Examples
    /// ```no_run
    /// # use neolan_lib::network::udp::UdpTransport;
    /// # use neolan_lib::NeoLanError;
    /// let udp = UdpTransport::bind(2425)?;
    /// # Ok::<(), NeoLanError>(())
    /// ```
    pub fn bind(port: u16) -> Result<Self> {
        tracing::info!("Binding UDP socket to port {}", port);

        // Bind to specified address (0.0.0.0 means all interfaces)
        let addr = format!("0.0.0.0:{}", port);
        let socket = UdpSocket::bind(&addr).map_err(|e| NeoLanError::Network(e))?;

        // Get the actual bound port (in case port was 0)
        let local_addr = socket.local_addr().map_err(|e| NeoLanError::Network(e))?;
        let actual_port = local_addr.port();

        tracing::info!("UDP socket bound to port {}", actual_port);

        Ok(UdpTransport {
            socket,
            port: actual_port,
        })
    }

    /// Bind to a specific port with retry logic
    ///
    /// # Arguments
    /// * `port` - UDP port to bind to (use 0 for OS-assigned port)
    /// * `max_retries` - Maximum number of retries with different ports
    ///
    /// # Returns
    /// * `Ok(UdpTransport)` - Successfully bound transport
    /// * `Err(NeoLanError)` - Binding failed after all retries
    pub fn bind_with_retry(port: u16, max_retries: u16) -> Result<Self> {
        let start_port = if port == 0 { DEFAULT_UDP_PORT } else { port };
        let end_port = start_port + max_retries;

        for attempt_port in start_port..end_port {
            match Self::bind(attempt_port) {
                Ok(transport) => {
                    tracing::info!(
                        "UDP socket bound to port {} (attempt {})",
                        transport.port,
                        attempt_port - start_port + 1
                    );
                    return Ok(transport);
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to bind to port {}: {}, retrying...",
                        attempt_port,
                        e
                    );
                    if attempt_port == end_port - 1 {
                        return Err(e);
                    }
                }
            }
        }
        Err(NeoLanError::Network(std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            format!("Failed to bind UDP socket after {} attempts", max_retries),
        )))
    }

    /// Enable or disable broadcast mode
    ///
    /// # Arguments
    /// * `enabled` - true to enable broadcast, false to disable
    ///
    /// # Returns
    /// * `Ok(())` - Broadcast setting updated
    /// * `Err(NeoLanError)` - Failed to update setting
    ///
    /// # Examples
    /// ```no_run
    /// # use neolan_lib::network::udp::UdpTransport;
    /// # use neolan_lib::NeoLanError;
    /// let udp = UdpTransport::bind(2425)?;
    /// udp.set_broadcast_enabled(true)?;
    /// # Ok::<(), NeoLanError>(())
    /// ```
    pub fn set_broadcast_enabled(&self, enabled: bool) -> Result<()> {
        tracing::debug!("Setting broadcast to {}", enabled);
        self.socket
            .set_broadcast(enabled)
            .map_err(|e| NeoLanError::Network(e))?;
        Ok(())
    }

    /// Send data to broadcast address (255.255.255.255)
    ///
    /// # Arguments
    /// * `data` - Data to broadcast
    ///
    /// # Returns
    /// * `Ok(())` - Data sent successfully
    /// * `Err(NeoLanError)` - Send failed
    ///
    /// # Examples
    /// ```no_run
    /// # use neolan_lib::network::udp::UdpTransport;
    /// # use neolan_lib::NeoLanError;
    /// let udp = UdpTransport::bind(2425)?;
    /// udp.set_broadcast_enabled(true)?;
    /// udp.broadcast(b"Hello, LAN!")?;
    /// # Ok::<(), NeoLanError>(())
    /// ```
    pub fn broadcast(&self, data: &[u8]) -> Result<()> {
        // Use the actual bound port for broadcast target
        // This allows discovery to work with custom UDP port configurations
        let addr: SocketAddr = format!("{}:{}", BROADCAST_ADDR, self.port)
            .parse()
            .map_err(|_| {
                NeoLanError::Network(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid broadcast address",
                ))
            })?;

        tracing::debug!("Broadcasting to {}", addr);
        self.send_to(data, addr)
    }

    /// Send data to a specific address
    ///
    /// # Arguments
    /// * `data` - Data to send
    /// * `addr` - Target socket address
    ///
    /// # Returns
    /// * `Ok(())` - Data sent successfully
    /// * `Err(NeoLanError)` - Send failed
    ///
    /// # Examples
    /// ```no_run
    /// # use neolan_lib::network::udp::UdpTransport;
    /// # use neolan_lib::NeoLanError;
    /// # use std::net::SocketAddr;
    /// let udp = UdpTransport::bind(2425)?;
    /// let addr = "192.168.1.100:2425".parse::<SocketAddr>().unwrap();
    /// udp.send_to(b"Hello, Peer!", addr)?;
    /// # Ok::<(), NeoLanError>(())
    /// ```
    pub fn send_to(&self, data: &[u8], addr: SocketAddr) -> Result<()> {
        let bytes_sent = self
            .socket
            .send_to(data, addr)
            .map_err(|e| NeoLanError::Network(e))?;

        tracing::trace!("Sent {} bytes to {}", bytes_sent, addr.ip());

        Ok(())
    }

    /// Receive data from any peer
    ///
    /// # Arguments
    /// * `buffer` - Buffer to store received data
    ///
    /// # Returns
    /// * `Ok((usize, SocketAddr))` - Number of bytes received and sender address
    /// * `Err(NeoLanError)` - Receive failed
    ///
    /// # Examples
    /// ```no_run
    /// # use neolan_lib::network::udp::UdpTransport;
    /// # use neolan_lib::NeoLanError;
    /// let udp = UdpTransport::bind(2425)?;
    /// let mut buffer = [0u8; 65535];
    /// let (len, sender) = udp.recv_from(&mut buffer)?;
    /// println!("Received {} bytes from {}", len, sender);
    /// # Ok::<(), NeoLanError>(())
    /// ```
    pub fn recv_from(&self, buffer: &mut [u8]) -> Result<(usize, SocketAddr)> {
        let (bytes_received, addr) = self
            .socket
            .recv_from(buffer)
            .map_err(|e| NeoLanError::Network(e))?;

        // Only accept IPv4 addresses
        if addr.is_ipv6() {
            return Err(NeoLanError::Network(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "IPv6 not supported",
            )));
        }

        tracing::trace!("Received {} bytes from {}", bytes_received, addr.ip());

        Ok((bytes_received, addr))
    }

    /// Get the bound port
    ///
    /// # Returns
    /// * `u16` - The port number this socket is bound to
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the local socket address
    ///
    /// # Returns
    /// * `Result<SocketAddr>` - Local socket address
    pub fn local_addr(&self) -> Result<SocketAddr> {
        let addr = self
            .socket
            .local_addr()
            .map_err(|e| NeoLanError::Network(e))?;

        Ok(addr)
    }

    /// Set receive timeout
    ///
    /// # Arguments
    /// * `duration_ms` - Timeout in milliseconds (None for no timeout)
    ///
    /// # Returns
    /// * `Ok(())` - Timeout set successfully
    /// * `Err(NeoLanError)` - Failed to set timeout
    pub fn set_read_timeout(&self, duration_ms: Option<u64>) -> Result<()> {
        let timeout = duration_ms.map(|d| std::time::Duration::from_millis(d));
        self.socket
            .set_read_timeout(timeout)
            .map_err(|e| NeoLanError::Network(e))?;
        Ok(())
    }

    /// Join a multicast group
    ///
    /// # Arguments
    /// * `multiaddr` - Multicast address to join
    ///
    /// # Returns
    /// * `Ok(())` - Successfully joined multicast group
    /// * `Err(NeoLanError)` - Failed to join
    #[cfg(target_os = "linux")]
    pub fn join_multicast(&self, multiaddr: &str) -> Result<()> {
        use std::net::IpAddr;

        let addr: IpAddr = multiaddr.parse().map_err(|_| {
            NeoLanError::Network(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid multicast address",
            ))
        })?;

        self.socket
            .join_multicast_v4(&addr, std::net::Ipv4Addr::UNSPECIFIED)
            .map_err(|e| NeoLanError::Network(e))?;

        tracing::info!("Joined multicast group {}", multiaddr);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_to_port() {
        let udp = UdpTransport::bind(0).unwrap();
        let port = udp.port();

        // Port should be non-zero (OS-assigned)
        assert!(port > 0);

        // Should be able to get local address
        let addr = udp.local_addr().unwrap();
        assert_eq!(addr.port(), port);
    }

    #[test]
    fn test_bind_specific_port() {
        // Use a high port number to avoid conflicts
        let port = 15425;
        let udp = UdpTransport::bind(port).unwrap();

        assert_eq!(udp.port(), port);
    }

    #[test]
    fn test_set_broadcast_enabled() {
        let udp = UdpTransport::bind(0).unwrap();

        // Should be able to enable broadcast
        udp.set_broadcast_enabled(true).unwrap();

        // Should be able to disable broadcast
        udp.set_broadcast_enabled(false).unwrap();
    }

    #[test]
    fn test_send_to_loopback() {
        // Bind to explicit loopback address for reliable testing
        // On Windows, binding to 0.0.0.0:0 and then sending to that address doesn't work properly
        let sender_addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
        let receiver_addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();

        let sender_socket = UdpSocket::bind(sender_addr).unwrap();
        let receiver_socket = UdpSocket::bind(receiver_addr).unwrap();

        let receiver_local = receiver_socket.local_addr().unwrap();
        let sender_port = sender_socket.local_addr().unwrap().port();
        let test_data = b"Hello, UDP!";

        // Send from sender to receiver
        sender_socket.send_to(test_data, receiver_local).unwrap();

        // Receive the data
        let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];
        let (len, addr) = receiver_socket.recv_from(&mut buffer).unwrap();

        assert_eq!(len, test_data.len());
        assert_eq!(&buffer[..len], test_data);
        assert_eq!(addr.port(), sender_port);
    }

    #[test]
    fn test_set_read_timeout() {
        let udp = UdpTransport::bind(0).unwrap();

        // Set timeout to 100ms
        udp.set_read_timeout(Some(100)).unwrap();

        // Clear timeout
        udp.set_read_timeout(None).unwrap();
    }

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_UDP_PORT, 2425);
        assert_eq!(BROADCAST_ADDR, "255.255.255.255");
        assert_eq!(DEFAULT_BUFFER_SIZE, 65535);
    }

    #[test]
    fn test_broadcast_address_parsing() {
        // Test that broadcast address parses correctly
        let addr: SocketAddr = format!("{}:{}", BROADCAST_ADDR, DEFAULT_UDP_PORT)
            .parse()
            .unwrap();
        assert_eq!(addr.ip().to_string(), BROADCAST_ADDR);
        assert_eq!(addr.port(), DEFAULT_UDP_PORT);
    }
}
