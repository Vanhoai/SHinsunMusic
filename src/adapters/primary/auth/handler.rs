use axum::{routing::post, Router};

use crate::adapters::primary::auth::routes::oauth_func;

use super::routes::{otp_func, sign_in_func};

pub fn execute() -> Router {
    println!("/auth/oauth");
    println!("/auth/sign-in");
    println!("/auth/otp");

    Router::new()
        .route("/oauth", post(oauth_func::execute))
        .route("/sign-in", post(sign_in_func::execute))
        .route("/otp", post(otp_func::execute))
}
