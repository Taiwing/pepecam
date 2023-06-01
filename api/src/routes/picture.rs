use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::payload::PictureId;
use crate::pictures;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_serde_to_sqlx;
use crate::uuid::SqlxUuid;
use photon_rs::{multiple, native, PhotonImage};
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use std::fs;
use std::str::FromStr;

pub mod comment;
pub mod comments;
pub mod like;

fn load_user_picture(raw_bytes: Vec<u8>) -> Result<PhotonImage, String> {
    let user_picture = match native::open_image_from_bytes(raw_bytes.as_slice())
    {
        Err(_) => {
            return Err(String::from("invalid user picture"));
        }
        Ok(user_picture) => user_picture,
    };

    if user_picture.get_width() < pictures::SUPERPOSABLES_SIDE
        || user_picture.get_height() < pictures::SUPERPOSABLES_SIDE
    {
        return Err(String::from("user picture too small"));
    }

    Ok(user_picture)
}

async fn create_picture(
    mut user_picture: PhotonImage,
    superposable: pictures::Superposable,
    account_id: &SqlxUuid,
    db: &mut Connection<PostgresDb>,
) -> Result<SqlxUuid, ()> {
    let filename = &format!("{}.png", superposable.as_ref());
    let superposable_picture = match native::open_image(&format!(
        "{}/{}",
        pictures::SUPERPOSABLES_PATH,
        filename
    )) {
        Err(_) => {
            return Err(());
        }
        Ok(image) => image,
    };
    let y: u32 = user_picture.get_height() - pictures::SUPERPOSABLES_SIDE;
    multiple::watermark(&mut user_picture, &superposable_picture, 0, y);
    let picture_id = match query::post_picture(db, account_id).await {
        Err(_) => {
            return Err(());
        }
        Ok(picture_id) => picture_id,
    };
    let filename =
        format!("{}/{}.jpg", pictures::PATH, picture_id.to_hyphenated());
    native::save_image(user_picture, &filename);
    Ok(picture_id)
}

#[post("/<superposable>", data = "<picture>", format = "image/jpeg")]
pub async fn post(
    superposable: &str,
    picture: Data<'_>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let superposable = match pictures::Superposable::from_str(superposable) {
        Err(_) => {
            return ApiResult::Failure {
                status: Status::NotFound,
                message: String::from("invalid superposable"),
            };
        }
        Ok(superposable) => superposable,
    };

    match picture
        .open(pictures::SIZEMAX.mebibytes())
        .into_bytes()
        .await
    {
        Err(_) => ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from("file upload failure"),
        },
        Ok(transfer) if !transfer.is_complete() => ApiResult::Failure {
            status: Status::BadRequest,
            message: format!("file too big ({} MiB max)", pictures::SIZEMAX),
        },
        Ok(transfer) => {
            let user_picture = match load_user_picture(transfer.into_inner()) {
                Err(message) => {
                    return ApiResult::Failure {
                        status: Status::BadRequest,
                        message,
                    };
                }
                Ok(user_picture) => user_picture,
            };
            match create_picture(
                user_picture,
                superposable,
                &from_serde_to_sqlx(&sess.account_id),
                &mut db,
            )
            .await
            {
                Err(_) => ApiResult::Failure {
                    status: Status::InternalServerError,
                    message: String::from("failed to create new picture"),
                },
                Ok(picture_id) => ApiResult::Success {
                    status: Status::Created,
                    payload: DefaultResponse {
                        response: format!(
                            "picture '{}' successfully created",
                            picture_id.to_hyphenated()
                        ),
                    },
                },
            }
        }
    }
}

#[delete("/", data = "<picture>", format = "json")]
pub async fn delete(
    picture: Json<PictureId>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let picture_id = picture.into_inner().picture_id;
    let filename =
        format!("{}/{}.jpg", pictures::PATH, picture_id.hyphenated());
    match query::delete_picture(
        &mut db,
        &from_serde_to_sqlx(&picture_id),
        &from_serde_to_sqlx(&sess.account_id),
    )
    .await
    {
        Err(_) => ApiResult::Failure {
            status: Status::InternalServerError,
            message: format!(
                "failed to delete '{}' picture",
                picture_id.hyphenated()
            ),
        },
        Ok(count) if count == 0 => ApiResult::Failure {
            status: Status::BadRequest,
            message: format!(
                "could not find '{}' picture for current user",
                picture_id.hyphenated()
            ),
        },
        Ok(_) => match fs::remove_file(&filename) {
            Err(_) => ApiResult::Failure {
                status: Status::InternalServerError,
                message: format!(
                    "could not remove '{}' picture file",
                    picture_id.hyphenated()
                ),
            },
            Ok(_) => ApiResult::Success {
                status: Status::Ok,
                payload: DefaultResponse {
                    response: format!(
                        "picture '{}' successfully deleted",
                        picture_id.hyphenated()
                    ),
                },
            },
        },
    }
}
