use axum::{
    extract::{multipart::Multipart, Path as AxumPath},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use crate::core::media;
use crate::http::admin_auth::{AdminAuthState, AdminUser};

async fn list_media(_admin: AdminUser) -> impl IntoResponse {
    let files = media::list_media().await;
    Json(files).into_response()
}

async fn upload_media(_admin: AdminUser, mut multipart: Multipart) -> impl IntoResponse {
    while let Ok(Some(field)) = multipart.next_field().await {
        let Some(filename) = field.file_name().map(ToString::to_string) else {
            continue;
        };

        let Ok((path, mut file)) = media::create_media_file(&filename).await else {
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
    match media::delete_media(&filename).await {
        Ok(()) => Json(serde_json::json!({"status": "deleted"})).into_response(),
        Err(media::MediaError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "File not found"})),
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
        .route("/media", get(list_media))
        .route("/media", post(upload_media))
        .route("/media/{filename}", delete(delete_media))
}
