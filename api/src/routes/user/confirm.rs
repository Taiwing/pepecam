use crate::{
    cache::Cache,
    payload::{DefaultResponse, NewUser, Token},
    query::{self, PostgresDb},
    result::ApiResult,
    session,
};
use rocket::serde::json::Json;
use rocket::{http::Status, State};
use rocket_db_pools::Connection;

/// Confirm new user account with the registration token.
#[post("/confirm", data = "<registration_token>", format = "json")]
pub async fn post(
    registration_token: Json<Token>,
    _sess: session::Unconnected,
    db: Connection<PostgresDb>,
    cache: &State<Cache<NewUser>>,
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
    match query::create_account(db, new_user).await {
        //TODO: log user on success
        Ok(response) => ApiResult::Success {
            status: Status::Created,
            payload: DefaultResponse { response },
        },
        Err(_) => ApiResult::Failure {
            status: Status::Conflict,
            message: format!("could not create new user account"),
        },
    }
}
