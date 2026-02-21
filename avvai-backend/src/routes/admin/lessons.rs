use axum::{
    extract::{Json as AxumJson, Path as AxumPath},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::core::lesson;
use crate::http::admin_auth::{AdminAuthState, AdminUser};

#[derive(Deserialize)]
struct LessonCreateRequest {
    lesson: lesson::Lesson,
    filename: Option<String>,
}

#[derive(Deserialize)]
struct LessonUpdateRequest {
    lesson: lesson::Lesson,
}

async fn list_lessons(_admin: AdminUser) -> impl IntoResponse {
    let items = lesson::list_admin_items().await;
    Json(items).into_response()
}

async fn get_lesson(_admin: AdminUser, AxumPath(id): AxumPath<String>) -> impl IntoResponse {
    lesson::get_lesson(&id).await.map_or_else(
        || {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Lesson not found"})),
            )
                .into_response()
        },
        |lesson| Json(lesson).into_response(),
    )
}

async fn create_lesson(
    _admin: AdminUser,
    AxumJson(payload): AxumJson<LessonCreateRequest>,
) -> impl IntoResponse {
    match lesson::create_lesson(&payload.lesson, payload.filename).await {
        Ok(lesson) => Json(lesson).into_response(),
        Err(lesson::LessonStoreError::AlreadyExists) => (
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error": "Lesson already exists"})),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.message()})),
        )
            .into_response(),
    }
}

async fn update_lesson(
    _admin: AdminUser,
    AxumPath(id): AxumPath<String>,
    AxumJson(payload): AxumJson<LessonUpdateRequest>,
) -> impl IntoResponse {
    match lesson::update_lesson(&id, &payload.lesson).await {
        Ok(lesson) => Json(lesson).into_response(),
        Err(lesson::LessonStoreError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Lesson not found"})),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.message()})),
        )
            .into_response(),
    }
}

async fn delete_lesson(_admin: AdminUser, AxumPath(id): AxumPath<String>) -> impl IntoResponse {
    match lesson::delete_lesson(&id).await {
        Ok(()) => Json(serde_json::json!({"status": "deleted"})).into_response(),
        Err(lesson::LessonStoreError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Lesson not found"})),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.message()})),
        )
            .into_response(),
    }
}

pub fn router() -> Router<Arc<AdminAuthState>> {
    Router::new()
        .route("/lessons", get(list_lessons))
        .route("/lessons", post(create_lesson))
        .route("/lessons/{id}", get(get_lesson))
        .route("/lessons/{id}", put(update_lesson))
        .route("/lessons/{id}", delete(delete_lesson))
}
