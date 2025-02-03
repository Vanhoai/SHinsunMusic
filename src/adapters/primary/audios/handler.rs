use std::sync::Arc;

use axum::{routing::post, Router};

use crate::state::AppState;

use super::routes::download_func;

pub fn execute() -> Router<Arc<AppState>> {
    println!("/audios/download");

    Router::new().route("/download", post(download_func::execute))
}
