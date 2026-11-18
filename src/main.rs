use deid_server::extract::{self, Day, Extracted};
use deid_server::{document, engine};

use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, Path};
use axum::http::StatusCode;
use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

/// The largest document the server will read.
const MAX_UPLOAD_BYTES: usize = 10 * 1024 * 1024;

#[derive(Deserialize)]
struct ProcessRequest {
    text: String,
}

#[derive(Serialize)]
struct ProcessResponse {
    /// The de-identified record.
    text: String,
    /// The research columns read from it.
    extracted: Extracted,
}

async fn process(Json(req): Json<ProcessRequest>) -> Json<ProcessResponse> {
    let text = engine::process(&req.text);
    let extracted = extract::extract(&req.text, &text, Day::today());
    Json(ProcessResponse { text, extracted })
}

#[derive(Serialize)]
struct ReadFileResponse {
    /// The plain text of the uploaded file, not yet de-identified: one record
    /// for a Word document, one per row for a CSV file. The page treats them
    /// exactly as if they had been pasted.
    records: Vec<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Reads an uploaded document into text. The file's name is never sent: a
/// file name often contains the patient's name.
async fn read_file(
    Path(kind): Path<String>,
    body: Bytes,
) -> Result<Json<ReadFileResponse>, (StatusCode, Json<ErrorResponse>)> {
    let result = match kind.as_str() {
        "docx" => document::text_from_docx(&body).map(|text| vec![text]),
        "csv" => document::records_from_csv(&body),
        "pdf" => document::text_from_pdf(&body).map(|text| vec![text]),
        _ => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse { error: "This kind of file is not supported.".to_string() }),
            ));
        }
    };
    result
        .map(|records| Json(ReadFileResponse { records }))
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, Json(ErrorResponse { error: e.to_string() })))
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "web/dist".to_string());

    let app = Router::new()
        .route("/api/process", post(process))
        .route(
            "/api/read-file/{kind}",
            post(read_file).layer(DefaultBodyLimit::max(MAX_UPLOAD_BYTES)),
        )
        .fallback_service(ServeDir::new(static_dir));

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("cannot bind {addr}: {e}"));
    println!("listening on http://127.0.0.1:{port}");
    axum::serve(listener, app).await.expect("server error");
}
