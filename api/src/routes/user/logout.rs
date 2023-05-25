use crate::auth::session;
use crate::cache::Cache;
use crate::payload::DefaultResponse;
use crate::result::ApiResult;
use rocket::{
    http::{Cookie, CookieJar, Status},
    State,
};

/// Log out of the application.
#[post("/logout")]
pub fn post(
    cookies: &CookieJar<'_>,
    is_connected: session::IsConnected,
    sessions: &State<Cache<session::Connected>>,
) -> ApiResult<DefaultResponse> {
    if let Some(sess) = is_connected.0 {
        sessions.del(&sess.account_id.to_string());
    }
    cookies.remove(Cookie::named("session"));
    ApiResult::Success {
        status: Status::Ok,
        payload: DefaultResponse {
            response: String::from("logged out"),
        },
    }
}
