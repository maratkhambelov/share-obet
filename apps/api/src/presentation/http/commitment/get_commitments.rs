use std::sync::Arc;
use axum::extract::{State, Json};
use chrono::{DateTime, Utc};
use serde::{ Serialize};
use uuid::Uuid;
use crate::app_state::AppState;
use crate::domain::commitment::{
    Commitment,
};


#[derive(Serialize)]
pub struct CommitmentListItemResponse {
    pub id: Uuid,

    pub title: String,

    pub status: String,

    pub end_date: DateTime<Utc>,
}


impl From<Commitment> for CommitmentListItemResponse {
    fn from(commitment: Commitment) -> Self {
        Self {
            id: commitment.id,
            title: commitment.title,
            status: format!("{:?}", commitment.status),
            end_date: commitment.end_date,
        }
    }
}

#[derive(Serialize)]
pub struct GetCommitmentsResponse {
    pub items: Vec<CommitmentListItemResponse>,
}

pub async fn get_commitments(
    State(state): State<Arc<AppState>>,
) ->  Json<GetCommitmentsResponse> {

    let commitment_vecs = state
        .get_commitments
        .execute()
        .await;

    let items = commitment_vecs
        .into_iter()
        .map(CommitmentListItemResponse::from)
        .collect();


    Json(GetCommitmentsResponse {
        items
    })
}