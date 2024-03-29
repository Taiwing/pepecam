//! Re-usable payloads for the routes' incoming or outgoing json data.

use crate::pictures::Superposable;
use rocket::serde::{uuid::Uuid, Deserialize, Serialize};
use std::fmt;

/// Default response for routes that do not return a specific data structure.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DefaultResponse {
    pub response: String,
}

/// One-time token for user registration, password and email reset.
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub token: Uuid,
}

/// Request payload /reset and Cache structure for email reset.
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Email {
    pub email: String,
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

/// User profile data
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserProfile {
    pub username: String,
    pub email: String,
    pub email_notifications: bool,
}

/// Picture data
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Picture {
    pub picture_id: Uuid,
    pub account_id: Uuid,
    pub superposable: Superposable,
    pub creation_ts: i64,
    pub author: String,
    pub like_count: i64,
    pub dislike_count: i64,
    pub comment_count: i64,
    pub liked: Option<bool>,
    pub disliked: Option<bool>,
}

/// Picture ID
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PictureId {
    pub picture_id: Uuid,
}

/// Comment data
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Comment {
    pub picture_id: Uuid,
    pub account_id: Uuid,
    pub creation_ts: i64,
    pub content: String,
    pub author: String,
}
