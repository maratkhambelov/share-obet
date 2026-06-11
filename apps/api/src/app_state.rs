use std::sync::Arc;

use crate::{
    application::create_commitment::CreateCommitment,
};
use crate::application::authenticate_telegram_user::AuthenticateTelegramUser;
use crate::application::get_commitments::GetCommitments;
use crate::application::get_commitment::GetCommitment;
use crate::infrastructure::telegram::telegram_auth_validator::TelegramAuthValidator;

pub struct AppState {
    pub create_commitment: Arc<CreateCommitment>,
    pub get_commitments: Arc<GetCommitments>,
    pub get_commitment: Arc<GetCommitment>,
    pub authenticate_telegram_user:
        Arc<AuthenticateTelegramUser>,
    pub telegram_auth_validator:
        Arc<TelegramAuthValidator>,
}