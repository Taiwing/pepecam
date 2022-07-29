use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::SerdeUuid;
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket_db_pools::Connection;
use std::fs;
use std::path::Path;
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

struct DummyPicture; //TODO remove this when adding photon

fn create_picture(
    raw_picture_bytes: Vec<u8>,
    superposable: Superposable,
    mut db: Connection<PostgresDb>,
) -> Result<String, ()> {
    Err(())
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
            match create_picture(raw_picture_bytes, superposable, db) {
                Err(_) => ApiResult::Failure {
                    status: Status::InternalServerError,
                    message: String::from("failed to create new picture"),
                },
                Ok(response) => ApiResult::Success {
                    status: Status::Created,
                    payload: DefaultResponse { response },
                },
            }
        }
    }
    //TODO: create the new picture by using the superposable
    //TODO: use photon_rs for that check this out for loading the picture:
    // https://docs.rs/photon-rs/latest/photon_rs/native/fn.open_image_from_bytes.html
    // This means that the picture must be read into a byte buffer, not into a
    // file, which is actually better since there is no need to delete it then
    // (if the transfer fails or whatever). Search 'watermark' for the function
    // we need to use. This should do the trick.
    //TODO: add the new picture to the database (maybe add a superposable field to the picture table)
}
