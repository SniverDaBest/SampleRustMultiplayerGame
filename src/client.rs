/* Written by SniverDaBest
 * Last updated 2/21/24
 */
use std::io::{self, Read, Write, Error};
use std::net::TcpStream;
use std::env;

#[derive(Debug)]
enum ClientError {
    BadAddressError,
    ConnectionFailError,
    IoError(Error),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ClientError::BadAddressError => write!(f, "No server address given."),
            ClientError::ConnectionFailError => write!(f, "Failed to connect to server."),
            ClientError::IoError(ref err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl std::error::Error for ClientError {}

// Implementing the From trait for std::io::Error to ClientError
impl From<io::Error> for ClientError {
    fn from(error: io::Error) -> Self {
        ClientError::IoError(error)
    }
}

fn main() -> Result<(), ClientError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <   2 {
        eprintln!("FATAL: NO SERVER ADDRESS GIVEN. QUITTING...");
        return Err(ClientError::BadAddressError);
    }

    let servaddr: &String = &args[1];
    println!("Connecting to server at {} on port   1169", servaddr);
    let mut stream = TcpStream::connect(format!("{}:1169", servaddr)).map_err(|_| ClientError::ConnectionFailError)?;
    stream.write(&[65])?;
    let mut buffer = [0;   128];
    stream.read(&mut buffer)?;
    Ok(())
}
