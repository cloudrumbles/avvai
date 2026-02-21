use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Deserialize;

use crate::core::lesson;

#[derive(Deserialize)]
struct GetParams {
    id: Option<String>,
}

async fn list() -> impl IntoResponse {
    let lessons = lesson::list_summaries().await;
    Json(lessons)
}

async fn get_lesson(Query(params): Query<GetParams>) -> impl IntoResponse {
    let Some(id) = params.id else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing id parameter"})),
        )
            .into_response();
    };

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

pub fn router() -> Router {
    Router::new()
        .route("/list", get(list))
        .route("/get", get(get_lesson))
}
