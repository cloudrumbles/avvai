use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use serde::{Deserialize, Serialize};

use crate::core::lemmatise;

async fn lemmatise(Json(req): Json<LemmatiseRequest>) -> impl IntoResponse {
    match lemmatise::lemmatise(&req.word).await {
        Ok(lemma) => (StatusCode::OK, Json(LemmatiseResponse { lemma })).into_response(),
        Err(lemmatise::LemmatiseError::ApiKeyMissing) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "OPENROUTER_API_KEY not set"})),
        )
            .into_response(),
        Err(lemmatise::LemmatiseError::RequestFailed(message)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": message})),
        )
            .into_response(),
    }
}

pub fn router() -> Router {
    Router::new().route("/", post(lemmatise))
}

#[derive(Deserialize)]
struct LemmatiseRequest {
    word: String,
}

#[derive(Serialize)]
struct LemmatiseResponse {
    lemma: String,
}
