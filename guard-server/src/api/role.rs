use std::sync::Arc;

use poem::{Result};
use poem::i18n::Locale;
use poem::web::{Data};
use poem_openapi::{ApiResponse, Object, OpenApi};
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use serde::Deserialize;
use tokio::sync::Mutex;
use guard::enforce::EnforceRequest;

use guard::role::{Role, RoleRepository};
use guard_postgres::PostgresRepository;

use crate::api::jwt::AuthenticatedUser;
use crate::error::{handle_enforce, UnknownError};

pub struct RoleApi;

#[derive(Object)]
struct RoleList {
    roles: Vec<Role>,
}

#[derive(ApiResponse)]
enum RoleResponse {
    /// Role has been assigned
    #[oai(status = 201)]
    RoleAssigned,
    /// Role has been removed
    #[oai(status = 204)]
    RoleRemoved,
    /// Role is already assigned
    #[oai(status = 409)]
    RoleAlreadyAssigned(PlainText<String>)
}

#[derive(Object, Deserialize)]
struct RoleRequest {
    pub subject: String,
    pub domain: String,
    pub role: String,
}

impl Into<Role> for RoleRequest {
    fn into(self) -> Role {
        Role {
            subject: self.subject,
            domain: self.domain,
            role: self.role
        }
    }
}

#[OpenApi]
impl RoleApi {
    #[oai(path = "/namespaces/:id/roles", method = "post")]
    async fn grant_role(
        &self,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        locale: Locale,
        user: AuthenticatedUser,
        id: Path<String>,
        request: Json<RoleRequest>
    ) -> Result<RoleResponse> {
        let namespace = id.0;
        let enforce_request = EnforceRequest {
            subject: user.0.sub,
            namespace: "guard".to_owned(),
            domain: namespace.to_owned(),
            object: "role".to_owned(),
            action: "edit".to_owned()
        };
        handle_enforce(&locale, &repository, &enforce_request).await?;

        match repository.0.lock().await.assign_role(&namespace, &request.0.into()).await {
            Ok(_) => Ok(RoleResponse::RoleAssigned),
            Err(_) => {
                let message = locale
                    .text("role-already-assigned")
                    .unwrap_or_else(|_| "error".to_owned());

                Ok(RoleResponse::RoleAlreadyAssigned(PlainText(message)))
            }
        }
    }

    #[oai(path = "/namespaces/:id/roles", method = "delete")]
    async fn remove_role(
        &self,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        locale: Locale,
        user: AuthenticatedUser,
        id: Path<String>,
        request: Json<RoleRequest>
    ) -> Result<RoleResponse> {
        let namespace = id.0;
        let enforce_request = EnforceRequest {
            subject: user.0.sub,
            namespace: "guard".to_owned(),
            domain: namespace.to_owned(),
            object: "role".to_owned(),
            action: "edit".to_owned()
        };
        handle_enforce(&locale, &repository, &enforce_request).await?;

        match repository.0.lock().await.remove_role(&namespace, &request.0.into()).await {
            Ok(_) => Ok(RoleResponse::RoleRemoved),
            Err(error) => {
                tracing::warn!("Error while removing role {}", error.to_string());
                let message = locale
                    .text("error")
                    .unwrap_or("error".to_owned());
                Err(UnknownError::new(message).into())
            }
        }
    }
}
