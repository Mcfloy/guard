use std::time::Instant;
use rand::Rng;

use tonic::metadata::MetadataValue;
use guard::jwt::{encode, Principal};

use guard_grpc::EnforceRequest;

async fn call_grpc(number: u32) {

    let mut client = guard_grpc::EnforcerClient::connect("http://127.0.0.1:3000")
        .await
        .unwrap();

    let principal: Principal = Principal {
        sub: format!("owner-{}", number),
        namespace: "guard".to_string(),
        exp: 2177452800
    };

    let token = encode(&principal).unwrap();

    let mut request = tonic::Request::new(EnforceRequest {
        dom: "test".to_string(),
        obj: "permission".to_string(),
        act: "edit".to_string()
    });

    // let jwt_value = std::env::var("JWT").unwrap();
    let metadata = MetadataValue::from_str(&token).unwrap();

    request.metadata_mut()
        .insert("authorization", metadata);

    client.enforce(request).await.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let mut response_times: Vec<f64> = vec![];

    let mut rng = rand::thread_rng();

    for _ in 1..100 {
        let number: u32 = rng.gen_range(1000..1000000);
        let now = Instant::now();
        call_grpc(number).await;
        response_times.push(now.elapsed().as_millis() as f64);
    }

    let avg: f64 = response_times.iter().sum::<f64>() / response_times.len() as f64;
    println!("Average time: {} ms", avg);
    Ok(())
}
