use std::sync::Arc;

use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};

use crate::{
    application::authenticate_telegram_user::AuthenticateTelegramUserRequest,
    app_state::AppState,
};

#[derive(Deserialize)]
pub struct AuthenticateTelegramRequest {
    pub init_data: String,
}

#[derive(Serialize)]
pub struct AuthenticateTelegramResponse {
    pub user_id: String,

    pub display_name: String,
}



// #[derive(Deserialize)]
// struct TelegramUser {
//     id: i64,
//
//     first_name: String,
// }


// pub fn parse_telegram_user(
//     init_data: &str,
// ) -> Result<TelegramUser, String> {
//     let params: HashMap<String, String> =
//         url::form_urlencoded::parse(
//             init_data.as_bytes(),
//         )
//             .into_owned()
//             .collect();
//
//     let user_json = params
//         .get("user")
//         .ok_or("user field is missing")?;
//
//     let user: TelegramUser =
//         serde_json::from_str(user_json)
//             .map_err(|error| error.to_string())?;
//
//     Ok(user)
// }


pub async fn authenticate_telegram(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AuthenticateTelegramRequest>,
) -> Json<AuthenticateTelegramResponse> {
    let auth_data = state
        .telegram_auth_validator
        .validate(
            &request.init_data,
        )
        .expect("invalid telegram auth");

    let result = state
        .authenticate_telegram_user
        .execute(
            AuthenticateTelegramUserRequest {
                telegram_id:
                auth_data.telegram_id,

                display_name:
                auth_data.display_name,
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
//
// use std::sync::Arc;
// use axum::extract::{State, Json};
// use serde::{Deserialize, Serialize};
// use crate::application::authenticate_telegram_user::AuthenticateTelegramUserRequest;
// use crate::app_state::AppState;
//
// #[derive(Deserialize)]
// pub struct AuthenticateTelegramRequest {
//     pub telegram_id: String,
//
//     pub display_name: String,
// }
//
// #[derive(Serialize)]
// pub struct AuthenticateTelegramResponse {
//     pub user_id: String,
//
//     pub display_name: String,
// }
//
//
// pub async fn authenticate_telegram(
//     State(state): State<Arc<AppState>>,
//     Json(request): Json<AuthenticateTelegramRequest>,
// ) -> Json<AuthenticateTelegramResponse> {
//     let result = state
//         .authenticate_telegram_user
//         .execute(
//             AuthenticateTelegramUserRequest {
//                 telegram_id:
//                 request.telegram_id,
//
//                 display_name:
//                 request.display_name,
//             },
//         )
//         .await;
//
//     Json(
//         AuthenticateTelegramResponse {
//             user_id:
//             result.user.id.to_string(),
//
//             display_name:
//             result.user.display_name,
//         },
//     )
// }