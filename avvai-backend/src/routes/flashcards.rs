use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use fsrs::{FSRS, MemoryState, DEFAULT_PARAMETERS};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use super::lesson::{ContentSection, Lesson};

const DEFAULT_DESIRED_RETENTION: f32 = 0.9;
const SETTINGS_KEY_RETENTION: &str = "desired_retention";
const DEFAULT_LIMIT: usize = 30;

#[derive(Clone)]
pub struct FlashcardsState {
    db: Arc<Mutex<Connection>>,
}

#[derive(Deserialize)]
struct DueParams {
    limit: Option<usize>,
}

#[derive(Serialize, Clone)]
struct Flashcard {
    id: String,
    front: String,
    back: String,
}

#[derive(Deserialize)]
struct ReviewRequest {
    #[serde(rename = "cardId")]
    card_id: String,
    rating: u32,
}

#[derive(Serialize)]
struct ReviewResponse {
    success: bool,
    due_date: String,
    interval_days: i64,
}

#[derive(Clone)]
struct StoredState {
    stability: f32,
    difficulty: f32,
    last_review: Option<DateTime<Utc>>,
    due_date: Option<DateTime<Utc>>,
}

pub fn init_state() -> FlashcardsState {
    let db_path = std::env::var("FSRS_DB_PATH").unwrap_or_else(|_| {
        let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
        base.join("fsrs.sqlite3").to_string_lossy().to_string()
    });

    let db_path = PathBuf::from(db_path);
    if let Some(parent) = db_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let conn = init_db(&db_path).expect("Failed to initialize flashcards database");
    FlashcardsState {
        db: Arc::new(Mutex::new(conn)),
    }
}

pub fn router(state: FlashcardsState) -> Router {
    Router::new()
        .route("/due", get(get_due))
        .route("/review", post(review_card))
        .route("/settings", get(get_settings).post(update_settings))
        .with_state(state)
}

fn init_db(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS fsrs_cards (
            card_id TEXT PRIMARY KEY,
            stability REAL NOT NULL,
            difficulty REAL NOT NULL,
            last_review TEXT,
            due_date TEXT,
            interval_days INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS fsrs_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        ",
    )?;
    Ok(conn)
}

async fn get_due(
    State(state): State<FlashcardsState>,
    Query(params): Query<DueParams>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(DEFAULT_LIMIT);
    let now = Utc::now();

    let cards = match load_vocabulary_cards() {
        Ok(cards) => cards,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": err})),
            )
                .into_response();
        }
    };

    let stored_states = match load_states(&state.db) {
        Ok(states) => states,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": err})),
            )
                .into_response();
        }
    };

    let mut due_cards = Vec::new();
    for card in cards {
        let is_due = match stored_states.get(&card.id) {
            Some(state) => state
                .due_date
                .map(|due| due <= now)
                .unwrap_or(true),
            None => true,
        };

        if is_due {
            due_cards.push(card);
        }

        if due_cards.len() >= limit {
            break;
        }
    }

    Json(due_cards).into_response()
}

