use async_trait::async_trait;
use std::sync::Arc;

use crate::application::repositories::account_repository::AccountRepository;

#[async_trait]
pub trait AuthService: Send + Sync {}

pub struct AuthServiceImpl {
    pub repository: Arc<dyn AccountRepository>,
}

impl AuthServiceImpl {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        AuthServiceImpl { repository }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {}
