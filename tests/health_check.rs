use std::{
    net::{SocketAddr, TcpListener},
};
use diesel::{
    prelude::*,
    r2d2,
};
use zero2prod::db::{
    models::Subscriber,
    DbPool,
};
use zero2prod::configuration::{
    get_configuration,
    DatabaseSettings,
};

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

    let db_conn = ctx.pool.get().expect("Failed");
    use zero2prod::db::schema::subscriptions::dsl::*;
    let saved: Subscriber = subscriptions
        .first(&db_conn)
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
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
    pool: DbPool,
}

impl Context {
    async fn try_new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = format!("assets/generated/{}", uuid::Uuid::new_v4().to_string());
        let pool = configure_database(&configuration.database);
        let addr = serve(pool.clone()).await?;

        Ok(Self { addr, pool })
    }
}

async fn serve(pool: DbPool) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    // TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(&addr)?;
    let addr = listener.local_addr()?;

    tokio::spawn(async move { 
        zero2prod::startup::run(listener, pool).await 
    });

    Ok(addr)
}

fn configure_database(config: &DatabaseSettings) -> DbPool {
    let connection_string = config.connection_string();
    let pool = DbPool::builder()
        .build(r2d2::ConnectionManager::new(&connection_string))
        .expect("Failed to connect to sqlite.");
    let db_conn = pool.get().unwrap();
    diesel_migrations::run_pending_migrations(&db_conn)
        .expect("Failed to run migrations!");
    pool

}