async fn review_card(
    State(state): State<FlashcardsState>,
    Json(body): Json<ReviewRequest>,
) -> impl IntoResponse {
    if !(1..=4).contains(&body.rating) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "rating must be 1..=4"})),
        )
            .into_response();
    }

    let now = Utc::now();

    let stored_state = match load_state(&state.db, &body.card_id) {
        Ok(state) => state,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": err})),
            )
                .into_response();
        }
    };

    let previous_memory = stored_state.as_ref().map(|state| MemoryState {
        stability: state.stability,
        difficulty: state.difficulty,
    });

    let elapsed_days = stored_state
        .as_ref()
        .and_then(|state| state.last_review)
        .map(|last| (now - last).num_days().max(0) as u32)
        .unwrap_or(0);

    let desired_retention = match load_desired_retention(&state.db) {
        Ok(value) => value,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": err})),
            )
                .into_response();
        }
    };

    let fsrs = match FSRS::new(Some(&DEFAULT_PARAMETERS)) {
        Ok(fsrs) => fsrs,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "failed to initialize FSRS"})),
            )
                .into_response();
        }
    };
    let next_states = match fsrs.next_states(previous_memory, desired_retention, elapsed_days) {
        Ok(states) => states,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "invalid FSRS state"})),
            )
                .into_response();
        }
    };

    let next_state = match body.rating {
        1 => next_states.again,
        2 => next_states.hard,
        3 => next_states.good,
        4 => next_states.easy,
        _ => next_states.good,
    };

    let interval_days = next_state.interval.round().max(1.0) as i64;
    let due_date = now + Duration::days(interval_days);

    if let Err(err) = save_state(
        &state.db,
        &body.card_id,
        next_state.memory,
        now,
        due_date,
        interval_days,
    ) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": err})),
        )
            .into_response();
    }

    Json(ReviewResponse {
        success: true,
        due_date: due_date.to_rfc3339(),
        interval_days,
    })
    .into_response()
}

fn lessons_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/lessons")
}

fn load_vocabulary_cards() -> Result<Vec<Flashcard>, String> {
    let mut cards = Vec::new();
    let lessons_path = lessons_dir();

    let entries = fs::read_dir(&lessons_path)
        .map_err(|err| format!("Failed to read lessons directory: {err}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.extension().map_or(false, |ext| ext == "json") {
            continue;
        }

        let content = fs::read_to_string(&path)
            .map_err(|err| format!("Failed to read lesson file: {err}"))?;
        let lesson: Lesson = serde_json::from_str(&content)
            .map_err(|err| format!("Failed to parse lesson JSON: {err}"))?;

        for section in lesson.sections.iter() {
            if let ContentSection::Vocabulary(vocab) = section {
                for (index, entry) in vocab.entries.iter().enumerate() {
                    let id = format!("{}:vocab:{}", lesson.id, index);
                    cards.push(Flashcard {
                        id,
                        front: entry.word.clone(),
                        back: entry.meaning.clone(),
                    });
                }
            }
        }
    }

    Ok(cards)
}

fn load_states(
    db: &Arc<Mutex<Connection>>,
) -> Result<HashMap<String, StoredState>, String> {
    let conn = db.lock().map_err(|_| "DB lock poisoned".to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT card_id, stability, difficulty, last_review, due_date FROM fsrs_cards",
        )
        .map_err(|err| format!("Failed to prepare statement: {err}"))?;

    let rows = stmt
        .query_map([], |row| {
            let card_id: String = row.get(0)?;
            let stability: f32 = row.get(1)?;
            let difficulty: f32 = row.get(2)?;
            let last_review: Option<String> = row.get(3)?;
            let due_date: Option<String> = row.get(4)?;

            let last_review = last_review
                .and_then(|value| DateTime::parse_from_rfc3339(&value).ok())
                .map(|dt| dt.with_timezone(&Utc));
            let due_date = due_date
                .and_then(|value| DateTime::parse_from_rfc3339(&value).ok())
                .map(|dt| dt.with_timezone(&Utc));

            Ok((
                card_id,
                StoredState {
                    stability,
                    difficulty,
                    last_review,
                    due_date,
                },
            ))
        })
        .map_err(|err| format!("Failed to query states: {err}"))?;

    let mut map = HashMap::new();
    for row in rows {
        let (card_id, state) = row.map_err(|err| format!("Failed to read row: {err}"))?;
        map.insert(card_id, state);
    }

    Ok(map)
}

