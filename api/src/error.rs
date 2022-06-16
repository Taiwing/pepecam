use rocket::{Request, http::Method};
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
	status: u16,
	error: String,
	message: String,
	method: Method,
	path: String,
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ApiError> {
	Json(ApiError {
		status: 404,
		error: String::from("Not Found"),
		message: String::from("Requested resource does not exist"),
		method: req.method(),
		path: req.uri().path().to_string(),
	})
}
