use std::time::Instant;
use guard_grpc::EnforceRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    let mut client = guard_grpc::EnforcerClient::connect("http://127.0.0.1:3000")
        .await
        .unwrap();

    let request = tonic::Request::new(EnforceRequest {
        sub: "lucas.perreau@ext.leroymerlin.fr".to_string(),
        ns: "lastmile".to_string(),
        dom: "014".to_string(),
        obj: "delivery".to_string(),
        act: "list".to_string()
    });
    let response = client.enforce(request).await?;
    println!("RESPONSE in {}={:?}", now.elapsed().as_millis() ,response.into_inner().authorized);
    Ok(())
}
