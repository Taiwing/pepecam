pub mod confirm;
pub mod login;
pub mod logout;
pub mod register;
pub mod reset;

use crate::auth::session;

#[put("/")]
pub fn put(sess: session::Connected) -> String {
    format!(
        "PUT user ({}): change username, password and/or email settings\n",
        sess.account_id
    )
}
