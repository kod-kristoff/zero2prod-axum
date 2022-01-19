use axum::extract::{Extension, Form};
use chrono::Utc;
use http::StatusCode;
use uuid::Uuid;

use crate::db::DbPool;

#[derive(serde::Deserialize)]
pub struct Subscribe {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: Form<Subscribe>, Extension(pool): Extension<DbPool>) -> StatusCode {
    match insert_subscriber(&form, &pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(form: &Subscribe, pool: &DbPool) -> Result<(), sqlx::Error> {
    let subscriber_id = Uuid::new_v4();
    let subscribed_at = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        subscriber_id,
        form.email,
        form.name,
        subscribed_at
    )
    .execute(pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {:?}", err);
        err
    })?;
    Ok(())
}
