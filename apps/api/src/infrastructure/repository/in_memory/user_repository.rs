use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::domain::user::{
    User,
    UserId,
};

pub struct InMemoryUserRepository {
    users: RwLock<HashMap<
        UserId,
        User,
    >>,
}


impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(
                HashMap::new(),
            ),
        }
    }

    pub async fn create(
        &self,
        user: User,
    ) {
        let mut users =
            self.users.write().await;

        users.insert(
            user.id,
            user,
        );
    }

    pub async fn get(
        &self,
        id: UserId,
    ) -> Option<User> {
        let users =
            self.users.read().await;

        users.get(&id).cloned()
    }

    pub async fn find_by_telegram_id(
        &self,
        telegram_id: &str,
    ) -> Option<User> {
        self.users
            .read()
            .await
            .values()
            .find(|user| {
                user.telegram_id
                    == telegram_id
            })
            .cloned()
    }
}