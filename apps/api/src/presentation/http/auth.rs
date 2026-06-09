use std::sync::Arc;
use axum::extract::{State, Json};
use serde::{Deserialize, Serialize};
use crate::application::authenticate_telegram_user::AuthenticateTelegramUserRequest;
use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct AuthenticateTelegramRequest {
    pub telegram_id: String,

    pub display_name: String,
}

#[derive(Serialize)]
pub struct AuthenticateTelegramResponse {
    pub user_id: String,

    pub display_name: String,
}


pub async fn authenticate_telegram(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AuthenticateTelegramRequest>,
) -> Json<AuthenticateTelegramResponse> {
    let result = state
        .authenticate_telegram_user
        .execute(
            AuthenticateTelegramUserRequest {
                telegram_id:
                request.telegram_id,

                display_name:
                request.display_name,
            },
        )
        .await;

    Json(
        AuthenticateTelegramResponse {
            user_id:
            result.user.id.to_string(),

            display_name:
            result.user.display_name,
        },
    )
}