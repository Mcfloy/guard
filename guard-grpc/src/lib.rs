use tonic::{Request, Response, Status};
use guard::access::{Access, AccessRepository};
use crate::definitions::enforcer_server::Enforcer;
use crate::definitions::{EnforceRequest, EnforcerResponse};

pub mod definitions {
    tonic::include_proto!("guard");
}

pub struct MyEnforcer<R: AccessRepository> {
    repository: R
}

fn to_access(request: &EnforceRequest) -> Access {
    Access {
        subject: request.sub.clone(),
        namespace: request.ns.clone(),
        domain: request.dom.clone(),
        object: request.obj.clone(),
        action: request.act.clone()
    }
}

#[tonic::async_trait]
impl<R: AccessRepository> Enforcer for MyEnforcer<R> {
    async fn enforce(&self, request: Request<EnforceRequest>) -> Result<Response<EnforcerResponse>, Status> {
        let request = request.into_inner();
        if validate_request(request.clone()) {
            return Err(Status::invalid_argument("Request is incorrect"));
        }
        let authorized = self.repository.enforce(&to_access(&request)).await
            .map_err(|error| Status::internal(error.to_string()))
            .unwrap();
        Ok(Response::new(EnforcerResponse {
            authorized
        }))
    }
}

fn validate_request(request: EnforceRequest) -> bool {
    if request.sub.is_empty()
        || request.ns.is_empty()
        || request.dom.is_empty()
        || request.obj.is_empty()
        || request.act.is_empty() {
        return false;
    }
    true
}
