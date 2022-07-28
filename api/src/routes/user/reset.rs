use crate::cache::Cache;
use crate::payload::{DefaultResponse, Token};
use crate::query::{get_user_by_username, put_user, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::{from_serde_to_sqlx, from_sqlx_to_serde};
use crate::validation;
use rocket::http::Status;
use rocket::serde::{json::Json, uuid::Uuid, Deserialize, Serialize};
use rocket::State;
use rocket_db_pools::Connection;
use std::time::Duration;

/// Request payload containing the username for the GET /reset route
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResetUsername {
    username: String,
}

/// Reset token to be sent to the user
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResetToken {
    reset_token: Uuid,
}

/// Request payload containing the new password for the PUT /reset route
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PasswordReset {
    reset_token: Uuid,
    password: String,
}

/// Reset request containing the account id stored in the cache
#[derive(Clone)]
pub struct Request {
    account_id: Uuid,
}

// Time during which the reset can be used in seconds.
const RESET_TOKEN_LIFETIME: u64 = 300; // 5 minutes

#[get("/reset", data = "<reset_username>", format = "json")]
pub async fn get(
    reset_username: Json<ResetUsername>,
    mut db: Connection<PostgresDb>,
    reset_requests: &State<Cache<Request>>,
) -> Option<Json<ResetToken>> {
    let reset_username = reset_username.into_inner();
    if let Some(account) =
        get_user_by_username(&reset_username.username, &mut db).await
    {
        let token = Token::new();
        let token_name = format!("reset_token:{}", token);
        let request = Request {
            account_id: from_sqlx_to_serde(&account.account_id),
        };
        reset_requests.set(
            &token_name,
            &request,
            Duration::from_secs(RESET_TOKEN_LIFETIME),
        );
        //TODO: send it via mail instead of doing this
        return Some(Json(ResetToken {
            reset_token: token.token,
        }));
    }
    None
}

#[put("/reset", data = "<password_reset>", format = "json")]
pub async fn put(
    password_reset: Json<PasswordReset>,
    mut db: Connection<PostgresDb>,
    reset_requests: &State<Cache<Request>>,
) -> ApiResult<DefaultResponse> {
    let password_reset = password_reset.into_inner();

    if let Err(message) = validation::password(&password_reset.password) {
        return ApiResult::Failure {
            status: Status::BadRequest,
            message,
        };
    }

    let token_name = format!("reset_token:{}", password_reset.reset_token);
    let request = match reset_requests.del(&token_name) {
        Some(item) => item,
        None => {
            return ApiResult::Failure {
                status: Status::BadRequest,
                message: format!(
                    "invalid reset token '{}'",
                    password_reset.reset_token
                ),
            };
        }
    };

    match put_user(
        &mut db,
        &from_serde_to_sqlx(&request.account_id),
        None,
        Some(password_reset.password),
        None,
        None,
    )
    .await
    {
        Ok(_) => ApiResult::Success {
            status: Status::Ok,
            payload: DefaultResponse {
                response: String::from("Password successfully reset!"),
            },
        },
        Err(_) => ApiResult::Failure {
            status: Status::InternalServerError,
            message: String::from("Failed to reset password."),
        },
    }
}
