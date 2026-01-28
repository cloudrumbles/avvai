mod routes;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/dictionary", routes::dictionary::router())
        .nest("/lesson", routes::lesson::router())
        .nest("/progress", routes::progress::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");

    println!("avvai backend listening on http://localhost:3001");

    axum::serve(listener, app).await.expect("Server error");
}
