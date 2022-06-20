#[post("/confirm/<confirmation_token>")]
pub fn post(confirmation_token: u128) -> String {
    format!("confirm your email with: {}\n", confirmation_token)
    //TODO: check confirmation token
    //add new user to database and log user if the token is valid
    //return an error otherwise
}
