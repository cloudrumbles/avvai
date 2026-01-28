use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LookupParams {
    word: Option<String>,
}

#[derive(Serialize)]
struct DictionaryEntry {
    word: String,
    definition: String,
    examples: Vec<String>,
}

async fn lookup(Query(params): Query<LookupParams>) -> impl IntoResponse {
    let Some(word) = params.word else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing word parameter"})),
        )
            .into_response();
    };

    let entry = DictionaryEntry {
        word: word.clone(),
        definition: format!("Mock definition for «{word}»"),
        examples: vec![format!("This is an example sentence using «{word}».")],
    };

    Json(entry).into_response()
}

pub fn router() -> Router {
    Router::new().route("/lookup", get(lookup))
}
