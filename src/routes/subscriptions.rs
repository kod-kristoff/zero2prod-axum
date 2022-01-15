use axum::{
    extract::{Extension, Form},
};
use diesel::prelude::*;
use http::StatusCode;
use crate::db::{
    DbPool,
    models::{NewSubscriber},
    sql_types::Uuid,
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
    use crate::db::schema::subscriptions::dsl::*;

    let request_id = uuid::Uuid::new_v4();
    tracing::info!(
        "request_id {} - Adding {} {} as a new subscriber",
        request_id,
        form.email,
        form.name
    );
    tracing::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );
    let db_conn = pool.get().expect("Failed to get connection");
    let new_subscriber = NewSubscriber {
        id: Uuid::new_v4(),
        email: &form.email,
        name: &form.name,
    };

    let result = diesel::insert_into(subscriptions)
        .values(&new_subscriber)
        .execute(&db_conn);
    match result {
        Ok(res) => {
            tracing::info!("request_id {} - New subscriber saved", request_id);
            StatusCode::OK
        }
        Err(err) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

