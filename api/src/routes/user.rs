pub mod confirm;
pub mod email;
pub mod login;
pub mod logout;
pub mod register;
pub mod reset;

use crate::auth::session;
use crate::cache::Cache;
use crate::config;
use crate::mail::Mailer;
use crate::payload::{DefaultResponse, Email, Token, UserProfile};
use crate::query::{get_user_by_account_id, put_user, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_serde_to_sqlx;
use crate::validation;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket_db_pools::Connection;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserChanges {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    email_notifications: Option<bool>,
}

// Time during which the email can be used in seconds.
const EMAIL_TOKEN_LIFETIME: u64 = 300; // 5 minutes

#[put("/", data = "<user_changes>", format = "json")]
pub async fn put(
    user_changes: Json<UserChanges>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
    new_emails: &State<Cache<Email>>,
    mailer: &State<Mailer>,
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

        let token = Token::new();
        let token_name = format!("email_token:{}", token);
        let new_email = Email {
            email: email.clone(),
        };
        new_emails.set(
            &token_name,
            &new_email,
            Duration::from_secs(EMAIL_TOKEN_LIFETIME),
        );
        let link = format!(
            "{}/email.html?token={}",
            config::FRONT_LINK.as_str(),
            token
        );
        _ = mailer.send(email, "Confirm your email", &link);
    }

    match put_user(
        &mut db,
        &from_serde_to_sqlx(&sess.account_id),
        user_changes.username,
        user_changes.password,
        None,
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
