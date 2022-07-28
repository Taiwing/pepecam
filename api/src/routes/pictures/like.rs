use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_serde_to_sqlx;
use rocket::http::Status;
use rocket::serde::{json::Json, uuid::Uuid, Deserialize};
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Picture {
    picture_id: Uuid,
    like: Option<bool>,
}

#[put("/like", data = "<picture>", format = "json")]
pub async fn put(
    picture: Json<Picture>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let picture = picture.into_inner();
    match query::like(
        &mut db,
        picture.like,
        &from_serde_to_sqlx(&picture.picture_id),
        &from_serde_to_sqlx(&sess.account_id),
    )
    .await
    {
        //TODO change response depending on the like value
        Ok(_) => ApiResult::Success {
            status: Status::Ok,
            payload: DefaultResponse {
                response: format!(
                    "like on picture '{}' successfully toggled",
                    picture.picture_id
                ),
            },
        },
        Err(_) => ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from("invalid picture id"),
        },
    }
}
