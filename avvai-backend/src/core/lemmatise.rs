use std::env;

use crate::core::openrouter;

const MODEL: &str = "google/gemini-2.5-flash-lite";

#[derive(Debug)]
pub enum LemmatiseError {
    ApiKeyMissing,
    RequestFailed(String),
}

pub async fn lemmatise(word: &str) -> Result<String, LemmatiseError> {
    let api_key = env::var("OPENROUTER_API_KEY").map_err(|_| LemmatiseError::ApiKeyMissing)?;

    let prompt = format!(
        "You are a Tamil language expert. Lemmatize the following Tamil word. A lemma is the base/dictionary form of a word. Return ONLY the lemma as a single word, nothing else.\n\nWord: {word}"
    );

    let content = openrouter::chat_completion(&api_key, MODEL, prompt)
        .await
        .map_err(LemmatiseError::RequestFailed)?;

    Ok(content)
}
