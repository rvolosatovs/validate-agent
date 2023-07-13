//! User-Agent string validation server library

#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![forbid(clippy::unwrap_used)]

use tonic::transport::server::Router;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status};
use tracing::{instrument, trace};
use validate_agent_api::validator_server::ValidatorServer;
use validate_agent_api::{validator_server, ValidationRequest, ValidationResponse};
use woothee::parser::WootheeResult;

/// User-Agent string validator
#[derive(Default)]
pub struct Validator {
    ua_parser: woothee::parser::Parser,
}

impl Validator {
    /// Contruct a [Validator] and return it as a [tonic::transport::server::Router]
    #[instrument]
    pub fn router() -> Router {
        Self::default().into()
    }
}

impl From<Validator> for Router {
    fn from(v: Validator) -> Self {
        Server::builder().add_service(ValidatorServer::new(v))
    }
}

#[tonic::async_trait]
impl validator_server::Validator for Validator {
    #[instrument(skip(self))]
    async fn validate(
        &self,
        request: Request<ValidationRequest>,
    ) -> tonic::Result<Response<ValidationResponse>> {
        let ValidationRequest { user_agent } = request.into_inner();

        trace!(user_agent, "parse User-Agent string");
        let WootheeResult { name, .. } = self
            .ua_parser
            .parse(&user_agent)
            .ok_or_else(|| Status::new(Code::InvalidArgument, "invalid User-Agent string"))?;
        match name {
            "Firefox" => Ok(Response::new(ValidationResponse { allowed: true })),
            "Safari" => Ok(Response::new(ValidationResponse::default())),
            _ => Err(Status::new(Code::Unimplemented, "unknown User-Agent")),
        }
    }
}
