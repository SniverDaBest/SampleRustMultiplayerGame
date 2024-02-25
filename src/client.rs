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
    BadNameError,
    BadArgsError,
    ConnectionFailError,
    IoError(Error),
    ReadFail(Error),
    SendFail(Error),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ClientError::BadAddressError => write!(f, "Bad server address given."),
            ClientError::BadPortError => write!(f, "Bad port given."),
            ClientError::BadNameError => write!(f, "Bad username given."),
            ClientError::BadArgsError => write!(f, "Bad arguments given."),
            ClientError::ConnectionFailError => write!(f, "Failed to connect to server."),
            ClientError::IoError(ref err) => write!(f, "IO Error: {}", err),
            ClientError::ReadFail(ref err) => write!(f, "Failed to get data from server. Error: {}", err),
            ClientError::SendFail(ref err) => write!(f, "Failed to send data to server. Error: {}", err),
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
    if args.len() != 4 {
        eprintln!("FATAL: TOO FEW/TOO MANY ARGUMENTS. QUITTING...");
        return Err(ClientError::BadArgsError);
    }

    if args[1] == "" {
        eprintln!("FATAL: INVALID ADDRESS GIVEN. QUITTING...");
        return Err(ClientError::BadAddressError);
    }
    
    if args[2] == "" {
        eprintln!("FATAL: INVALID PORT GIVEN. QUITTING...");
        return Err(ClientError::BadPortError);
    }

    if args[3] == "" {
        eprintln!("FATAL: INVALID USERNAME GIVEN. QUITTING...");
        return Err(ClientError::BadNameError);
    }

    let servaddr = &args[1];
    let servport = &args[2];
    let mut name = &args[3];
    let mut name_bytes = name.as_bytes();

    println!("Connecting to server at {servaddr} on port {servport}");
    let mut stream = TcpStream::connect(format!("{}:{}", servaddr, servport)).map_err(|_| ClientError::ConnectionFailError)?;
    stream.write(&name_bytes);
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)?;
        let input_trimmed = input.trim(); // Trim the input for comparison
        if input_trimmed == "!quit" || input_trimmed == "!q" {
            println!("Bye!");
            return Ok(())
        }

        if input_trimmed.contains("!name") || input_trimmed.contains("!n") {
            /*print!("Change your name to: ");
            io::stdin().read_line(&mut input)?; // Modify input here
            name = &input.trim().to_string(); // Trim the new input
            stream.write(name.as_bytes())?;
            println!("Successfully set your name to \"{}\".", name);*/
            println!("Unimplemented. Check in a later version if it's added.");
        } else {
            // Only create input_ascii if we're not changing the name
            let input_ascii = input.as_bytes();
            stream.write(input_ascii)?;
        }

        if input_trimmed.contains("!help") || input_trimmed.contains("!h") {
            println!("!q[uit]          Quits this application.");
            println!("!n[ame] [name]   Changes your name.");
            println!("!h[elp]          Shows this message.");
        } 
        input.clear(); // Clear the input for the next iteration
    } 
    Ok(())
}