fn load_state(
    db: &Arc<Mutex<Connection>>,
    card_id: &str,
) -> Result<Option<StoredState>, String> {
    let conn = db.lock().map_err(|_| "DB lock poisoned".to_string())?;
    let row = conn
        .query_row(
            "SELECT stability, difficulty, last_review, due_date FROM fsrs_cards WHERE card_id = ?1",
            [card_id],
            |row| {
                let stability: f32 = row.get(0)?;
                let difficulty: f32 = row.get(1)?;
                let last_review: Option<String> = row.get(2)?;
                let due_date: Option<String> = row.get(3)?;
                let last_review = last_review
                    .and_then(|value| DateTime::parse_from_rfc3339(&value).ok())
                    .map(|dt| dt.with_timezone(&Utc));
                let due_date = due_date
                    .and_then(|value| DateTime::parse_from_rfc3339(&value).ok())
                    .map(|dt| dt.with_timezone(&Utc));

                Ok(StoredState {
                    stability,
                    difficulty,
                    last_review,
                    due_date,
                })
            },
        )
        .optional()
        .map_err(|err| format!("Failed to read card state: {err}"))?;

    Ok(row)
}

fn save_state(
    db: &Arc<Mutex<Connection>>,
    card_id: &str,
    memory: MemoryState,
    last_review: DateTime<Utc>,
    due_date: DateTime<Utc>,
    interval_days: i64,
) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "DB lock poisoned".to_string())?;
    conn.execute(
        "
        INSERT INTO fsrs_cards (card_id, stability, difficulty, last_review, due_date, interval_days)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ON CONFLICT(card_id) DO UPDATE SET
            stability = excluded.stability,
            difficulty = excluded.difficulty,
            last_review = excluded.last_review,
            due_date = excluded.due_date,
            interval_days = excluded.interval_days
        ",
        params![
            card_id,
            memory.stability,
            memory.difficulty,
            last_review.to_rfc3339(),
            due_date.to_rfc3339(),
            interval_days,
        ],
    )
    .map_err(|err| format!("Failed to save card state: {err}"))?;

    Ok(())
}

async fn get_settings(State(state): State<FlashcardsState>) -> impl IntoResponse {
    let desired_retention = match load_desired_retention(&state.db) {
        Ok(value) => value,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": err})),
            )
                .into_response();
        }
    };

    Json(serde_json::json!({ "desired_retention": desired_retention }))
        .into_response()
}

#[derive(Deserialize)]
struct SettingsRequest {
    #[serde(rename = "desiredRetention")]
    desired_retention: f32,
}

async fn update_settings(
    State(state): State<FlashcardsState>,
    Json(body): Json<SettingsRequest>,
) -> impl IntoResponse {
    if !(0.7..=0.99).contains(&body.desired_retention) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "desiredRetention must be between 0.7 and 0.99"})),
        )
            .into_response();
    }

    if let Err(err) = save_desired_retention(&state.db, body.desired_retention) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": err})),
        )
            .into_response();
    }

    Json(serde_json::json!({ "success": true }))
        .into_response()
}

fn load_desired_retention(db: &Arc<Mutex<Connection>>) -> Result<f32, String> {
    let conn = db.lock().map_err(|_| "DB lock poisoned".to_string())?;
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM fsrs_settings WHERE key = ?1",
            [SETTINGS_KEY_RETENTION],
            |row| row.get(0),
        )
        .optional()
        .map_err(|err| format!("Failed to read settings: {err}"))?;

    match value {
        Some(raw) => raw
            .parse::<f32>()
            .map_err(|_| "Invalid desired retention value".to_string()),
        None => Ok(DEFAULT_DESIRED_RETENTION),
    }
}

fn save_desired_retention(
    db: &Arc<Mutex<Connection>>,
    value: f32,
) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "DB lock poisoned".to_string())?;
    conn.execute(
        "
        INSERT INTO fsrs_settings (key, value)
        VALUES (?1, ?2)
        ON CONFLICT(key) DO UPDATE SET value = excluded.value
        ",
        params![SETTINGS_KEY_RETENTION, value.to_string()],
    )
    .map_err(|err| format!("Failed to save settings: {err}"))?;
    Ok(())
}
