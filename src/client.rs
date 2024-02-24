/* Written by SniverDaBest
 * Last updated 2/23/24
 * I know this is bad code, just let me know how to fix it and *maybe* I'll fix it.
 */
use std::io::{self, Read, Write, Error};
use std::net::TcpStream;
use std::env;

#[derive(Debug)]
enum ClientError {
    BadAddressError,
    BadPortError,
    ConnectionFailError,
    IoError(Error),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ClientError::BadAddressError => write!(f, "Bad server address given."),
            ClientError::BadPortError => write!(f, "Bad port given."),
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
    if args.len() < 3 {
        eprintln!("FATAL: NOT ENOUGH ARGUMENTS. QUITTING...");
        return Err(ClientError::BadAddressError);
    }

    if args[2] == "" {
        eprintln!("FATAL: INVALID PORT GIVEN. QUITTING...");
        return Err(ClientError::BadPortError);
    }

    let servaddr = &args[1];
    let servport = &args[2];

    println!("Connecting to server at {servaddr} on port {servport}");
    let mut stream = TcpStream::connect(format!("{}:{}", servaddr, servport)).map_err(|_| ClientError::ConnectionFailError)?;
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input)?;
        let mut input_ascii = input.as_bytes();
        if input.trim() == "!quit" {
            println!("Bye!");
            return Ok(())
        }
        stream.write(input_ascii)?;
        let mut buffer = [0;   128];
        stream.read(&mut buffer)?;
        input = "".to_string();
    }
    Ok(())
}
