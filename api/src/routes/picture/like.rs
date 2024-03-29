use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::payload::PictureId;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_serde_to_sqlx;
use rocket::http::Status;
use rocket::serde::{json::Json, uuid::Uuid, Deserialize};
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PictureLike {
    picture_id: Uuid,
    like: bool,
}

#[put("/like", data = "<picture>", format = "json")]
pub async fn put(
    picture: Json<PictureLike>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let picture = picture.into_inner();
    match query::put_like(
        &mut db,
        picture.like,
        &from_serde_to_sqlx(&picture.picture_id),
        &from_serde_to_sqlx(&sess.account_id),
    )
    .await
    {
        Ok(_) => {
            let action = match picture.like {
                true => "like",
                false => "dislike",
            };
            let response = format!(
                "{} on picture '{}' successfully set",
                action, picture.picture_id
            );
            ApiResult::Success {
                status: Status::Created,
                payload: DefaultResponse { response },
            }
        }
        Err(_) => ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from("invalid picture id"),
        },
    }
}

#[delete("/like", data = "<picture>", format = "json")]
pub async fn delete(
    picture: Json<PictureId>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let picture = picture.into_inner();
    match query::delete_like(
        &mut db,
        &from_serde_to_sqlx(&picture.picture_id),
        &from_serde_to_sqlx(&sess.account_id),
    )
    .await
    {
        Ok(_) => {
            let response = format!(
                "like on picture '{}' successfully unset",
                picture.picture_id
            );
            ApiResult::Success {
                status: Status::Ok,
                payload: DefaultResponse { response },
            }
        }
        Err(_) => ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from("invalid picture id"),
        },
    }
}
