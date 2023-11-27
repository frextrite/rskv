use tonic::{transport::Server, Request, Response, Status};

pub mod kvs_proto_gen {
    tonic::include_proto!("kvs");
}

use kvs_proto_gen::key_value_store_server::{KeyValueStore, KeyValueStoreServer};
use kvs_proto_gen::{EchoRequest, EchoReply};

#[derive(Debug, Default)]
pub struct KeyValueStoreService {}

#[tonic::async_trait]
impl KeyValueStore for KeyValueStoreService {
    async fn echo_me(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoReply>, Status> {
        println!("Received request from {:?}", request.remote_addr());

        let echo_request = request.into_inner();
        let echo_message = echo_request.message;

        let reply = EchoReply { message: echo_message };

        Ok(Response::new(reply))
    }
}

async fn run_key_value_store() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(KeyValueStoreServer::new(KeyValueStoreService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
