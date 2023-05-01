use crate::payload::Comment;
use crate::query::{self, PostgresDb};
use crate::uuid::from_serde_to_sqlx;
use rocket::serde::{json::Json, uuid::Uuid};
use rocket_db_pools::Connection;

#[get("/comments?<picture>")]
pub async fn get(
    picture: Uuid,
    mut db: Connection<PostgresDb>,
) -> Option<Json<Vec<Comment>>> {
    query::comments(&mut db, &from_serde_to_sqlx(&picture))
        .await
        .map(Json)
}
