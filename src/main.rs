#[cfg(feature = "gtk")]
#[macro_use]
extern crate gtk_macros;

use std::env;
use std::error::Error;

use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = crate::consts::APP_NAME)]
#[clap(author = "louib")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Multi-Factor authentication agent for Linux.", long_about = None)]
struct MFAAgent {
    /// Run the agent as a proxy
    #[clap(long, short)]
    proxy: bool,

    /// Prompt for password on the command line
    #[clap(long)]
    password_prompt: bool,
}

mod api;
#[cfg(feature = "gtk")]
mod app;
#[cfg(feature = "bluetooth")]
mod bluetooth;
mod config;
mod connection;
mod hid;
mod consts;
mod event;
mod logger;
mod secrets;
mod tcp;
mod utils;
#[cfg(feature = "gtk")]
mod widgets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();

    let args = MFAAgent::parse();

    #[cfg(feature = "gtk")]
    crate::app::MFAAgentApplication::start_app();

    Ok(())
}
