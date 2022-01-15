use std::net::{SocketAddr, TcpListener};

use diesel::r2d2;

use zero2prod::{
    configuration::get_configuration,
    db::DbPool,
    startup,
};

#[tokio::main]
async fn main() { // -> Result<(), hyper::Error> {
    use tracing_subscriber::{fmt, EnvFilter};
    use tracing_subscriber::prelude::*;

    let fmt_layer = fmt::layer()
        .with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
    // tracing_subscriber::fmt::init();

    let configuration = get_configuration().expect("Failed to read configuration");
    let pool = DbPool::builder()
        .build(
            r2d2::ConnectionManager::new(
                &configuration.database.connection_string()))
        .expect("Failed to connect to sqlite");
    // TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.app_port));
    let listener = TcpListener::bind(&addr).unwrap();
    tracing::info!("listening on {}", addr);
    startup::run(listener, pool).await.unwrap()
}
