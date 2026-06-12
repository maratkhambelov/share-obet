use std::sync::Arc;
use axum::extract::{State, Json};
use axum::{http::StatusCode};
use crate::domain::user::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::application::create_commitment::CreateCommitmentInput;
use crate::app_state::AppState;
use crate::domain::commitment::{ CommitmentId};
use crate::presentation::http::auth::current_user_extractor::CurrentUser;

#[derive(Deserialize)]
pub struct CreateCommitmentRequest {
    pub verifier_id: UserId,

    pub witness_ids: Vec<UserId>,

    pub title: String,

    pub description: String,

    pub end_date: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct CreateCommitmentResponse {
    pub id: CommitmentId,
}

pub async fn create_commitment(
    State(state): State<Arc<AppState>>,
    current_user: CurrentUser,
    Json(request): Json<CreateCommitmentRequest>,
) -> (StatusCode, Json<CreateCommitmentResponse>) {
    println!(
        "current user id = {}",
        current_user.id,
    );

    let input = CreateCommitmentInput {
        promisor_id: current_user.id,

        verifier_id: request.verifier_id,

        witness_ids: request.witness_ids,

        title: request.title,

        description: request.description,

        end_date: request.end_date,
    };

    let commitment = state
        .create_commitment
        .execute(input)
        .await;

    (
        StatusCode::CREATED,
        Json(CreateCommitmentResponse {
            id: commitment.id
        })
    )

}