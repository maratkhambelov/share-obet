use std::sync::Arc;

use chrono::{DateTime, Utc};
use uuid::Uuid;


use crate::{
    infrastructure::repository::
    in_memory::commitment_repository::
    InMemoryCommitmentRepository,
};
use crate::domain::commitment::{Commitment, CommitmentStatus};
use crate::domain::user::UserId;

pub struct CreateCommitment {
    repository:
        Arc<InMemoryCommitmentRepository>,
}

// #[derive(Deserialize)]
pub struct CreateCommitmentInput {
    pub promisor_id: UserId,

    pub verifier_id: UserId,

    pub witness_ids: Vec<UserId>,

    pub title: String,

    pub description: String,

    pub end_date: DateTime<Utc>,
}

impl CreateCommitment {
    pub fn new(
        repository:
        Arc<InMemoryCommitmentRepository>,
    ) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: CreateCommitmentInput,
    ) -> Commitment {
        let commitment = Commitment {
            id: Uuid::new_v4(),

            promisor_id: input.promisor_id,

            verifier_id: input.verifier_id,

            witness_ids: input.witness_ids,

            title: input.title,

            description: input.description,

            created_at: Utc::now(),

            end_date: input.end_date,

            status: CommitmentStatus::Draft,
        };

        self.repository
            .create(commitment.clone())
            .await;

        commitment
    }
}