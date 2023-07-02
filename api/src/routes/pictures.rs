use crate::auth::session;
use crate::payload::Picture;
use crate::query::{self, PostgresDb};
use crate::uuid::from_serde_to_sqlx;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

pub mod superposable;

#[get("/?<index>&<count>&<username>")]
pub async fn get(
    index: u32,
    count: u32,
    username: Option<&str>,
    mut db: Connection<PostgresDb>,
    is_connected: session::IsConnected,
) -> Option<Json<Vec<Picture>>> {
    if count == 0 {
        return None;
    }

    let account_id = match is_connected.0 {
        Some(session) => Some(from_serde_to_sqlx(&session.account_id)),
        None => None,
    };

    query::pictures(&mut db, index, count, account_id, username)
        .await
        .map(Json)
}
