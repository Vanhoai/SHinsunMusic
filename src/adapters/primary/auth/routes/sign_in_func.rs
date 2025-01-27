use axum::{http::StatusCode, response::IntoResponse};

pub async fn execute() -> Result<impl IntoResponse, StatusCode> {
    Ok("Hello World !!")
}
