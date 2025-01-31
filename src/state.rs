use std::sync::Arc;

use crate::core::jwt::interface::JwtService;

#[derive(Clone)]
pub struct AppState {
    pub jwt_service: Arc<dyn JwtService>,
}
