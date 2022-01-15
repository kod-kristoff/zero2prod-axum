use std::net::{SocketAddr, TcpListener};

use diesel::r2d2;
use zero2prod::{
    configuration::get_configuration,
    db::DbPool,
    startup,
};

#[tokio::main]
async fn main() { // -> Result<(), hyper::Error> {

    let configuration = get_configuration().expect("Failed to read configuration");
    let pool = DbPool::builder()
        .build(
            r2d2::ConnectionManager::new(
                &configuration.database.connection_string()))
        .expect("Failed to connect to sqlite");
    // TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.app_port));
    let listener = TcpListener::bind(&addr).unwrap();

    startup::run(listener, pool).await.unwrap()
}
