use axum::{routing::post, Router};

use crate::adapters::primary::auth::routes::{sign_in_func, verify_func};

pub fn execute() -> Router {
    println!("/auth/verify-token");
    println!("/auth/sign-in");
    println!("/auth/otp");

    Router::new()
        .route("/verify-token", post(verify_func::execute))
        .route("/sign-in", post(sign_in_func::execute))
}
