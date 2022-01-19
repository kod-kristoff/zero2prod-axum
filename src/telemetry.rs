use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    tracing_log::LogTracer::init().expect("Failed to set logger");
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(env_filter))
        .expect("Failed to init env_filter");
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}
