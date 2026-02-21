use std::net::{Shutdown, SocketAddr, TcpStream};

use clap::{Parser, Subcommand};
use kvs::{Request, Response};
// No local Result type — we handle errors explicitly with unwrap_or_else/eprintln

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Set {
        key: String,
        value: String,
        #[arg(long, default_value = "127.0.0.1:4000")]
        addr: String,
    },
    Get {
        key: String,
        #[arg(long, default_value = "127.0.0.1:4000")]
        addr: String,
    },
    Rm {
        key: String,
        #[arg(long, default_value = "127.0.0.1:4000")]
        addr: String,
    },
}

/// Helper: connect to server, send request, read response
pub fn send_request(addr: &str, request: &Request) -> Response {
    // Parse address
    let addr: SocketAddr = addr.parse().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    // Connect
    let mut stream = TcpStream::connect(addr).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    // Write request as JSON
    serde_json::to_writer(&mut stream, request).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    // Shut down write half so server knows we're done sending
    stream.shutdown(Shutdown::Write).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    // Read response
    let response: Response = serde_json::from_reader(&mut stream).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    response
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Set { key, value, addr } => {
            let response = send_request(&addr, &Request::Set { key, value });
            match response {
                Response::Ok(_) => {}
                Response::Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        Command::Get { key, addr } => {
            let response = send_request(&addr, &Request::Get { key });
            match response {
                Response::Ok(Some(value)) => println!("{}", value),
                Response::Ok(None) => println!("Key not found"),
                Response::Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        Command::Rm { key, addr } => {
            let response = send_request(&addr, &Request::Remove { key });
            match response {
                Response::Ok(_) => {}
                Response::Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
