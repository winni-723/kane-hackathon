//! Prints the de-identified text of a test document, one record at a time.
//! Usage: cargo run --example show_document -- docx-03
//!        cargo run --example show_document -- csv-02
//!        cargo run --example show_document -- pdf-01
//!        cargo run --example show_document -- pdf-01 --read   (the text as read, before de-identification)
use deid_server::{document, engine};
use std::fs;

fn main() {
    let id = std::env::args().nth(1).expect("usage: show_document <id> [--read]");
    let raw = std::env::args().any(|a| a == "--read");
    let records = if id.starts_with("csv") {
        let bytes = fs::read(format!("tests/documents/{id}.csv")).expect("document exists");
        document::records_from_csv(&bytes).expect("table is readable")
    } else if id.starts_with("pdf") {
        let bytes = fs::read(format!("tests/documents/pdf/{id}.pdf")).expect("document exists");
        vec![document::text_from_pdf(&bytes).unwrap_or_else(|e| e.to_string())]
    } else {
        let bytes = fs::read(format!("tests/documents/{id}.docx")).expect("document exists");
        vec![document::text_from_docx(&bytes).expect("document is readable")]
    };
    for record in records {
        println!("{}\n", if raw { record } else { engine::process(&record) });
    }
}
