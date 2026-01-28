use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LessonSummary {
    id: String,
    title: String,
    description: String,
}

#[derive(Serialize)]
struct Lesson {
    id: String,
    title: String,
    description: String,
    content: String,
}

#[derive(Deserialize)]
struct GetParams {
    id: Option<String>,
}

async fn list() -> impl IntoResponse {
    // Mock data - replace with database later
    let lessons = vec![
        LessonSummary {
            id: "thirukkural-intro".into(),
            title: "திருக்குறள் அறிமுகம்".into(),
            description: "Introduction to Thirukkural".into(),
        },
        LessonSummary {
            id: "kural-1".into(),
            title: "குறள் 1: அகர முதல".into(),
            description: "The first Kural about the primacy of 'A'".into(),
        },
        LessonSummary {
            id: "kural-2".into(),
            title: "குறள் 2: கற்றதனால்".into(),
            description: "The second Kural about learning".into(),
        },
    ];

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

    // Mock data - replace with database later
    let lesson = match id.as_str() {
        "thirukkural-intro" => Lesson {
            id: id.clone(),
            title: "திருக்குறள் அறிமுகம்".into(),
            description: "Introduction to Thirukkural".into(),
            content: "திருக்குறள் பற்றிய அறிமுகம்\n\nதிருக்குறள், சுருக்கமாக குறள், ஒரு தொன்மையான தமிழ் நூலாகும். இது திருவள்ளுவர் என்ற புலவரால் இயற்றப்பட்டது.\n\nஅறத்துப்பால்\nபொருட்பால்\nஇன்பத்துப்பால்\n\nஎன மூன்று பகுதிகளாகப் பிரிக்கப்பட்டுள்ளது.".into(),
        },
        "kural-1" => Lesson {
            id: id.clone(),
            title: "குறள் 1: அகர முதல".into(),
            description: "The first Kural".into(),
            content: "குறள் 1\n\nஅகர முதல எழுத்தெல்லாம் ஆதி\nபகவன் முதற்றே உலகு.\n\nபொருள்: எழுத்துக்கள் எல்லாம் அகரத்தை அடிப்படையாகக் கொண்டிருக்கின்றன. அதுபோல், உலகம் கடவுளை அடிப்படையாகக் கொண்டிருக்கிறது.".into(),
        },
        "kural-2" => Lesson {
            id: id.clone(),
            title: "குறள் 2: கற்றதனால்".into(),
            description: "The second Kural".into(),
            content: "குறள் 2\n\nகற்றதனால் ஆய பயனென்கொல் வாலறிவன்\nநற்றாள் தொழாஅர் எனின்.\n\nபொருள்: தூய அறிவு வடிவான இறைவனின் திருவடிகளை வணங்காதவர்களுக்கு கல்வியால் என்ன பயன்?".into(),
        },
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Lesson not found"})),
            )
                .into_response();
        }
    };

    Json(lesson).into_response()
}

pub fn router() -> Router {
    Router::new()
        .route("/list", get(list))
        .route("/get", get(get_lesson))
}
