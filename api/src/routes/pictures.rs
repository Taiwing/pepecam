use crate::auth::session;
use crate::payload::Picture;
use crate::pictures;
use crate::query::{self, PostgresDb};
use crate::uuid::from_serde_to_sqlx;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_db_pools::Connection;

pub mod superposable;

#[get("/?<index>&<count>&<username>&<superposable>&<start>&<end>&<picture>")]
pub async fn get(
    index: u32,
    count: u32,
    username: Option<&str>,
    mut superposable: Vec<pictures::Superposable>,
    start: Option<i64>,
    end: Option<i64>,
    picture: Option<Uuid>,
    mut db: Connection<PostgresDb>,
    is_connected: session::IsConnected,
) -> Option<Json<Vec<Picture>>> {
    superposable.sort();
    superposable.dedup();

    if count == 0 {
        return None;
    }

    let account_id = match is_connected.0 {
        Some(session) => Some(from_serde_to_sqlx(&session.account_id)),
        None => None,
    };

    let picture_id = match picture {
        Some(picture) => Some(from_serde_to_sqlx(&picture)),
        None => None,
    };

    query::pictures(
        &mut db,
        index,
        count,
        account_id,
        username,
        superposable,
        start,
        end,
        picture_id,
    )
    .await
    .map(Json)
}
