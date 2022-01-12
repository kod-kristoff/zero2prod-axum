use std::net::TcpListener;

use axum::Router;

use crate::routes;

pub async fn run(listener: TcpListener) -> Result<(),std::io::Error> {

    use axum::routing::{get, post};
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        ;
    
    axum::Server::from_tcp(listener).unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
