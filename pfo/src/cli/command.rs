use anyhow::Result;

use crate::client::Client;

pub trait Command: clap::Subcommand {
    async fn handle(self, client: Client) -> Result<()>;
}
