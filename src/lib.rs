use std::net::{SocketAddr, TcpListener};

use axum::{
    body::Bytes,
    Router,
};
use http::StatusCode;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn run(listener: TcpListener) -> Result<(),std::io::Error> {

    use axum::routing::get;
    let app = Router::new()
        .route("/health_check", get(health_check));
    
    axum::Server::from_tcp(listener).unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
