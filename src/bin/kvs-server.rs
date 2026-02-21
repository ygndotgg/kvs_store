use std::{env, fmt::Display, fs::create_dir, net::SocketAddr};

use clap::Parser;
use kvs::thread_pool::SharedQueueThreadPool;
use kvs::thread_pool::ThreadPool;
use kvs::{KvServer, KvStore, SledKvsEngine};
use log::info;

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

    let mut current_dir = env::current_dir().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    current_dir.push("logs");
    create_dir(&current_dir).unwrap_or_else(|e| {
        eprintln!("Unable to create: {}", e);
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

    let pool = SharedQueueThreadPool::new(num_cpus::get() as u32).unwrap_or_else(|e| {
        eprintln!("Failed to create thread pool: {}", e);
        std::process::exit(1);
    });

    match engine {
        EngineName::Kvs => {
            let store = KvStore::open(&current_dir).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });
            let server = KvServer::new(addr, store, pool).unwrap_or_else(|e| {
                eprintln!("Failed to start server: {}", e);
                std::process::exit(1);
            });
            server.run();
        }
        EngineName::Sled => {
            let store = SledKvsEngine::open(&current_dir).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });
            let server = KvServer::new(addr, store, pool).unwrap_or_else(|e| {
                eprintln!("Failed to start server: {}", e);
                std::process::exit(1);
            });
            server.run();
        }
    }
}
