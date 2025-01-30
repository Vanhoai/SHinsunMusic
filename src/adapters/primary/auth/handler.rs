use axum::{routing::post, Router};

use super::routes::{otp_func, sign_in_func};
use crate::adapters::primary::auth::routes::verify_func;

pub fn execute() -> Router {
    println!("/auth/verify-token");
    println!("/auth/sign-in");
    println!("/auth/otp");

    Router::new()
        .route("/verify-token", post(verify_func::execute))
        .route("/sign-in", post(sign_in_func::execute))
        .route("/otp", post(otp_func::execute))
}
