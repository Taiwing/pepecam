#[macro_use]
extern crate rocket;
extern crate rand;

mod query;
mod result;
mod routes;
mod session;

use query::PostgresDb;
use rocket_db_pools::Database;

//TODO: Use this to create a BackgroundJob structure and manage a redis-like
//LocalCache for temporary data. The background job would be used to enforce
//expiries on said data (like for confirmation/reset tokens for example).
/*
use rocket::fairing::AdHoc;
use rocket::tokio::time::{sleep, Duration};
*/

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
        .mount("/pictures", routes![routes::pictures::user::get])
        .mount("/pictures", routes![routes::pictures::like::put])
        .mount("/pictures", routes![routes::pictures::comment::put])
        .mount("/pictures", routes![routes::pictures::get])
        .register("/", catchers![result::not_found])
}
