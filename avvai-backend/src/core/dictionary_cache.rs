use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use tokio::task;

use crate::core::db;

#[derive(Serialize, Deserialize, Clone)]
pub struct DictionaryEntry {
    pub word: String,
    pub definition: String,
    pub examples: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct DictionaryCacheEntry {
    pub key: String,
    pub entry: DictionaryEntry,
}

pub fn normalise(word: &str) -> String {
    word.trim().to_lowercase()
}

pub async fn get(word_key: &str) -> Option<DictionaryEntry> {
    let key = normalise(word_key);
    if key.is_empty() {
        return None;
    }
    let db = db::db();
    task::spawn_blocking(move || {
        let row = {
            let conn = db.lock().map_err(|_| "DB lock poisoned".to_string())?;
            conn.query_row(
                "SELECT word, definition, examples FROM dictionary_cache WHERE cache_key = ?1",
                [key],
                |row| {
                    let word: String = row.get(0)?;
                    let definition: String = row.get(1)?;
                    let examples_json: String = row.get(2)?;
                    let examples = serde_json::from_str::<Vec<String>>(&examples_json)
                        .unwrap_or_default();
                    Ok(DictionaryEntry {
                        word,
                        definition,
                        examples,
                    })
                },
            )
            .optional()
            .map_err(|err| format!("Failed to query dictionary cache: {err}"))?
        };

        Ok::<Option<DictionaryEntry>, String>(row)
    })
    .await
    .ok()
    .and_then(Result::ok)
    .flatten()
}

pub async fn set(word_key: &str, mut entry: DictionaryEntry) {
    let key = normalise(word_key);
    if key.is_empty() {
        return;
    }

    if entry.word.trim().is_empty() {
        entry.word.clone_from(&key);
    }

    let db = db::db();
    let examples_json = serde_json::to_string(&entry.examples).unwrap_or_else(|_| "[]".to_string());
    let word = entry.word.clone();
    let definition = entry.definition.clone();
    task::spawn_blocking(move || {
        db.lock()
            .map_err(|_| "DB lock poisoned".to_string())?
            .execute(
            "
            INSERT INTO dictionary_cache (cache_key, word, definition, examples, updated_at)
            VALUES (?1, ?2, ?3, ?4, datetime('now'))
            ON CONFLICT(cache_key) DO UPDATE SET
                word = excluded.word,
                definition = excluded.definition,
                examples = excluded.examples,
                updated_at = excluded.updated_at
            ",
            [key, word, definition, examples_json],
            )
            .map_err(|err| format!("Failed to upsert dictionary cache: {err}"))?;
        Ok::<(), String>(())
    })
    .await
    .ok();
}

pub async fn remove(word_key: &str) -> bool {
    let key = normalise(word_key);
    if key.is_empty() {
        return false;
    }
    let db = db::db();
    task::spawn_blocking(move || {
        let rows = db
            .lock()
            .map_err(|_| "DB lock poisoned".to_string())?
            .execute("DELETE FROM dictionary_cache WHERE cache_key = ?1", [key])
            .map_err(|err| format!("Failed to delete dictionary cache: {err}"))?;
        Ok::<bool, String>(rows > 0)
    })
    .await
    .ok()
    .and_then(Result::ok)
    .unwrap_or(false)
}

pub async fn list() -> Vec<DictionaryCacheEntry> {
    let db = db::db();
    task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock poisoned".to_string())?;
        let entries = {
            let mut stmt = conn
                .prepare(
                    "SELECT cache_key, word, definition, examples FROM dictionary_cache ORDER BY cache_key ASC",
                )
                .map_err(|err| format!("Failed to prepare query: {err}"))?;

            let rows = stmt
                .query_map([], |row| {
                    let key: String = row.get(0)?;
                    let word: String = row.get(1)?;
                    let definition: String = row.get(2)?;
                    let examples_json: String = row.get(3)?;
                    let examples = serde_json::from_str::<Vec<String>>(&examples_json)
                        .unwrap_or_default();
                    Ok(DictionaryCacheEntry {
                        key,
                        entry: DictionaryEntry {
                            word,
                            definition,
                            examples,
                        },
                    })
                })
                .map_err(|err| format!("Failed to query dictionary cache: {err}"))?;

            let mut entries = Vec::new();
            for row in rows {
                entries.push(row.map_err(|err| format!("Failed to read row: {err}"))?);
            }
            entries
        };
        drop(conn);
        Ok::<Vec<DictionaryCacheEntry>, String>(entries)
    })
    .await
    .ok()
    .and_then(Result::ok)
    .unwrap_or_default()
}
