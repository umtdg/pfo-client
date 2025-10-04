mod cli;
mod client;
mod config;
mod fund;
mod portfolio;
mod problem_detail;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = config::Config::from_args(&args).context("Failed to read config from args")?;
    let client = client::Client::new(config);

    args.command.handle(client).await
}
