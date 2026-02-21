use chrono::{DateTime, Duration, Utc};
use fsrs::{FSRS, MemoryState, DEFAULT_PARAMETERS};
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use std::{
    collections::HashMap,
    convert::TryFrom,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::task;

use crate::core::db;
use crate::core::lesson::{self, ContentSection, Lesson};

const DEFAULT_DESIRED_RETENTION: f32 = 0.9;
const SETTINGS_KEY_RETENTION: &str = "desired_retention";
const DEFAULT_LIMIT: usize = 30;

#[derive(Clone)]
pub struct FlashcardsState {
    pub db: Arc<Mutex<Connection>>,
}

#[derive(Serialize, Clone)]
pub struct Flashcard {
    pub id: String,
    pub front: String,
    pub back: String,
}

#[derive(Serialize)]
pub struct ReviewResult {
    pub due_date: String,
    pub interval_days: i64,
}

#[derive(Clone)]
struct StoredState {
    stability: f32,
    difficulty: f32,
    last_review: Option<DateTime<Utc>>,
    due_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum FlashcardsError {
    BadRequest(String),
    Internal(String),
}

impl FlashcardsError {
    pub fn message(&self) -> String {
        match self {
            Self::BadRequest(msg) | Self::Internal(msg) => msg.clone(),
        }
    }
}

pub fn init_state() -> FlashcardsState {
    FlashcardsState { db: db::db() }
}

pub async fn get_due(
    state: &FlashcardsState,
    limit: Option<usize>,
) -> Result<Vec<Flashcard>, FlashcardsError> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    let now = Utc::now();

    let cards = load_vocabulary_cards().map_err(FlashcardsError::Internal)?;
    let stored_states = load_states(state.db.clone())
        .await
        .map_err(FlashcardsError::Internal)?;

    let mut due_cards = Vec::new();
    for card in cards {
        let is_due = stored_states
            .get(&card.id)
            .is_none_or(|state| state.due_date.is_none_or(|due| due <= now));

        if is_due {
            due_cards.push(card);
        }

        if due_cards.len() >= limit {
            break;
        }
    }

    Ok(due_cards)
}

pub async fn review_card(
    state: &FlashcardsState,
    card_id: &str,
    rating: u32,
) -> Result<ReviewResult, FlashcardsError> {
    if !(1..=4).contains(&rating) {
        return Err(FlashcardsError::BadRequest(
            "rating must be 1..=4".to_string(),
        ));
    }

    let now = Utc::now();

    let stored_state = load_state(state.db.clone(), card_id)
        .await
        .map_err(FlashcardsError::Internal)?;

    let previous_memory = stored_state.as_ref().map(|state| MemoryState {
        stability: state.stability,
        difficulty: state.difficulty,
    });

    let elapsed_days = stored_state
        .as_ref()
        .and_then(|state| state.last_review)
        .map_or(0, |last| {
            let days = (now - last).num_days().max(0);
            u32::try_from(days).unwrap_or(u32::MAX)
        });

    let desired_retention = load_desired_retention(state.db.clone())
        .await
        .map_err(FlashcardsError::Internal)?;

    let fsrs = FSRS::new(Some(&DEFAULT_PARAMETERS))
        .map_err(|_| FlashcardsError::Internal("failed to initialize FSRS".to_string()))?;

    let next_states = fsrs
        .next_states(previous_memory, desired_retention, elapsed_days)
        .map_err(|_| FlashcardsError::BadRequest("invalid FSRS state".to_string()))?;

    let next_state = match rating {
        1 => next_states.again,
        2 => next_states.hard,
        4 => next_states.easy,
        _ => next_states.good,
    };

    #[allow(clippy::cast_possible_truncation)]
    let interval_days = next_state.interval.round().max(1.0) as i64;
    let due_date = now + Duration::days(interval_days);

    save_state(
        state.db.clone(),
        card_id,
        next_state.memory,
        now,
        due_date,
        interval_days,
    )
    .await
    .map_err(FlashcardsError::Internal)?;

    Ok(ReviewResult {
        due_date: due_date.to_rfc3339(),
        interval_days,
    })
}

pub async fn get_settings(state: &FlashcardsState) -> Result<f32, FlashcardsError> {
    load_desired_retention(state.db.clone())
        .await
        .map_err(FlashcardsError::Internal)
}

pub async fn update_settings(
    state: &FlashcardsState,
    desired_retention: f32,
) -> Result<(), FlashcardsError> {
    if !(0.7..=0.99).contains(&desired_retention) {
        return Err(FlashcardsError::BadRequest(
            "desiredRetention must be between 0.7 and 0.99".to_string(),
        ));
    }

    save_desired_retention(state.db.clone(), desired_retention)
        .await
        .map_err(FlashcardsError::Internal)
}

fn lessons_dir() -> PathBuf {
    lesson::lessons_dir()
}

fn load_vocabulary_cards() -> Result<Vec<Flashcard>, String> {
    let mut cards = Vec::new();
    let lessons_path = lessons_dir();

    let entries = fs::read_dir(&lessons_path)
        .map_err(|err| format!("Failed to read lessons directory: {err}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }

        let content = fs::read_to_string(&path)
            .map_err(|err| format!("Failed to read lesson file: {err}"))?;
        let lesson: Lesson =
            serde_json::from_str(&content).map_err(|err| format!("Failed to parse lesson JSON: {err}"))?;

        for section in &lesson.sections {
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

async fn load_states(db: Arc<Mutex<Connection>>) -> Result<HashMap<String, StoredState>, String> {
    task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock poisoned".to_string())?;
        let map = {
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
            map
        };
        drop(conn);

        Ok(map)
    })
    .await
    .map_err(|_| "Failed to join blocking task".to_string())?
}

async fn load_state(
    db: Arc<Mutex<Connection>>,
    card_id: &str,
) -> Result<Option<StoredState>, String> {
    let card_id = card_id.to_string();
    task::spawn_blocking(move || {
        let row = db
            .lock()
            .map_err(|_| "DB lock poisoned".to_string())?
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
    })
    .await
    .map_err(|_| "Failed to join blocking task".to_string())?
}

async fn save_state(
    db: Arc<Mutex<Connection>>,
    card_id: &str,
    memory: MemoryState,
    last_review: DateTime<Utc>,
    due_date: DateTime<Utc>,
    interval_days: i64,
) -> Result<(), String> {
    let card_id = card_id.to_string();
    task::spawn_blocking(move || {
        db.lock()
            .map_err(|_| "DB lock poisoned".to_string())?
            .execute(
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
    })
    .await
    .map_err(|_| "Failed to join blocking task".to_string())?
}

async fn load_desired_retention(db: Arc<Mutex<Connection>>) -> Result<f32, String> {
    task::spawn_blocking(move || {
        let value: Option<String> = db
            .lock()
            .map_err(|_| "DB lock poisoned".to_string())?
            .query_row(
                "SELECT value FROM fsrs_settings WHERE key = ?1",
                [SETTINGS_KEY_RETENTION],
                |row| row.get(0),
            )
            .optional()
            .map_err(|err| format!("Failed to read settings: {err}"))?;

        value.map_or(Ok(DEFAULT_DESIRED_RETENTION), |raw| {
            raw.parse::<f32>()
                .map_err(|_| "Invalid desired retention value".to_string())
        })
    })
    .await
    .map_err(|_| "Failed to join blocking task".to_string())?
}

async fn save_desired_retention(db: Arc<Mutex<Connection>>, value: f32) -> Result<(), String> {
    task::spawn_blocking(move || {
        db.lock()
            .map_err(|_| "DB lock poisoned".to_string())?
            .execute(
            "
            INSERT INTO fsrs_settings (key, value)
            VALUES (?1, ?2)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value
            ",
            params![SETTINGS_KEY_RETENTION, value.to_string()],
            )
            .map_err(|err| format!("Failed to save settings: {err}"))?;
        Ok(())
    })
    .await
    .map_err(|_| "Failed to join blocking task".to_string())?
}
