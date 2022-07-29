use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::SerdeUuid;
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket_db_pools::Connection;
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

//TODO: set the path to be directly accessible by the front container
const PICTURE_PATH: &str = "..";

const PICTURE_SIZEMAX: usize = 10;

#[post("/<superposable>", data = "<picture>")]
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
    let filename =
        format!("{}/{}", PICTURE_PATH, picture_id.hyphenated().to_string());

    match picture
        .open(PICTURE_SIZEMAX.mebibytes())
        .into_file(&Path::new(&filename))
        .await
    {
        Ok(transfer) if transfer.is_complete() => {}
        _ => {
            return ApiResult::Failure {
                status: Status::BadRequest,
                message: String::from("invalid picture"),
            };
        }
    }
    //TODO: create the new picture by using the superposable
    //TODO: add the new picture to the database (maybe add a superposable field to the db)
    let response = format!(
        "new picture {} successfully created",
        picture_id.hyphenated().to_string()
    );
    ApiResult::Success {
        status: Status::Created,
        payload: DefaultResponse { response },
    }
}
