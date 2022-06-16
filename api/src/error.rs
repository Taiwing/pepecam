use rocket::{Request, http::Method, http::Status};
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
	let status = Status::NotFound;
	Json(ApiError {
		status: status.code,
		error: String::from(status.reason_lossy()),
		message: String::from("Requested resource does not exist"),
		method: req.method(),
		path: req.uri().path().to_string(),
	})
}
