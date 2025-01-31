use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{adapters::primary::auth::routes::sign_in_func, state::AppState};

pub fn execute() -> Router<Arc<AppState>> {
    println!("/auth/sign-in");

    Router::new().route("/sign-in", post(sign_in_func::execute))
}
