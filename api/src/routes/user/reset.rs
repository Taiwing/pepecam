use crate::result::ApiResult;
use rocket::http::Status;
use rocket::serde::{json::Json, uuid::Uuid, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResetToken {
    reset_token: Uuid,
}

#[get("/reset")]
pub fn get() -> Option<Json<ResetToken>> {
    None
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PasswordReset {
    reset_token: Uuid,
    password: String,
}

//TODO: move this to 'result' module for use everywhere
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RouteResponse {
    response: String,
}

#[put("/reset", data = "<password_reset>", format = "json")]
pub fn put(password_reset: Json<PasswordReset>) -> ApiResult<RouteResponse> {
    let password_reset = password_reset.into_inner();
    ApiResult::Success {
        status: Status::Ok,
        payload: RouteResponse {
            response: format!(
                "password: use reset-token '{}' to reset password to '{}'\n",
                password_reset.reset_token, password_reset.password
            ),
        },
    }
}
