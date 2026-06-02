use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::domain::commitment::{
    Commitment,
    CommitmentId,
};

pub struct InMemoryCommitmentRepository {
    commitments: RwLock<HashMap<
        CommitmentId,
        Commitment,
    >>,
}

impl InMemoryCommitmentRepository {
    pub fn new() -> Self {
        Self {
            commitments: RwLock::new(
                HashMap::new(),
            ),
        }
    }

    pub async fn create(
        &self,
        commitment: Commitment,
    ) {
        let mut commitments =
            self.commitments.write().await;

        commitments.insert(
            commitment.id,
            commitment,
        );
    }

    pub async fn find_all(&self) -> Vec<Commitment> {
        self.commitments
            .read()
            .await
            .values()
            .cloned()
            .collect()
    }
    // pub async fn get(
    //     &self,
    //     id: CommitmentId,
    // ) -> Option<Commitment> {
    //     let commitments =
    //         self.commitments.read().await;
    //
    //     commitments.get(&id).cloned()
    // }
    //
    // pub async fn list(
    //     &self,
    // ) -> Vec<Commitment> {
    //     let commitments =
    //         self.commitments.read().await;
    //
    //     commitments
    //         .values()
    //         .cloned()
    //         .collect()
    // }
}