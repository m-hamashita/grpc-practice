use sample::sample_client::SampleClient;
use sample::{GetCounterRequest, GetModelRequest};

pub mod sample {
    tonic::include_proto!("sample");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SampleClient::connect("http://[::1]:50051").await?;

    for _ in 0..10 {
        // let request = tonic::Request::new(GetCounterRequest {});
        // let response = client.get_counter(request).await?;
        // println!("Counter value: {}", response.into_inner().value);

        let request = tonic::Request::new(GetModelRequest {});
        let response = client.get_model(request).await?;
        println!("Model: {:?}", response.into_inner());

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }

    Ok(())
}
