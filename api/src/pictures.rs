//! Constants and enums used to manipulate pictures and superposables

use strum::{self, AsRefStr, EnumIter, EnumString};

// Superposable picture names
#[derive(EnumString, AsRefStr, EnumIter)]
#[strum(serialize_all = "lowercase")] // Every Superposable name is in lowercase
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
