use axum::{
    extract::Json as AxumJson, http::StatusCode, response::IntoResponse, routing::post, Json,
    Router,
};
use serde::Deserialize;

use crate::core::dictionary;

#[derive(Deserialize)]
struct LookupRequest {
    word: Option<String>,
}


async fn lookup(AxumJson(params): AxumJson<LookupRequest>) -> impl IntoResponse {
    let Some(word) = params.word else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing word parameter"})),
        )
            .into_response();
    };

    match dictionary::lookup(&word).await {
        Ok(entry) => Json(entry).into_response(),
        Err(dictionary::DictionaryError::EmptyWord) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Empty word parameter"})),
        )
            .into_response(),
        Err(dictionary::DictionaryError::ApiKeyMissing) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "OPENROUTER_API_KEY not set"})),
        )
            .into_response(),
        Err(dictionary::DictionaryError::RequestFailed(message)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": message})),
        )
            .into_response(),
        Err(dictionary::DictionaryError::InvalidJson) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Model returned invalid JSON"})),
        )
            .into_response(),
    }
}

pub fn router() -> Router {
    Router::new().route("/lookup", post(lookup))
}
