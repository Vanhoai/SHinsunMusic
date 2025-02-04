use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// struct HttpResponse
/// This is structure that will be returned to the client without error.
/// The `message` field is message send to client notify him that everything is ok.
/// The `status` field is HTTP status code.
/// The `payload` field is actual data that will be returned to the client.
pub struct HttpResponse<T: Serialize> {
    pub status: StatusCode,
    pub message: String,
    pub payload: T,
}

/// Create a new `HttpResponse` from a `StatusCode`, `String` and `T`.
impl<T: Serialize> HttpResponse<T> {
    pub fn new(status: StatusCode, message: String, payload: T) -> Self {
        HttpResponse {
            status,
            message,
            payload,
        }
    }
}

/// I want to be able to convert a `HttpResponse` into a `axum::response::Response`.
/// In server alway return in header with status code is 200 and body is json.
/// In body will show actual error message and status code.
/// 1. message - is message send to client
/// 2. status - is HTTP status code -> convert to u16 (it a number like 200, 400, 500)
/// 3. payload - is actual data
impl<T: Serialize> IntoResponse for HttpResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let response = Json(json!({
            "message": self.message,
            "status": self.status.as_u16(),
            "payload": self.payload,
        }));

        (StatusCode::OK, response).into_response()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub page: u32,
    pub page_size: u32,
    pub total_page: u32,
    pub total_record: u32,
}

pub struct HttpPaginationResponse<T: Serialize> {
    pub status: StatusCode,
    pub message: String,
    pub meta: Meta,
    pub payload: Vec<T>,
}

impl<T: Serialize> HttpPaginationResponse<T> {
    pub fn new(status: StatusCode, message: String, meta: Meta, payload: Vec<T>) -> Self {
        HttpPaginationResponse {
            status,
            message,
            meta,
            payload,
        }
    }
}

impl<T: Serialize> IntoResponse for HttpPaginationResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let response = Json(json!({
            "message": self.message,
            "status": self.status.as_u16(),
            "meta": self.meta,
            "payload": self.payload,
        }));

        (StatusCode::OK, response).into_response()
    }
}
