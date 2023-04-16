use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::payload::PictureId;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_serde_to_sqlx;
use crate::uuid::SqlxUuid;
use photon_rs::multiple;
use photon_rs::native;
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use std::fs;
use std::str::FromStr;
use strum::{self, AsRefStr, EnumString};

pub mod comment;
pub mod like;

#[derive(EnumString, AsRefStr)] // Convert from &str to Superposable and back
#[strum(serialize_all = "lowercase")] // Every Superposable name is in lowercase
pub enum Superposable {
    Chic,
    Cry,
    Honk,
    Rage,
    Smirk,
    Soned,
    Sweat,
}

//TODO: remove the relative PATH when the API is containerized
//TODO: also maybe use an env variable for the picture directory location
const PICTURE_PATH: &str = "../front/pictures";
//const PICTURE_PATH: &str = "/pictures";

//TODO: same as above, replace by containerized version
const SUPERPOSABLE_PATH: &str = concat!("../front/pictures", "/superposables");
//const SUPERPOSABLE_PATH: &str = concat!("/pictures", "/superposables");

const PICTURE_SIZEMAX: usize = 10;

async fn create_picture(
    raw_picture_bytes: Vec<u8>,
    superposable: Superposable,
    account_id: &SqlxUuid,
    db: &mut Connection<PostgresDb>,
) -> Result<SqlxUuid, ()> {
    let mut base_picture =
        match native::open_image_from_bytes(raw_picture_bytes.as_slice()) {
            Err(_) => {
                return Err(());
            }
            Ok(base_picture) => base_picture,
        };
    let filename = &format!("{}.png", superposable.as_ref());
    let superposable_picture = match native::open_image(&format!(
        "{}/{}",
        SUPERPOSABLE_PATH, filename
    )) {
        Err(_) => {
            return Err(());
        }
        Ok(image) => image,
    };
    multiple::watermark(&mut base_picture, &superposable_picture, 0, 0);
    let picture_id = match query::post_picture(db, account_id).await {
        Err(_) => {
            return Err(());
        }
        Ok(picture_id) => picture_id,
    };
    let filename =
        format!("{}/{}.jpg", PICTURE_PATH, picture_id.to_hyphenated());
    native::save_image(base_picture, &filename);
    Ok(picture_id)
}

#[post("/<superposable>", data = "<picture>", format = "image/jpeg")]
pub async fn post(
    superposable: &str,
    picture: Data<'_>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let superposable = match Superposable::from_str(superposable) {
        Err(_) => {
            return ApiResult::Failure {
                status: Status::NotFound,
                message: String::from("invalid superposable"),
            };
        }
        Ok(superposable) => superposable,
    };

    match picture.open(PICTURE_SIZEMAX.mebibytes()).into_bytes().await {
        Err(_) => ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from("file upload failure"),
        },
        Ok(transfer) if !transfer.is_complete() => ApiResult::Failure {
            status: Status::BadRequest,
            message: format!("file too big ({} MiB max)", PICTURE_SIZEMAX),
        },
        Ok(transfer) => {
            let raw_picture_bytes = transfer.into_inner();
            match create_picture(
                raw_picture_bytes,
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
    let filename = format!("{}/{}.jpg", PICTURE_PATH, picture_id.hyphenated());
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
