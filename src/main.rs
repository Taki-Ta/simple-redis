use anyhow::Result;
use simple_redis::{stream_handler, Backend};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    info!("Listening on: {}", &addr);
    let listener = TcpListener::bind(addr).await?;

    let backend = Backend::new();
    loop {
        let (stream, saddr) = listener.accept().await?;
        info!("Accepted connection from: {}", saddr);
        let backend = backend.clone();
        tokio::spawn(async move {
            if let Err(e) = stream_handler(stream, backend).await {
                info!("Error handling connection: {:?}", e);
            }
        });
    }
}
