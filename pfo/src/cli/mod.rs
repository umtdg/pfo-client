use anyhow::Result;

use args::Args;

use crate::client::Client;

pub mod args;
pub mod commands;

pub async fn run(args: Args, client: Client) -> Result<()> {
    match args.command {
        args::Commands::Portfolio { command } => commands::portfolio::handle(command, client).await,
        args::Commands::Fund { command } => commands::fund::handle(command, client).await,
    }
}
