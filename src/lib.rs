use std::net::{TcpListener};

use axum::{
    extract::Form,
    Router,
};
use http::StatusCode;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(serde::Deserialize)]
struct Subscribe {
    email: String,
    name: String,
}

async fn subscribe(_form: Form<Subscribe>) -> StatusCode {
    StatusCode::OK
}

pub async fn run(listener: TcpListener) -> Result<(),std::io::Error> {

    use axum::routing::{get, post};
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        ;
    
    axum::Server::from_tcp(listener).unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
