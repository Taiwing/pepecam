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
    use crate::cache::Cache;
    use crate::query::{self, PostgresDb};
    use crate::uuid::{SerdeUuid, SqlxUuid};
    use rocket::request::{FromRequest, Outcome, Request};
    use rocket::serde::{json, Deserialize, Serialize};
    use rocket::{http::Status, State};
    use rocket_db_pools::Connection;
    use std::fmt;

    /// Check if the user account exists.
    async fn is_valid_account(
        account_id: &str,
        db: &mut Connection<PostgresDb>,
    ) -> bool {
        match SqlxUuid::parse_str(account_id) {
            Err(_) => false,
            Ok(uuid) => query::account_exists(&uuid, db).await,
        }
    }

    /// The user must not be logged in to use the given route.
    pub struct Unconnected {}

    /// The user must be logged in to use the given route.
    #[derive(Serialize, Deserialize, Clone)]
    #[serde(crate = "rocket::serde")]
    pub struct Connected {
        /// account identifier in the database
        pub account_id: SerdeUuid,
        /// unique session identifier
        pub session_id: SerdeUuid,
    }

    /// The user may or may not be logged in to use the given route.
    pub struct IsConnected(pub Option<Connected>);

    impl Connected {
        /// Create a new connected session for the given user
        pub fn new(account_id: SerdeUuid) -> Self {
            Connected {
                account_id,
                session_id: SerdeUuid::new_v4(),
            }
        }

        /// Create a new connected session from a string
        pub fn from_str(session: &str) -> Option<Self> {
            match json::from_str(session) {
                Ok(connected) => Some(connected),
                Err(_) => None,
            }
        }

        /// Check that the stored session matches the given cookie and that the
        /// user actually exists.
        pub async fn is_valid(
            &self,
            sessions: &Cache<Connected>,
            db: &mut Connection<PostgresDb>,
        ) -> bool {
            let account_id = self.account_id.to_string();
            match sessions.get(&account_id) {
                Some(stored_session) => {
                    stored_session.session_id == self.session_id
                        && is_valid_account(&account_id, db).await
                }
                None => false,
            }
        }
    }

    impl fmt::Display for Connected {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            let string = json::to_string(self).unwrap();
            fmt.write_str(&string)?;
            Ok(())
        }
    }

    #[derive(Debug)]
    pub enum Error {
        LoggedIn,
        NotLoggedIn,
        InvalidSession,
    }

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for Unconnected {
        type Error = Error;

        async fn from_request(
            request: &'r Request<'_>,
        ) -> Outcome<Self, Self::Error> {
            match request.cookies().get("session") {
                None => Outcome::Success(Unconnected {}),
                Some(_session_cookie) => {
                    Outcome::Failure((Status::Forbidden, Error::LoggedIn))
                }
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
            let sessions =
                request.guard::<&State<Cache<Connected>>>().await.unwrap();
            match request.cookies().get("session") {
                None => {
                    Outcome::Failure((Status::Unauthorized, Error::NotLoggedIn))
                }
                Some(cookie) => match Connected::from_str(cookie.value()) {
                    Some(session)
                        if session.is_valid(sessions, &mut db).await =>
                    {
                        Outcome::Success(session)
                    }
                    _ => Outcome::Failure((
                        Status::BadRequest,
                        Error::InvalidSession,
                    )),
                },
            }
        }
    }

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for IsConnected {
        type Error = Error;

        async fn from_request(
            request: &'r Request<'_>,
        ) -> Outcome<Self, Self::Error> {
            let mut db =
                request.guard::<Connection<PostgresDb>>().await.unwrap();
            let sessions =
                request.guard::<&State<Cache<Connected>>>().await.unwrap();
            match request.cookies().get("session") {
                None => Outcome::Success(IsConnected { 0: None }),
                Some(cookie) => match Connected::from_str(cookie.value()) {
                    Some(session)
                        if session.is_valid(sessions, &mut db).await =>
                    {
                        Outcome::Success(IsConnected { 0: Some(session) })
                    }
                    _ => Outcome::Success(IsConnected { 0: None }),
                },
            }
        }
    }
}
