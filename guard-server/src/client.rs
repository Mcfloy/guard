use std::time::Instant;

use tonic::metadata::MetadataValue;

use guard_grpc::EnforceRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    let now = Instant::now();

    let mut client = guard_grpc::EnforcerClient::connect("http://127.0.0.1:3000")
        .await
        .unwrap();

    let mut request = tonic::Request::new(EnforceRequest {
        dom: "014".to_string(),
        obj: "delivery".to_string(),
        act: "list".to_string()
    });

    let jwt_value = std::env::var("JWT").unwrap();
    let metadata = MetadataValue::from_str(&jwt_value).unwrap();

    request.metadata_mut()
        .insert("authorization", metadata);

    let response = client.enforce(request).await?;
    println!("RESPONSE in {}={:?}", now.elapsed().as_millis() ,response.into_inner().authorized);
    Ok(())
}
