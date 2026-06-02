use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::user::UserId;

pub type CommitmentId = Uuid;
pub type CommitmentParticipantId = Uuid;

#[derive(Clone)]
pub struct Commitment {
    pub id: CommitmentId,

    /// Клятвенник
    pub promisor_id: UserId,

    pub verifier_id: UserId,

    pub witness_ids: Vec<UserId>,


    pub title: String,
    pub description: String,

    pub created_at: DateTime<Utc>,
    pub end_date: DateTime<Utc>,

    pub status: CommitmentStatus,
}

pub struct CommitmentParticipant {
    pub id: CommitmentParticipantId,

    pub commitment_id: CommitmentId,

    pub user_id: UserId,

    pub role: CommitmentParticipantRole,
}

pub enum CommitmentParticipantRole {
    Witness,
    Verifier,
}

#[derive(Clone, Debug)]
pub enum CommitmentStatus {
    Draft,

    Active,

    VerificationPending,

    Completed,

    Failed,

    Terminated,
}