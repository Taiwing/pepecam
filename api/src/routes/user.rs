pub mod confirm;
pub mod login;
pub mod logout;
pub mod register;
pub mod reset;

use crate::auth::session;
use crate::payload::{DefaultResponse, UserProfile};
use crate::query::{get_user_by_account_id, put_user, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_serde_to_sqlx;
use crate::validation;
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

#[put("/", data = "<user_changes>", format = "json")]
pub async fn put(
    user_changes: Json<UserChanges>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<DefaultResponse> {
    let user_changes = user_changes.into_inner();

    if let UserChanges {
        username: None,
        password: None,
        email: None,
        email_notifications: None,
    } = user_changes
    {
        return ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from("empty payload"),
        };
    }

    if let Some(ref username) = user_changes.username {
        if let Err(message) = validation::username(username) {
            return ApiResult::Failure {
                status: Status::BadRequest,
                message,
            };
        }
    }

    if let Some(ref password) = user_changes.password {
        if let Err(message) = validation::password(password) {
            return ApiResult::Failure {
                status: Status::BadRequest,
                message,
            };
        }
    }

    if let Some(ref email) = user_changes.email {
        if let Err(message) = validation::email(email) {
            return ApiResult::Failure {
                status: Status::BadRequest,
                message,
            };
        }
    }

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
            status: Status::Conflict,
            message: String::from("Failed to update user account."),
        },
    }
}

#[get("/")]
pub async fn get(
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
) -> Option<Json<UserProfile>> {
    let account_id = from_serde_to_sqlx(&sess.account_id);

    match get_user_by_account_id(&account_id, &mut db).await {
        Some(user) => Some(Json(UserProfile {
            username: user.username,
            email: user.email,
            email_notifications: user.email_notifications,
        })),
        None => None,
    }
}
