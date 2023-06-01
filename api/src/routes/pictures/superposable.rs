use crate::pictures;
use rocket::serde::json::Json;
use strum::IntoEnumIterator;

#[get("/superposable")]
pub async fn get() -> Option<Json<Vec<String>>> {
    let mut superposables = Vec::new();
    for superposable in pictures::Superposable::iter() {
        superposables.push(superposable.as_ref().to_string());
    }

    if superposables.is_empty() {
        return None;
    }

    Some(Json(superposables))
}
