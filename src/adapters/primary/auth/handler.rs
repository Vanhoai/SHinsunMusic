use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{
    adapters::primary::auth::routes::{refresh_token_func, sign_in_func},
    state::AppState,
};

pub fn execute() -> Router<Arc<AppState>> {
    println!("/auth/sign-in");
    println!("/auth/refresh-token");

    Router::new()
        .route("/sign-in", post(sign_in_func::execute))
        .route("/refresh-token", post(refresh_token_func::execute))
}
