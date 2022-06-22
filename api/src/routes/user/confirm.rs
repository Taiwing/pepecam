use crate::{
    cache::Cache,
    payload::{DefaultResponse, NewUser, Token},
    result::ApiResult,
};
use rocket::serde::json::Json;
use rocket::{http::Status, State};

/// Confirm new user account with the registration token.
#[post("/confirm", data = "<registration_token>", format = "json")]
pub fn post(
    registration_token: Json<Token>,
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
    println!("{:?}", new_user);
    //TODO: add new user to database and log user
    todo!()
}
