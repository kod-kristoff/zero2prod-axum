use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use zero2prod::{configuration::get_configuration, startup, telemetry};

#[tokio::main]
async fn main() {
    // -> Result<(), hyper::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(&configuration.database.with_db())
        .expect("Failed to connect to sqlite");
    // TCP listener
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port,
    );
    let listener = TcpListener::bind(&address).unwrap();
    tracing::info!("listening on {}", address);
    startup::run(listener, pool).await.unwrap()
}
