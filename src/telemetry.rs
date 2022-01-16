
use tracing::Subscriber;
use tracing_bunyan_formatter::{
    BunyanFormattingLayer,
    JsonStorageLayer,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    EnvFilter,
    Registry,
};

pub fn init_subscriber(
    subscriber: impl Subscriber + Send + Sync,
) {
    tracing_log::LogTracer::init()
        .expect("Failed to set logger");
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");
}

pub fn get_subscriber(
    name: String,
    env_filter: String,
) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(env_filter))
        .expect("Failed to init env_filter");
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        std::io::stdout,
    );

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}
