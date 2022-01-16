use std::net::{SocketAddr, TcpListener};

use tracing_bunyan_formatter::{
    BunyanFormattingLayer,
    JsonStorageLayer,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    EnvFilter,
    Registry,
};
use zero2prod::{
    configuration::get_configuration,
    db::DbPool,
    startup,
};

#[tokio::main]
async fn main() { // -> Result<(), hyper::Error> {
    tracing_log::LogTracer::init()
        .expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        std::io::stdout,
    );

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    // tracing_subscriber::registry()
    //      .with(filter_layer)
    //      .with(fmt_layer)
    //      .init();
    // tracing_subscriber::fmt::init();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

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
