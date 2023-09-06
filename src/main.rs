use anyhow::{Ok, Result};
use clap::Parser;

#[macro_use]
extern crate log;

mod forward;

#[derive(Parser, Debug)]
#[command(
    author = "abcdlsj",
    version = "0.1.0",
    about = "Rape is a forward tool like socat",
    long_about = "Rape is a forward tool like socat"
)]
pub struct Cli {
    #[arg(required = true, long, short)]
    src: String,
    #[arg(required = true, long, short)]
    dst: String,
}

#[tokio::main]
async fn run(cli: Cli) -> Result<()> {
    let Cli { src, dst } = cli;
    {
        info!("Forwarding {} to {}", src, dst);
        forward::run(src, dst).await?;
    }

    Ok(())
}

fn main() {
    pretty_env_logger::init();
    run(Cli::parse()).unwrap();
}
