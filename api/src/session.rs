use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

pub struct Unconnected {}

pub struct Connected {
	pub account_id: String,
}

#[derive(Debug)]
pub enum Error {
	LoggedIn,
	NotLoggedIn,
	InvalidSession,
}

fn is_valid_account(account_id: &str) -> bool {
	account_id == "valid_account_id" //TODO: check in db
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Unconnected {
	type Error = Error;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		match request.cookies().get_private("account_id") {
			None => Outcome::Success(
				Unconnected {}
			),
			Some(cookie) if is_valid_account(cookie.value()) => Outcome::Failure(
				(Status::BadRequest, Error::LoggedIn)
			),
			Some(_) => Outcome::Failure(
				(Status::BadRequest, Error::InvalidSession)
			),
		}
	}
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Connected {
	type Error = Error;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		match request.cookies().get_private("account_id") {
			None => Outcome::Failure(
				(Status::BadRequest, Error::NotLoggedIn)
			),
			Some(cookie) if is_valid_account(cookie.value()) => Outcome::Success(
				Connected { account_id: cookie.value().to_string() }
			),
			Some(_) => Outcome::Failure(
				(Status::BadRequest, Error::InvalidSession)
			),
		}
	}
}
