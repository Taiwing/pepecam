use crate::payload::Picture;
use crate::query::{self, PostgresDb};
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserPicturePage {
    username: String,
    index: u32,
    count: u32,
}

#[get("/user", data = "<page>", format = "json")]
pub async fn get(
    page: Json<UserPicturePage>,
    mut db: Connection<PostgresDb>,
) -> Option<Json<Vec<Picture>>> {
    let page = page.into_inner();
    match query::user_pictures(&mut db, &page.username, page.index, page.count)
        .await
    {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}
