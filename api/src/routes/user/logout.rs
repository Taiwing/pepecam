use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::result::ApiResult;
use rocket::http::{Cookie, CookieJar, Status};

//TODO: Fix this (remove_private is not enough because if the cookie has been
// kept on the user side it still can be used for authentication). Change the
// session cookie to add a cookie unique identifier so that it can be retrieved
// in a session cache. Also add an expiry field to it (which will be the same as
// the cookie itself), so that it can be removed from the cache when it is
// expired.

/// Log out of the application.
#[put("/logout")]
pub fn put(
    _sess: session::Connected,
    cookies: &CookieJar<'_>,
) -> ApiResult<DefaultResponse> {
    cookies.remove_private(Cookie::named("session"));
    ApiResult::Success {
        status: Status::Ok,
        payload: DefaultResponse {
            response: String::from("logged out"),
        },
    }
}
