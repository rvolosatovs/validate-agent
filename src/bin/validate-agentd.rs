//! User-Agent string validation server binary

#![warn(clippy::pedantic)]
#![warn(missing_docs)]

use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::process::ExitCode;

use anyhow::Context;
use clap::Parser;
use tokio::signal;
use tracing::{error, trace};
use tracing_subscriber::prelude::*;
use validate_agent_server::Validator;

/// User-Agent validation service
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address to listen on
    #[arg(long, default_value_t = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 50000))]
    addr: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let Args { addr } = Args::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().without_time())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let shutdown = async {
        if let Err(e) = signal::ctrl_c()
            .await
            .context("failed to listen for Ctrl-C")
        {
            error!("{e}");
        }
    };
    trace!("serve on {addr}");
    Validator::router()
        .serve_with_shutdown(addr, shutdown)
        .await?;
    Ok(ExitCode::SUCCESS)
}
