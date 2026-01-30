use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, sync::OnceLock};
use tokio::sync::Mutex;

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

type DictionaryCache = HashMap<String, DictionaryEntry>;

static DICTIONARY_CACHE: OnceLock<Mutex<DictionaryCache>> = OnceLock::new();

fn cache_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/dictionary-cache.json")
}

pub fn normalise(word: &str) -> String {
    word.trim().to_lowercase()
}

fn load_cache_from_disk() -> DictionaryCache {
    let path = cache_path();
    let Ok(contents) = fs::read_to_string(path) else {
        return HashMap::new();
    };

    serde_json::from_str::<DictionaryCache>(&contents).unwrap_or_default()
}

fn save_cache_to_disk(cache: &DictionaryCache) {
    let path = cache_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if let Ok(contents) = serde_json::to_string_pretty(cache) {
        let _ = fs::write(path, contents);
    }
}

fn cache_handle() -> &'static Mutex<DictionaryCache> {
    DICTIONARY_CACHE.get_or_init(|| Mutex::new(load_cache_from_disk()))
}

pub async fn get(word_key: &str) -> Option<DictionaryEntry> {
    let key = normalise(word_key);
    if key.is_empty() {
        return None;
    }

    let cache = cache_handle();
    let guard = cache.lock().await;
    guard.get(&key).cloned()
}

pub async fn set(word_key: &str, mut entry: DictionaryEntry) {
    let key = normalise(word_key);
    if key.is_empty() {
        return;
    }

    if entry.word.trim().is_empty() {
        entry.word = key.clone();
    }

    let cache = cache_handle();
    let mut guard = cache.lock().await;
    guard.insert(key, entry);
    save_cache_to_disk(&guard);
}

pub async fn remove(word_key: &str) -> bool {
    let key = normalise(word_key);
    if key.is_empty() {
        return false;
    }

    let cache = cache_handle();
    let mut guard = cache.lock().await;
    let removed = guard.remove(&key).is_some();
    if removed {
        save_cache_to_disk(&guard);
    }
    removed
}

pub async fn list() -> Vec<DictionaryCacheEntry> {
    let cache = cache_handle();
    let guard = cache.lock().await;
    let mut entries: Vec<DictionaryCacheEntry> = guard
        .iter()
        .map(|(key, entry)| DictionaryCacheEntry {
            key: key.clone(),
            entry: entry.clone(),
        })
        .collect();

    entries.sort_by(|a, b| a.key.cmp(&b.key));
    entries
}
