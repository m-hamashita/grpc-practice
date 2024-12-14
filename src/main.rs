use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use tokio::time::{sleep, Duration};
use sample::sample_server::{Sample, SampleServer};
use sample::{GetCounterRequest, GetCounterResponse, GetModelRequest, GetModelResponse};

pub mod sample {
    tonic::include_proto!("sample");
}

#[derive(Debug)]
pub struct Model {
    name: String,
    data: String,
}

#[derive(Debug)]
pub struct SampleService {
    counter: Arc<Mutex<i32>>,
    model: Arc<Mutex<Model>>,
}

#[tonic::async_trait]
impl Sample for SampleService {
    async fn get_model(
        &self,
        _request: Request<GetModelRequest>,
    ) -> Result<Response<GetModelResponse>, Status> {
        let model = self.model.lock().unwrap();
        Ok(Response::new(GetModelResponse {
            name: model.name.clone(),
            data: model.data.clone(),
        }))
    }
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
    let model = Arc::new(Mutex::new(Model {
        name: "Sample".to_string(),
        data: "sample data".to_string(),
    }));
    let counter = Arc::new(Mutex::new(0));

    let counter_clone = Arc::clone(&counter);
    let model_clone = Arc::clone(&model);
    tokio::spawn(async move {
        loop {
            {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                let new_model = Model {
                    name: format!("Model {}", num),
                    data: "sample data".to_string(),
                };
                let mut model = model_clone.lock().unwrap();
                *model = new_model;
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    let addr = "[::1]:50051".parse()?;
    let sample_service = SampleService { counter, model };

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(SampleServer::new(sample_service))
        .serve(addr)
        .await?;

    Ok(())
}
