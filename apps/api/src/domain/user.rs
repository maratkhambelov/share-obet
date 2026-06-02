use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type UserId = Uuid;


pub struct User {
    pub id: UserId,

    pub username: String,

    pub created_at: DateTime<Utc>,
}

pub struct UserIdentity {
    pub user_id: UserId,

    pub provider: AuthProvider,

    pub external_id: String,
}

pub enum AuthProvider {
    Telegram,
}

// type UserId = Uuid;
// type ContractId = Uuid;
// type ContractParticipantId = Uuid;
//
// struct Contract {
//     id: ContractId,
//
//     promisor_id: UserId,
//
//     title: String,
//     description: String,
//
//     created_at: DateTime<Utc>,
//     end_date: DateTime<Utc>,
//
//     status: ContractStatus,
// }
//
//
// struct ContractParticipant {
//     id: ContractParticipantId,
//     contract_id: ContractId,
//     user_id: UserId,
//     role: ContractParticipantRole ,
// }
//
// enum ContractParticipantRole {
//     Witness,
//     Verifier,
// }
// enum ContractStatus {
//     Draft,
//     // PendingFunding,
//     // PendingLaunch,
//     Active,
//     VerificationPending,
//     Completed,
//     Failed,
//     Terminated,
// }
//
// struct User {
//     id: UserId,
//
//     username: String,
//
//     created_at: DateTime<Utc>,
// }
//
// struct UserIdentity {
//     user_id: UserId,
//
//     provider: AuthProvider,
//
//     external_id: String,
// }
//
// enum AuthProvider {
//     Telegram,
// }