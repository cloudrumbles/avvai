mod routes;
pub mod services;

use axum::{routing::get, Router};
use dotenv::from_filename;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    from_filename(".env.local").ok();

    let flashcards_state = routes::flashcards::init_state();

    let admin_state = Arc::new(
        services::admin_auth::AdminAuthState::from_env()
            .expect("Missing admin auth configuration"),
    );

    let admin_router = routes::admin::router().with_state(admin_state);

    let app = Router::new()
        .nest("/dictionary", routes::dictionary::router())
        .nest("/lesson", routes::lesson::router())
        .nest("/progress", routes::progress::router())
        .nest("/flashcards", routes::flashcards::router(flashcards_state))
        .nest("/dictionary/lemmatise", routes::lemmatise::router())
        .nest("/admin", admin_router)
        .route("/health", get(|| async { "ok" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");

    println!("avvai backend listening on http://localhost:3001");

    axum::serve(listener, app).await.expect("Server error");
}
