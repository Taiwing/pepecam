/// Use Argon2 to create a password and check if it is valid.
pub mod password {
    use argon2::{self, Config};
    use rand;

    /// Hash a password with a random salt and default Argon2 configuration.
    pub fn hash(password: &str) -> String {
        let config = Config::default();
        let salt: [u8; 16] = rand::random();
        argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
    }

    /// Check if the password matches against a given hash.
    pub fn verify(password: &str, hash: &str) -> bool {
        argon2::verify_encoded(hash, password.as_bytes()).unwrap()
    }
}

/// Manage user sessions with private cookies.
pub mod session {
    use crate::query::{self, PostgresDb};
    use rocket::http::Status;
    use rocket::request::{FromRequest, Outcome, Request};
    use rocket_db_pools::Connection;
    use sqlx::types::Uuid;

    /// The user must not be logged in to use the given route.
    pub struct Unconnected {}

    /// The user must be logged in to use the given route.
    pub struct Connected {
        pub account_id: String,
    }

    #[derive(Debug)]
    pub enum Error {
        LoggedIn,
        NotLoggedIn,
        InvalidSession,
    }

    /// Check if the user account exists.
    async fn is_valid_account(
        account_id: &str,
        db: &mut Connection<PostgresDb>,
    ) -> bool {
        match Uuid::parse_str(account_id) {
            Err(_) => false,
            Ok(uuid) => query::account_exists(&uuid, db).await,
        }
    }

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for Unconnected {
        type Error = Error;

        async fn from_request(
            request: &'r Request<'_>,
        ) -> Outcome<Self, Self::Error> {
            let mut db =
                request.guard::<Connection<PostgresDb>>().await.unwrap();
            match request.cookies().get_private("account_id") {
                None => Outcome::Success(Unconnected {}),
                Some(cookie)
                    if is_valid_account(cookie.value(), &mut db).await =>
                {
                    Outcome::Failure((Status::Forbidden, Error::LoggedIn))
                }
                Some(_) => Outcome::Failure((
                    Status::BadRequest,
                    Error::InvalidSession,
                )),
            }
        }
    }

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for Connected {
        type Error = Error;

        async fn from_request(
            request: &'r Request<'_>,
        ) -> Outcome<Self, Self::Error> {
            let mut db =
                request.guard::<Connection<PostgresDb>>().await.unwrap();
            match request.cookies().get_private("account_id") {
                None => {
                    Outcome::Failure((Status::Unauthorized, Error::NotLoggedIn))
                }
                Some(cookie)
                    if is_valid_account(cookie.value(), &mut db).await =>
                {
                    Outcome::Success(Connected {
                        account_id: cookie.value().to_string(),
                    })
                }
                Some(_) => Outcome::Failure((
                    Status::BadRequest,
                    Error::InvalidSession,
                )),
            }
        }
    }
}
