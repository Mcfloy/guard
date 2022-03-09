use std::sync::Arc;
use poem::error::ResponseError;
use poem::i18n::Locale;
use poem::web::Data;
use tokio::sync::Mutex;
use guard::permission::{Permission, PermissionRepository};
use guard_postgres::PostgresRepository;
use crate::StatusCode;

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ForbiddenError {
    message: String
}

impl ResponseError for ForbiddenError {
    fn status(&self) -> StatusCode {
        StatusCode::FORBIDDEN
    }
}

impl ForbiddenError {
    pub fn new(locale: &Locale, permission: &Permission) -> Self {
        let message = locale
            .text_with_args("forbidden", (
                ("action", permission.action.clone()),
                ("object", permission.object.clone()),
                ("namespace", permission.namespace.clone()),
                ("domain", permission.domain.clone())
            ))
            .unwrap_or_else(|_| "error".to_string());

        ForbiddenError {
            message
        }

    }
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct UnknownError {
    message: String
}

impl ResponseError for UnknownError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl UnknownError {
    pub fn new(message: String) -> Self {
        UnknownError {
            message
        }
    }
}

pub async fn handle_enforce(locale: &Locale, repository: &Data<&Arc<Mutex<PostgresRepository>>>, guard_permission: &Permission) ->
    poem::Result<()> {
    let enforce_result = repository.0.lock().await.enforce(&guard_permission).await;
    match enforce_result {
        Ok(true) => Ok(()),
        Ok(false) => {
            return Err(ForbiddenError::new(&locale, &guard_permission).into());
        },
        Err(error) => {
            return Err(UnknownError::new(error.to_string()).into());
        }
    }
}
