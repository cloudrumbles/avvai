use axum::{
    extract::Json as AxumJson,
    http::StatusCode,
    response::IntoResponse,
    routing::delete,
    routing::get,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::services::admin_auth::{AdminAuthState, AdminUser};
use crate::services::dictionary_cache::{self, DictionaryEntry};

#[derive(Deserialize)]
struct CacheKeyRequest {
    key: Option<String>,
}

#[derive(Deserialize)]
struct CacheUpsertRequest {
    key: Option<String>,
    entry: Option<DictionaryEntry>,
}

async fn list_entries(_admin: AdminUser) -> impl IntoResponse {
    let entries = dictionary_cache::list().await;
    Json(entries).into_response()
}

async fn get_entry(_admin: AdminUser, AxumJson(params): AxumJson<CacheKeyRequest>) -> impl IntoResponse {
    let Some(key) = params.key else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing key"})),
        )
            .into_response();
    };

    match dictionary_cache::get(&key).await {
        Some(entry) => Json(entry).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Entry not found"})),
        )
            .into_response(),
    }
}

async fn upsert_entry(_admin: AdminUser, AxumJson(params): AxumJson<CacheUpsertRequest>) -> impl IntoResponse {
    let Some(key) = params.key else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing key"})),
        )
            .into_response();
    };

    let Some(entry) = params.entry else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing entry"})),
        )
            .into_response();
    };

    dictionary_cache::set(&key, entry.clone()).await;
    Json(entry).into_response()
}

async fn delete_entry(_admin: AdminUser, AxumJson(params): AxumJson<CacheKeyRequest>) -> impl IntoResponse {
    let Some(key) = params.key else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing key"})),
        )
            .into_response();
    };

    let removed = dictionary_cache::remove(&key).await;
    if removed {
        Json(serde_json::json!({"status": "deleted"})).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Entry not found"})),
        )
            .into_response()
    }
}

pub fn router() -> Router<Arc<AdminAuthState>> {
    Router::new()
        .route("/list", get(list_entries))
        .route("/get", post(get_entry))
        .route("/upsert", post(upsert_entry))
        .route("/delete", delete(delete_entry))
}
