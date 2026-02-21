use rusqlite::params;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task;

pub async fn load_progress(
    db: Arc<std::sync::Mutex<rusqlite::Connection>>,
) -> Result<HashMap<String, bool>, String> {
    task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock poisoned".to_string())?;
        let map = {
            let mut stmt = conn
                .prepare("SELECT lesson_id, completed FROM progress")
                .map_err(|err| format!("Failed to prepare query: {err}"))?;

            let rows = stmt
                .query_map([], |row| {
                    let lesson_id: String = row.get(0)?;
                    let completed: i64 = row.get(1)?;
                    Ok((lesson_id, completed != 0))
                })
                .map_err(|err| format!("Failed to query progress: {err}"))?;

            let mut map = HashMap::new();
            for row in rows {
                let (lesson_id, completed) =
                    row.map_err(|err| format!("Failed to read row: {err}"))?;
                map.insert(lesson_id, completed);
            }
            map
        };
        drop(conn);

        Ok::<HashMap<String, bool>, String>(map)
    })
    .await
    .map_err(|_| "Failed to join blocking task".to_string())?
}

pub async fn upsert_progress(
    db: Arc<std::sync::Mutex<rusqlite::Connection>>,
    lesson_id: &str,
    completed: bool,
) -> Result<(), String> {
    let lesson_id = lesson_id.to_string();
    task::spawn_blocking(move || {
        db.lock()
            .map_err(|_| "DB lock poisoned".to_string())?
            .execute(
            "
            INSERT INTO progress (lesson_id, completed, updated_at)
            VALUES (?1, ?2, datetime('now'))
            ON CONFLICT(lesson_id) DO UPDATE SET
                completed = excluded.completed,
                updated_at = excluded.updated_at
            ",
            params![lesson_id, i64::from(completed)],
            )
            .map_err(|err| format!("Failed to save progress: {err}"))?;
        Ok::<(), String>(())
    })
    .await
    .map_err(|_| "Failed to join blocking task".to_string())?
}
