#[macro_use]
extern crate rocket;
extern crate rand;

mod cache;
mod query;
mod result;
mod routes;
mod session;

use cache::Cache;
use query::PostgresDb;
use rocket::fairing::AdHoc;
use rocket::tokio::time::{sleep, Duration};
use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    let cleanup_job =
        AdHoc::try_on_ignite("Cache Cleanup Job", |rocket| async {
            let cache_string = rocket.state::<Cache<String>>().unwrap().clone();
            let cache_u128 = rocket.state::<Cache<u128>>().unwrap().clone();
            rocket::tokio::task::spawn(async move {
                loop {
                    cache_string.cleanup();
                    cache_u128.cleanup();
                    sleep(Duration::from_secs(5)).await;
                }
            });
            Ok(rocket)
        });

    rocket::build()
        .attach(PostgresDb::init())
        .manage(Cache::<String>::new())
        .manage(Cache::<u128>::new())
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
