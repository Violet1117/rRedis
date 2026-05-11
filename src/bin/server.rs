use std::{io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}};
use std::string::String;
use std::thread;

fn handle_client(stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut stream = stream;
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            println!("Received request: {}", request);
        },
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
        }
    }
    let response = "Hello from the server!";
    if let Err(e) = stream.write(response.as_bytes()) {
        eprintln!("Failed to write to client: {}", e);
    }
}

fn main() -> std::io::Result<()>{
    let addrs = [
        SocketAddr::from(([127,0,0,1], 8080))
    ];
    let lisener = TcpListener::bind(addrs[0])?;
    let mut threads = Vec::new();

    for stream in lisener.incoming() {
        let stream = stream?;
        let thread = thread::spawn(move || {
            handle_client(stream);
        });
        threads.push(thread);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    Ok(()) 
}
