mod engine;

use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct ProcessRequest {
    text: String,
}

#[derive(Serialize)]
struct ProcessResponse {
    text: String,
}

async fn process(Json(req): Json<ProcessRequest>) -> Json<ProcessResponse> {
    Json(ProcessResponse {
        text: engine::process(&req.text),
    })
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "web/dist".to_string());

    let app = Router::new()
        .route("/api/process", post(process))
        .fallback_service(ServeDir::new(static_dir));

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("cannot bind {addr}: {e}"));
    println!("listening on http://127.0.0.1:{port}");
    axum::serve(listener, app).await.expect("server error");
}
