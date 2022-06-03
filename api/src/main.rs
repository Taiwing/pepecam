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
	"PUT user/: change username, password and/or email\n"
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
}
