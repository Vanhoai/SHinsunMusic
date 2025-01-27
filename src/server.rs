use axum::Router;

use crate::{
    adapters::primary::{audio, auth},
    core::configs::app_config::APP_CONFIG,
};

pub fn init_router() -> Router {
    let prefix = APP_CONFIG.common.api_version.clone();
    let auth_prefix = format!("/{}/auth", prefix);
    let audio_prefix = format!("/{}/audio", prefix);

    Router::new()
        .nest(auth_prefix.as_str(), auth::handler::execute())
        .nest(audio_prefix.as_str(), audio::handler::execute())
}
