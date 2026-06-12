use sqlx::PgPool;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use crate::domain::user::User;
use async_trait::async_trait;
use crate::domain::user_repository::UserRepository;

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            telegram_id: row.telegram_id,
            display_name: row.display_name,
            created_at: row.created_at,
        }
    }
}
#[derive(Debug, FromRow)]
struct UserRow {
    id: Uuid,
    telegram_id: String,
    display_name: String,
    created_at: DateTime<Utc>,
}


pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(
        pool: PgPool,
    ) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_telegram_id(
        &self,
        telegram_id: &str,
    ) -> Option<User> {
        let user =
            sqlx::query_as::<_, UserRow>(
                r#"
                SELECT
                    id,
                    telegram_id,
                    display_name,
                    created_at
                FROM users
                WHERE telegram_id = $1
                "#
            )
                .bind(telegram_id)
                .fetch_optional(&self.pool)
                .await
                .unwrap();

        user.map(Into::into)
    }

    async fn create(
        &self,
        user: User,
    ) {
        sqlx::query(
            r#"
        INSERT INTO users (
            id,
            telegram_id,
            display_name,
            created_at
        )
        VALUES (
            $1,
            $2,
            $3,
            $4
        )
        "#
        )
            .bind(user.id)
            .bind(user.telegram_id)
            .bind(user.display_name)
            .bind(user.created_at)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}