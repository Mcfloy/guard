use poem_openapi::{ApiResponse, Object};
use poem_openapi::payload::Json;
use crate::api::namespace::handlers::Namespace;

#[derive(ApiResponse)]
pub enum NamespaceListResponse {
    #[oai(status = 200)]
    List(Json<NamespaceList>)
}

#[derive(Object)]
pub struct NamespaceList {
    #[oai(skip_serializing_if_is_none)]
    pub(crate) namespaces: Vec<Namespace>
}