use std::sync::Arc;
use poem::error::ResponseError;
use poem::i18n::Locale;
use poem::web::Data;
use tokio::sync::Mutex;
use guard::enforce::{EnforceRepository, EnforceRequest};
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
    pub fn new(locale: &Locale, enforce_request: &EnforceRequest) -> Self {
        let message = locale
            .text_with_args("forbidden", (
                ("action", enforce_request.action.clone()),
                ("object", enforce_request.object.clone()),
                ("namespace", enforce_request.namespace.to_owned()),
                ("domain", enforce_request.domain.clone())
            ))
            .unwrap_or_else(|_| "error".to_owned());

        Self {
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
        Self {
            message
        }
    }
}

pub async fn handle_enforce(locale: &Locale, repository: &Data<&Arc<Mutex<PostgresRepository>>>, enforce_request: &EnforceRequest) ->
    poem::Result<()> {
    let enforce_result = repository.0.lock().await
        .enforce(&enforce_request).await;
    match enforce_result {
        Ok(true) => Ok(()),
        Ok(false) => {
            return Err(ForbiddenError::new(&locale, &enforce_request).into());
        },
        Err(error) => {
            return Err(UnknownError::new(error.to_string()).into());
        }
    }
}
