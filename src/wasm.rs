//! The engine as the page calls it in the visitor's browser.
//!
//! Built only with `--features wasm` for `wasm32-unknown-unknown`
//! (scripts/build-wasm.sh). The page runs it in a Web Worker, so a record or
//! a file never leaves the visitor's computer. Each function answers with a
//! JSON string in the shape the server's API used to send.
use wasm_bindgen::prelude::*;

use crate::extract::{self, Day, Extracted};
use crate::{document, engine};

#[derive(serde::Serialize)]
struct Processed {
    /// The de-identified record.
    text: String,
    /// The research columns read from it.
    extracted: Extracted,
}

/// De-identifies one record: `{"text", "extracted"}`.
///
/// `today` comes from the page: the browser has no system clock the engine
/// can read, and ages from a date of birth are counted up to it.
#[wasm_bindgen]
pub fn process(text: &str, year: i32, month: u32, day: u32) -> String {
    let deidentified = engine::process(text);
    let extracted = extract::extract(text, &deidentified, Day { year, month, day });
    json(&Processed { text: deidentified, extracted })
}

/// Reads a file into plain-text records, not yet de-identified: one for a
/// Word document or a PDF, one per row for a CSV file.
/// `{"records": [...]}`, or `{"error": "..."}` for a file it cannot read.
#[wasm_bindgen]
pub fn read_file(kind: &str, bytes: &[u8]) -> String {
    let result = match kind {
        "docx" => document::text_from_docx(bytes).map(|text| vec![text]),
        "csv" => document::records_from_csv(bytes),
        "pdf" => document::text_from_pdf(bytes).map(|text| vec![text]),
        _ => return error("This kind of file is not supported."),
    };
    match result {
        Ok(records) => serde_json::json!({ "records": records }).to_string(),
        Err(e) => error(&e.to_string()),
    }
}

fn error(message: &str) -> String {
    serde_json::json!({ "error": message }).to_string()
}

fn json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| error("The result could not be encoded."))
}
