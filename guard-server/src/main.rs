use std::sync::Arc;

use poem::{EndpointExt, handler, Route, Server};
use poem::endpoint::TowerCompatExt;
use poem::http::StatusCode;
use poem::i18n::I18NResources;
use poem::listener::TcpListener;
use poem::middleware::Tracing;
use poem_openapi::{OpenApiService};
use tokio::sync::Mutex;

use guard_grpc::{EnforcerServer, GrpcServer};
use guard_postgres::PostgresRepository;
use crate::api::permission::PermissionApi;

use crate::api::namespace::NamespacesApi;
use crate::api::role::RoleApi;
use crate::api::root::RootApi;

mod error;
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
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "guard=debug;guard-server=debug;");
    }
    tracing_subscriber::fmt::init();

    let resources = I18NResources::builder()
        .add_path("guard-server/resources")
        .build()
        .unwrap();

    let repository = Arc::new(Mutex::new(PostgresRepository::new().await));

    let server = GrpcServer::new(Arc::clone(&repository));

    let api_service = OpenApiService::new(
        (RootApi, PermissionApi, NamespacesApi, RoleApi), "Guard API", "1.0"
    )
        .server(format!("/api"));

    let docs = api_service.swagger_ui();

    let app = Route::new()
        .nest("/api", api_service
            .data(Arc::clone(&repository))
            .data(resources)
            // .catch_error(|error: ForbiddenError| async move {
            //     println!("{:?}", error);
            //     Response::builder()
            //         .status(StatusCode::FORBIDDEN)
            //         .body(error.message)
            // })
        )
        .nest("/docs", docs)
        .nest_no_strip("/", tonic::transport::Server::builder()
            .add_service(EnforcerServer::new(server))
            .into_service()
            .compat())
        .with(Tracing);

    // tonic::transport::Server::builder()
    //     .add_service(EnforcerServer::new(server))
    //     .serve("127.0.0.1:50551".parse().unwrap())
    //     .await
    //     .unwrap();
    //
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
