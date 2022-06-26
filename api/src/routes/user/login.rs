use crate::auth;
use crate::auth::session;
use crate::payload::DefaultResponse;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::{json::Json, Deserialize};
use rocket::time::{Duration, OffsetDateTime};
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    username: String,
    password: String,
}

// One day
const DAY: Duration = Duration::days(1);

/// Login in to the application.
#[put("/login", data = "<credentials>", format = "json")]
pub async fn put(
    credentials: Json<Credentials>,
    _sess: session::Unconnected,
    mut db: Connection<PostgresDb>,
    cookies: &CookieJar<'_>,
) -> ApiResult<DefaultResponse> {
    let credentials = credentials.into_inner();
    if let Some(account) =
        query::get_user_by_username(&credentials.username, &mut db).await
    {
        if auth::password::verify(&credentials.password, &account.password_hash)
            == true
        {
            let mut session_cookie =
                Cookie::new("account_id", account.account_id.to_string());
            session_cookie.set_expires(OffsetDateTime::now_utc() + DAY);
            cookies.add_private(session_cookie);
            return ApiResult::Success {
                status: Status::Ok,
                payload: DefaultResponse {
                    response: String::from("great authentication success!"),
                },
            };
        }
    }
    ApiResult::Failure {
        status: Status::BadRequest,
        message: String::from("invalid credentials"),
    }
}
