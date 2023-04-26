//! Every api route handler.

pub mod picture;
pub mod pictures;
pub mod user;

/// CORS preflight handler.
#[options("/<_..>")]
pub fn options() {}
