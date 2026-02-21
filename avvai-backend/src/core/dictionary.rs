use std::env;

use crate::core::dictionary_cache::{self, DictionaryEntry};
use crate::core::openrouter;

const MODEL: &str = "google/gemini-2.5-flash-lite";

#[derive(Debug)]
pub enum DictionaryError {
    EmptyWord,
    ApiKeyMissing,
    RequestFailed(String),
    InvalidJson,
}

pub async fn lookup(word: &str) -> Result<DictionaryEntry, DictionaryError> {
    let normalised = dictionary_cache::normalise(word);
    if normalised.is_empty() {
        return Err(DictionaryError::EmptyWord);
    }

    if let Some(entry) = dictionary_cache::get(&normalised).await {
        return Ok(entry);
    }

    let api_key = env::var("OPENROUTER_API_KEY").map_err(|_| DictionaryError::ApiKeyMissing)?;

    let prompt = format!(
        "You are a Tamil-English dictionary. Given a Tamil word, return a JSON object with fields: word (the original word), definition (short English definition), and examples (array of 1-2 short Tamil example sentences). Return ONLY valid JSON, nothing else.\n\nWord: {normalised}"
    );

    let content = openrouter::chat_completion(&api_key, MODEL, prompt)
        .await
        .map_err(DictionaryError::RequestFailed)?;

    let cleaned = clean_json_response(&content);
    let entry = serde_json::from_str::<DictionaryEntry>(&cleaned)
        .map_err(|_| DictionaryError::InvalidJson)?;

    dictionary_cache::set(&normalised, entry.clone()).await;

    Ok(entry)
}

fn clean_json_response(raw: &str) -> String {
    let trimmed = raw.trim();

    let without_fence = if trimmed.starts_with("```") {
        let stripped = trimmed
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```");
        stripped.trim()
    } else {
        trimmed
    };

    if let (Some(start), Some(end)) = (without_fence.find('{'), without_fence.rfind('}'))
        && start <= end
    {
        return without_fence[start..=end].trim().to_string();
    }

    without_fence.to_string()
}
