#[macro_use]
extern crate rocket;
extern crate rand;

use regex::Regex;
use rocket::http::{Method, Status};
use rocket::serde::{json::Json, uuid::Uuid, Deserialize, Serialize};
use rocket_db_pools::{Connection, Database};

mod query;
mod result;
mod session;

use query::PostgresDb;
use result::{ApiError, ApiResult};

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

#[post("/register", data = "<new_user>", format = "json")]
async fn register(
    new_user: Json<NewUser>,
    _sess: session::Unconnected,
    mut db: Connection<PostgresDb>,
) -> ApiResult<Token> {
    let user = new_user.into_inner();
    if query::username_exists(&user.username, db).await {
        return ApiResult::Failure {
            status: Status::Conflict,
            message: format!("username '{}' is already taken", &user.username),
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
async fn get_pictures(
    mut db: Connection<PostgresDb>,
) -> Option<Json<Vec<String>>> {
    match query::pictures(db).await {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}

#[get("/<username>")]
async fn get_user_pictures(
    username: &str,
    mut db: Connection<PostgresDb>,
) -> Option<Json<Vec<String>>> {
    match query::user_pictures(db, username).await {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}

#[put("/like/<picture_id>")]
fn like(picture_id: Uuid, sess: session::Connected) -> String {
    format!("PUT toggle like on picture {}\n", picture_id)
}

#[put("/comment/<picture_id>", data = "<content>")]
fn comment(
    picture_id: Uuid,
    content: &str,
    sess: session::Connected,
) -> String {
    format!("PUT comment '{}' on picture {}\n", content, picture_id)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PostgresDb::init())
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
