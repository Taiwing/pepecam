//! Build `Json` responses for the api.

use rocket::http::{ContentType, Method, Status};
use rocket::serde::{json::Json, Serialize};
use rocket::{
    response::{self, Responder},
    Request, Response,
};

/// Return this in case of request failure. This documents the failing request
/// and can be customized with the `message` field.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    status: u16,
    error: String,
    message: String,
    method: Method,
    path: String,
}

impl ApiError {
    fn new(status: Status, message: &str, method: Method, path: &str) -> Self {
        ApiError {
            status: status.code,
            error: String::from(status.reason_lossy()),
            message: String::from(message),
            method,
            path: String::from(path),
        }
    }
}

/// Result structure for the api.
///
/// The route response will be built from this since it implements rocket's
/// `Responder` trait. The final response will have the `status` given to
/// `ApiResult` and will contain a `Json` payload of type `T` on success
/// (typically from the [`payload`](crate::payload) module) as long as it
/// implements serde's `Serialize` trait. Otherwise it will return an `ApiError`
/// containing the given `message`.
///
/// # Example
///
/// ```rust
/// // Outgoing data payload that can be 'jsonified'.
/// #[derive(Serialize)]
/// pub struct OutgoingData {
///     field: String,
/// }
///
/// // Route returning an ApiResult depending on the input value.
/// #[post("/my-post-route", data = "<number>")]
/// fn my_post_route(number: u32) -> ApiResult<OutgoingData> {
///     if number > 0 {
///         ApiResult::Success {
///             status: Status::Ok,
///             payload: OutgoingData {
///                 field: number.to_string()
///             }
///         }
///     } else {
///         ApiResult::Failure {
///             status: Status::BadRequest,
///             message: String::from("this is not right!"),
///         }
///     }
/// }
/// ```
pub enum ApiResult<T: Serialize> {
    Success { status: Status, payload: T },
    Failure { status: Status, message: String },
}

fn build_success_response<'r, T: Serialize>(
    status: Status,
    payload: T,
    request: &Request,
) -> Response<'r> {
    let body = Json(payload);
    Response::build_from(
        body.respond_to(request).expect("failed to build response"),
    )
    .header(ContentType::JSON)
    .status(status)
    .finalize()
}

fn build_failure_response<'r, 's, 't>(
    status: Status,
    message: &'r str,
    request: &'s Request,
) -> Response<'t> {
    let body = Json(ApiError::new(
        status,
        message,
        request.method(),
        &request.uri().path().to_string(),
    ));
    Response::build_from(
        body.respond_to(request).expect("failed to build response"),
    )
    .header(ContentType::JSON)
    .status(status)
    .finalize()
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResult<T> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        match self {
            ApiResult::Success { status, payload } => {
                Ok(build_success_response(status, payload, request))
            }
            ApiResult::Failure { status, message } => {
                Ok(build_failure_response(status, &message, request))
            }
        }
    }
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        status,
        "unexepected error",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(400)]
pub fn bad_request(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::BadRequest,
        "invalid request",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::Unauthorized,
        "user must be logged in to execute this request",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(403)]
pub fn forbidden(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::Forbidden,
        "this request is not allowed",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::NotFound,
        "requested resource does not exist",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::UnprocessableEntity,
        "the request cannot be processed because the payload is ill-formed",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(500)]
pub fn internal_error(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::InternalServerError,
        "Ooooooops.... Looks like we messed up, sorry :)",
        req.method(),
        &req.uri().path().to_string(),
    ))
}
