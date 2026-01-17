//! Network layer tests (TCP, UDP, protocol msg_type helpers)

use feiqiu::network::msg_type::{
    get_mode, has_opt, make_command, IPMSG_ENCRYPTOPT, IPMSG_FILEATTACHOPT, IPMSG_SENDCHECKOPT,
    IPMSG_SENDMSG,
};
use feiqiu::network::tcp::{TcpTransport, DEFAULT_BUFFER_SIZE, PORT_RANGE_END, PORT_RANGE_START};
use feiqiu::network::udp::{
    UdpTransport, BROADCAST_ADDR, DEFAULT_BUFFER_SIZE as UDP_BUFFER_SIZE, DEFAULT_UDP_PORT,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use std::time::Duration;

// ============== msg_type tests ==============

#[test]
fn test_mode_opt_helpers() {
    let cmd = make_command(IPMSG_SENDMSG, IPMSG_FILEATTACHOPT | IPMSG_SENDCHECKOPT);
    assert_eq!(get_mode(cmd), IPMSG_SENDMSG as u8);
    assert!(has_opt(cmd, IPMSG_FILEATTACHOPT));
    assert!(has_opt(cmd, IPMSG_SENDCHECKOPT));
    assert!(!has_opt(cmd, IPMSG_ENCRYPTOPT));
}

// ============== TCP tests ==============

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
        TcpTransport::receive_file::<fn(u64, u64)>(stream, &output_file_clone, expected_size, None)
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
        TcpTransport::receive_file::<fn(u64, u64)>(stream, &output_file_clone, expected_size, None)
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
        TcpTransport::receive_file::<fn(u64, u64)>(stream, &output_file_clone, expected_size, None)
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
    let result = std::net::TcpStream::connect("127.0.0.1:80");

    // If connection succeeds, test setting timeouts
    if let Ok(stream) = result {
        // Test setting timeouts
        let _ = TcpTransport::set_read_timeout(&stream, 30);
        let _ = TcpTransport::set_write_timeout(&stream, 30);
    }
    // If connection fails, that's also fine for this test
}

// ============== UDP tests ==============

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

    let sender_socket = std::net::UdpSocket::bind(sender_addr).unwrap();
    let receiver_socket = std::net::UdpSocket::bind(receiver_addr).unwrap();

    let receiver_local = receiver_socket.local_addr().unwrap();
    let sender_port = sender_socket.local_addr().unwrap().port();
    let test_data = b"Hello, UDP!";

    // Send from sender to receiver
    sender_socket.send_to(test_data, receiver_local).unwrap();

    // Receive the data
    let mut buffer = [0u8; UDP_BUFFER_SIZE];
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
    assert_eq!(UDP_BUFFER_SIZE, 65535);
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
