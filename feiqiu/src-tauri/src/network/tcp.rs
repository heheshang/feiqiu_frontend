// TCP transport module - handles TCP socket for file data transfer
//
// This module provides TCP socket functionality for:
// - Binding to available ports for file transfer
// - Connecting to peers for data transfer
// - Sending file data in chunks
// - Receiving file data in chunks

use crate::config::AppConfig;
use crate::{NeoLanError, Result};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;

/// Default buffer size for file transfer (re-exported from AppConfig)
pub const DEFAULT_BUFFER_SIZE: usize = AppConfig::TCP_BUFFER_SIZE;

/// TCP port range for file transfer (re-exported from AppConfig)
pub const PORT_RANGE_START: u16 = AppConfig::DEFAULT_TCP_PORT_START;
pub const PORT_RANGE_END: u16 = AppConfig::DEFAULT_TCP_PORT_END;

/// Default bind IP address (re-exported from AppConfig)
pub const DEFAULT_BIND_IP: &str = AppConfig::DEFAULT_BIND_IP;

/// TCP transport wrapper
///
/// Provides a high-level interface for TCP socket operations.
/// Supports binding to available ports and connecting to peers.
pub struct TcpTransport;

impl TcpTransport {
    /// Bind to an available port in the configured range
    ///
    /// # Returns
    /// * `Ok((TcpListener, u16))` - Successfully bound listener and port number
    /// * `Err(NeoLanError)` - Binding failed
    ///
    /// # Examples
    /// ```no_run
    /// # use feiqiu::network::tcp::TcpTransport;
    /// # use feiqiu::NeoLanError;
    /// let (listener, port) = TcpTransport::bind_available()?;
    /// println!("Bound to port {}", port);
    /// # Ok::<(), NeoLanError>(())
    /// ```
    pub fn bind_available() -> Result<(TcpListener, u16)> {
        for port in PORT_RANGE_START..PORT_RANGE_END {
            let addr = format!("{}:{}", DEFAULT_BIND_IP, port);
            match TcpListener::bind(&addr) {
                Ok(listener) => {
                    tracing::info!("TCP listener bound to port {}", port);
                    return Ok((listener, port));
                }
                Err(_) => continue,
            }
        }

        Err(NeoLanError::Network(std::io::Error::new(
            std::io::ErrorKind::AddrInUse,
            format!(
                "No available ports in range {}-{}",
                PORT_RANGE_START, PORT_RANGE_END
            ),
        )))
    }

    /// Connect to a remote peer
    ///
    /// # Arguments
    /// * `addr` - Target socket address
    ///
    /// # Returns
    /// * `Ok(TcpStream)` - Successfully connected stream
    /// * `Err(NeoLanError)` - Connection failed
    ///
    /// # Examples
    /// ```no_run
    /// # use feiqiu::network::tcp::TcpTransport;
    /// # use feiqiu::NeoLanError;
    /// # use std::net::SocketAddr;
    /// let addr = "192.168.1.100:8001".parse::<SocketAddr>().unwrap();
    /// let stream = TcpTransport::connect(addr)?;
    /// # Ok::<(), NeoLanError>(())
    /// ```
    pub fn connect(addr: SocketAddr) -> Result<TcpStream> {
        tracing::debug!("Connecting to TCP peer: {}", addr);

        let stream = TcpStream::connect(addr).map_err(NeoLanError::Network)?;

        tracing::info!("Connected to TCP peer: {}", addr);

        Ok(stream)
    }

