use std::sync::Arc;

use poem::{Error, Result};
use poem::http::StatusCode;
use poem::web::Data;
use poem_openapi::{ApiResponse, Object, OpenApi};
use poem_openapi::payload::{Json};
use tokio::sync::Mutex;

use guard::namespace::NamespaceRepository;
use guard_postgres::PostgresRepository;

use crate::security::AuthenticatedUser;

pub struct NamespacesApi;

#[derive(Object)]
pub struct NamespaceList {
    subject: Option<String>,
    namespaces: Vec<String>
}

#[derive(ApiResponse)]
enum NamespaceResponse {
    #[oai(status = 200)]
    List(Json<NamespaceList>)
}

#[OpenApi]
impl NamespacesApi {
    #[oai(path="/namespaces", method = "get")]
    async fn get_namespaces(
        &self,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        _user: AuthenticatedUser
    ) -> Result<NamespaceResponse> {
        let namespaces = repository.0.lock().await.get_namespaces().await
            .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(NamespaceResponse::List(Json(NamespaceList {
            subject: None,
            namespaces
        })))
    }

    #[oai(path="/me/namespaces", method = "get")]
    async fn get_my_namespaces(
        &self,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        principal: AuthenticatedUser
    ) -> Result<NamespaceResponse> {
        let subject = principal.0.sub;
        let namespaces = repository.0.lock().await.get_namespaces_of_subject(&subject).await
            .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(NamespaceResponse::List(Json(NamespaceList {
            subject: Some(subject),
            namespaces
        })))
    }
}
