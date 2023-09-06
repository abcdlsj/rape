
#[macro_use]
extern crate log;
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

mod client;
mod packet;
mod proxy;
mod server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Rape is a tunnel/ngork tool.")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Server starting")]
    Server {
        #[arg(short, long, default_value = "8910")]
        port: u16,
    },
    #[command(about = "Tunnel management")]
    Tunnel {
        #[arg(required = true, short, long)]
        port: u16,
        #[arg(short, long, default_value = "127.0.0.1:8910")]
        server_addr: String,
    },
}

#[tokio::main]
async fn run(cli: Cli) -> Result<()> {
    match cli.commands {
        Commands::Server { port } => {
            info!("Server starting on {}", port);
            server::start(port).await.unwrap();
        }
        Commands::Tunnel { server_addr, port } => {
            info!("Server addr: {}", server_addr);
            client::start(port, server_addr).await.unwrap();
        }
    }

    Ok(())
}

fn main() {
    pretty_env_logger::init();

    run(Cli::parse()).unwrap();
}
