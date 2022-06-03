#[macro_use] extern crate rocket;

#[post("/register")]
fn register() -> &'static str {
	"register\n"
}

#[post("/confirm/<confirmation_token>")]
fn confirm(confirmation_token: u128) -> String {
	format!("confirm {}\n", confirmation_token)
}

#[put("/login")]
fn login() -> &'static str {
	"login\n"
}

#[put("/logout")]
fn logout() -> &'static str {
	"logout\n"
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/user", routes![register, confirm, login, logout])
}
