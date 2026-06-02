use std::sync::Arc;

use crate::{
    application::create_commitment::CreateCommitment,
};
use crate::application::get_commitments::GetCommitments;

pub struct AppState {
    pub create_commitment: Arc<CreateCommitment>,
    pub get_commitments: Arc<GetCommitments>,
}