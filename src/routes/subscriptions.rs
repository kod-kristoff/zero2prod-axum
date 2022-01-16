use axum::{
    extract::{Extension, Form},
};
use chrono::Utc;
use http::StatusCode;
use tracing::Instrument;
use uuid::Uuid;

use crate::db::{
    DbPool,
};

#[derive(serde::Deserialize)]
pub struct Subscribe {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: Form<Subscribe>,
    Extension(pool): Extension<DbPool>,
) -> StatusCode {

    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );
    let subscriber_id = Uuid::new_v4();
    let subscribed_at = Utc::now();
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        subscriber_id,
        form.email,
        form.name,
        subscribed_at)
    .execute(&pool)
    .instrument(query_span)
    .await 
    {
        Ok(_) => {
            StatusCode::OK
        }
        Err(err) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