    /// Send a file over TCP stream
    ///
    /// # Arguments
    /// * `stream` - TCP stream to send data over
    /// * `path` - Path to the file to send
    /// * `progress_callback` - Optional callback for progress updates
    ///
    /// # Returns
    /// * `Ok(u64)` - Number of bytes sent
    /// * `Err(NeoLanError)` - Send failed
    ///
    /// # Process
    /// 1. Open file
    /// 2. Read in chunks (4KB)
    /// 3. Send each chunk over TCP
    /// 4. Update progress if callback provided
    pub fn send_file<F>(
        mut stream: TcpStream,
        path: &Path,
        mut progress_callback: Option<F>,
    ) -> Result<u64>
    where
        F: FnMut(u64, u64), // (sent_bytes, total_bytes)
    {
        // Open file
        let mut file = std::fs::File::open(path).map_err(|e| {
            NeoLanError::FileTransfer(format!("Failed to open file {}: {}", path.display(), e))
        })?;

        // Get file size
        let file_size = file
            .metadata()
            .map_err(|e| NeoLanError::FileTransfer(format!("Failed to get file metadata: {}", e)))?
            .len();

        tracing::info!(
            "Sending file {} ({} bytes) via TCP",
            path.display(),
            file_size
        );

        let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];
        let mut total_sent = 0u64;

        // Read and send file in chunks
        loop {
            let n = file
                .read(&mut buffer)
                .map_err(|e| NeoLanError::FileTransfer(format!("Failed to read file: {}", e)))?;

            if n == 0 {
                break; // EOF
            }

            // Send chunk
            stream.write_all(&buffer[..n]).map_err(|e| {
                NeoLanError::FileTransfer(format!("Failed to send file data: {}", e))
            })?;

            total_sent += n as u64;

            // Update progress
            if let Some(ref mut callback) = progress_callback {
                callback(total_sent, file_size);
            }

            tracing::trace!(
                "Sent {}/{} bytes ({}%)",
                total_sent,
                file_size,
                (total_sent as f64 / file_size as f64 * 100.0) as u32
            );
        }

        // Flush stream
        stream
            .flush()
            .map_err(|e| NeoLanError::FileTransfer(format!("Failed to flush stream: {}", e)))?;

        tracing::info!("File send complete: {} bytes sent", total_sent);

