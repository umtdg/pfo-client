mod cli;
mod client;
mod fund;
mod portfolio;
mod problem_detail;

use anyhow::Result;
use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = client::Client::new(
        args.host.unwrap_or("localhost".into()),
        args.port.unwrap_or(8080),
    );

    args.command.handle(client).await
}
