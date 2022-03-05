use std::sync::Arc;

use poem::{EndpointExt, handler, Route, Server};
use poem::endpoint::TowerCompatExt;
use poem::listener::TcpListener;
use poem_openapi::{OpenApiService};
use tokio::sync::Mutex;

use guard_grpc::{EnforcerServer, GrpcServer};
use guard_postgres::PostgresRepository;
use crate::api::access::AccessApi;

use crate::api::namespace::NamespacesApi;

mod user;
mod api;
mod security;
mod links;


#[handler]
fn root_route() -> String {
    "ok".to_string()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let repository = Arc::new(Mutex::new(PostgresRepository::new().await));

    let server = GrpcServer::new(Arc::clone(&repository));

    let api_service = OpenApiService::new(
        (AccessApi, NamespacesApi), "Guard API", "1.0"
    )
        .server(format!("/v1"));

    let docs = api_service.swagger_ui();

    let app = Route::new()
        .nest("/v1", api_service.data(Arc::clone(&repository)))
        .at("/api", root_route)
        .nest_no_strip("/grpc", tonic::transport::Server::builder()
            .add_service(EnforcerServer::new(server))
            .into_service()
            .compat())
        .nest("/docs", docs);

    // tonic::transport::Server::builder()
    //     .add_service(EnforcerServer::new(server))
    //     .serve("127.0.0.1:50551".parse().unwrap())
    //     .await
    //     .unwrap();
    //
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;

    Ok(())
}
