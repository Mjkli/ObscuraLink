

mod proxy;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // proxy service to catch reuqests
    let _ = proxy::proxy().await?;
    

    Ok(())
}
