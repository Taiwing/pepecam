use crate::payload::Picture;
use crate::query::{self, PostgresDb};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[get("/?<index>&<count>&<username>")]
pub async fn get(
    index: u32,
    count: u32,
    username: Option<String>,
    mut db: Connection<PostgresDb>,
) -> Option<Json<Vec<Picture>>> {
    if count == 0 {
        return None;
    }

    let result = match username {
        None => query::pictures(&mut db, index, count).await,
        Some(ref username) => {
            query::user_pictures(&mut db, username, index, count).await
        }
    };
    result.map(Json)
}
