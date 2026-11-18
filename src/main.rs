//! Serves the page and nothing else.
//!
//! The de-identification engine runs in the visitor's browser (src/wasm.rs,
//! compiled to WebAssembly), so this server never receives a record or a
//! file: it has no route that accepts one.
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "web/dist".to_string());

    let app = axum::Router::new().fallback_service(ServeDir::new(static_dir));

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("cannot bind {addr}: {e}"));
    println!("listening on http://127.0.0.1:{port}");
    axum::serve(listener, app).await.expect("server error");
}
