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
    username: Option<String>,
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

    let result = match username {
        None => query::pictures(&mut db, index, count, account_id).await,
        Some(ref username) => {
            query::user_pictures(&mut db, username, index, count, account_id)
                .await
        }
    };
    result.map(Json)
}
