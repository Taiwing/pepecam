use crate::result::ApiResult;
use crate::{
    auth::session,
    cache::Cache,
    payload::{NewUser, Token},
    query::{self, PostgresDb},
};
use regex::{Regex, RegexSet};
use rocket::serde::json::Json;
use rocket::{http::Status, State};
use rocket_db_pools::Connection;
use std::time::Duration;

// Username is simply a string of six to sixty-four word characters.
const USERNAME_REGEX: &str = r"^\w{6,64}$";

const PASSWORD_REGEX_COUNT: usize = 5;

// Password must contain one lowercase letter, one uppercase letter, a digit, a
// special character and must be at least 8 characters long.
const PASSWORD_REGEX: [&'static str; PASSWORD_REGEX_COUNT] =
    [r"[a-z]+", r"[A-Z]+", r"\d+", r"\W+", r".{8,}"];

const PASSWORD_REGEX_ERRORS: [&'static str; PASSWORD_REGEX_COUNT] = [
    "password must contain at least one lower case letter",
    "password must contain at least one upper case letter",
    "password must contain at least one digit",
    "password must contain at least one special character",
    "password must be at least eight characters long",
];

// HTML5 email regex. The email addres must only contain alphanumeric and non
// whitespace special characters for the first part. An '@' symbol and a domain
// name with a least a '.' in it.
const EMAIL_REGEX: &str =
    r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$";

// Time during which the registration_token can be used in seconds.
const REGISTRATION_TOKEN_LIFETIME: u64 = 300; // 5 minutes

/// Register a new user account.
#[post("/register", data = "<new_user>", format = "json")]
pub async fn post(
    new_user: Json<NewUser>,
    _sess: session::Unconnected,
    mut db: Connection<PostgresDb>,
    new_users: &State<Cache<NewUser>>,
) -> ApiResult<Token> {
    let user = new_user.into_inner();

    //TODO: maybe use lazy_static macro crate to optimize this
    let re = Regex::new(USERNAME_REGEX).unwrap();
    if re.is_match(&user.username) == false {
        return ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from(
                "username must be a word of 6 to 64 characters long",
            ),
        };
    }

    let set = RegexSet::new(&PASSWORD_REGEX).unwrap();
    let matches: Vec<_> = set.matches(&user.password).into_iter().collect();
    if matches.len() != PASSWORD_REGEX_COUNT {
        for first_error in 0..PASSWORD_REGEX_COUNT {
            if !matches.contains(&first_error) {
                return ApiResult::Failure {
                    status: Status::BadRequest,
                    message: String::from(PASSWORD_REGEX_ERRORS[first_error]),
                };
            }
        }
    }

    let re = Regex::new(EMAIL_REGEX).unwrap();
    if re.is_match(&user.email) == false || user.email.len() > 256 {
        return ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from("invalid email format"),
        };
    }

    if query::is_taken("username", &user.username, &mut db).await {
        return ApiResult::Failure {
            status: Status::Conflict,
            message: format!("username '{}' is already taken", &user.username),
        };
    }

    if query::is_taken("email", &user.email, &mut db).await {
        return ApiResult::Failure {
            status: Status::Conflict,
            message: format!("email '{}' is already taken", &user.email),
        };
    }

    let token = Token::new();
    let token_name = format!("registration_token:{}", token);
    new_users.set(
        &token_name,
        &user,
        Duration::from_secs(REGISTRATION_TOKEN_LIFETIME),
    );

    //TODO: send token through an email instead of this
    ApiResult::Success {
        status: Status::Created,
        payload: token,
    }
}
