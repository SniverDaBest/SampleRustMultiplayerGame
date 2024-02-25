/* Made by SniverDaBest
 * Last modified 2/23/24
 * I know this is insecure, but if you send an issue on GitHub, I *maybe* will fix it.
 */
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn handle_client(mut stream: TcpStream, ip_sent_messages: Arc<Mutex<HashMap<String, bool>>>, connections: Arc<Mutex<Vec<TcpStream>>>) -> Result<(), Error> {
    let peer_addr = stream.peer_addr()?.to_string();
    let mut name: Option<String> = None;
    println!("Got client: {}", peer_addr);
    let mut buf = [0;   512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read ==   0 {
            println!("Client disconnected: {}", peer_addr);
            return Ok(());
        }
        let message = String::from_utf8_lossy(&buf[..bytes_read]).to_string();

        let mut ip_sent_messages = ip_sent_messages.lock().unwrap();
        if name.is_none() {
            // This is the first message, treat it as the username
            name = Some(message.clone());
            println!("Client name set to: {}", name.as_ref().unwrap());
            ip_sent_messages.insert(peer_addr.clone(), true);
        } else {
            // This is not the first message, broadcast it to all other clients
            println!("{}: {}", name.as_ref().unwrap(), message);
            let mut connections = connections.lock().unwrap();
            for client_stream in connections.iter_mut() {
                if client_stream.peer_addr()? != stream.peer_addr()? {
                    client_stream.write(&buf[..bytes_read])?;
                }
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1169").expect("Could not bind");
    let ip_sent_messages: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    let connections: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Failed to accept connection: {}", e),
            Ok(stream) => {
                let ip_sent_messages = Arc::clone(&ip_sent_messages);
                let connections = Arc::clone(&connections);
                thread::spawn(move || {
                    connections.lock().unwrap().push(stream.try_clone().expect("Failed to clone stream"));
                    handle_client(stream, ip_sent_messages, connections).unwrap_or_else(|error| eprintln!("Error handling client: {:?}", error));
                });
            }
        }
    }
}

