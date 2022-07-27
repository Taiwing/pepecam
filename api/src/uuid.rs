//! handle the two different Uuid types from serde and sqlx

pub use rocket::serde::uuid::Uuid as SerdeUuid;
pub use sqlx::types::Uuid as SqlxUuid;

/// Convert Sqlx Uuid to Serde Uuid
pub fn from_sqlx_to_serde(uuid: &SqlxUuid) -> SerdeUuid {
    SerdeUuid::from_bytes(*uuid.as_bytes())
}

/// Convert Serde Uuid to Sqlx Uuid
pub fn from_serde_to_sqlx(uuid: &SerdeUuid) -> SqlxUuid {
    SqlxUuid::from_bytes(*uuid.as_bytes())
}
