#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
	"Hello, web!\n"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
	format!("Hello, {}!\n", name)
}

#[get("/")]
fn mdr() -> &'static str {
	"MDRRRRR\n"
}

#[get("/lol")]
fn lol() -> &'static str {
	"LOOOOOOOOOOOOOOOL\n"
}

#[get("/xd")]
fn xd () -> &'static str {
	"XDDD\n"
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index])
		.mount("/", routes![hello])
		.mount("/mdr", routes![mdr, lol, xd])
}
