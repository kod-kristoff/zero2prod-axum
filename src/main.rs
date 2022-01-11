use axum::{
    routing::get,
    Router,
};
use http::StatusCode;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
}
