use axum::{extract::Json as AxumJson, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::env;

use crate::services::dictionary_cache::{self, DictionaryEntry};

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

    let normalised = dictionary_cache::normalise(&word);
    if normalised.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Empty word parameter"})),
        )
            .into_response();
    }

    if let Some(entry) = dictionary_cache::get(&normalised).await {
        return Json(entry).into_response();
    }

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
                "You are a Tamil-English dictionary. Given a Tamil word, return a JSON object with fields: word (the original word), definition (short English definition), and examples (array of 1-2 short Tamil example sentences). Return ONLY valid JSON, nothing else.\n\nWord: {}",
                normalised
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

    let content = parsed
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    let cleaned = clean_json_response(&content);

    let Ok(entry) = serde_json::from_str::<DictionaryEntry>(&cleaned) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Model returned invalid JSON"})),
        )
            .into_response();
    };

    dictionary_cache::set(&normalised, entry.clone()).await;

    Json(entry).into_response()
}

pub fn router() -> Router {
    Router::new().route("/lookup", post(lookup))
}

fn clean_json_response(raw: &str) -> String {
    let trimmed = raw.trim();

    let without_fence = if trimmed.starts_with("```") {
        let stripped = trimmed
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```");
        stripped.trim()
    } else {
        trimmed
    };

    if let (Some(start), Some(end)) = (without_fence.find('{'), without_fence.rfind('}')) {
        if start <= end {
            return without_fence[start..=end].trim().to_string();
        }
    }

    without_fence.to_string()
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
