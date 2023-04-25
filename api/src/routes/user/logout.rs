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
    sess: session::Connected,
    cookies: &CookieJar<'_>,
    sessions: &State<Cache<session::Connected>>,
) -> ApiResult<DefaultResponse> {
    sessions.del(&sess.account_id.to_string());
    cookies.remove_private(Cookie::named("session"));
    ApiResult::Success {
        status: Status::Ok,
        payload: DefaultResponse {
            response: String::from("logged out"),
        },
    }
}
