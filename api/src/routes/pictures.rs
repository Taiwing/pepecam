pub mod comment;
pub mod like;
pub mod user;

use crate::query::{self, PostgresDb};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[get("/")]
pub async fn get(mut db: Connection<PostgresDb>) -> Option<Json<Vec<String>>> {
    match query::pictures(&mut db).await {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}
