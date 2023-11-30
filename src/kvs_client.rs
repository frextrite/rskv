use tonic::transport::Endpoint;
use tower::service_fn;

#[cfg(windows)]
use tokio::net::TcpStream as Stream;
#[cfg(unix)]
use tokio::net::UnixStream as Stream;

pub mod kvs_proto_gen {
    tonic::include_proto!("kvs");
}

use kvs_proto_gen::key_value_store_client::KeyValueStoreClient;
use kvs_proto_gen::EchoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = if cfg!(unix) {
        "\0/tmp/key_value_store"
    } else {
        "127.0.0.1:50051"
    };

    println!("INFO: Connecting to remote endpoint {}", addr);

    let channel = Endpoint::try_from("http://localhost/")? // TODO: try removing this?
        .connect_with_connector(service_fn(move |_| Stream::connect(addr)))
        .await?;

    let mut client = KeyValueStoreClient::new(channel);

    let request = tonic::Request::new(EchoRequest {
        message: "sphinx of black quartz, judge my vow".to_string(),
    });

    let response = client.echo_me(request).await?;

    println!(
        "INFO: Received response as \"{}\"",
        response.into_inner().message
    );

    Ok(())
}
