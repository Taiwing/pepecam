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
pub struct PictureComment {
    picture_id: Uuid,
    comment: String,
}

#[post("/comment", data = "<picture_comment>", format = "json")]
pub async fn post(
    picture_comment: Json<PictureComment>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let picture_comment = picture_comment.into_inner();
    match query::comment(
        &mut db,
        &picture_comment.comment,
        &from_serde_to_sqlx(&picture_comment.picture_id),
        &from_serde_to_sqlx(&sess.account_id),
    )
    .await
    {
        Ok(_) => ApiResult::Success {
            status: Status::Ok,
            payload: DefaultResponse {
                response: String::from("Comment successfully set!"),
            },
        },
        Err(_) => ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from("invalid picture id"),
        },
    }
}
