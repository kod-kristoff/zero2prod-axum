use std::{
    net::TcpListener,
    sync::Arc,
};

use axum::{
    AddExtensionLayer,
    Router,
};
use crate::db::DbPool;

use crate::routes;

pub async fn run(
    listener: TcpListener,
    db_pool: DbPool,
) -> Result<(),std::io::Error> {

    // let shared_pool = Arc::new(db_pool);
    use axum::routing::{get, post};
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        .layer(AddExtensionLayer::new(db_pool.clone()))
        ;
    
    axum::Server::from_tcp(listener).unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
