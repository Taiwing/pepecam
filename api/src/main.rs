//! Api crate for 42's Camagru web project.

#[macro_use]
extern crate rocket;
extern crate argon2;
extern crate lazy_static;
extern crate lettre;
extern crate photon_rs;
extern crate rand;

mod auth;
mod cache;
mod config;
mod cors;
mod mail;
mod payload;
mod pictures;
mod query;
mod result;
mod routes;
mod uuid;
mod validation;

use auth::session;
use cache::Cache;
use cors::Cors;
use mail::Mailer;
use payload::{Email, NewUser};
use query::PostgresDb;
use rocket::fairing::AdHoc;
use rocket::tokio::time::{sleep, Duration};
use rocket_db_pools::Database;
use routes::user::reset;

#[launch]
fn rocket() -> _ {
    // Remember to add the Cache cleanup call here when creating a managed Cache
    let cleanup_job =
        AdHoc::try_on_ignite("Cache Cleanup Job", |rocket| async {
            let new_users = rocket
                .state::<Cache<NewUser>>()
                .expect("Failed to get NewUser cache")
                .clone();
            let sessions = rocket
                .state::<Cache<session::Connected>>()
                .expect("Failed to get connected session cache")
                .clone();
            let reset_requests = rocket
                .state::<Cache<reset::Request>>()
                .expect("Failed to get reset request cache")
                .clone();
            let new_emails = rocket
                .state::<Cache<Email>>()
                .expect("Failed to get new email cache")
                .clone();
            rocket::tokio::task::spawn(async move {
                loop {
                    new_users.cleanup();
                    sessions.cleanup();
                    reset_requests.cleanup();
                    new_emails.cleanup();
                    sleep(Duration::from_secs(*config::CACHE_CLEANUP_INTERVAL))
                        .await;
                }
            });
            Ok(rocket)
        });

    rocket::build()
        .attach(PostgresDb::init())
        .manage(Mailer::new())
        .manage(Cache::<NewUser>::new())
        .manage(Cache::<session::Connected>::new())
        .manage(Cache::<reset::Request>::new())
        .manage(Cache::<Email>::new())
        .attach(cleanup_job)
        .attach(Cors)
        .mount("/", routes![routes::options])
        .mount("/user", routes![routes::user::register::post])
        .mount("/user", routes![routes::user::confirm::post])
        .mount("/user", routes![routes::user::login::post])
        .mount("/user", routes![routes::user::logout::post])
        .mount("/user", routes![routes::user::reset::post])
        .mount("/user", routes![routes::user::reset::put])
        .mount("/user", routes![routes::user::email::post])
        .mount("/user", routes![routes::user::put])
        .mount("/user", routes![routes::user::get])
        .mount("/picture", routes![routes::picture::like::put])
        .mount("/picture", routes![routes::picture::like::delete])
        .mount("/picture", routes![routes::picture::comment::post])
        .mount("/picture", routes![routes::picture::comments::get])
        .mount("/picture", routes![routes::picture::post])
        .mount("/picture", routes![routes::picture::delete])
        .mount("/pictures", routes![routes::pictures::superposable::get])
        .mount("/pictures", routes![routes::pictures::get])
        .register("/", catchers![result::default])
        .register("/", catchers![result::bad_request])
        .register("/", catchers![result::unauthorized])
        .register("/", catchers![result::forbidden])
        .register("/", catchers![result::not_found])
        .register("/", catchers![result::unprocessable_entity])
        .register("/", catchers![result::internal_error])
}
