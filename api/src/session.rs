use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

pub struct UnconnectedSession {}

pub struct ConnectedSession {
	pub account_id: String,
}

#[derive(Debug)]
pub enum SessionError {
	LoggedIn,
	NotLoggedIn,
	InvalidSession,
}

fn is_valid_account(account_id: &str) -> bool {
	account_id == "valid_account_id" //TODO: check in db
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UnconnectedSession {
	type Error = SessionError;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		match request.cookies().get_private("account_id") {
			None => Outcome::Success(
				UnconnectedSession {}
			),
			Some(cookie) if is_valid_account(cookie.value()) => Outcome::Failure(
				(Status::BadRequest, SessionError::LoggedIn)
			),
			Some(_) => Outcome::Failure(
				(Status::BadRequest, SessionError::InvalidSession)
			),
		}
	}
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConnectedSession {
	type Error = SessionError;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		match request.cookies().get_private("account_id") {
			None => Outcome::Failure(
				(Status::BadRequest, SessionError::NotLoggedIn)
			),
			Some(cookie) if is_valid_account(cookie.value()) => Outcome::Success(
				ConnectedSession { account_id: cookie.value().to_string() }
			),
			Some(_) => Outcome::Failure(
				(Status::BadRequest, SessionError::InvalidSession)
			),
		}
	}
}
