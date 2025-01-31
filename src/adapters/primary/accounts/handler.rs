use std::sync::Arc;

use axum::{middleware, routing::get, Router};

use crate::{
    adapters::primary::accounts::routes::find_profile_func, core::middlewares::auth_middleware,
    state::AppState,
};

pub fn execute(state: Arc<AppState>) -> Router<Arc<AppState>> {
    println!("/accounts/profile");

    let public_routes = Router::new();

    let private_routes = Router::new()
        .route("/profile", get(find_profile_func::execute))
        .layer(middleware::from_fn_with_state(
            state,
            auth_middleware::execute,
        ));

    Router::new().merge(public_routes).merge(private_routes)
}
