use rusqlite::Connection;
use std::{
    env,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

static DB: OnceLock<Arc<Mutex<Connection>>> = OnceLock::new();

pub fn db_path() -> PathBuf {
    if let Ok(path) = env::var("APP_DB_PATH") {
        return PathBuf::from(path);
    }
    if let Ok(path) = env::var("FSRS_DB_PATH") {
        return PathBuf::from(path);
    }

    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("fsrs.sqlite3")
}

pub fn db() -> Arc<Mutex<Connection>> {
    DB.get_or_init(|| {
        let path = db_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(path).expect("Failed to open database");
        init_tables(&conn).expect("Failed to initialize database tables");
        Arc::new(Mutex::new(conn))
    })
    .clone()
}

fn init_tables(conn: &Connection) -> rusqlite::Result<()> {
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
        CREATE TABLE IF NOT EXISTS progress (
            lesson_id TEXT PRIMARY KEY,
            completed INTEGER NOT NULL,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS dictionary_cache (
            cache_key TEXT PRIMARY KEY,
            word TEXT NOT NULL,
            definition TEXT NOT NULL,
            examples TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        ",
    )?;

    Ok(())
}
