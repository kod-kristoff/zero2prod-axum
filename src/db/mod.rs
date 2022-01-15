pub mod models;
pub mod schema;
pub mod sql_types;

use diesel::{r2d2, SqliteConnection};
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;


