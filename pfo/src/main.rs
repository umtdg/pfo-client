mod cli;
mod client;
mod fund;
mod none_serialize;
mod portfolio;
mod problem_detail;
mod query;

use anyhow::Result;
use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let env = env_logger::Env::default()
        .filter_or("PFO_LOG_LEVEL", "info")
        .write_style_or("PFO_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let args = Args::parse();

    let client = client::PfoClient::new(
        args.host.unwrap_or("localhost".into()),
        args.port.unwrap_or(8080),
    )?;

    args.command.handle(client).await
}
