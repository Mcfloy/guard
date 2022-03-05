use std::sync::Arc;
use poem::{Result};
use poem::web::{Data, Json};
use poem_openapi::{ApiResponse, OpenApi};
use tokio::sync::Mutex;
use guard::permission::{Permission, PermissionRepository};
use guard_postgres::PostgresRepository;
use crate::security::AuthenticatedUser;

pub struct AccessApi;

#[derive(ApiResponse)]
enum AccessResponse {
    #[oai(status = 201)]
    AccessCreated
}

#[OpenApi]
impl AccessApi {
    #[oai(path = "/access", method = "post")]
    async fn grant_access(
        &self,
        access_form: Json<Permission>,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        user: AuthenticatedUser
    ) -> Result<AccessResponse> {
        let form_ns = access_form.namespace.clone();
        let guard_access = Permission {
            subject: user.0.sub,
            namespace: "guard".to_string(),
            domain: form_ns.to_string(),
            object: "access".to_string(),
            action: "edit".to_string()
        };
        repository.0.lock().await.enforce(&guard_access).await.unwrap();

        // TODO: Check that the access already exists

        repository.0.lock().await.grant_permission(&access_form.0).await.unwrap();

        Ok(AccessResponse::AccessCreated)
    }
}
