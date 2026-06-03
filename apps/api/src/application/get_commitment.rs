use std::sync::Arc;
use crate::domain::commitment::{Commitment, CommitmentId};
use crate::{
    infrastructure::repository::
    in_memory::commitment_repository::
    InMemoryCommitmentRepository,
};


pub struct GetCommitment {
    repository: Arc<InMemoryCommitmentRepository>,
}


impl GetCommitment {
    pub fn new(
        repository:
        Arc<InMemoryCommitmentRepository>,
    ) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: CommitmentId,
    ) -> Option<Commitment> {

        self.repository.get(id)
            .await
    }
}