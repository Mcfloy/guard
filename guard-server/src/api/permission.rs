use std::sync::Arc;

use poem::i18n::{Locale};
use poem::{Result};
use poem::web::{Data};
use poem_openapi::{ApiResponse, Object, OpenApi};
use poem_openapi::payload::{Json, PlainText};
use serde::{Deserialize};
use tokio::sync::Mutex;

use guard::permission::{Permission, PermissionRepository};
use guard_postgres::PostgresRepository;

use crate::security::AuthenticatedUser;

pub struct PermissionApi;

#[derive(ApiResponse)]
enum PermissionResponse {
    #[oai(status = 201)]
    /// Permission is granted
    PermissionGranted,
    #[oai(status = 409)]
    /// Permission already exists
    PermissionAlreadyExists(PlainText<String>),
    /// Unknown error
    #[oai(status = 500)]
    UnknownError(PlainText<String>)
}

#[derive(Object, Deserialize)]
struct PermissionRequest {
    pub subject: String,
    pub namespace: String,
    pub domain: String,
    pub object: String,
    pub action: String
}

impl Into<Permission> for PermissionRequest {
    fn into(self) -> Permission {
        Permission {
            subject: self.subject,
            namespace: self.namespace,
            domain: self.domain,
            object: self.object,
            action: self.action
        }
    }
}

#[OpenApi]
impl PermissionApi {
    #[oai(path = "/permission", method = "post")]
    async fn grant_permission(
        &self,
        permission_form: Json<PermissionRequest>,
        locale: Locale,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        user: AuthenticatedUser
    ) -> Result<PermissionResponse> {
        let form_ns = permission_form.namespace.clone();
        let guard_permission = Permission {
            subject: user.0.sub,
            namespace: "guard".to_string(),
            domain: form_ns.to_string(),
            object: "permission".to_string(),
            action: "edit".to_string()
        };
        if let Err(error) = repository.0.lock().await.enforce(&guard_permission).await {
            let message = locale
                .text_with_args(
                    "error-reason",
                    (("reason", error.to_string()),)
                )
                .unwrap_or_else(|_| "error".to_string());
            return Ok(PermissionResponse::UnknownError(PlainText(message)));
        }

        match repository.0.lock().await.grant_permission(&permission_form.0.into()).await {
            Ok(_) => Ok(PermissionResponse::PermissionGranted),
            Err(_) => {
                let message = locale
                    .text("permission-already-exists")
                    .unwrap_or_else(|_| "error".to_string());
                Ok(PermissionResponse::PermissionAlreadyExists(PlainText(message)))
            }
        }
    }
}
