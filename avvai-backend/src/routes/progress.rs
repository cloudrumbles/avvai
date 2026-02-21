use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::core::{db, progress};

#[derive(Clone)]
struct ProgressState {
    db: Arc<std::sync::Mutex<rusqlite::Connection>>,
}

#[derive(Deserialize)]
struct UpdateRequest {
    #[serde(rename = "lessonId")]
    lesson_id: String,
    completed: bool,
}

#[derive(Serialize)]
struct UpdateResponse {
    success: bool,
}

pub fn router() -> Router {
    let state = ProgressState { db: db::db() };
    let state_for_get = state.clone();
    let state_for_update = state;

    Router::new()
        .route("/get", get({
            let state = state_for_get;
            move || async move {
                match load_progress(state.db.clone()).await {
                    Ok(progress) => Json(progress).into_response(),
                    Err(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": error})),
                    )
                        .into_response(),
                }
            }
        }))
        .route("/update", post({
            let state = state_for_update;
            move |Json(body): Json<UpdateRequest>| async move {
                match upsert_progress(state.db.clone(), &body.lesson_id, body.completed).await {
                    Ok(()) => Json(UpdateResponse { success: true }).into_response(),
                    Err(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": error})),
                    )
                        .into_response(),
                }
            }
        }))
}

async fn load_progress(
    db: Arc<std::sync::Mutex<rusqlite::Connection>>,
) -> Result<std::collections::HashMap<String, bool>, String> {
    progress::load_progress(db).await
}

async fn upsert_progress(
    db: Arc<std::sync::Mutex<rusqlite::Connection>>,
    lesson_id: &str,
    completed: bool,
) -> Result<(), String> {
    progress::upsert_progress(db, lesson_id, completed).await
}
