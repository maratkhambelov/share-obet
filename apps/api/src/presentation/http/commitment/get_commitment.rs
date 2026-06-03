use std::sync::Arc;
use axum::extract::{Path, State, Json};
use chrono::{DateTime, Utc};
use serde::{ Serialize};
use axum::{
    http::StatusCode,
};
use uuid::Uuid;
use crate::app_state::AppState;
use crate::domain::commitment::{Commitment};

#[derive(Serialize)]
pub struct GetCommitmentResponse {
    pub title: String,
    pub status: String,
    pub end_date: DateTime<Utc>,
    pub id: Uuid,
    pub promisor_id: Uuid,
    pub verifier_id: Uuid,
    pub witness_ids: Vec<Uuid>,
    pub description: String,
    pub created_at: DateTime<Utc>,
}


impl From<Commitment> for GetCommitmentResponse {
    fn from(commitment: Commitment) -> Self {
        Self {
            id: commitment.id,

            promisor_id: commitment.promisor_id,
            verifier_id: commitment.verifier_id,
            witness_ids: commitment.witness_ids,

            title: commitment.title,
            description: commitment.description,

            created_at: commitment.created_at,
            end_date: commitment.end_date,

            status: format!("{:?}", commitment.status),
        }
    }
}

pub async fn get_commitment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) ->  Result<Json<GetCommitmentResponse>, StatusCode> {

    let commitment = match state.get_commitment.execute(id).await {
        Some(commitment) => commitment,
        None => return Err(StatusCode::NOT_FOUND),
    };


    Ok(Json(GetCommitmentResponse::from(commitment) ))
}