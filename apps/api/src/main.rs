pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod app_state;

use std::sync::{Arc};
use axum::{routing::get, Router};
use axum::routing::post;

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

};

#[tokio::main]
async fn main() {

    let repository = Arc::new(
        InMemoryCommitmentRepository::new(),
    );

    let create_commitment_use_case  = Arc::new(
        CreateCommitment::new(repository.clone()),
    );

    let get_commitments_use_case  = Arc::new(
        GetCommitments::new(repository.clone()),
    );

    let state = Arc::new(AppState {
        create_commitment: create_commitment_use_case,
        get_commitments: get_commitments_use_case
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
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server started on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}


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
