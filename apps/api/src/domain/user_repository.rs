use async_trait::async_trait;

use crate::domain::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        user: User,
    );

    async fn find_by_telegram_id(
        &self,
        telegram_id: &str,
    ) -> Option<User>;
}