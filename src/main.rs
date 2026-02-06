use clap::{Parser, Subcommand};
use kvs::kvs::KvStore;

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
enum ResultValue {
    Optional(Option<String>),
    Nothing,
}

pub fn main() {
    let ag = Cli::parse();
    let mut d = KvStore::new();
    let pkd: ResultValue = match ag.func {
        Method::Set { key, value } => {
            d.set(key, value);
            println!("Setted");
            ResultValue::Nothing
        }
        Method::Get { key } => ResultValue::Optional(d.get(key)),
        Method::Rm { key } => {
            println!("Removed");
            d.remove(key);
            ResultValue::Nothing
        }
    };
    match pkd {
        ResultValue::Optional(st) => {
            println!("{:?}", st);
        }
        ResultValue::Nothing => {}
    }
}
