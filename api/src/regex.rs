// Username is simply a string of six to sixty-four word characters.
pub const USERNAME_REGEX: &str = r"^\w{6,64}$";

pub const PASSWORD_REGEX_COUNT: usize = 5;

// Password must contain one lowercase letter, one uppercase letter, a digit, a
// special character and must be at least 8 characters long.
pub const PASSWORD_REGEX: [&'static str; PASSWORD_REGEX_COUNT] =
    [r"[a-z]+", r"[A-Z]+", r"\d+", r"\W+", r".{8,}"];

pub const PASSWORD_REGEX_ERRORS: [&'static str; PASSWORD_REGEX_COUNT] = [
    "password must contain at least one lower case letter",
    "password must contain at least one upper case letter",
    "password must contain at least one digit",
    "password must contain at least one special character",
    "password must be at least eight characters long",
];

// HTML5 email regex. The email addres must only contain alphanumeric and non
// whitespace special characters for the first part. An '@' symbol and a domain
// name with a least a '.' in it.
pub const EMAIL_REGEX: &str =
    r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$";
