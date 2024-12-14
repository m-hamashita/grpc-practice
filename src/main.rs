use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use tokio::time::{sleep, Duration};
use sample::sample_server::{Sample, SampleServer};
use sample::{GetCounterRequest, GetCounterResponse};

pub mod sample {
    tonic::include_proto!("sample");
}

#[derive(Debug)]
pub struct SampleService {
    counter: Arc<Mutex<i32>>,
}

#[tonic::async_trait]
impl Sample for SampleService {
    async fn get_counter(
        &self,
        _request: Request<GetCounterRequest>,
    ) -> Result<Response<GetCounterResponse>, Status> {
        let counter = self.counter.lock().unwrap();
        Ok(Response::new(GetCounterResponse { value: *counter }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let counter = Arc::new(Mutex::new(0));

    let counter_clone = Arc::clone(&counter);
    tokio::spawn(async move {
        loop {
            {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    let addr = "[::1]:50051".parse()?;
    let sample_service = SampleService { counter };

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(SampleServer::new(sample_service))
        .serve(addr)
        .await?;

    Ok(())
}
