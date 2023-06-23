use crate::result::ApiResult;
use crate::{
    auth::session,
    cache::Cache,
    mail::Mailer,
    payload::{NewUser, Token},
    query::{self, PostgresDb},
    validation,
};
use rocket::serde::json::Json;
use rocket::{http::Status, State};
use rocket_db_pools::Connection;
use std::time::Duration;

// Time during which the registration_token can be used in seconds.
const REGISTRATION_TOKEN_LIFETIME: u64 = 300; // 5 minutes

/// Register a new user account.
#[post("/register", data = "<new_user>", format = "json")]
pub async fn post(
    new_user: Json<NewUser>,
    _sess: session::Unconnected,
    mut db: Connection<PostgresDb>,
    new_users: &State<Cache<NewUser>>,
    mailer: &State<Mailer>,
) -> ApiResult<Token> {
    let user = new_user.into_inner();

    if let Err(message) = validation::username(&user.username) {
        return ApiResult::Failure {
            status: Status::BadRequest,
            message,
        };
    }

    if let Err(message) = validation::password(&user.password) {
        return ApiResult::Failure {
            status: Status::BadRequest,
            message,
        };
    }

    if let Err(message) = validation::email(&user.email) {
        return ApiResult::Failure {
            status: Status::BadRequest,
            message,
        };
    }

    if query::is_taken("username", &user.username, &mut db).await {
        return ApiResult::Failure {
            status: Status::Conflict,
            message: format!("username '{}' is already taken", &user.username),
        };
    }

    if query::is_taken("email", &user.email, &mut db).await {
        return ApiResult::Failure {
            status: Status::Conflict,
            message: format!("email '{}' is already taken", &user.email),
        };
    }

    let token = Token::new();
    let token_name = format!("registration_token:{}", token);
    new_users.set(
        &token_name,
        &user,
        Duration::from_secs(REGISTRATION_TOKEN_LIFETIME),
    );

    match mailer.send(&user.email, "registration", token.to_string().as_str()) {
        Ok(_) => (),
        Err(_) => {
            return ApiResult::Failure {
                status: Status::InternalServerError,
                message: "Failed to send registration email".to_string(),
            };
        }
    }

    //TODO: send token through an email instead of this
    ApiResult::Success {
        status: Status::Created,
        payload: token,
    }
}
