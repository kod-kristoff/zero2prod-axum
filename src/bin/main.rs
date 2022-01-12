use std::net::{SocketAddr, TcpListener};

use zero2prod::{
    configuration::get_configuration,
    startup,
};

#[tokio::main]
async fn main() { // -> Result<(), hyper::Error> {

    let configuration = get_configuration().expect("Failed to read configuration.");
    // TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.app_port));
    let listener = TcpListener::bind(&addr).unwrap();

    startup::run(listener).await.unwrap()
}
