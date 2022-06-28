use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::result::ApiResult;
use rocket::http::{Cookie, CookieJar, Status};

//TODO: fix this (remove_private is not enough because if the cookie has been
//kept on the user side it still can be used for authentication)

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
