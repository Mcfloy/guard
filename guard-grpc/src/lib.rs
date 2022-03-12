use std::sync::Arc;
use tokio::sync::Mutex;

use tonic::{Request, Response, Status};

pub use definitions::enforcer_client::EnforcerClient;
use guard::enforce::{EnforceRepository, EnforceRequest};
use guard::jwt;
use guard::jwt::Principal;

pub use crate::definitions::{EnforceRequest as ServerEnforceRequest, EnforcerResponse};
use crate::definitions::enforcer_server::Enforcer;
pub use crate::definitions::enforcer_server::EnforcerServer;

mod definitions {
    tonic::include_proto!("guard");
}

pub struct GrpcServer<EnforceRepo: EnforceRepository> {
    permission_repository: Arc<Mutex<EnforceRepo>>
}

impl<EnforceRepo: EnforceRepository> GrpcServer<EnforceRepo> {
    pub fn new(repository: Arc<Mutex<EnforceRepo>>) -> Self {
        GrpcServer {
            permission_repository: repository,
        }
    }
}

fn to_permission(principal: &Principal, request: &ServerEnforceRequest) -> EnforceRequest {
    EnforceRequest {
        subject: principal.sub.clone().to_lowercase(),
        namespace: principal.namespace.clone().to_lowercase(),
        domain: request.dom.clone().to_lowercase(),
        object: request.obj.clone().to_lowercase(),
        action: request.act.clone().to_lowercase()
    }
}

#[tonic::async_trait]
impl<R: EnforceRepository> Enforcer for GrpcServer<R> {
    async fn enforce(&self, request: Request<ServerEnforceRequest>) -> Result<Response<EnforcerResponse>, Status> {
        let principal = match request.metadata().get(AUTHORIZATION_HEADER) {
            Some(header) => {
                jwt::decode(header.to_str().unwrap())
                    .map_err(|_| Status::unauthenticated("Authorization header is invalid."))
            },
            None => Err(Status::unauthenticated("Authorization header is empty."))
        }?;
        let request = request.into_inner();
        if !validate_request(request.clone()) {
            return Err(Status::invalid_argument("Request is incorrect"));
        }

        let authorized = self.permission_repository.lock().await.enforce(&to_permission(&principal, &request)).await
            .map_err(|error| Status::internal(error.to_string()))
            .unwrap();
        Ok(Response::new(EnforcerResponse {
            authorized
        }))
    }
}

fn validate_request(request: ServerEnforceRequest) -> bool {
    if request.dom.is_empty()
        || request.obj.is_empty()
        || request.act.is_empty() {
        return false;
    }
    true
}

const AUTHORIZATION_HEADER: &str = "authorization";
