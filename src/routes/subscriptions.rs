use axum::{
    extract::Form,
};
use http::StatusCode;


#[derive(serde::Deserialize)]
pub struct Subscribe {
    email: String,
    name: String,
}

pub async fn subscribe(_form: Form<Subscribe>) -> StatusCode {
    StatusCode::OK
}

