use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    adapters::primary::audios::routes::{
        create_with_exist_func, download_thumbnail_func, search_func,
    },
    state::AppState,
};

use super::routes::download_func;

pub fn execute() -> Router<Arc<AppState>> {
    println!("/audios/download");
    println!("/audios/download-thumbnail");
    println!("/audios/create-with-exist");
    println!("audios/search");

    Router::new()
        .route(
            "/download-thumbnail",
            post(download_thumbnail_func::execute),
        )
        .route("/download", post(download_func::execute))
        .route("/create-with-exist", post(create_with_exist_func::execute))
        .route("/search", get(search_func::execute))
}
