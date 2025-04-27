use std::io::{self, Write, Read};
use std::net::{TcpStream, UdpSocket};
use std::net::Ipv4Addr;

use rust_game::DISCOVERY_PORT;
use rust_game::ECHO_PORT;
use rust_game::MULTICAST_IP;

fn discover() -> std::io::Result<Option<String>> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", DISCOVERY_PORT))?;
    
    socket.join_multicast_v4(
        &MULTICAST_IP.parse().unwrap(),
        &Ipv4Addr::UNSPECIFIED,
    )?;
    let mut buf = [0; 1024];
    
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let msg = String::from_utf8_lossy(&buf[..amt]);
        println!("Discovered server: {} - {}", src.ip(), msg);

        if msg.contains("ECHO_SERVER") {
            return Ok(Some(format!("{}", src.ip())));
        }
    }
}

fn main() -> std::io::Result<()> {
    match discover() {
        Ok(Some(server)) => {
            let mut stream = TcpStream::connect(format!("{}:{}", server, ECHO_PORT))?;
            println!("Connected to server: {}:{}", server, ECHO_PORT);

            let stdin = io::stdin();
            loop {
                print!("You: ");
                io::stdout().flush()?; // Flush to show prompt

                let mut input = String::new();
                stdin.read_line(&mut input)?;

                if input.trim() == "exit" {
                    break;
                }

                stream.write_all(input.as_bytes())?;

                let mut buffer = [0; 512];
                let n = stream.read(&mut buffer)?;
                println!("Echoed: {}", String::from_utf8_lossy(&buffer[..n]));
            }
        }
        Ok(None) => {
            println!("No servers found.");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
