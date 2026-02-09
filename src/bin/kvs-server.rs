use std::{fmt::Display, net::SocketAddr, process};

use clap::Parser;
use log::info;

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

pub fn main() {
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
}
