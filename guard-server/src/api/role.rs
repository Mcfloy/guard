use std::sync::Arc;

use poem::{Error, Result};
use poem::web::Data;
use poem_openapi::{ApiResponse, Object, OpenApi};
use poem_openapi::payload::Json;
use tokio::sync::Mutex;

use guard::role::{Role, RoleRepository};
use guard_postgres::PostgresRepository;

use crate::api::jwt::AuthenticatedUser;
use crate::StatusCode;

pub struct RoleApi;

#[derive(Object)]
struct RoleList {
    roles: Vec<Role>
}

#[derive(ApiResponse)]
enum RoleResponse {
    #[oai(status = 200)]
    List(Json<RoleList>)
}

#[OpenApi]
impl RoleApi {
    #[oai(path = "/roles", method = "get")]
    async fn get_roles(
        &self,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        user: AuthenticatedUser
    ) -> Result<RoleResponse> {
        let roles = repository.0.lock().await
            .list_roles(&user.0.namespace, "*", &user.0.sub).await
            .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(RoleResponse::List(Json(RoleList {
            roles
        })))
    }
}
