//! Api crate for 42's Camagru web project.

#[macro_use]
extern crate rocket;
extern crate argon2;
extern crate photon_rs;
extern crate rand;

mod auth;
mod cache;
mod cors;
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
use payload::NewUser;
use query::PostgresDb;
use rocket::fairing::AdHoc;
use rocket::tokio::time::{sleep, Duration};
use rocket_db_pools::Database;
use routes::user::reset;

// Interval in seconds for cleanup of expired values from the caches
const CACHE_CLEANUP_INTERVAL: u64 = 5;

#[launch]
fn rocket() -> _ {
    // Remember to add the Cache cleanup call here when creating a managed Cache
    let cleanup_job =
        AdHoc::try_on_ignite("Cache Cleanup Job", |rocket| async {
            let new_users = rocket.state::<Cache<NewUser>>().unwrap().clone();
            let sessions =
                rocket.state::<Cache<session::Connected>>().unwrap().clone();
            let reset_requests =
                rocket.state::<Cache<reset::Request>>().unwrap().clone();
            rocket::tokio::task::spawn(async move {
                loop {
                    new_users.cleanup();
                    sessions.cleanup();
                    reset_requests.cleanup();
                    sleep(Duration::from_secs(CACHE_CLEANUP_INTERVAL)).await;
                }
            });
            Ok(rocket)
        });

    rocket::build()
        .attach(PostgresDb::init())
        .manage(Cache::<NewUser>::new())
        .manage(Cache::<session::Connected>::new())
        .manage(Cache::<reset::Request>::new())
        .attach(cleanup_job)
        .attach(Cors)
        .mount("/", routes![routes::options])
        .mount("/user", routes![routes::user::register::post])
        .mount("/user", routes![routes::user::confirm::post])
        .mount("/user", routes![routes::user::login::post])
        .mount("/user", routes![routes::user::logout::post])
        .mount("/user", routes![routes::user::reset::get])
        .mount("/user", routes![routes::user::reset::post])
        .mount("/user", routes![routes::user::put])
        .mount("/user", routes![routes::user::get])
        .mount("/picture", routes![routes::picture::like::put])
        .mount("/picture", routes![routes::picture::like::delete])
        .mount("/picture", routes![routes::picture::comment::post])
        .mount("/picture", routes![routes::picture::comments::get])
        .mount("/picture", routes![routes::picture::post])
        .mount("/picture", routes![routes::picture::delete])
        .mount("/pictures", routes![routes::pictures::get])
        .register("/", catchers![result::default])
        .register("/", catchers![result::bad_request])
        .register("/", catchers![result::unauthorized])
        .register("/", catchers![result::forbidden])
        .register("/", catchers![result::not_found])
        .register("/", catchers![result::unprocessable_entity])
        .register("/", catchers![result::internal_error])
}
