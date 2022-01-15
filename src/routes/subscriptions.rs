use axum::{
    extract::{Extension, Form},
};
use chrono::Utc;
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
        Ok(_) => StatusCode::OK,
        Err(err) => {
            eprintln!("Failed to execute query: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

