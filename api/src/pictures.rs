//! Constants and enums used to manipulate pictures and superposables

use rocket::serde::Serialize;
use rocket_db_pools::sqlx;
use strum::{self, AsRefStr, EnumIter, EnumString};

// Superposable picture names
#[derive(EnumString, AsRefStr, EnumIter, Serialize, sqlx::Type, Clone)]
#[strum(serialize_all = "lowercase")] // Every Superposable name is in lowercase
#[serde(crate = "rocket::serde")]
#[sqlx(type_name = "superposable", rename_all = "lowercase")] // For postgresql
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
