use std::net::{SocketAddr, TcpListener};

use zero2prod::{
    configuration::get_configuration,
    db::DbPool,
    startup,
    telemetry,
};

#[tokio::main]
async fn main() { // -> Result<(), hyper::Error> {
    let subscriber = telemetry::get_subscriber(
        "zero2prod".into(), "info".into()
    );
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let pool = DbPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to sqlite");
    // TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.app_port));
    let listener = TcpListener::bind(&addr).unwrap();
    tracing::info!("listening on {}", addr);
    startup::run(listener, pool).await.unwrap()
}
