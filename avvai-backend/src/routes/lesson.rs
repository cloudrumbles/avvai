use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// =============================================================================
// LESSON SUMMARY (for list view)
// =============================================================================

#[derive(Serialize, Deserialize)]
struct LessonSummary {
    id: String,
    title: String,
    description: String,
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

fn lessons_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/lessons")
}

fn load_lesson_from_file(id: &str) -> Option<Lesson> {
    // Scan all JSON files and find the one with matching id
    let lessons_path = lessons_dir();
    let lessons_path = lessons_path.as_path();
    if let Ok(entries) = fs::read_dir(lessons_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(lesson) = serde_json::from_str::<Lesson>(&content) {
                        if lesson.id == id {
                            return Some(lesson);
                        }
                    }
                }
            }
        }
    }
    None
}

fn get_all_lesson_summaries() -> Vec<LessonSummary> {
    let mut summaries = Vec::new();

    let lessons_path = lessons_dir();
    let lessons_path = lessons_path.as_path();
    if let Ok(entries) = fs::read_dir(lessons_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(lesson) = serde_json::from_str::<Lesson>(&content) {
                        summaries.push(LessonSummary {
                            id: lesson.id,
                            title: lesson.title,
                            description: lesson.description,
                        });
                    }
                }
            }
        }
    }

    summaries
}

// =============================================================================
// REQUEST/RESPONSE HANDLERS
// =============================================================================

#[derive(Deserialize)]
struct GetParams {
    id: Option<String>,
}

async fn list() -> impl IntoResponse {
    let lessons = get_all_lesson_summaries();
    Json(lessons)
}

async fn get_lesson(Query(params): Query<GetParams>) -> impl IntoResponse {
    let Some(id) = params.id else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing id parameter"})),
        )
            .into_response();
    };

    match load_lesson_from_file(&id) {
        Some(lesson) => Json(lesson).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Lesson not found"})),
        )
            .into_response(),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/list", get(list))
        .route("/get", get(get_lesson))
}