        Ok(total_sent)
    }

    /// Receive a file over TCP stream
    ///
    /// # Arguments
    /// * `stream` - TCP stream to receive data from
    /// * `path` - Path to save the received file
    /// * `expected_size` - Expected file size (for progress and validation)
    /// * `progress_callback` - Optional callback for progress updates
    ///
    /// # Returns
    /// * `Ok(u64)` - Number of bytes received
    /// * `Err(NeoLanError)` - Receive failed
    ///
    /// # Process
    /// 1. Create output file
    /// 2. Read data from TCP stream in chunks (4KB)
    /// 3. Write chunks to file
    /// 4. Update progress if callback provided
    /// 5. Validate received size matches expected size
    pub fn receive_file<F>(
        mut stream: TcpStream,
        path: &Path,
        expected_size: u64,
        mut progress_callback: Option<F>,
    ) -> Result<u64>
    where
        F: FnMut(u64, u64), // (received_bytes, total_bytes)
    {
        // Create output file
        let mut file = std::fs::File::create(path).map_err(|e| {
            NeoLanError::FileTransfer(format!("Failed to create file {}: {}", path.display(), e))
        })?;

        tracing::info!(
            "Receiving file {} (expected {} bytes) via TCP",
            path.display(),
            expected_size
        );

        let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];
        let mut total_received = 0u64;

        // Read and write file in chunks
        loop {
            let n = stream.read(&mut buffer).map_err(|e| {
                NeoLanError::FileTransfer(format!("Failed to read from stream: {}", e))
            })?;

            if n == 0 {
                break; // Connection closed
            }

            // Write chunk to file
            file.write_all(&buffer[..n])
                .map_err(|e| NeoLanError::FileTransfer(format!("Failed to write file: {}", e)))?;

            total_received += n as u64;

            // Update progress
            if let Some(ref mut callback) = progress_callback {
                callback(total_received, expected_size);
            }

            tracing::trace!(
                "Received {}/{} bytes ({}%)",
                total_received,
                expected_size,
                (total_received as f64 / expected_size as f64 * 100.0) as u32
            );

            // Check if we've received all data
            if expected_size > 0 && total_received >= expected_size {
                break;
            }
        }

        // Flush file
        file.flush()
            .map_err(|e| NeoLanError::FileTransfer(format!("Failed to flush file: {}", e)))?;

        tracing::info!("File receive complete: {} bytes received", total_received);

        Ok(total_received)
    }

    /// Set read timeout for TCP stream
    ///
    /// # Arguments
    /// * `stream` - TCP stream to configure
    /// * `timeout_secs` - Timeout in seconds
    ///
    /// # Returns
    /// * `Ok(())` - Timeout set successfully
    /// * `Err(NeoLanError)` - Setting timeout failed
    pub fn set_read_timeout(stream: &TcpStream, timeout_secs: u64) -> Result<()> {
        let duration = std::time::Duration::from_secs(timeout_secs);
        stream
            .set_read_timeout(Some(duration))
            .map_err(NeoLanError::Network)
    }

    /// Set write timeout for TCP stream
    ///
    /// # Arguments
    /// * `stream` - TCP stream to configure
    /// * `timeout_secs` - Timeout in seconds
    ///
    /// # Returns
    /// * `Ok(())` - Timeout set successfully
    /// * `Err(NeoLanError)` - Setting timeout failed
    pub fn set_write_timeout(stream: &TcpStream, timeout_secs: u64) -> Result<()> {
        let duration = std::time::Duration::from_secs(timeout_secs);
        stream
            .set_write_timeout(Some(duration))
            .map_err(NeoLanError::Network)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_bind_available() {
        let (listener, port) = TcpTransport::bind_available().unwrap();

        assert!(port >= PORT_RANGE_START);
        assert!(port < PORT_RANGE_END);

        // Verify we can get the local address
        let addr = listener.local_addr().unwrap();
        assert_eq!(addr.port(), port);

        println!("Bound to port {}", port);
    }

    #[test]
    fn test_bind_multiple() {
        // Bind to multiple ports
        let (_listener1, port1) = TcpTransport::bind_available().unwrap();
        let (_listener2, port2) = TcpTransport::bind_available().unwrap();

        assert_ne!(port1, port2);
        assert!(port1 >= PORT_RANGE_START && port1 < PORT_RANGE_END);
        assert!(port2 >= PORT_RANGE_START && port2 < PORT_RANGE_END);

        println!("Bound to ports {} and {}", port1, port2);
    }

    #[test]
    fn test_connect_to_listener() {
        // Create a listener
        let (listener, port) = TcpTransport::bind_available().unwrap();

        // Spawn a thread to accept connection
        thread::spawn(move || {
            let _stream = listener.incoming().next().unwrap().unwrap();
            println!("Server accepted connection");
        });

        // Connect to the listener
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let _stream = TcpTransport::connect(addr).unwrap();

        println!("Client connected to {}", addr);

        // Give thread time to complete
        thread::sleep(Duration::from_millis(100));
    }

    #[test]
    fn test_send_and_receive_file() {
        // Create a test file
        let test_file = std::env::temp_dir().join("test_send.txt");
        let output_file = std::env::temp_dir().join("test_receive.txt");

        // Write test data (less than 4KB to fit in one chunk)
        let test_data = b"Hello, TCP File Transfer!";
        std::fs::write(&test_file, test_data).unwrap();

        // Create listener
        let (listener, port) = TcpTransport::bind_available().unwrap();
        let expected_size = test_data.len() as u64;

        // Spawn server thread
        let output_file_clone = output_file.clone();
        thread::spawn(move || {
            let stream = listener.incoming().next().unwrap().unwrap();

            // Receive file
            TcpTransport::receive_file::<fn(u64, u64)>(
                stream,
                &output_file_clone,
                expected_size,
                None,
            )
            .unwrap();

            println!("Server received file");
        });

        // Connect and send file
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let stream = TcpTransport::connect(addr).unwrap();

        let sent = TcpTransport::send_file::<fn(u64, u64)>(stream, &test_file, None).unwrap();

        assert_eq!(sent as usize, test_data.len());

        // Wait for server to finish
        thread::sleep(Duration::from_millis(200));

        // Verify received file
        let received_data = std::fs::read(&output_file).unwrap();
        assert_eq!(received_data, test_data);

        // Clean up
        std::fs::remove_file(&test_file).unwrap();
        std::fs::remove_file(&output_file).unwrap();

        println!("File transfer test completed successfully");
    }

    #[test]
    fn test_send_large_file() {
        // Create a larger test file (about 8KB, 2 chunks)
        let test_file = std::env::temp_dir().join("test_large.txt");
        let output_file = std::env::temp_dir().join("test_large_received.txt");

        let test_data = vec![b'A'; DEFAULT_BUFFER_SIZE * 2];
        std::fs::write(&test_file, &test_data).unwrap();

        // Create listener
        let (listener, port) = TcpTransport::bind_available().unwrap();
        let expected_size = test_data.len() as u64;

        // Spawn server thread
        let output_file_clone = output_file.clone();
        thread::spawn(move || {
            let stream = listener.incoming().next().unwrap().unwrap();
            TcpTransport::receive_file::<fn(u64, u64)>(
                stream,
                &output_file_clone,
                expected_size,
                None,
            )
            .unwrap();
        });

        // Connect and send
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let stream = TcpTransport::connect(addr).unwrap();

        let sent = TcpTransport::send_file::<fn(u64, u64)>(stream, &test_file, None).unwrap();

        assert_eq!(sent as usize, test_data.len());

        // Wait for server to finish
        thread::sleep(Duration::from_millis(300));

        // Verify
        let received_data = std::fs::read(&output_file).unwrap();
        assert_eq!(received_data.len(), test_data.len());
        assert_eq!(received_data, test_data);

        // Clean up
        std::fs::remove_file(&test_file).unwrap();
        std::fs::remove_file(&output_file).unwrap();

        println!("Large file transfer test completed");
    }

    #[test]
    fn test_send_with_progress_callback() {
        let test_file = std::env::temp_dir().join("test_progress.txt");
        let output_file = std::env::temp_dir().join("test_progress_received.txt");

        let test_data = vec![b'X'; DEFAULT_BUFFER_SIZE * 3];
        std::fs::write(&test_file, &test_data).unwrap();

        // Track progress
        let mut progress_updates = vec![];

        // Create listener
        let (listener, port) = TcpTransport::bind_available().unwrap();
        let expected_size = test_data.len() as u64;

        // Spawn server thread
        let output_file_clone = output_file.clone();
        thread::spawn(move || {
            let stream = listener.incoming().next().unwrap().unwrap();
            TcpTransport::receive_file::<fn(u64, u64)>(
                stream,
                &output_file_clone,
                expected_size,
                None,
            )
            .unwrap();
        });

        // Connect and send with progress
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let stream = TcpTransport::connect(addr).unwrap();

        TcpTransport::send_file::<_>(
            stream,
            &test_file,
            Some(|sent, total| {
                progress_updates.push((sent, total));
            }),
        )
        .unwrap();

        // Wait for completion
        thread::sleep(Duration::from_millis(300));

        // Verify progress was tracked
        assert!(!progress_updates.is_empty());
        assert_eq!(progress_updates.last().unwrap().0, test_data.len() as u64);

        println!("Progress updates: {}", progress_updates.len());

        // Clean up
        std::fs::remove_file(&test_file).unwrap();
        std::fs::remove_file(&output_file).unwrap();
    }

    #[test]
    fn test_set_timeouts() {
        // Try to connect to a port that's likely not listening
        // This may fail with connection refused, which is expected
        let result = TcpStream::connect("127.0.0.1:80");

        // If connection succeeds, test setting timeouts
        if let Ok(stream) = result {
            // Test setting timeouts
            let _ = TcpTransport::set_read_timeout(&stream, 30);
            let _ = TcpTransport::set_write_timeout(&stream, 30);
        }
        // If connection fails, that's also fine for this test
    }
}
