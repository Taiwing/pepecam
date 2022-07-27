use crate::cache::Cache;
use crate::payload::{DefaultResponse, Token};
use crate::query::{get_user_by_username, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_sqlx_to_serde;
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

//TODO: do not forget to validate passwords with the same regex as for the register route

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
pub fn put(password_reset: Json<PasswordReset>) -> ApiResult<DefaultResponse> {
    let password_reset = password_reset.into_inner();
    ApiResult::Success {
        status: Status::Ok,
        payload: DefaultResponse {
            response: format!(
                "password: use reset-token '{}' to reset password to '{}'\n",
                password_reset.reset_token, password_reset.password
            ),
        },
    }
}
