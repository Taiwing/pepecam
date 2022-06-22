//! Api crate for 42's Camagru web project.

#[macro_use]
extern crate rocket;
extern crate rand;

mod cache;
mod payload;
mod query;
mod result;
mod routes;
mod session;

use cache::Cache;
use payload::NewUser;
use query::PostgresDb;
use rocket::fairing::AdHoc;
use rocket::tokio::time::{sleep, Duration};
use rocket_db_pools::Database;

// Interval in seconds for cleanup of expired values from the caches
const CACHE_CLEANUP_INTERVAL: u64 = 5;

#[launch]
fn rocket() -> _ {
    // Remember to add the Cache cleanup call here when creating a managed Cache
    let cleanup_job =
        AdHoc::try_on_ignite("Cache Cleanup Job", |rocket| async {
            let new_users = rocket.state::<Cache<NewUser>>().unwrap().clone();
            rocket::tokio::task::spawn(async move {
                loop {
                    new_users.cleanup();
                    sleep(Duration::from_secs(CACHE_CLEANUP_INTERVAL)).await;
                }
            });
            Ok(rocket)
        });

    rocket::build()
        .attach(PostgresDb::init())
        .manage(Cache::<NewUser>::new())
        .attach(cleanup_job)
        .mount("/user", routes![routes::user::register::post])
        .mount("/user", routes![routes::user::confirm::post])
        .mount("/user", routes![routes::user::login::put])
        .mount("/user", routes![routes::user::logout::put])
        .mount("/user", routes![routes::user::reset::get])
        .mount("/user", routes![routes::user::reset::put])
        .mount("/user", routes![routes::user::put])
        .mount("/pictures", routes![routes::pictures::user::get])
        .mount("/pictures", routes![routes::pictures::like::put])
        .mount("/pictures", routes![routes::pictures::comment::put])
        .mount("/pictures", routes![routes::pictures::get])
        .register("/", catchers![result::default])
        .register("/", catchers![result::bad_request])
        .register("/", catchers![result::unauthorized])
        .register("/", catchers![result::forbidden])
        .register("/", catchers![result::not_found])
        .register("/", catchers![result::unprocessable_entity])
        .register("/", catchers![result::internal_error])
}
