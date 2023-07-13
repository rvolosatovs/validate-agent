//! User-Agent string validation client binary

#![warn(clippy::pedantic)]
#![warn(missing_docs)]

use std::process::ExitCode;

use anyhow::{anyhow, Context};
use clap::Parser;
use tonic::Request;
use tracing::{info, trace};
use tracing_subscriber::prelude::*;
use validate_agent_api::validator_client::ValidatorClient;
use validate_agent_api::{ValidationRequest, ValidationResponse};

/// User-Agent validation CLI
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server address
    #[arg(long, default_value_t = String::from("http://[::1]:50000"))]
    addr: String,
    /// User-Agent string
    user_agent: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().without_time())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let Args { user_agent, addr } = Args::parse();

    trace!("connect to `{addr}`");
    let mut client = ValidatorClient::connect(addr)
        .await
        .context("failed to connect to server")?;

    trace!(?user_agent, "validate User-Agent string");
    let ValidationResponse { allowed } = client
        .validate(Request::new(ValidationRequest { user_agent }))
        .await
        .map_err(|e| anyhow!(e.message().to_string()))
        .context("failed to validate User-Agent string")?
        .into_inner();
    if allowed {
        info!("User-Agent allowed");
        Ok(ExitCode::SUCCESS)
    } else {
        info!("User-Agent denied");
        Ok(ExitCode::FAILURE)
    }
}
