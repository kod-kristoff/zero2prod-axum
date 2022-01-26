use std::net::TcpListener;

use secrecy::ExposeSecret;

use zero2prod::{configuration::get_configuration, db::DbPool, startup, telemetry};

#[tokio::main]
async fn main() {
    // -> Result<(), hyper::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let pool = DbPool::connect_lazy(&configuration.database.connection_string().expose_secret())
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
