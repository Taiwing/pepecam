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

//TODO: remove the relative PATH when the API is containerized
//TODO: also maybe use an env variable for the picture directory location
// Pictures directory path
pub const PATH: &str = "../front/pictures";
//const PATH: &str = "/pictures";

// Maximum size of pictures in mebibytes
pub const SIZEMAX: usize = 10;

//TODO: same as above, replace by containerized version
// Superposable pictures directory path
pub const SUPERPOSABLES_PATH: &str =
    concat!("../front/pictures", "/superposables");
//const SUPERPOSABLES_PATH: &str = concat!("/pictures", "/superposables");

// Superposable picture width and height in pixels
pub const SUPERPOSABLES_SIDE: u32 = 512;
