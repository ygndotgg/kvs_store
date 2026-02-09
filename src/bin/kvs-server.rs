use std::{
    fmt::Display,
    net::{SocketAddr, TcpListener},
    process,
};

use clap::Parser;
use log::{error, info};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: String,
    #[arg(long)]
    engine: Option<String>,
}

#[derive(Debug)]
enum EngineName {
    Sled,
    Kvs,
}

impl Display for EngineName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fd = match self {
            EngineName::Sled => "sled",
            EngineName::Kvs => "kvs",
        };
        write!(f, "{}", fd)
    }
}

pub fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let addr = cli.addr.parse::<SocketAddr>().unwrap_or_else(|k| {
        println!("{:?}", k);
        process::exit(1);
    });

    // let engine:EngineName =
    let engine = match cli.engine {
        Some(s) => s,
        None => "kvs".to_string(),
    };
    let engine = match engine.to_lowercase().as_str() {
        "sled" => EngineName::Sled,
        "kvs" => EngineName::Kvs,
        _ => {
            eprintln!("Engine Not found");
            std::process::exit(1);
        }
    };
    info!("kvs-server version:{}", env!("CARGO_PKG_VERSION"));
    info!("engine:{}", engine);
    info!("listening on:{}", addr);
    let listener = TcpListener::bind(addr).unwrap_or_else(|e| {
        eprintln!("Failed to bind:{}", e);
        std::process::exit(1);
    });
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => {
                info!("accepted connection from {}", stream.peer_addr()?);
            }
            Err(e) => {
                error!("connection failed:{}", e);
            }
        };
    }
    Ok(())
}
