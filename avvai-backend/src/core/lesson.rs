use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::sync::{Mutex, OnceCell};

// =============================================================================
// LESSON SUMMARY (for list view)
// =============================================================================

#[derive(Serialize, Deserialize)]
pub struct LessonSummary {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct LessonListItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub filename: String,
}

// =============================================================================
// FULL LESSON STRUCTURE
// =============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceMetadata>,
    pub sections: Vec<ContentSection>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SourceMetadata {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verse_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poetic_form: Option<String>,
}

// =============================================================================
// CONTENT SECTION ENUM
// =============================================================================

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentSection {
    Prose(ProseSection),
    Poetry(PoetrySection),
    Vocabulary(VocabularySection),
    Exercises(ExercisesSection),
    Media(MediaSection),
    Dialogue(DialogueSection),
}

// =============================================================================
// PROSE SECTION
// =============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct ProseSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub paragraphs: Vec<String>,
}

// =============================================================================
// POETRY SECTION
// =============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct PoetrySection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub verses: Vec<Verse>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Verse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,
    pub lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
}

// =============================================================================
// VOCABULARY SECTION
// =============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct VocabularySection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub entries: Vec<VocabularyEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VocabularyEntry {
    pub word: String,
    pub meaning: String,
}

// =============================================================================
// DIALOGUE SECTION (for drama/plays)
// =============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct DialogueSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<SceneInfo>,
    pub lines: Vec<DialogueLine>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SceneInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DialogueLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<bool>,
}

// =============================================================================
// EXERCISES SECTION
// =============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct ExercisesSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub exercise_groups: Vec<ExerciseGroup>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExerciseGroup {
    pub group_type: ExerciseGroupType,
    pub instructions: String,
    pub exercises: Vec<Exercise>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseGroupType {
    MultipleChoice,
    FillInBlank,
    ShortAnswer,
    LongAnswer,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Exercise {
    pub id: String,
    pub content: ExerciseContent,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "exercise_type", rename_all = "snake_case")]
pub enum ExerciseContent {
    MultipleChoice(MultipleChoiceExercise),
    FillInBlank(FillInBlankExercise),
    ShortAnswer(ShortAnswerExercise),
    LongAnswer(LongAnswerExercise),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MultipleChoiceExercise {
    pub question: String,
    pub options: Vec<ChoiceOption>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChoiceOption {
    pub id: String,
    pub text: String,
    pub correct: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FillInBlankExercise {
    pub text_before: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_after: Option<String>,
    pub accepted_answers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShortAnswerExercise {
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_answer: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LongAnswerExercise {
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_words: Option<u32>,
}

// =============================================================================
// MEDIA SECTION
// =============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct MediaSection {
    pub media_type: MediaType,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Audio,
    Video,
}

// =============================================================================
// FILE LOADING
// =============================================================================

pub fn lessons_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/lessons")
}

pub fn lesson_filename_for_id(id: &str) -> String {
    format!("{id}.json")
}

static LESSON_INDEX: OnceCell<Mutex<HashMap<String, PathBuf>>> = OnceCell::const_new();

async fn build_lesson_index() -> HashMap<String, PathBuf> {
    let mut index = HashMap::new();
    let lessons_path = lessons_dir();

    let Ok(mut entries) = fs::read_dir(&lessons_path).await else {
        return index;
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }
        if let Ok(content) = fs::read_to_string(&path).await
            && let Ok(lesson) = serde_json::from_str::<Lesson>(&content)
        {
            index.insert(lesson.id, path);
        }
    }

    index
}

async fn lesson_index() -> &'static Mutex<HashMap<String, PathBuf>> {
    LESSON_INDEX
        .get_or_init(|| async { Mutex::new(build_lesson_index().await) })
        .await
}

async fn refresh_lesson_index() -> HashMap<String, PathBuf> {
    build_lesson_index().await
}

async fn load_lesson_from_file(id: &str) -> Option<Lesson> {
    let index = lesson_index().await;
    let path = {
        let map = index.lock().await;
        map.get(id).cloned()
    };
    if let Some(path) = path
        && let Ok(content) = fs::read_to_string(&path).await
        && let Ok(lesson) = serde_json::from_str::<Lesson>(&content)
    {
        return Some(lesson);
    }

    // Refresh index once in case new files were added
    let refreshed = refresh_lesson_index().await;
    let path = {
        let mut map = index.lock().await;
        *map = refreshed;
        map.get(id).cloned()
    };
    if let Some(path) = path
        && let Ok(content) = fs::read_to_string(&path).await
        && let Ok(lesson) = serde_json::from_str::<Lesson>(&content)
    {
        return Some(lesson);
    }

    None
}

async fn load_lesson_from_path(path: &Path) -> Option<Lesson> {
    let Ok(content) = fs::read_to_string(path).await else {
        return None;
    };
    serde_json::from_str::<Lesson>(&content).ok()
}

async fn save_lesson_to_path(path: &Path, lesson: &Lesson) -> Result<(), LessonStoreError> {
    let contents =
        serde_json::to_string_pretty(lesson).map_err(|_| LessonStoreError::Serialize)?;
    fs::write(path, contents)
        .await
        .map_err(|err| LessonStoreError::Io(err.to_string()))?;
    Ok(())
}

async fn update_index(id: &str, path: Option<PathBuf>) {
    let index = lesson_index().await;
    let mut map = index.lock().await;
    match path {
        Some(path) => {
            map.insert(id.to_string(), path);
        }
        None => {
            map.remove(id);
        }
    }
}

// =============================================================================
// CORE API
// =============================================================================

pub async fn get_lesson(id: &str) -> Option<Lesson> {
    load_lesson_from_file(id).await
}

pub async fn list_summaries() -> Vec<LessonSummary> {
    let mut summaries = Vec::new();

    let lessons_path = lessons_dir();
    let Ok(mut entries) = fs::read_dir(&lessons_path).await else {
        return summaries;
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }
        if let Ok(content) = fs::read_to_string(&path).await
            && let Ok(lesson) = serde_json::from_str::<Lesson>(&content)
        {
            summaries.push(LessonSummary {
                id: lesson.id,
                title: lesson.title,
                description: lesson.description,
            });
        }
    }

