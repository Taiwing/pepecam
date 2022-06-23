use crate::auth::session;
use rocket::serde::{json::Json, uuid::Uuid, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Picture {
    picture_id: Uuid,
}

#[put("/like", data = "<picture>", format = "json")]
pub fn put(picture: Json<Picture>, sess: session::Connected) -> String {
    let picture_id = &picture.into_inner().picture_id;
    format!(
        "PUT toggle like on picture {} as {}\n",
        picture_id, sess.account_id
    )
}
