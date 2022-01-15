
use crate::db::{
    schema::subscriptions,
    sql_types::Uuid,
};

#[derive(serde::Serialize, diesel::Queryable)]
pub struct Subscriber {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub subscribed_at: chrono::NaiveDateTime,
}

#[derive(diesel::Insertable)]
#[table_name = "subscriptions"]
pub struct NewSubscriber<'a> {
    pub id: Uuid,
    pub email: &'a str,
    pub name: &'a str,
}
