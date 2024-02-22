use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Got client: {}", stream.peer_addr()?);
    let mut buf = [0;  512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read ==  0 {
            return Ok(());
        }
        // Process the data read from the stream.
        // For example, you could print it:
        println!("Received: {}", String::from_utf8_lossy(&buf[..bytes_read]));
        // Echo the data back to the client.
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1169").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Failed to accept connection: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("Error handling client: {:?}", error));
                });
            }
        }
    }
}
