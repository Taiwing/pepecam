#[macro_use] extern crate rocket;

mod session;
use session::*;

#[post("/register")]
fn register(session: UnconnectedSession) -> &'static str {
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
fn login(session: UnconnectedSession) -> &'static str {
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
fn put_user(session: ConnectedSession) -> String {
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
fn like(picture_id: &str, session: ConnectedSession) -> String {
	format!("PUT toggle like on picture {}\n", picture_id)
}

#[put("/comment/<picture_id>", data = "<content>")]
fn comment(picture_id: &str, content: &str, session: ConnectedSession) -> String {
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
