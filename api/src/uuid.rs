//! handle the two different Uuid types from serde and sqlx

pub use rocket::serde::uuid::Uuid as SerdeUuid;
pub use sqlx::types::Uuid as SqlxUuid;

/// Convert sqlx Uuid to Serde Uuid
pub fn from_sqlx_to_serde(uuid: &SqlxUuid) -> SerdeUuid {
    SerdeUuid::from_bytes(*uuid.as_bytes())
}
