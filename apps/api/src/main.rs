

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod app_state;
pub mod config;

use std::sync::{Arc};
use axum::{routing::get, Router};
use axum::routing::post;
use tower_http::cors::CorsLayer;
use dotenvy::dotenv;

async fn health() -> &'static str {
    "OK"
}

use crate::{
    app_state::AppState,
    application::create_commitment::CreateCommitment,
    application::get_commitments::GetCommitments,
    infrastructure::repository::in_memory::commitment_repository::InMemoryCommitmentRepository,
    presentation::http::commitment::create_commitment::create_commitment,
    presentation::http::commitment::get_commitments::get_commitments,
    presentation::http::commitment::get_commitment::get_commitment

};
use crate::application::authenticate_telegram_user::AuthenticateTelegramUser;
use crate::application::get_commitment::GetCommitment;
use crate::config::Config;
use crate::infrastructure::repository::in_memory::user_repository::InMemoryUserRepository;
use crate::infrastructure::telegram::telegram_auth_validator::TelegramAuthValidator;
use crate::presentation::http::auth::authenticate_telegram::authenticate_telegram;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::load();

    let repository = Arc::new(
        InMemoryCommitmentRepository::new(),
    );

    let user_repository = Arc::new(
        InMemoryUserRepository::new(),
    );

    let telegram_auth_validator =
        Arc::new(
            TelegramAuthValidator::new(
                config.telegram_bot_token,
            ),
        );

    let create_commitment_use_case  = Arc::new(
        CreateCommitment::new(repository.clone()),
    );

    let get_commitments_use_case  = Arc::new(
        GetCommitments::new(repository.clone()),
    );
    let get_commitment_use_case  = Arc::new(
        GetCommitment::new(repository.clone()),
    );
    let authenticate_telegram_user =
        Arc::new(
            AuthenticateTelegramUser::new(
                user_repository.clone(),
            ),
        );
    let state = Arc::new(AppState {
        create_commitment: create_commitment_use_case,
        get_commitments: get_commitments_use_case,
        get_commitment: get_commitment_use_case,
        authenticate_telegram_user: authenticate_telegram_user,
        telegram_auth_validator: telegram_auth_validator
    });

    let app = Router::new()
        .route("/health", get(health))
        .route(
            "/commitments",
            post(create_commitment),
        )
        .route(
            "/commitments",
            get(get_commitments),
        )
        .route(
            "/commitments/{id}",
            get(get_commitment),
        )
        .route(
            "/auth/telegram",
            post(authenticate_telegram),
        )
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server started on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

//TODO later

//use axum::http::{
//     HeaderValue,
//     Method,
// };
// use tower_http::cors::CorsLayer;
//
// let cors = CorsLayer::new()
//     .allow_origin(
//         "http://localhost:5173"
//             .parse::<HeaderValue>()
//             .unwrap(),
//     )
//     .allow_methods([
//         Method::GET,
//         Method::POST,
//         Method::PUT,
//         Method::DELETE,
//     ])
//     .allow_headers(Any);
//
// let app = Router::new()
//     .route("/commitments", get(get_commitments))
//     .layer(cors);


// 1. Создать структуру проекта
// 2,3 создать структуру для postgressql, делать на моках
// 4. Сделать AuthenticateTelegramUser
//
// 5. Сделать CreateCommitment
//
// 6. Сделать GetCommitment
//
// 7. Сделать ListCommitments
//
// 8. Только потом подключать фронт


// исключены
// 2. Поднять PostgreSQL
// 3. Сделать миграции
// 2,3 #[async_trait]
// pub trait CommitmentRepository {
//     async fn create(
//         &self,
//         commitment: Commitment,
//     ) -> Result<Commitment>;
//
//     async fn get(
//         &self,
//         id: CommitmentId,
//     ) -> Result<Option<Commitment>>;
//
//     async fn list(
//         &self,
//     ) -> Result<Vec<Commitment>>;
// }

//pub struct InMemoryCommitmentRepository {
//     commitments: RwLock<HashMap<CommitmentId, Commitment>>,
// }
