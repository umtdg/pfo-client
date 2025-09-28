use anyhow::Result;

use crate::cli::Args;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_args(args: &Args) -> Result<Self> {
        Ok(Config {
            host: args.host.clone().unwrap_or("localhost".into()),
            port: args.port.unwrap_or(8080),
        })
    }
}
