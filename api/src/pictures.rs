//! Constants and enums used to manipulate pictures and superposables

use rocket::request::FromParam;
use rocket::serde::Serialize;
use rocket_db_pools::sqlx;
use std::str::FromStr;
use strum::{self, AsRefStr, EnumIter, EnumString};

// Superposable picture names
#[derive(
    Clone,
    Debug,
    PartialEq,
    Ord,
    PartialOrd,
    Eq,
    EnumString,
    AsRefStr,
    EnumIter,
    Serialize,
    sqlx::Type,
    FromFormField,
)]
#[strum(serialize_all = "lowercase")] // Every Superposable name is in lowercase
#[serde(crate = "rocket::serde")]
#[sqlx(type_name = "superposable", rename_all = "lowercase")] // For postgresql
#[serde(rename_all = "lowercase")] // For json
pub enum Superposable {
    Chic,
    Cry,
    Honk,
    Rage,
    Sad,
    Smirk,
    Stoned,
    Sweat,
}

impl<'a> FromParam<'a> for Superposable {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match Self::from_str(param) {
            Ok(superposable) => Ok(superposable),
            Err(_) => Err(param),
        }
    }
}
