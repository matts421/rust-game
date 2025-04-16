use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server.");

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

    Ok(())
}
