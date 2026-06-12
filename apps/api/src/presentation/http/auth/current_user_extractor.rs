use std::sync::Arc;
use crate::domain::user::UserId;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};

use crate::{
    AppState,
};

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: UserId,
}

const TELEGRAM_INIT_DATA_HEADER: &str =
    "X-Telegram-Init-Data";


impl
FromRequestParts<Arc<AppState>>
for CurrentUser
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<
        Self,
        Self::Rejection,
    > {
        let init_data = parts
            .headers
            .get(
                TELEGRAM_INIT_DATA_HEADER,
            )
            .and_then(|v| v.to_str().ok())
            .ok_or(
                StatusCode::UNAUTHORIZED,
            )?;

        let validator =
            state
                .telegram_auth_validator
                .clone();

        let auth_data = validator
            .validate(init_data)
            .map_err(|_| {
                StatusCode::UNAUTHORIZED
            })?;

        let user = state
            .user_repository
            .find_by_telegram_id(
                &auth_data.telegram_id,
            )
            .await
            .ok_or(
                StatusCode::UNAUTHORIZED,
            )?;

        Ok(CurrentUser {
            id: user.id,
        })
    }
}