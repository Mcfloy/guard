use std::sync::Arc;

use poem::{Error, Result};
use poem::http::StatusCode;
use poem::i18n::Locale;
use poem::web::{Data};
use poem_openapi::{ApiResponse, Object, OpenApi};
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use tokio::sync::Mutex;

use guard::namespace::NamespaceRepository;
use guard_postgres::PostgresRepository;

use crate::api::jwt::AuthenticatedUser;
use crate::api::namespace::NamespaceResponse::NamespaceNotFound;
use crate::error::UnknownError;
use crate::links::{Link, Links};

pub struct NamespacesApi;

#[derive(Object)]
struct NamespaceList {
    #[oai(skip_serializing_if_is_none)]
    namespaces: Vec<Namespace>
}

#[derive(ApiResponse)]
enum NamespaceResponse {
    #[oai(status = 200)]
    List(Json<NamespaceList>),
    #[oai(status = 204,)]
    Links(
        #[oai(header = "Link")] String
    ),
    #[oai(status = 204)]
    Delete,
    #[oai(status = 404)]
    NamespaceNotFound,
    #[oai(status = 201)]
    Created
}

#[derive(Object)]
pub struct Namespace {
    name: String,
    links: Links
}

#[OpenApi]
impl NamespacesApi {
    #[oai(path = "/namespaces", method = "get")]
    async fn get_namespaces(
        &self,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        _user: AuthenticatedUser
    ) -> Result<NamespaceResponse> {
        let namespaces = repository.0.lock().await.get_namespaces().await
            .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?
            .iter()
            .map(|n| {
                let namespace = n.as_str();
                let mut links = Links::new();
                links.push("href", Link::new("/namespaces/{id}", "GET", namespace));
                Namespace {
                    name: namespace.to_owned(),
                    links
                }
            })
            .collect();

        Ok(NamespaceResponse::List(Json(NamespaceList {
            namespaces
        })))
    }

    #[oai(path = "/namespaces", method = "post")]
    async fn create_namespace(&self) -> Result<NamespaceResponse> {
        Ok(NamespaceResponse::Created)
    }

    #[oai(path = "/namespaces/:id", method = "head")]
    async fn get_namespace_links(
        &self,
        locale: Locale,
        repository: Data<&Arc<Mutex<PostgresRepository>>>,
        _user: AuthenticatedUser,
        id: Path<String>
    ) -> Result<NamespaceResponse> {
        let namespace = id.0;
        let result = repository.0.lock().await.does_namespace_exists(&namespace).await
            .map_err(|error| {
                tracing::warn!("Error while getting namespace links {}", error.to_string());
                let message = locale
                    .text("error")
                    .unwrap_or("error".to_owned());
                UnknownError::new(message)
            })?;
        if result == false {
            return Ok(NamespaceNotFound);
        }

        let mut links = Links::new();

        let namespace_id_uri = format!("/namespaces/{}/roles", namespace);
        links.push("roles", Link::new(&namespace_id_uri, "HEAD", ""));

        Ok(NamespaceResponse::Links(
            links.to_header()
        ))
    }

    #[oai(path = "/namespaces/:id", method = "delete")]
    async fn delete_namespace(
        &self,
        _repository: Data<&Arc<Mutex<PostgresRepository>>>,
        _user: AuthenticatedUser,
        id: Path<i64>
    ) -> Result<NamespaceResponse> {
        Ok(NamespaceResponse::Delete)
    }
}
