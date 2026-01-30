use axum::{
    extract::{multipart::Multipart, Path as AxumPath},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use std::{fs, path::Path, sync::Arc};
use tokio::io::AsyncWriteExt;

use crate::services::admin_auth::{AdminAuthState, AdminUser};

const MEDIA_DIR: &str = "static/media";

fn media_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(MEDIA_DIR)
}

async fn list_media(_admin: AdminUser) -> impl IntoResponse {
    let dir = media_root();
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    files.push(name.to_string_lossy().to_string());
                }
            }
        }
    }

    files.sort();
    Json(files).into_response()
}

async fn upload_media(_admin: AdminUser, mut multipart: Multipart) -> impl IntoResponse {
    let root = media_root();
    if let Err(error) = fs::create_dir_all(&root) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response();
    }

    while let Ok(Some(field)) = multipart.next_field().await {
        let Some(filename) = field.file_name().map(|f| f.to_string()) else {
            continue;
        };

        let sanitized = filename.replace(['/', '\\'], "_");
        let path = root.join(sanitized);
        let Ok(mut file) = tokio::fs::File::create(&path).await else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to create file"})),
            )
                .into_response();
        };

        let mut stream = field;
        while let Ok(Some(chunk)) = stream.chunk().await {
            if let Err(error) = file.write_all(&chunk).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": error.to_string()})),
                )
                    .into_response();
            }
        }

        return Json(serde_json::json!({"status": "uploaded", "filename": path.file_name().unwrap().to_string_lossy()})).into_response();
    }

    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"error": "No file provided"})),
    )
        .into_response()
}

async fn delete_media(_admin: AdminUser, AxumPath(filename): AxumPath<String>) -> impl IntoResponse {
    let sanitized = filename.replace(['/', '\\'], "_");
    let path = media_root().join(sanitized);
    if !path.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "File not found"})),
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
        .route("/media", get(list_media))
        .route("/media", post(upload_media))
        .route("/media/{filename}", delete(delete_media))
}
