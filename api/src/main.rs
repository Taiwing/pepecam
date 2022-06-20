#[macro_use]
extern crate rocket;
extern crate rand;

use rocket::serde::{json::Json, uuid::Uuid, Deserialize};
use rocket_db_pools::{Connection, Database};

//TODO: Use this to create a BackgroundJob structure and manage a redis-like
//LocalCache for temporary data. The background job would be used to enforce
//expiries on said data (like for confirmation/reset tokens for example).
/*
use rocket::fairing::AdHoc;
use rocket::tokio::time::{sleep, Duration};
*/

mod query;
mod result;
mod routes;
mod session;

use query::PostgresDb;

#[get("/")]
async fn get_pictures(db: Connection<PostgresDb>) -> Option<Json<Vec<String>>> {
    match query::pictures(db).await {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    username: String,
}

#[get("/user", data = "<user>", format = "json")]
async fn get_user_pictures(
    user: Json<User>,
    db: Connection<PostgresDb>,
) -> Option<Json<Vec<String>>> {
    let username: &str = &user.into_inner().username;
    match query::user_pictures(db, username).await {
        None => None,
        Some(pictures) => Some(Json(pictures)),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Picture {
    picture_id: Uuid,
}

#[put("/like", data = "<picture>", format = "json")]
fn like(picture: Json<Picture>, sess: session::Connected) -> String {
    let picture_id = &picture.into_inner().picture_id;
    format!(
        "PUT toggle like on picture {} as {}\n",
        picture_id, sess.account_id
    )
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PictureComment {
    picture_id: Uuid,
    comment: String,
}

#[put("/comment", data = "<picture_comment>", format = "json")]
fn comment(
    picture_comment: Json<PictureComment>,
    sess: session::Connected,
) -> String {
    let picture_comment = picture_comment.into_inner();
    format!(
        "PUT comment '{}' on picture {} as {}\n",
        &picture_comment.comment, picture_comment.picture_id, sess.account_id
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PostgresDb::init())
        /*
        .attach(AdHoc::try_on_ignite("Background Job", |rocket| async {
            rocket::tokio::task::spawn(async {
                let mut iter: u128 = 0;
                loop {
                    iter = iter + 1;
                    println!("iter: {}", iter);
                    sleep(Duration::from_secs(5)).await;
                }
            });
            Ok(rocket)
        }))
        */
        .mount("/user", routes![routes::user::register::post])
        .mount("/user", routes![routes::user::confirm::post])
        .mount("/user", routes![routes::user::login::put])
        .mount("/user", routes![routes::user::logout::put])
        .mount("/user", routes![routes::user::reset::get])
        .mount("/user", routes![routes::user::reset::put])
        .mount("/user", routes![routes::user::put])
        .mount("/pictures", routes![get_pictures])
        .mount("/pictures", routes![get_user_pictures])
        .mount("/pictures", routes![like])
        .mount("/pictures", routes![comment])
        .register("/", catchers![result::not_found])
}
