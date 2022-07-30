use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_serde_to_sqlx;
use crate::uuid::SqlxUuid;
use photon_rs::native;
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket_db_pools::Connection;
use std::str::FromStr;

pub mod comment;
pub mod like;

//TODO: Change the Superposable names when they will actually exist
pub enum Superposable {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl FromStr for Superposable {
    type Err = ();

    fn from_str(input: &str) -> Result<Superposable, Self::Err> {
        match input.to_lowercase().as_str() {
            "first" => Ok(Superposable::First),
            "second" => Ok(Superposable::Second),
            "third" => Ok(Superposable::Third),
            "fourth" => Ok(Superposable::Fourth),
            "fifth" => Ok(Superposable::Fifth),
            _ => Err(()),
        }
    }
}

//TODO: remove the relative PATH when the API is containerized
const PICTURE_PATH: &str = "../front/pictures";
//const PICTURE_PATH: &str = "/pictures";

const PICTURE_SIZEMAX: usize = 10;

async fn create_picture(
    raw_picture_bytes: Vec<u8>,
    superposable: Superposable,
    account_id: &SqlxUuid,
    db: &mut Connection<PostgresDb>,
) -> Result<SqlxUuid, ()> {
    let base_picture =
        match native::open_image_from_bytes(raw_picture_bytes.as_slice()) {
            Err(_) => {
                return Err(());
            }
            Ok(base_picture) => base_picture,
        };
    //TODO: open the superposable here and "watermark" it to the base_picture
    let new_picture = base_picture; //TEMP
    let picture_id = match query::post_picture(db, account_id).await {
        Err(_) => {
            return Err(());
        }
        Ok(picture_id) => picture_id,
    };
    let filename =
        format!("{}/{}.jpg", PICTURE_PATH, picture_id.to_hyphenated());
    native::save_image(new_picture, &filename);
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
                        response: picture_id.to_hyphenated().to_string(),
                    },
                },
            }
        }
    }
}

/*
#[delete("/", data = "<picture>", format = "json")]
pub async fn delete(
	picture: Json<PictureId>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
	todo()
}
*/
