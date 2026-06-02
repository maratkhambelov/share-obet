use std::sync::Arc;
use crate::domain::commitment::{Commitment, CommitmentId, CommitmentStatus};
use crate::{
    infrastructure::repository::
    in_memory::commitment_repository::
    InMemoryCommitmentRepository,
};


pub struct GetCommitments {
    repository: Arc<InMemoryCommitmentRepository>,
}


impl GetCommitments {
    pub fn new(
        repository:
        Arc<InMemoryCommitmentRepository>
    ) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self
    ) -> Vec<Commitment> {

        self.repository.find_all()
            .await

    }
}