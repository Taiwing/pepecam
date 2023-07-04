use crate::auth::session;
use crate::config;
use crate::mail::Mailer;
use crate::payload::Comment;
use crate::query::{self, PostgresDb};
use crate::result::ApiResult;
use crate::uuid::from_serde_to_sqlx;
use rocket::http::Status;
use rocket::serde::{json::Json, uuid::Uuid, Deserialize};
use rocket::State;
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PictureComment {
    picture_id: Uuid,
    comment: String,
}

async fn send_notification_email(
    mailer: &State<Mailer>,
    db: &mut Connection<PostgresDb>,
    picture_id: &Uuid,
    author: &str,
) {
    if let Some(email) =
        query::has_email_notifications(db, &from_serde_to_sqlx(picture_id))
            .await
    {
        let url = format!(
            "{}/index.html?picture={}",
            config::FRONT_LINK.as_str(),
            picture_id
        );
        _ = mailer.send(
            &email,
            &format!("New comment on {}", picture_id),
            &format!("{} commented on your picture: {}", author, url),
        );
    }
}

#[post("/comment", data = "<picture_comment>", format = "json")]
pub async fn post(
    picture_comment: Json<PictureComment>,
    sess: session::Connected,
    mut db: Connection<PostgresDb>,
    mailer: &State<Mailer>,
) -> ApiResult<Comment> {
    let picture_comment = picture_comment.into_inner();
    match query::comment(
        &mut db,
        &picture_comment.comment,
        &from_serde_to_sqlx(&picture_comment.picture_id),
        &from_serde_to_sqlx(&sess.account_id),
    )
    .await
    {
        Ok(comment) => {
            send_notification_email(
                mailer,
                &mut db,
                &comment.picture_id,
                &comment.author,
            )
            .await;
            ApiResult::Success {
                status: Status::Created,
                payload: comment,
            }
        }
        Err(_) => ApiResult::Failure {
            status: Status::BadRequest,
            message: String::from("invalid picture id"),
        },
    }
}
