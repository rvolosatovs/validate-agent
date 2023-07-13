use std::net::Ipv6Addr;

use anyhow::{ensure, Context};
use tokio::{net::TcpListener, sync::oneshot};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{Code, Request};
use tracing_subscriber::prelude::*;
use validate_agent_api::{
    validator_client::ValidatorClient, ValidationRequest, ValidationResponse,
};
use validate_agent_server::Validator;

#[tokio::test]
async fn integration() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty().without_time())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::new("info,validate_agent=trace")
            }),
        )
        .init();

    let tcp = TcpListener::bind((Ipv6Addr::UNSPECIFIED, 0))
        .await
        .context("failed to bind on TCP socket")?;
    let port = tcp
        .local_addr()
        .context("failed to query local address")?
        .port();
    let (abort_tx, abort_rx) = oneshot::channel();

    let server = tokio::spawn(async {
        Validator::router()
            .serve_with_incoming_shutdown(TcpListenerStream::new(tcp), async {
                abort_rx.await.expect("failed to await abort");
            })
            .await
            .context("failed to serve")
    });

    // NOTE: Examples were taken from
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent

    let mut client = ValidatorClient::connect(format!("http://[::1]:{port}"))
        .await
        .context("failed to connect to server")?;
    for user_agent in [
        "🦀",
        "invalid agent",
        "PostmanRuntime/7.26.5", // woothee fails to parse this agent
    ] {
        let res = client
            .validate(Request::new(ValidationRequest {
                user_agent: user_agent.into(),
            }))
            .await;
        ensure!(matches!(res, Err(e) if e.code() == Code::InvalidArgument));
    }

    for user_agent in [
        "", // woothee assumes this agent to be valid
        "curl/7.64.1",
        "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
        "Mozilla/5.0 (compatible; YandexAccessibilityBot/3.0; +http://yandex.com/bots)",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59",
        "Opera/9.60 (Windows NT 6.0; U; en) Presto/2.1.1",
        "Opera/9.80 (Macintosh; Intel Mac OS X; U; en) Presto/2.2.15 Version/10.00",
        "Chrome/51.0.2704.106 Safari/537.36 OPR/38.0.2220.41",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko)",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko)",
        "Chrome/51.0.2704.103 Safari/537.36",
    ] {
        let res = client
            .validate(Request::new(ValidationRequest {
                user_agent: user_agent.into(),
            }))
            .await;
        ensure!(matches!(res, Err(e) if e.code() == Code::Unimplemented));
    }

    for user_agent in [
        "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.1 Mobile/15E148 Safari/604.1",
    ] {
        let ValidationResponse { allowed } = client
            .validate(Request::new(ValidationRequest {
                user_agent: user_agent.into(),
            }))
            .await
            .context("failed to validate User-Agent string")?
            .into_inner();
        ensure!(!allowed)
    }

    for user_agent in [
        "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X x.y; rv:42.0) Gecko/20100101 Firefox/42.0",
    ] {
        let ValidationResponse { allowed } = client
            .validate(Request::new(ValidationRequest {
                user_agent: user_agent.into(),
            }))
            .await
            .context("failed to validate User-Agent string")?
            .into_inner();
        ensure!(allowed)
    }

    abort_tx.send(()).expect("failed abort server execution");
    server.await?
}
