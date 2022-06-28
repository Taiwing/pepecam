use super::login::{login, Credentials};
use crate::{
    auth::session,
    cache::Cache,
    payload::{DefaultResponse, NewUser, Token},
    query::{self, PostgresDb},
    result::ApiResult,
};
use rocket::serde::json::Json;
use rocket::{
    http::{CookieJar, Status},
    State,
};
use rocket_db_pools::Connection;

/// Confirm new user account with the registration token.
#[post("/confirm", data = "<registration_token>", format = "json")]
pub async fn post(
    registration_token: Json<Token>,
    _sess: session::Unconnected,
    mut db: Connection<PostgresDb>,
    cache: &State<Cache<NewUser>>,
    cookies: &CookieJar<'_>,
) -> ApiResult<DefaultResponse> {
    let token = registration_token.into_inner();
    let token_name = format!("registration_token:{}", token);

    let new_user = match cache.del(&token_name) {
        Some(item) => item,
        None => {
            return ApiResult::Failure {
                status: Status::BadRequest,
                message: format!("invalid registration token '{}'", token),
            };
        }
    };
    match query::create_account(&mut db, &new_user).await {
        Ok(mut response) => {
            let credentials = Credentials {
                username: new_user.username,
                password: new_user.password,
            };
            if let Err(_) = login(&credentials, &mut db, cookies).await {
                response = String::from(
                    "user account created but an error occured on login",
                );
            }
            ApiResult::Success {
                status: Status::Created,
                payload: DefaultResponse { response },
            }
        }
        Err(_) => ApiResult::Failure {
            status: Status::Conflict,
            message: String::from("could not create new user account"),
        },
    }
}
