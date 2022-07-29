use crate::payload::Picture;
use crate::query::{self, PostgresDb};
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PicturePage {
    username: Option<String>,
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

    let result = match page.username {
        None => query::pictures(&mut db, page.index, page.count).await,
        Some(ref username) => {
            query::user_pictures(&mut db, username, page.index, page.count)
                .await
        }
    };
    result.map(Json)
}
