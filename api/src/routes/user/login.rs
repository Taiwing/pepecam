use crate::auth;
use crate::auth::session;
use crate::cache::Cache;
use crate::payload::DefaultResponse;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_sqlx_to_serde;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::{json::Json, Deserialize};
use rocket::time::{Duration, OffsetDateTime};
use rocket::State;
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

// Session default duration (1 day in seconds)
const SESSION_DURATION: f64 = 86_400.0;

/// Helper function checking credentials and creating the session on success.
pub async fn login(
    credentials: &Credentials,
    db: &mut Connection<PostgresDb>,
    cookies: &CookieJar<'_>,
    sessions: &State<Cache<session::Connected>>,
) -> Result<String, String> {
    if let Some(account) =
        query::get_user_by_username(&credentials.username, db).await
    {
        if auth::password::verify(&credentials.password, &account.password_hash)
        {
            let session = session::Connected::new(from_sqlx_to_serde(
                &account.account_id,
            ));
            sessions.set(
                &session.account_id.to_string(),
                &session,
                std::time::Duration::from_secs_f64(SESSION_DURATION),
            );
            let mut cookie = Cookie::new("session", session.to_string());
            cookie.set_expires(
                OffsetDateTime::now_utc()
                    + Duration::seconds_f64(SESSION_DURATION),
            );
            cookies.add_private(cookie);
            return Ok(String::from("great authentication success!"));
        }
    }
    Err(String::from("invalid credentials"))
}

/// Route handler to login into the application.
#[post("/login", data = "<credentials>", format = "json")]
pub async fn post(
    credentials: Json<Credentials>,
    _sess: session::Unconnected,
    mut db: Connection<PostgresDb>,
    sessions: &State<Cache<session::Connected>>,
    cookies: &CookieJar<'_>,
) -> ApiResult<DefaultResponse> {
    let credentials = credentials.into_inner();
    match login(&credentials, &mut db, cookies, sessions).await {
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
