use crate::query::{self, PostgresDb};
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    username: String,
}

#[get("/user", data = "<user>", format = "json")]
pub async fn get(
    user: Json<User>,
    db: Connection<PostgresDb>,
) -> Option<Json<Vec<String>>> {
    let username: &str = &user.into_inner().username;
    match query::user_pictures(db, username).await {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}
