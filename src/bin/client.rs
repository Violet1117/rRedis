use std::{io::{Read, Write}};
use std::string::String;
fn main() -> std::io::Result<()> {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:8080")?;
    let request = "Hello from the client!";
    stream.write(request.as_bytes())?;
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let response = String::from_utf8_lossy(&buffer);
            println!("Received response: {}", response);
        },
        Err(e) => {
            eprintln!("Failed to read from server: {}", e);
        }
    }
    Ok(())
}
