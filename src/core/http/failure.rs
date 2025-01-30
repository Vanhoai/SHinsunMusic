use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

/// Failures that can be returned by the API.
/// These are all HTTP errors.
/// The `message` field is the message that will be returned to the client.
/// Sample use:
/// ```rust
/// use shared::http::failure::{Failure, HttpFailure};
///
/// let failure = Failure::BadRequest("Invalid request".to_string());
/// let response = HttpFailure::new(failure);
/// ```
#[derive(Debug)]
pub enum Failure {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    InternalServerError(String),
    MethodNotAllowed(String),
    UnknownFailure(String),
    Conflict(String),
    DatabaseError(String),
}

/// This is correct structure will be returned to the client.
/// The `message` field is the message that will be returned to the client.
pub struct HttpFailure {
    status: StatusCode,
    message: String,
}

/// Create a new `HttpFailure` from a `Failure`.
/// When create a new `HttpFailure` from a `Failure`, the `status` field will be set to the appropriate HTTP status code.
/// The `message` field will be set to the appropriate message for the `Failure`.
/// The `status` field will be set to the appropriate HTTP status code.
impl HttpFailure {
    pub fn new(failure: Failure) -> Self {
        let (status, message) = match failure {
            Failure::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Failure::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            Failure::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            Failure::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Failure::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Failure::MethodNotAllowed(msg) => (StatusCode::METHOD_NOT_ALLOWED, msg),
            Failure::UnknownFailure(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Failure::Conflict(msg) => (StatusCode::CONFLICT, msg),
            Failure::DatabaseError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        HttpFailure { status, message }
    }
}

/// I want to be able to convert a `Failure` into a `HttpFailure`.
/// In server alway return in header with status code is 200 and body is json.
/// In body will show actual error message and status code.
impl IntoResponse for HttpFailure {
    fn into_response(self) -> axum::response::Response {
        let response = Json(json!({
            "message": self.message,
            "status": self.status.as_u16(),
        }));

        (StatusCode::OK, response).into_response()
    }
}
