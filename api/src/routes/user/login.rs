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
    pub username: String,
    pub password: String,
}

// One day
const DAY: Duration = Duration::days(1);

/// Helper function checking credentials and creating the session on success.
pub async fn login(
    credentials: &Credentials,
    db: &mut Connection<PostgresDb>,
    cookies: &CookieJar<'_>,
) -> Result<String, String> {
    if let Some(account) =
        query::get_user_by_username(&credentials.username, db).await
    {
        if auth::password::verify(&credentials.password, &account.password_hash)
            == true
        {
            let mut session_cookie =
                Cookie::new("account_id", account.account_id.to_string());
            session_cookie.set_expires(OffsetDateTime::now_utc() + DAY);
            cookies.add_private(session_cookie);
            return Ok(String::from("great authentication success!"));
        }
    }
    Err(String::from("invalid credentials"))
}

/// Route handler to login into the application.
#[put("/login", data = "<credentials>", format = "json")]
pub async fn put(
    credentials: Json<Credentials>,
    _sess: session::Unconnected,
    mut db: Connection<PostgresDb>,
    cookies: &CookieJar<'_>,
) -> ApiResult<DefaultResponse> {
    let credentials = credentials.into_inner();
    match login(&credentials, &mut db, cookies).await {
        Ok(response) => ApiResult::Success {
            status: Status::Ok,
            payload: DefaultResponse { response },
        },
        Err(message) => ApiResult::Failure {
            status: Status::BadRequest,
            message,
        },
    }
}
