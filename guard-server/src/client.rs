use std::time::Instant;

use tonic::metadata::MetadataValue;

use guard_grpc::EnforceRequest;

async fn call_grpc() {
    let mut client = guard_grpc::EnforcerClient::connect("http://127.0.0.1:3000")
        .await
        .unwrap();

    let mut request = tonic::Request::new(EnforceRequest {
        dom: "test".to_string(),
        obj: "delivery".to_string(),
        act: "list".to_string()
    });

    let jwt_value = std::env::var("JWT").unwrap();
    let metadata = MetadataValue::from_str(&jwt_value).unwrap();

    request.metadata_mut()
        .insert("authorization", metadata);

    client.enforce(request).await.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let mut response_times: Vec<f64> = vec![];

    for _ in 1..1000 {
        let now = Instant::now();
        call_grpc().await;
        response_times.push(now.elapsed().as_millis() as f64);
    }

    let avg: f64 = response_times.iter().sum::<f64>() / response_times.len() as f64;
    println!("Average time: {} ms", avg);
    Ok(())
}
