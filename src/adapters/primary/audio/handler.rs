use axum::{routing::post, Router};

use super::routes::download_func;

pub fn execute() -> Router {
    println!("/audio/download");
    Router::new().route("/download", post(download_func::execute))
}
