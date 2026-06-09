use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use crate::{
    domain::user::User,
    infrastructure::repository::in_memory::user_repository::InMemoryUserRepository,
};

pub struct AuthenticateTelegramUserRequest {
    pub telegram_id: String,
    pub display_name: String,
}

pub struct AuthenticateTelegramUserResponse {
    pub user: User,
}

pub struct AuthenticateTelegramUser{
    pub repository:
        Arc<InMemoryUserRepository>,
}

impl AuthenticateTelegramUser {
    pub fn new(
        repository:
        Arc<InMemoryUserRepository>,
    ) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        request: AuthenticateTelegramUserRequest,
    ) -> AuthenticateTelegramUserResponse {
        if let Some(user) = self
            .repository
            .find_by_telegram_id(
                &request.telegram_id,
            )
            .await
        {
            return AuthenticateTelegramUserResponse {
                user,
            };
        }

        let user = User {
            id: Uuid::new_v4(),

            telegram_id: request.telegram_id,

            display_name: request.display_name,

            created_at: Utc::now(),
        };

        self.repository
            .create(user.clone())
            .await;

        AuthenticateTelegramUserResponse {
            user,
        }
    }
}