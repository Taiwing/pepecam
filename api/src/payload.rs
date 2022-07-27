//! Re-usable payloads for the routes' incoming or outgoing json data.

use rocket::serde::{uuid::Uuid, Deserialize, Serialize};
use std::fmt;

/// Default response for routes that do not return a specific data structure.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DefaultResponse {
    pub response: String,
}

/// One-time token for user registration and password reset.
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub token: Uuid,
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
#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}
