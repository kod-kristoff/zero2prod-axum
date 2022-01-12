use std::{
    net::{SocketAddr, TcpListener},
};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let ctx = Context::try_new().await.expect("Failed to spawn app");

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://{}/health_check", ctx.addr))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

struct Context {
    addr: SocketAddr,
}

impl Context {
    async fn try_new() -> Result<Self, Box<dyn std::error::Error>> {
        let addr = serve().await?;

        Ok(Self { addr })
    }
}

async fn serve() -> Result<SocketAddr, Box<dyn std::error::Error>> {
    // TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(&addr)?;
    let addr = listener.local_addr()?;

    tokio::spawn(async move { zero2prod::run(listener).await });

    Ok(addr)
}
