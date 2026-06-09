use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type UserId = Uuid;
pub type UserIdentityId = Uuid;
pub type WalletId = Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,

    pub telegram_id: String,

    pub display_name: String,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Wallet {
    pub id: WalletId,

    pub user_id: UserId,

    pub network: WalletNetwork,

    pub address: String,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletNetwork {
    Ton,
}
// #[derive(Debug, Clone)]
// pub struct UserIdentity {
//     pub id: UserIdentityId,
//
//     pub user_id: UserId,
//
//     pub provider: AuthProvider,
//
//     /// Идентификатор пользователя
//     /// во внешней системе.
//     ///
//     /// Например:
//     /// Telegram -> "123456789"
//     pub external_id: String,
//
//     pub created_at: DateTime<Utc>,
// }


// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum AuthProvider {
//     Telegram,
// }


