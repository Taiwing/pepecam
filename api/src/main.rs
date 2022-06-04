#[macro_use] extern crate rocket;

#[post("/register")]
fn register() -> &'static str {
	"register\n"
}

#[post("/confirm/<confirmation_token>")]
fn confirm(confirmation_token: u128) -> String {
	format!("confirm your email with: {}\n", confirmation_token)
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

#[put("/password")]
fn password() -> &'static str {
	"password: use reset-token to reset password\n"
}

#[put("/")]
fn put_user() -> &'static str {
	"PUT user/: change username, password and/or email settings\n"
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
fn like(picture_id: &str) -> String {
	format!("PUT toggle like on picture {}\n", picture_id)
}

#[put("/comment/<picture_id>", data = "<content>")]
fn comment(picture_id: &str, content: &str) -> String {
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
