use axum::{routing::post, Router};

use super::routes::{otp_func, sign_in_func};

pub fn execute() -> Router {
    println!("/auth/sign-in");
    println!("/auth/otp");

    Router::new()
        .route("/sign-in", post(sign_in_func::execute))
        .route("/otp", post(otp_func::execute))
}
