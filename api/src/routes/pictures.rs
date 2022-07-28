pub mod comment;
pub mod like;
pub mod user;

use crate::payload::Picture;
use crate::query::{self, PostgresDb};
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PicturePage {
    index: u32,
    count: u32,
}

#[get("/", data = "<page>", format = "json")]
pub async fn get(
    page: Json<PicturePage>,
    mut db: Connection<PostgresDb>,
) -> Option<Json<Vec<Picture>>> {
    let page = page.into_inner();

    if page.count == 0 {
        return None;
    }

    match query::pictures(&mut db, page.index, page.count).await {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}
