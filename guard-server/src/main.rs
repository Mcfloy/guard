use guard_grpc::GrpcServer;
use guard_postgres::PostgresRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repository = PostgresRepository::new().await;
    let server = GrpcServer::new(repository);

    let address = "127.0.0.1:3000".parse().unwrap();
    server.run(address)
        .await?;

    Ok(())
}
