//! The engine as the page calls it in the visitor's browser.
//!
//! Built only with `--features wasm` for `wasm32-unknown-unknown`
//! (scripts/build-wasm.sh). The page runs it in a Web Worker, so a record or
//! a file never leaves the visitor's computer. Each function answers with a
//! JSON string in the shape the server's API used to send.
use wasm_bindgen::prelude::*;

use crate::extract::{self, Day, Extracted};
use crate::{assurance, document, engine};

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

/// The assurance report for a set of records, given as a JSON list of the
/// original texts: the report of `src/assurance.rs`, which holds no record
/// text. `{"error": "..."}` if the list cannot be read.
#[wasm_bindgen]
pub fn assure(records_json: &str, year: i32, month: u32, day: u32) -> String {
    match serde_json::from_str::<Vec<String>>(records_json) {
        Ok(records) => json(&assurance::report(&records, Day { year, month, day })),
        Err(_) => error("The records could not be read."),
    }
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

#[cfg(feature = "ocr")]
pub use ocr_api::*;

/// Reading scanned PDFs: only in the OCR build (`--features wasm,ocr`), which
/// the page loads in its own worker when a visitor asks for a scan to be read.
#[cfg(feature = "ocr")]
mod ocr_api {
    use super::error;
    use crate::ocr;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    thread_local! {
        /// The OCR models, loaded once by `ocr_load` and kept for every scan.
        static OCR: RefCell<Option<ocr::Reader>> = const { RefCell::new(None) };
    }

    /// Loads the OCR models (the page fetches them from its own site only when a
    /// visitor asks for a scan to be read). `{}`, or `{"error": "..."}`.
    #[wasm_bindgen]
    pub fn ocr_load(detection_model: Vec<u8>, recognition_model: Vec<u8>) -> String {
        match ocr::Reader::new(detection_model, recognition_model) {
            Ok(reader) => {
                OCR.with(|r| *r.borrow_mut() = Some(reader));
                "{}".to_string()
            }
            Err(_) => error("The text recognition models could not be loaded."),
        }
    }

    /// Whether `ocr_load` has run.
    #[wasm_bindgen]
    pub fn ocr_ready() -> bool {
        OCR.with(|r| r.borrow().is_some())
    }

    /// Reads a scanned PDF by OCR into one plain-text record, not yet
    /// de-identified: `{"records": [text]}`, or `{"error": "..."}`.
    #[wasm_bindgen]
    pub fn ocr_read(bytes: &[u8]) -> String {
        OCR.with(|r| match r.borrow().as_ref() {
            None => error("The text recognition models are not loaded."),
            Some(reader) => match reader.text_from_scan(bytes) {
                Ok(text) => serde_json::json!({ "records": [text] }).to_string(),
                Err(e) => error(&e.to_string()),
            },
        })
    }
}

fn error(message: &str) -> String {
    serde_json::json!({ "error": message }).to_string()
}

fn json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| error("The result could not be encoded."))
}
