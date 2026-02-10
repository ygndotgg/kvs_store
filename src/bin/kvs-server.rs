use std::{
    env,
    fmt::Display,
    io::BufReader,
    net::{SocketAddr, TcpListener, TcpStream},
};

use clap::Parser;
use kvs::SledKvsEngine;
use kvs::{KvStore, KvsEngine, Request, Response};
use log::{error, info};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: String,
    #[arg(long, default_value = "kvs")]
    engine: String,
}

#[derive(Debug)]
enum EngineName {
    Sled,
    Kvs,
}

impl Display for EngineName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            EngineName::Sled => "sled",
            EngineName::Kvs => "kvs",
        };
        write!(f, "{}", name)
    }
}

/// Handle one client connection: read Request, call engine, write Response
fn handle_client(stream: TcpStream, engine: &mut impl KvsEngine) {
    let peer = stream.peer_addr().ok();
    let reader = BufReader::new(&stream);

    let request: Request = match serde_json::from_reader(reader) {
        Ok(req) => req,
        Err(e) => {
            error!("failed to parse request: {}", e);
            return;
        }
    };

    let response = match request {
        Request::Set { key, value } => match engine.set(key, value) {
            Ok(()) => Response::Ok(None),
            Err(e) => Response::Err(e.to_string()),
        },
        Request::Get { key } => match engine.get(key) {
            Ok(val) => Response::Ok(val),
            Err(e) => Response::Err(e.to_string()),
        },
        Request::Remove { key } => match engine.remove(key) {
            Ok(()) => Response::Ok(None),
            Err(e) => Response::Err(e.to_string()),
        },
    };

    if let Err(e) = serde_json::to_writer(&stream, &response) {
        error!("failed to write response: {}", e);
    }

    if let Some(peer) = peer {
        info!("handled request from {}", peer);
    }
}

pub fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let cli = Cli::parse();

    let addr = cli.addr.parse::<SocketAddr>().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let engine = match cli.engine.to_lowercase().as_str() {
        "sled" => EngineName::Sled,
        "kvs" => EngineName::Kvs,
        _ => {
            eprintln!("Invalid engine name: {}", cli.engine);
            std::process::exit(1);
        }
    };

    info!("kvs-server version: {}", env!("CARGO_PKG_VERSION"));
    info!("engine: {}", engine);
    info!("listening on: {}", addr);

    let current_dir = env::current_dir().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let engine_file = current_dir.join("engine");
    let engine_name = if engine_file.exists() {
        let previous = std::fs::read_to_string(&engine_file).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
        let previous = previous.trim().to_string();
        let requested = cli.engine.to_lowercase();
        if requested != previous {
            eprintln!(
                "Wrong engine! Data was created with {} but {} was requested",
                previous, requested
            );
            std::process::exit(1);
        }
        previous
    } else {
        cli.engine.to_lowercase()
    };
    std::fs::write(&engine_file, &engine_name).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    // Open the engine ONCE, then listen and handle connections
    match engine {
        EngineName::Kvs => {
            let mut store = KvStore::open(&current_dir).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });
            run_server(addr, &mut store);
        }
        EngineName::Sled => {
            let mut store = SledKvsEngine::open(&current_dir).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });
            run_server(addr, &mut store);
        }
    }
}

fn run_server(addr: SocketAddr, engine: &mut impl KvsEngine) {
    let listener = TcpListener::bind(addr).unwrap_or_else(|e| {
        eprintln!("Failed to bind: {}", e);
        std::process::exit(1);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Ok(peer) = stream.peer_addr() {
                    info!("accepted connection from {}", peer);
                }
                handle_client(stream, engine);
            }
            Err(e) => {
                error!("connection failed: {}", e);
            }
        }
    }
}
