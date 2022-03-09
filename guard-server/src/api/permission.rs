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
use crate::error::{handle_enforce, UnknownError};
use crate::links::{Link, Links};

use crate::security::AuthenticatedUser;

pub struct PermissionApi;

#[derive(ApiResponse)]
enum PermissionResponse {
    #[oai(status = 201)]
    /// Permission is granted
    PermissionGranted,
    #[oai(status = 204)]
    PermissionRemoved,
    #[oai(status = 409)]
    /// Permission already exists
    PermissionAlreadyExists(PlainText<String>),
}

#[derive(Object, Deserialize)]
struct PermissionRequest {
    pub subject: String,
    pub namespace: String,
    pub domain: String,
    pub object: String,
    pub action: String,
}

impl Into<Permission> for PermissionRequest {
    fn into(self) -> Permission {
        Permission {
            subject: self.subject,
            namespace: self.namespace,
            domain: self.domain,
            object: self.object,
            action: self.action,
        }
    }
}

#[derive(ApiResponse)]
enum PermissionInfoResponse {
    #[oai(status = 204)]
    Ok(
        #[oai(header = "Link")] String
    )
}

#[OpenApi]
impl PermissionApi {
    #[oai(path = "/permissions", method = "head")]
    async fn permissions_info(&self, locale: Locale) -> Result<PermissionInfoResponse> {
        let mut links = Links::new();
        let title = locale.text("grant-permission").unwrap_or("".to_string());
        links.push("grant-permission", Link::new("/permissions", "POST", &title));

        let title = locale.text("remove-permission").unwrap_or("".to_string());
        links.push("remove-permission", Link::new("/permissions", "DELETE", &title));

        Ok(PermissionInfoResponse::Ok(
            links.to_header()
        ))
    }

    #[oai(path = "/permissions", method = "post")]
    async fn grant_permission(
        &self,
        permission_form: Json<PermissionRequest>,
        locale: Locale,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        user: AuthenticatedUser,
    ) -> Result<PermissionResponse> {
        let form_ns = permission_form.namespace.clone();
        let guard_permission = Permission {
            subject: user.0.sub,
            namespace: "guard".to_string(),
            domain: form_ns.to_string(),
            object: "permission".to_string(),
            action: "edit".to_string(),
        };
        handle_enforce(&locale, &repository, &guard_permission).await?;

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

    #[oai(path = "/permissions", method = "delete")]
    async fn remove_permission(&self,
                               permission_form: Json<PermissionRequest>,
                               locale: Locale,
                               repository: Data<&Arc<Mutex<PostgresRepository>>>,
                               user: AuthenticatedUser,
    ) -> Result<PermissionResponse> {
        let form_ns = permission_form.namespace.clone();
        let guard_permission = Permission {
            subject: user.0.sub,
            namespace: "guard".to_string(),
            domain: form_ns.to_string(),
            object: "permission".to_string(),
            action: "edit".to_string(),
        };
        handle_enforce(&locale, &repository, &guard_permission).await?;

        match repository.0.lock().await.remove_permission(&permission_form.0.into()).await {
            Ok(_) => Ok(PermissionResponse::PermissionRemoved),
            Err(error) => {
                tracing::warn!("Error while removing permission {}", error.to_string());
                let message = locale
                    .text("error")
                    .unwrap_or("error".to_string());
                Err(UnknownError::new(message).into())
            }
        }
    }
}
