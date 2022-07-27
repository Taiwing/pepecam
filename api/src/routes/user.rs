pub mod confirm;
pub mod login;
pub mod logout;
pub mod register;
pub mod reset;

use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::query::{put_user, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_serde_to_sqlx;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::Connection;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserChanges {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    email_notifications: Option<bool>,
}

//TODO: validate UserChanges fields (move the code from the register route to
//a dedicated module, maybe rename regex to validation and go from there)
//Also, return an error when the payload is empty -_-

#[put("/", data = "<user_changes>", format = "json")]
pub async fn put(
    user_changes: Json<UserChanges>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let user_changes = user_changes.into_inner();
    match put_user(
        &mut db,
        &from_serde_to_sqlx(&sess.account_id),
        user_changes.username,
        user_changes.password,
        user_changes.email,
        user_changes.email_notifications,
    )
    .await
    {
        Ok(_) => ApiResult::Success {
            status: Status::Ok,
            payload: DefaultResponse {
                response: String::from("User account successfully updated!"),
            },
        },
        Err(_) => ApiResult::Failure {
            status: Status::InternalServerError,
            message: String::from("Failed to update user account."),
        },
    }
}
