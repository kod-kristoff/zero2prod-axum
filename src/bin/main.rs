use std::net::{SocketAddr, TcpListener};

use zero2prod::run;

#[tokio::main]
async fn main() { // -> Result<(), hyper::Error> {

    // TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).unwrap();

    run(listener).await.unwrap()
}
