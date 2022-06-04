#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

struct Session {
	account_id: String,
}

#[derive(Debug)]
enum SessionError {
	LoggedIn,
	NotLoggedIn,
	InvalidSession,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
	type Error = SessionError;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		fn is_valid(account_id: &str) -> bool {
			account_id == "valid_account_id" //TODO: check in db
		}

		match request.cookies().get_private("account_id") {
			None => Outcome::Failure(
				(Status::BadRequest, SessionError::NotLoggedIn)
			),
			Some(cookie) if is_valid(cookie.value()) => Outcome::Success(
				Session { account_id: cookie.value().to_string() }
			),
			Some(_) => Outcome::Failure(
				(Status::BadRequest, SessionError::InvalidSession)
			),
		}
	}
}

#[post("/register")]
fn register() -> &'static str {
	"register\n"
	//TODO: generate confirmation_token and send it through an email
}

#[post("/confirm/<confirmation_token>")]
fn confirm(confirmation_token: u128) -> String {
	format!("confirm your email with: {}\n", confirmation_token)
	//TODO: check confirmation token
	//add new user to database and log user if the token is valid
	//return an error otherwise
}

#[put("/login")]
fn login() -> &'static str {
	"login\n"
}

#[put("/logout")]
fn logout() -> &'static str {
	"logout\n"
}

#[get("/reset-token")]
fn reset_token() -> &'static str {
	"reset-token: request reset token\n"
}

#[put("/password", data = "<reset_token>")]
fn password(reset_token: &str) -> String {
	format!("password: use reset-token '{}' to reset password\n", reset_token)
}

#[put("/")]
fn put_user(session: Session) -> String {
	format!("PUT user ({}): change username, password and/or email settings\n",
		session.account_id)
}

#[get("/")]
fn get_pictures() -> &'static str {
	"GET the pictures list\n"
}

#[get("/<username>")]
fn get_user_pictures(username: &str) -> String {
	format!("GET all the pictures for user {}\n", username)
}

#[put("/like/<picture_id>")]
fn like(picture_id: &str, session: Session) -> String {
	format!("PUT toggle like on picture {}\n", picture_id)
}

#[put("/comment/<picture_id>", data = "<content>")]
fn comment(picture_id: &str, content: &str, session: Session) -> String {
	format!("PUT comment '{}' on picture {}\n", content, picture_id)
}

#[launch]
fn rocket() -> _ {
	rocket::build()
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
}
