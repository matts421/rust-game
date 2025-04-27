use std::io::{Read, Write};
use std::{thread, time::Duration};
use std::net::{TcpListener, TcpStream, UdpSocket};

use rust_game::DISCOVERY_PORT;
use rust_game::ECHO_PORT;
use rust_game::MULTICAST_IP;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection was closed
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                // Echo it back
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            }
        }
    }
}

fn broadcast(socket: &UdpSocket) -> std::io::Result<()> {
    let message = format!("ECHO_SERVER:{}", ECHO_PORT);
    let message_bytes = message.as_bytes();

    loop {
        socket.send_to(message_bytes, format!("{}:{}", MULTICAST_IP, DISCOVERY_PORT))?;
        thread::sleep(Duration::from_secs(2));
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // any available port
    socket.set_broadcast(true)?;
    std::thread::spawn(move || broadcast(&socket));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", ECHO_PORT))?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected");
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}