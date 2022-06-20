#[macro_use]
extern crate rocket;
extern crate rand;

use regex::{Regex, RegexSet};
use rocket::http::Status;
use rocket::serde::{json::Json, uuid::Uuid, Deserialize, Serialize};
use rocket_db_pools::{Connection, Database};

//TODO: Use this to create a BackgroundJob structure and manage a redis-like
//LocalCache for temporary data. The background job would be used to enforce
//expiries on said data (like for confirmation/reset tokens for example).
/*
use rocket::fairing::AdHoc;
use rocket::tokio::time::{sleep, Duration};
*/

mod query;
mod result;
mod session;

use query::PostgresDb;
use result::ApiResult;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Token {
    token: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewUser {
    username: String,
    password: String,
    email: String,
}

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

/// Register a new User account.
#[post("/register", data = "<new_user>", format = "json")]
async fn register(
    new_user: Json<NewUser>,
    _sess: session::Unconnected,
    mut db: Connection<PostgresDb>,
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

    let rand_token: u128 = rand::random();

    //TODO: generate confirmation_token and send it through an email
    //instead of this
    ApiResult::Success {
        status: Status::Ok,
        payload: Token {
            token: format!("{:x}", rand_token),
        },
    }
}

#[post("/confirm/<confirmation_token>")]
fn confirm(confirmation_token: u128) -> String {
    format!("confirm your email with: {}\n", confirmation_token)
    //TODO: check confirmation token
    //add new user to database and log user if the token is valid
    //return an error otherwise
}

#[put("/login")]
fn login(_sess: session::Unconnected) -> &'static str {
    "login\n"
}

#[put("/logout")]
fn logout() -> &'static str {
    "logout\n"
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResetToken {
    reset_token: Uuid,
}

#[get("/reset-token")]
fn reset_token() -> Option<Json<ResetToken>> {
    None
}

#[put("/password", data = "<reset_token>")]
fn password(reset_token: &str) -> String {
    format!(
        "password: use reset-token '{}' to reset password\n",
        reset_token
    )
}

#[put("/")]
fn put_user(sess: session::Connected) -> String {
    format!(
        "PUT user ({}): change username, password and/or email settings\n",
        sess.account_id
    )
}

#[get("/")]
async fn get_pictures(db: Connection<PostgresDb>) -> Option<Json<Vec<String>>> {
    match query::pictures(db).await {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}

#[get("/<username>")]
async fn get_user_pictures(
    username: &str,
    db: Connection<PostgresDb>,
) -> Option<Json<Vec<String>>> {
    match query::user_pictures(db, username).await {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}

#[put("/like/<picture_id>")]
fn like(picture_id: Uuid, sess: session::Connected) -> String {
    format!(
        "PUT toggle like on picture {} as {}\n",
        picture_id, sess.account_id
    )
}

#[put("/comment/<picture_id>", data = "<content>")]
fn comment(
    picture_id: Uuid,
    content: &str,
    sess: session::Connected,
) -> String {
    format!(
        "PUT comment '{}' on picture {} as {}\n",
        content, picture_id, sess.account_id
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PostgresDb::init())
        /*
        .attach(AdHoc::try_on_ignite("Background Job", |rocket| async {
            rocket::tokio::task::spawn(async {
                let mut iter: u128 = 0;
                loop {
                    iter = iter + 1;
                    println!("iter: {}", iter);
                    sleep(Duration::from_secs(5)).await;
                }
            });
            Ok(rocket)
        }))
        */
        .mount("/user", routes![register])
        .mount("/user", routes![confirm])
        .mount("/user", routes![login])
        .mount("/user", routes![logout])
        .mount("/user", routes![reset_token])
        .mount("/user", routes![password])
        .mount("/user", routes![put_user])
        .mount("/pictures", routes![get_pictures])
        .mount("/pictures", routes![get_user_pictures])
        .mount("/pictures", routes![like])
        .mount("/pictures", routes![comment])
        .register("/", catchers![result::not_found])
}
