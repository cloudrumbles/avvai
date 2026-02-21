use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::core::flashcards::{self, FlashcardsState};

#[derive(Deserialize)]
struct DueParams {
    limit: Option<usize>,
}

#[derive(Deserialize)]
struct ReviewRequest {
    #[serde(rename = "cardId")]
    card_id: String,
    rating: u32,
}

#[derive(Deserialize)]
struct SettingsRequest {
    #[serde(rename = "desiredRetention")]
    desired_retention: f32,
}

pub fn init_state() -> FlashcardsState {
    flashcards::init_state()
}

pub fn router(state: FlashcardsState) -> Router {
    Router::new()
        .route("/due", get(get_due))
        .route("/review", post(review_card))
        .route("/settings", get(get_settings).post(update_settings))
        .with_state(state)
}

async fn get_due(
    State(state): State<FlashcardsState>,
    Query(params): Query<DueParams>,
) -> impl IntoResponse {
    match flashcards::get_due(&state, params.limit).await {
        Ok(cards) => Json(cards).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": err.message()})),
        )
            .into_response(),
    }
}

async fn review_card(
    State(state): State<FlashcardsState>,
    Json(body): Json<ReviewRequest>,
) -> impl IntoResponse {
    match flashcards::review_card(&state, &body.card_id, body.rating).await {
        Ok(result) => Json(result).into_response(),
        Err(flashcards::FlashcardsError::BadRequest(message)) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": message})),
        )
            .into_response(),
        Err(flashcards::FlashcardsError::Internal(message)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": message})),
        )
            .into_response(),
    }
}

async fn get_settings(State(state): State<FlashcardsState>) -> impl IntoResponse {
    match flashcards::get_settings(&state).await {
        Ok(desired_retention) => {
            Json(serde_json::json!({ "desired_retention": desired_retention })).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": err.message()})),
        )
            .into_response(),
    }
}

async fn update_settings(
    State(state): State<FlashcardsState>,
    Json(body): Json<SettingsRequest>,
) -> impl IntoResponse {
    match flashcards::update_settings(&state, body.desired_retention).await {
        Ok(()) => Json(serde_json::json!({ "success": true })).into_response(),
        Err(flashcards::FlashcardsError::BadRequest(message)) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": message})),
        )
            .into_response(),
        Err(flashcards::FlashcardsError::Internal(message)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": message})),
        )
            .into_response(),
    }
}
