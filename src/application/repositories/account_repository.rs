use async_trait::async_trait;

use crate::application::entities::account_entity::AccountEntity;
use crate::core::http::failure::Failure;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn create(&self, data: &AccountEntity) -> Result<AccountEntity, Failure>;
    async fn find_by_email(&self, email: &str) -> Result<Option<AccountEntity>, Failure>;
    async fn find_by_id(&self, id: &str) -> Result<Option<AccountEntity>, Failure>;
}
