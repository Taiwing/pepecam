use crate::auth::session;

#[put("/login")]
pub fn put(_sess: session::Unconnected) -> &'static str {
    "login\n"
}
