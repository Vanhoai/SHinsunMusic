use std::sync::Arc;

use axum::Router;

use crate::{
    adapters::primary::{accounts, audios, auth},
    core::configs::app_config::APP_CONFIG,
    state::AppState,
};

pub fn init_router(state: Arc<AppState>) -> Router {
    let prefix = APP_CONFIG.common.api_version.clone();

    let auth_prefix = format!("/{}/auth", prefix);
    let account_prefix = format!("/{}/accounts", prefix);
    let audio_prefix = format!("/{}/audios", prefix);

    Router::new()
        .nest(auth_prefix.as_str(), auth::handler::execute())
        .nest(
            account_prefix.as_str(),
            accounts::handler::execute(state.clone()),
        )
        .nest(audio_prefix.as_str(), audios::handler::execute())
        .with_state(state)
}
