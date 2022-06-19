#[macro_use] extern crate rocket;
extern crate rand;

use rocket_db_pools::{sqlx, Database, Connection};
use crate::rocket::futures::TryStreamExt;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::uuid::Uuid as SerdeUuid;
use sqlx::types::Uuid as SqlxUuid;
use sqlx::Row;

//TODO: find a way to remove the "postgres" string or to use the environment
//instead (something like 'std::env!("DATABASE_NAME")' if possible).
#[derive(Database)]
#[database("postgres")]
struct PostgresDb(sqlx::PgPool);

mod result;
mod session;

use result::ApiResult;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Token {
	token: String,
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewUser {
	username: String,
	password: String,
	confirmation: String,
	email: String,
}

#[post("/register", data = "<new_user>", format = "json")]
fn register(new_user: Json<NewUser>, _sess: session::Unconnected) -> ApiResult<Token> {
	println!("{:?}", new_user.into_inner());
	let rand_token: u128 = rand::random();
	//TODO: generate confirmation_token and send it through an email instead of this
	Ok(Json(Token { token: format!("{:x}", rand_token) }))
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
	reset_token: SerdeUuid,
}

#[get("/reset-token")]
fn reset_token() -> Option<Json<ResetToken>> {
	None
}

#[put("/password", data = "<reset_token>")]
fn password(reset_token: &str) -> String {
	format!("password: use reset-token '{}' to reset password\n", reset_token)
}

#[put("/")]
fn put_user(sess: session::Connected) -> String {
	format!("PUT user ({}): change username, password and/or email settings\n",
		sess.account_id)
}

#[get("/")]
async fn get_pictures(mut db: Connection<PostgresDb>) -> Option<Json<Vec<String>>> {
	let mut rows = sqlx::query("SELECT picture_id FROM pictures;")
		.fetch(&mut *db);
	let mut pictures: Vec<String> = vec![];
	while let Some(row) = rows.try_next().await.ok()? {
		let picture_id: SqlxUuid = row.try_get(0).ok()?;
		pictures.push(picture_id.to_hyphenated().to_string());
	}
	Some(Json(pictures))
}

#[get("/<username>")]
async fn get_user_pictures(username: &str, mut db: Connection<PostgresDb>) -> Option<Json<Vec<String>>> {
	let mut rows = sqlx::query("
		SELECT picture_id
		FROM pictures JOIN accounts ON accounts.username = $1
		WHERE accounts.account_id = pictures.account_id;
	").bind(username).fetch(&mut *db);
	let mut pictures: Vec<String> = vec![];
	while let Some(row) = rows.try_next().await.ok()? {
		let picture_id: SqlxUuid = row.try_get(0).ok()?;
		pictures.push(picture_id.to_hyphenated().to_string());
	}
	Some(Json(pictures))
}

#[put("/like/<picture_id>")]
fn like(picture_id: SerdeUuid, sess: session::Connected) -> String {
	format!("PUT toggle like on picture {}\n", picture_id)
}

#[put("/comment/<picture_id>", data = "<content>")]
fn comment(picture_id: SerdeUuid, content: &str, sess: session::Connected) -> String {
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
