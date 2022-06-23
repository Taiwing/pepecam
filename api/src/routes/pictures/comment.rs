use crate::auth::session;
use rocket::serde::{json::Json, uuid::Uuid, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PictureComment {
    picture_id: Uuid,
    comment: String,
}

#[put("/comment", data = "<picture_comment>", format = "json")]
pub fn put(
    picture_comment: Json<PictureComment>,
    sess: session::Connected,
) -> String {
    let picture_comment = picture_comment.into_inner();
    format!(
        "PUT comment '{}' on picture {} as {}\n",
        &picture_comment.comment, picture_comment.picture_id, sess.account_id
    )
}
