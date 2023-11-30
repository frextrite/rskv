pub mod kvs_proto_gen {
    tonic::include_proto!("kvs");
}

use kvs_proto_gen::key_value_store_client::KeyValueStoreClient;
use kvs_proto_gen::EchoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut client = KeyValueStoreClient::connect("http://127.0.0.1:50051/").await?;

   let request = tonic::Request::new(EchoRequest {
    message: "sphinx of black quartz, judge my vow".to_string(),
   });

   let response = client.echo_me(request).await?;

   println!("INFO: Received response as \"{}\"", response.into_inner().message);

   Ok(())
}
