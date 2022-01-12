use std::{
    net::{SocketAddr, TcpListener},
};
use sqlx::{Connection, Row, SqliteConnection};
use zero2prod::configuration::get_configuration;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let ctx = Context::try_new().await.expect("Failed to spawn app");

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://{}/health_check", ctx.addr))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let ctx = Context::try_new()
        .await
        .expect("Failed to spawn app");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let mut connection = SqliteConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to sqlite.");

    let client = reqwest::Client::new();

    // Act 
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("http://{}/subscriptions", ctx.addr)) 
        .header("Content-Type", "application/x-www-form-urlencoded") 
        .body(body) 
        .send() 
        .await 
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    let email: &str = saved.get("email");
    let name: &str = saved.get("name");
    assert_eq!(email, "ursula_le_guin@gmail.com");
    assert_eq!(name, "le guin");
}

#[tokio:: test] 
async fn subscribe_returns_a_400_when_data_is_missing() { 
    // Arrange 
    let ctx = Context::try_new()
        .await
        .expect("Failed to spawn app");

    let client = reqwest::Client:: new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email") 
    ];
    for (invalid_body, error_message) in test_cases { 
        // Act 
        let response = client 
            .post(&format!("http://{}/subscriptions", ctx.addr)) 
            .header("Content-Type", "application/x-www-form-urlencoded") 
            .body(invalid_body) 
            .send() 
            .await 
            .expect("Failed to execute request.");

        // Assert 
        assert_eq!(
            response.status().as_u16(),
            400,
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.", 
            error_message 
        );
    } 
}

struct Context {
    addr: SocketAddr,
}

impl Context {
    async fn try_new() -> Result<Self, Box<dyn std::error::Error>> {
        let addr = serve().await?;

        Ok(Self { addr })
    }
}

async fn serve() -> Result<SocketAddr, Box<dyn std::error::Error>> {
    // TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(&addr)?;
    let addr = listener.local_addr()?;

    tokio::spawn(async move { zero2prod::startup::run(listener).await });

    Ok(addr)
}
