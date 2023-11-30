use std::sync::{Arc, Mutex};

#[cfg(windows)]
use tokio::net::TcpListener as Listener;
#[cfg(windows)]
use tokio_stream::wrappers::TcpListenerStream as ListenerStream;

#[cfg(unix)]
use tokio::net::UnixListener as Listener;
#[cfg(unix)]
use tokio_stream::wrappers::UnixListenerStream as ListenerStream;

use tonic::{transport::Server, Request, Response, Status};

pub mod kvs_proto_gen {
    tonic::include_proto!("kvs");
}

use kvs_proto_gen::key_value_store_server::{KeyValueStore, KeyValueStoreServer};
use kvs_proto_gen::{EchoReply, EchoRequest, GetReply, GetRequest, SetReply, SetRequest};

use crate::kv::KeyValue;

#[derive(Debug, Default)]
pub struct KeyValueStoreService {
    kv: Arc<Mutex<KeyValue>>,
}

#[tonic::async_trait]
impl KeyValueStore for KeyValueStoreService {
    async fn echo_me(&self, request: Request<EchoRequest>) -> Result<Response<EchoReply>, Status> {
        println!("INFO: Received request from {:?}", request.remote_addr());

        let echo_request = request.into_inner();
        let echo_message = echo_request.message;

        let reply = EchoReply {
            message: echo_message,
        };

        Ok(Response::new(reply))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetReply>, Status> {
        println!(
            "INFO: Received SET request {:?} from {:?}",
            request.get_ref(),
            request.remote_addr()
        );

        let mut kv = self.kv.lock().unwrap();
        let req = request.into_inner();
        let old_value = kv.set(req.key.clone(), req.value);

        Ok(Response::new(SetReply { old_value }))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        println!(
            "INFO: Received GET request {:?} from {:?}",
            request.get_ref(),
            request.remote_addr(),
        );

        let kv = self.kv.lock().unwrap();
        let req = request.into_inner();
        let value = kv.get(req.key.clone());

        Ok(Response::new(GetReply {
            value: value.cloned(),
        }))
    }
}

pub async fn run_key_value_store() -> Result<(), Box<dyn std::error::Error>> {
    let addr = if cfg!(unix) {
        "\0/tmp/key_value_store"
    } else {
        "http://127.0.0.1:50051"
    };
    let listener = Listener::bind(addr)?;
    let stream = ListenerStream::new(listener);

    println!("INFO: Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(KeyValueStoreServer::new(KeyValueStoreService::default()))
        .serve_with_incoming(stream)
        .await?;

    Ok(())
}
