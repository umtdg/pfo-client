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
    let client = match &args.host {
        Some(host) => {
            eprintln!("warning: -H/--host and -p/--port are deprecated; use -u/--base-url");
            client::PfoClient::new(host, args.port.unwrap_or(8080))
        }
        None => client::PfoClient::from_url(&args.base_url),
    }?;

    args.command.handle(client).await
}
