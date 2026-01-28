use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// In-memory store for now - replace with database later
type ProgressStore = Arc<Mutex<HashMap<String, bool>>>;

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
    let store: ProgressStore = Arc::new(Mutex::new(HashMap::new()));

    Router::new()
        .route("/get", get({
            let store = store.clone();
            move || async move {
                let progress = store.lock().unwrap();
                Json(progress.clone())
            }
        }))
        .route("/update", post({
            let store = store.clone();
            move |Json(body): Json<UpdateRequest>| async move {
                let mut progress = store.lock().unwrap();
                progress.insert(body.lesson_id, body.completed);
                Json(UpdateResponse { success: true })
            }
        }))
}
