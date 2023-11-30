use rskv::kvs::run_key_value_store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_key_value_store().await?;
    Ok(())
}
