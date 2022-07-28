//! Api parameters validation.

use regex::{Regex, RegexSet};

// Username is simply a string of six to sixty-four word characters.
const USERNAME_REGEX: &str = r"^\w{6,64}$";

const PASSWORD_REGEX_COUNT: usize = 5;

// Password must contain one lowercase letter, one uppercase letter, a digit, a
// special character and must be at least 8 characters long.
const PASSWORD_REGEX: [&'static str; PASSWORD_REGEX_COUNT] =
    [r"[a-z]+", r"[A-Z]+", r"\d+", r"\W+", r".{8,}"];

const PASSWORD_REGEX_ERRORS: [&'static str; PASSWORD_REGEX_COUNT] = [
    "password must contain at least one lower case letter",
    "password must contain at least one upper case letter",
    "password must contain at least one digit",
    "password must contain at least one special character",
    "password must be at least eight characters long",
];

// HTML5 email regex. The email addres must only contain alphanumeric and non
// whitespace special characters for the first part. An '@' symbol and a domain
// name with a least a '.' in it.
const EMAIL_REGEX: &str =
    r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$";

//TODO: maybe use lazy_static macro crate to optimize the regex

/// Check that the given username is a valid string
pub fn username(username: &str) -> Result<(), String> {
    let re = Regex::new(USERNAME_REGEX).unwrap();
    if re.is_match(username) == false {
        return Err(String::from(
            "username must be a word of 6 to 64 characters long",
        ));
    }
    Ok(())
}

/// Check that the password is long and strong enough
pub fn password(password: &str) -> Result<(), String> {
    let set = RegexSet::new(&PASSWORD_REGEX).unwrap();
    let matches: Vec<_> = set.matches(password).into_iter().collect();
    if matches.len() != PASSWORD_REGEX_COUNT {
        for first_error in 0..PASSWORD_REGEX_COUNT {
            if !matches.contains(&first_error) {
                return Err(String::from(PASSWORD_REGEX_ERRORS[first_error]));
            }
        }
    }
    Ok(())
}

/// Check that the email actually looks like an email
pub fn email(email: &str) -> Result<(), String> {
    let re = Regex::new(EMAIL_REGEX).unwrap();
    if re.is_match(email) == false || email.len() > 256 {
        return Err(String::from("invalid email format"));
    }
    Ok(())
}
