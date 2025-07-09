use anyhow::{Ok, Result};

use crate::cli::args::Args;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

impl Config {
    pub fn from_args(args: &Args) -> Result<Self> {
        let host = args.host.clone().unwrap_or_else(|| "localhost".to_string());
        let port = args.port.unwrap_or(8080);
        let debug = args.debug;

        Ok(Config { host, port, debug })
    }
}
