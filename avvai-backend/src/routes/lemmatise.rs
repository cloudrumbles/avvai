use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct LemmatiseRequest {
    word: String,
}

#[derive(Serialize)]
struct LemmatiseResponse {
    lemma: String,
}

#[derive(Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<OpenRouterMessage>,
}

#[derive(Serialize)]
struct OpenRouterMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenRouterResponse {
    choices: Vec<OpenRouterChoice>,
}

#[derive(Deserialize)]
struct OpenRouterChoice {
    message: OpenRouterMessageContent,
}

#[derive(Deserialize)]
struct OpenRouterMessageContent {
    content: String,
}

async fn lemmatise(Json(req): Json<LemmatiseRequest>) -> impl IntoResponse {
    let api_key = env::var("OPENROUTER_API_KEY").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "OPENROUTER_API_KEY not set"})),
        )
    });

    let Ok(api_key) = api_key else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "OPENROUTER_API_KEY not set"})),
        )
            .into_response();
    };

    let client = reqwest::Client::new();

    let openrouter_req = OpenRouterRequest {
        model: "google/gemini-2.5-flash-lite".to_string(),
        messages: vec![OpenRouterMessage {
            role: "user".to_string(),
            content: format!(
                "You are a Tamil language expert. Lemmatize the following Tamil word. A lemma is the base/dictionary form of a word. Return ONLY the lemma as a single word, nothing else.\n\nWord: {}",
                req.word
            ),
        }],
    };

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "http://localhost:3001")
        .json(&openrouter_req)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        });

    let Ok(response) = response else {
        return response.unwrap_err().into_response();
    };

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("OpenRouter error: {}", error_text)})),
        )
            .into_response();
    }

    let Ok(parsed) = response.json::<OpenRouterResponse>().await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to parse OpenRouter response"})),
        )
            .into_response();
    };

    let lemma = parsed
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    (StatusCode::OK, Json(LemmatiseResponse { lemma })).into_response()
}

pub fn router() -> Router {
    Router::new().route("/", post(lemmatise))
}
