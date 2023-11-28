use std::sync::{Arc, Mutex};

use tonic::{transport::Server, Request, Response, Status};

pub mod kvs_proto_gen {
    tonic::include_proto!("kvs");
}

use kvs_proto_gen::key_value_store_server::{KeyValueStore, KeyValueStoreServer};
use kvs_proto_gen::{EchoRequest, EchoReply, GetRequest, GetReply, SetRequest, SetReply};

use crate::kv::KeyValue;

#[derive(Debug, Default)]
pub struct KeyValueStoreService {
    kv: Arc<Mutex<KeyValue>>,
}

#[tonic::async_trait]
impl KeyValueStore for KeyValueStoreService {
    async fn echo_me(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoReply>, Status> {
        println!("INFO: Received request from {:?}", request.remote_addr());

        let echo_request = request.into_inner();
        let echo_message = echo_request.message;

        let reply = EchoReply { message: echo_message };

        Ok(Response::new(reply))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetReply>, Status> {
        let mut kv = self.kv.lock().unwrap();
        let req = request.into_inner();
        let old_value = kv.set(req.key.clone(), req.value);

        Ok(Response::new(SetReply { old_value: old_value }))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        let kv = self.kv.lock().unwrap();
        let req = request.into_inner();
        let value = kv.get(req.key.clone());

        Ok(Response::new(GetReply { value: value.cloned() }))
    }
}

pub async fn run_key_value_store() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;

    println!("INFO: Starting gRPC server on {:?}", addr);

    Server::builder()
        .add_service(KeyValueStoreServer::new(KeyValueStoreService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
