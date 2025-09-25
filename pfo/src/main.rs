mod cli;
mod client;
mod config;
mod output;

use anyhow::{Context, Result};
use clap::Parser;
use cli::args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = config::Config::from_args(&args).context("Failed to read config from args")?;
    let client = client::Client::new(config);

    cli::run(args, client).await
}
