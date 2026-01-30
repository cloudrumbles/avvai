use axum::{
    extract::{Json as AxumJson, Path as AxumPath},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, sync::Arc};

use crate::services::admin_auth::{AdminAuthState, AdminUser};
use crate::routes::lesson::Lesson;

#[derive(Serialize)]
struct LessonListItem {
    id: String,
    title: String,
    description: String,
    filename: String,
}

#[derive(Deserialize)]
struct LessonCreateRequest {
    lesson: Lesson,
    filename: Option<String>,
}

#[derive(Deserialize)]
struct LessonUpdateRequest {
    lesson: Lesson,
}

const LESSONS_DIR: &str = "data/lessons";

fn lessons_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(LESSONS_DIR)
}

fn lesson_filename_for_id(id: &str) -> String {
    format!("{}.json", id)
}

fn load_lesson_from_file(path: &std::path::Path) -> Option<Lesson> {
    let Ok(content) = fs::read_to_string(path) else {
        return None;
    };
    serde_json::from_str::<Lesson>(&content).ok()
}

fn save_lesson_to_file(path: &std::path::Path, lesson: &Lesson) -> Result<(), String> {
    let Ok(contents) = serde_json::to_string_pretty(lesson) else {
        return Err("Failed to serialize lesson".to_string());
    };
    fs::write(path, contents).map_err(|e| e.to_string())
}

async fn list_lessons(_admin: AdminUser) -> impl IntoResponse {
    let mut items = Vec::new();
    let dir = lessons_dir();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Some(lesson) = load_lesson_from_file(&path) {
                    let filename = path
                        .file_name()
                        .map(|name| name.to_string_lossy().to_string())
                        .unwrap_or_default();
                    items.push(LessonListItem {
                        id: lesson.id,
                        title: lesson.title,
                        description: lesson.description,
                        filename,
                    });
                }
            }
        }
    }

    items.sort_by(|a, b| a.id.cmp(&b.id));
    Json(items).into_response()
}

async fn get_lesson(_admin: AdminUser, AxumPath(id): AxumPath<String>) -> impl IntoResponse {
    let path = lessons_dir().join(lesson_filename_for_id(&id));
    let Some(lesson) = load_lesson_from_file(&path) else {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Lesson not found"})),
        )
            .into_response();
    };
    Json(lesson).into_response()
}

async fn create_lesson(_admin: AdminUser, AxumJson(payload): AxumJson<LessonCreateRequest>) -> impl IntoResponse {
    let filename = payload
        .filename
        .unwrap_or_else(|| lesson_filename_for_id(&payload.lesson.id));
    let path = lessons_dir().join(filename);

    if path.exists() {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error": "Lesson already exists"})),
        )
            .into_response();
    }

    if let Err(error) = save_lesson_to_file(&path, &payload.lesson) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error})),
        )
            .into_response();
    }

    Json(payload.lesson).into_response()
}

async fn update_lesson(
    _admin: AdminUser,
    AxumPath(id): AxumPath<String>,
    AxumJson(payload): AxumJson<LessonUpdateRequest>,
) -> impl IntoResponse {
    let path = lessons_dir().join(lesson_filename_for_id(&id));
    if !path.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Lesson not found"})),
        )
            .into_response();
    }

    if let Err(error) = save_lesson_to_file(&path, &payload.lesson) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error})),
        )
            .into_response();
    }

    Json(payload.lesson).into_response()
}

async fn delete_lesson(_admin: AdminUser, AxumPath(id): AxumPath<String>) -> impl IntoResponse {
    let path = lessons_dir().join(lesson_filename_for_id(&id));
    if !path.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Lesson not found"})),
        )
            .into_response();
    }

    if let Err(error) = fs::remove_file(&path) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response();
    }

    Json(serde_json::json!({"status": "deleted"})).into_response()
}

pub fn router() -> Router<Arc<AdminAuthState>> {
    Router::new()
        .route("/lessons", get(list_lessons))
        .route("/lessons", post(create_lesson))
        .route("/lessons/{id}", get(get_lesson))
        .route("/lessons/{id}", put(update_lesson))
        .route("/lessons/{id}", delete(delete_lesson))
}
