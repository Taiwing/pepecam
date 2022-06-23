use argon2::{self, Config};
use rand;

/// Hash a password with a random salt and default Argon2 configuration.
pub fn hash(password: &str) -> String {
    let config = Config::default();
    let salt: [u8; 16] = rand::random();
    println!("salt: {:?}", salt);
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}

/// Check if the password matches against a given hash.
pub fn verify(password: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
}
