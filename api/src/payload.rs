//! Re-usable payloads for the routes' incoming or outgoing json data.

use rocket::serde::{uuid::Uuid, Deserialize, Serialize};
use std::fmt;

/// One-time token for user registration and password reset.
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    token: Uuid,
}

impl Token {
    pub fn new() -> Self {
        Token {
            token: Uuid::new_v4(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

/// New user data for user registration.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUser<'r> {
    pub username: &'r str,
    pub password: &'r str,
    pub email: &'r str,
}
