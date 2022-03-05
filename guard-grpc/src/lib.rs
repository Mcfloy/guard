use std::sync::Arc;
use tokio::sync::Mutex;

use tonic::{Request, Response, Status};

pub use definitions::enforcer_client::EnforcerClient;
use guard::permission::{Permission, PermissionRepository};
use guard::jwt;
use guard::jwt::Principal;

pub use crate::definitions::{EnforceRequest, EnforcerResponse};
use crate::definitions::enforcer_server::Enforcer;
pub use crate::definitions::enforcer_server::EnforcerServer;

mod definitions {
    tonic::include_proto!("guard");
}

pub struct GrpcServer<AccessRepo: PermissionRepository> {
    access_repository: Arc<Mutex<AccessRepo>>
}

impl<AccessRepo: PermissionRepository> GrpcServer<AccessRepo> {
    pub fn new(repository: Arc<Mutex<AccessRepo>>) -> Self {
        GrpcServer {
            access_repository: repository,
        }
    }
}

fn to_access(principal: &Principal, request: &EnforceRequest) -> Permission {
    Permission {
        subject: principal.sub.clone().to_lowercase(),
        namespace: principal.namespace.clone().to_lowercase(),
        domain: request.dom.clone().to_lowercase(),
        object: request.obj.clone().to_lowercase(),
        action: request.act.clone().to_lowercase()
    }
}

#[tonic::async_trait]
impl<R: PermissionRepository> Enforcer for GrpcServer<R> {
    async fn enforce(&self, request: Request<EnforceRequest>) -> Result<Response<EnforcerResponse>, Status> {
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

        let authorized = self.access_repository.lock().await.enforce(&to_access(&principal, &request)).await
            .map_err(|error| Status::internal(error.to_string()))
            .unwrap();
        Ok(Response::new(EnforcerResponse {
            authorized
        }))
    }
}

fn validate_request(request: EnforceRequest) -> bool {
    if request.dom.is_empty()
        || request.obj.is_empty()
        || request.act.is_empty() {
        return false;
    }
    true
}

const AUTHORIZATION_HEADER: &str = "authorization";
