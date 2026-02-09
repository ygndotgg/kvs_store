use clap::{Parser, Subcommand};
use kvs::KvsEngine;
use kvs::Result;
use kvs::kvs::KvStore;
use std::env;
use std::process;
#[derive(Parser)]
#[command(about,version,long_about=None)]
struct Cli {
    #[command(subcommand)]
    func: Method,
}

#[derive(Subcommand)]
enum Method {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    let current_dir = env::current_dir()?;

    match cli.func {
        Method::Set { key, value } => {
            let mut store = KvStore::open(&current_dir)?;
            store.set(key, value)?;
        }
        Method::Get { key } => {
            let mut store = KvStore::open(&current_dir)?;
            match store.get(key)? {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }
        }
        Method::Rm { key } => {
            let mut store = KvStore::open(&current_dir)?;
            if let Err(_) = store.remove(key) {
                println!("Key not found");
                process::exit(1);
            }
        }
    }

    Ok(())
}
