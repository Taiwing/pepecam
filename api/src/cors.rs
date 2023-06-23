use crate::config;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Attaching Cors headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r Request<'_>,
        response: &mut Response<'r>,
    ) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            config::FRONT_LINK.as_str(),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Credentials",
            "true",
        ));
    }
}
