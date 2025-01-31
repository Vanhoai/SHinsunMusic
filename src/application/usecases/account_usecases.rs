use crate::{application::entities::account_entity::AccountEntity, core::http::failure::Failure};

#[async_trait::async_trait]
pub trait ManageAccountUseCases: Send + Sync {
    async fn find_profile_with_id(&self, id: &str) -> Result<AccountEntity, Failure>;
    async fn update_profile(&self, data: &AccountEntity) -> Result<AccountEntity, Failure>;
}
