use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Set { key, value, addr } => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Command::Get { key, addr } => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Command::Rm { key, addr } => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
    }
}
