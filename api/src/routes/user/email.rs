use crate::{
    auth::session,
    cache::Cache,
    payload::{DefaultResponse, Email, Token},
    query::{self, PostgresDb},
    result::ApiResult,
};
use crate::uuid::from_serde_to_sqlx;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_db_pools::Connection;

/// Confirm new email address.
#[post("/email", data = "<email_token>", format = "json")]
pub async fn post(
    email_token: Json<Token>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
    new_emails: &State<Cache<Email>>,
) -> ApiResult<DefaultResponse> {
    let token = email_token.into_inner();
    let token_name = format!("email_token:{}", token);

    let email = match new_emails.del(&token_name) {
        Some(item) => item.email,
        None => {
            return ApiResult::Failure {
                status: Status::BadRequest,
                message: format!("invalid email token '{}'", token),
            };
        }
    };

    match query::put_user(
        &mut db,
        &from_serde_to_sqlx(&sess.account_id),
        None,
        None,
        Some(email),
        None,
    )
    .await
    {
        Ok(_) => ApiResult::Success {
            status: Status::Ok,
            payload: DefaultResponse {
                response: "Email updated successfully.".to_string(),
            },
        },
        Err(_) => ApiResult::Failure {
            status: Status::InternalServerError,
            message: "Failed to update email.".to_string(),
        },
    }
}
