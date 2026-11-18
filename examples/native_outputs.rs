//! Prints, as JSON, what the native engine makes of every test input, keyed by
//! path. scripts/wasm-matches-native.mjs runs the same inputs through the
//! engine built for the browser and requires identical answers.
//!
//!   cargo run --example native_outputs > native.json
use deid_server::{document, engine, extract};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;

/// The day ages are counted to; the comparison script passes the same one.
const TODAY: extract::Day = extract::Day { year: 2026, month: 11, day: 14 };

fn processed(records: Vec<String>) -> Value {
    let rows: Vec<Value> = records
        .iter()
        .map(|t| {
            let text = engine::process(t);
            let extracted = extract::extract(t, &text, TODAY);
            json!({ "text": text, "extracted": extracted })
        })
        .collect();
    json!({ "records": rows })
}

fn key(path: &Path) -> String {
    path.display().to_string().replace(std::path::MAIN_SEPARATOR, "/")
}

fn main() {
    let mut out = BTreeMap::new();
    for dir in ["tests/fixtures", "tests/heldout", "tests/synthea"] {
        for entry in std::fs::read_dir(dir).expect("read fixture dir") {
            let path = entry.expect("dir entry").path();
            if path.extension().is_some_and(|e| e == "json") {
                let fixture: Value =
                    serde_json::from_str(&std::fs::read_to_string(&path).expect("read fixture")).expect("fixture json");
                let text = fixture["text"].as_str().expect("fixture text").to_string();
                out.insert(key(&path), processed(vec![text]));
            }
        }
    }
    for dir in ["tests/documents", "tests/documents/pdf"] {
        for entry in std::fs::read_dir(dir).expect("read document dir") {
            let path = entry.expect("dir entry").path();
            let read = match path.extension().and_then(|e| e.to_str()) {
                Some("docx") => document::text_from_docx,
                Some("pdf") => document::text_from_pdf,
                Some("csv") => {
                    let bytes = std::fs::read(&path).expect("read csv");
                    out.insert(key(&path), document::records_from_csv(&bytes).map_or_else(|e| json!({ "error": e.to_string() }), processed));
                    continue;
                }
                _ => continue,
            };
            let bytes = std::fs::read(&path).expect("read document");
            out.insert(key(&path), read(&bytes).map_or_else(|e| json!({ "error": e.to_string() }), |t| processed(vec![t])));
        }
    }
    println!("{}", serde_json::to_string(&out).expect("encode"));
}
