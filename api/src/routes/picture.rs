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

#[post("/<superposable>", data = "<picture>", format = "image/jpeg")]
pub async fn post(
    superposable: &str,
    picture: Data<'_>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let superposable = match Superposable::from_str(superposable) {
        Ok(superposable) => superposable,
        Err(_) => {
            return ApiResult::Failure {
                status: Status::NotFound,
                message: String::from("invalid superposable"),
            };
        }
    };

    let picture_id = SerdeUuid::new_v4();
    let filename = format!("{}/{}", PICTURE_PATH, picture_id.hyphenated());
    let filepath = Path::new(&filename);

    match picture
        .open(PICTURE_SIZEMAX.mebibytes())
        .into_file(&filepath)
        .await
    {
        Ok(transfer) if transfer.is_complete() => {}
        Ok(_) => {
            let _ = fs::remove_file(&filepath);
            return ApiResult::Failure {
                status: Status::BadRequest,
                message: format!(
                    "picture too big (max is {} MiB)",
                    PICTURE_SIZEMAX
                ),
            };
        }
        _ => {
            return ApiResult::Failure {
                status: Status::BadRequest,
                message: String::from("picture upload failure"),
            };
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
    let response = format!(
        "new picture {} successfully created",
        picture_id.hyphenated()
    );
    ApiResult::Success {
        status: Status::Created,
        payload: DefaultResponse { response },
    }
}