    summaries
}

pub async fn list_admin_items() -> Vec<LessonListItem> {
    let mut items = Vec::new();
    let dir = lessons_dir();

    if let Ok(mut entries) = fs::read_dir(dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json")
                && let Some(lesson) = load_lesson_from_path(&path).await
            {
                let filename = path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_default();
                items.push(LessonListItem {
                    id: lesson.id,
                    title: lesson.title,
                    description: lesson.description,
                    filename,
                });
            }
        }
    }

    items.sort_by(|a, b| a.id.cmp(&b.id));
    items
}

pub async fn create_lesson(
    lesson: &Lesson,
    filename: Option<String>,
) -> Result<Lesson, LessonStoreError> {
    let filename = filename.unwrap_or_else(|| lesson_filename_for_id(&lesson.id));
    let path = lessons_dir().join(filename);

    if fs::try_exists(&path).await.unwrap_or(false) {
        return Err(LessonStoreError::AlreadyExists);
    }

    save_lesson_to_path(&path, lesson).await?;
    update_index(&lesson.id, Some(path)).await;
    Ok(lesson.clone())
}

pub async fn update_lesson(id: &str, lesson: &Lesson) -> Result<Lesson, LessonStoreError> {
    let path = lessons_dir().join(lesson_filename_for_id(id));
    if !fs::try_exists(&path).await.unwrap_or(false) {
        return Err(LessonStoreError::NotFound);
    }

    save_lesson_to_path(&path, lesson).await?;
    update_index(id, Some(path)).await;
    Ok(lesson.clone())
}

pub async fn delete_lesson(id: &str) -> Result<(), LessonStoreError> {
    let path = lessons_dir().join(lesson_filename_for_id(id));
    if !fs::try_exists(&path).await.unwrap_or(false) {
        return Err(LessonStoreError::NotFound);
    }

    fs::remove_file(&path)
        .await
        .map_err(|err| LessonStoreError::Io(err.to_string()))?;
    update_index(id, None).await;
    Ok(())
}

#[derive(Debug)]
pub enum LessonStoreError {
    NotFound,
    AlreadyExists,
    Serialize,
    Io(String),
}

impl LessonStoreError {
    pub fn message(&self) -> String {
        match self {
            Self::NotFound => "Lesson not found".to_string(),
            Self::AlreadyExists => "Lesson already exists".to_string(),
            Self::Serialize => "Failed to serialize lesson".to_string(),
            Self::Io(err) => err.clone(),
        }
    }
}
