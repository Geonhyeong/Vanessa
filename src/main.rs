use tonic::{transport::Server, Request, Response, Status};

// tonic-build가 생성한 코드를 include
pub mod vanessa {
    tonic::include_proto!("vanessa");
}

use vanessa::vanessa_service_server::{VanessaService, VanessaServiceServer};
use vanessa::{HelloReply, HelloRequest, StatusReply, StatusRequest};

#[derive(Debug, Default)]
pub struct MyVanessaService {}

#[tonic::async_trait]
impl VanessaService for MyVanessaService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    async fn get_status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusReply>, Status> {
        println!("Status request: {:?}", request);

        let reply = StatusReply {
            status: "Server is running".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let vanessa_service = MyVanessaService::default();

    // Reflection 서비스 설정
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/vanessa_descriptor.bin"
        )))
        .build_v1()?;

    println!("VanessaService listening on {}", addr);

    Server::builder()
        .add_service(VanessaServiceServer::new(vanessa_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
