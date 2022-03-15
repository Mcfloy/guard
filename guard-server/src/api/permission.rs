use std::sync::Arc;

use poem::i18n::{Locale};
use poem::{Result};
use poem::web::{Data};
use poem_openapi::{ApiResponse, Object, OpenApi};
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use serde::{Deserialize};
use tokio::sync::Mutex;
use guard::enforce::EnforceRequest;

use guard::permission::{Permission, PermissionRepository};
use guard_postgres::PostgresRepository;
use crate::error::{handle_enforce, UnknownError};

use crate::api::jwt::AuthenticatedUser;

pub struct PermissionApi;

#[derive(ApiResponse)]
enum PermissionResponse {
    #[oai(status = 201)]
    /// Permission has been granted
    PermissionGranted,
    #[oai(status = 204)]
    /// Permission has been removed
    PermissionRemoved,
    #[oai(status = 409)]
    /// Permission is already granted
    PermissionAlreadyGranted(PlainText<String>),
}

#[derive(Object, Deserialize)]
struct PermissionRequest {
    pub role: String,
    pub domain: String,
    pub object: String,
    pub action: String,
}

impl Into<Permission> for PermissionRequest {
    fn into(self) -> Permission {
        Permission {
            role: self.role,
            domain: self.domain,
            object: self.object,
            action: self.action,
        }
    }
}

#[OpenApi]
impl PermissionApi {
    #[oai(path = "/namespaces/:id/permissions", method = "post")]
    async fn create_permission(
        &self,
        locale: Locale,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        user: AuthenticatedUser,
        id: Path<String>,
        request: Json<PermissionRequest>
    ) -> Result<PermissionResponse> {
        let form_ns = id.0;
        let enforce_request = EnforceRequest {
            subject: user.0.sub,
            namespace: "guard".to_owned(),
            domain: form_ns.to_owned(),
            object: "permission".to_owned(),
            action: "edit".to_owned(),
        };
        handle_enforce(&locale, &repository, &enforce_request).await?;

        match repository.0.lock().await.grant_permission(&form_ns, &request.0.into()).await {
            Ok(_) => Ok(PermissionResponse::PermissionGranted),
            Err(_) => {
                let message = locale
                    .text("permission-already-exists")
                    .unwrap_or_else(|_| "error".to_owned());
                Ok(PermissionResponse::PermissionAlreadyGranted(PlainText(message)))
            }
        }
    }

    #[oai(path = "/namespaces/:id/permissions", method = "delete")]
    async fn delete_permission(
        &self,
        locale: Locale,
        request: Json<PermissionRequest>,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        user: AuthenticatedUser,
        id: Path<String>
    ) -> Result<PermissionResponse> {
        let form_ns = id.0;
        let enforce_request = EnforceRequest {
            subject: user.0.sub,
            namespace: "guard".to_owned(),
            domain: form_ns.to_owned(),
            object: "permission".to_owned(),
            action: "edit".to_owned(),
        };
        handle_enforce(&locale, &repository, &enforce_request).await?;

        match repository.0.lock().await.remove_permission(&form_ns, &request.0.into()).await {
            Ok(_) => Ok(PermissionResponse::PermissionRemoved),
            Err(error) => {
                tracing::warn!("Error while removing permission {}", error.to_string());
                let message = locale
                    .text("error")
                    .unwrap_or("error".to_owned());
                Err(UnknownError::new(message).into())
            }
        }
    }
}
