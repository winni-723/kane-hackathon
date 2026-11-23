//! Layer 1 check of the research-table extraction on Word and PDF documents,
//! against the answer keys in `tests/extraction-documents/`.
//!
//! The answer keys are the standard. Nothing here may be loosened to make a
//! test pass: the extraction changes, the keys and the gates do not shrink.

use deid_server::extract::{self, Day, Extracted};
use deid_server::{document, engine};
use serde_json::Value;
use std::fs;

/// Ages worked out from a date of birth are counted up to this day, the one
/// the answer keys were written for.
const REFERENCE_DAY: Day = Day { year: 2026, month: 11, day: 14 };

/// Columns the extraction must get completely right on every document. This
/// list only ever grows.
const GATES: &[&str] = &["age", "sex"];

const COLUMNS: [&str; 4] = ["age", "sex", "diagnoses", "medications"];

struct Key {
    id: String,
    age: Option<String>,
    age_source: Option<String>,
    sex: Option<String>,
    diagnoses: Vec<String>,
    medications: Vec<String>,
    must_not_list: Vec<String>,
    got: Extracted,
    deidentified: String,
    planted: Vec<String>,
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array()
        .expect("expected a list")
        .iter()
        .map(|s| s.as_str().expect("expected a string").to_string())
        .collect()
}

fn opt(v: &Value) -> Option<String> {
    v.as_str().map(str::to_string)
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}")))
        .unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn load() -> Vec<Key> {
    let mut paths: Vec<_> = fs::read_dir("tests/extraction-documents")
        .expect("tests/extraction-documents exists")
        .map(|e| e.expect("readable entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    paths.sort();
    let mut keys = Vec::new();
    for path in paths {
        let key = read_json(path.to_str().expect("utf-8 path"));
        let id = key["record_id"].as_str().expect("record_id").to_string();
        let (text, pii_key) = if id.starts_with("pdf") {
            let bytes = fs::read(format!("tests/documents/pdf/{id}.pdf")).expect("document exists");
            (document::text_from_pdf(&bytes).expect("readable PDF"), read_json(&format!("tests/documents/pdf/keys/{id}.json")))
        } else {
            let bytes = fs::read(format!("tests/documents/{id}.docx")).expect("document exists");
            (document::text_from_docx(&bytes).expect("readable document"), read_json(&format!("tests/documents/keys/{id}.json")))
        };
        let deidentified = engine::process(&text);
        let got = extract::extract(&text, &deidentified, REFERENCE_DAY);
        keys.push(Key {
            age: opt(&key["age"]),
            age_source: opt(&key["age_source"]),
            sex: opt(&key["sex"]),
            diagnoses: strings(&key["diagnoses"]),
            medications: strings(&key["medications"]),
            must_not_list: strings(&key["must_not_list"]),
            planted: pii_key["planted_pii"]
                .as_array()
                .expect("planted_pii")
                .iter()
                .map(|p| p["text"].as_str().expect("pii text").to_string())
                .collect(),
            id,
            got,
            deidentified,
        });
    }
    assert_eq!(keys.len(), 11, "one extraction key per Word and PDF test document");
    keys
}

fn has(cells: &[String], wanted: &str) -> bool {
    let wanted = wanted.to_lowercase();
    cells.iter().any(|c| c.to_lowercase().contains(&wanted))
}

/// What one document got wrong in one column, as text for messages.
fn mistakes(key: &Key, column: &str) -> Vec<String> {
    let mut wrong = Vec::new();
    match column {
        "age" => {
            if key.got.age != key.age || key.got.age_source.map(str::to_string) != key.age_source {
                wrong.push(format!(
                    "age: want {:?} ({:?}), got {:?} ({:?})",
                    key.age, key.age_source, key.got.age, key.got.age_source
                ));
            }
        }
        "sex" => {
            if key.got.sex.map(str::to_string) != key.sex {
                wrong.push(format!("sex: want {:?}, got {:?}", key.sex, key.got.sex));
            }
        }
        "diagnoses" | "medications" => {
            let (want, got) = if column == "diagnoses" {
                (&key.diagnoses, &key.got.diagnoses)
            } else {
                (&key.medications, &key.got.medications)
            };
            for item in want {
                if !has(got, item) {
                    wrong.push(format!("{column}: missing {item:?}"));
                }
            }
            for cell in got {
                let cell_lower = cell.to_lowercase();
                if !want.iter().any(|w| cell_lower.contains(&w.to_lowercase())) {
                    wrong.push(format!("{column}: extra {cell:?}"));
                }
            }
        }
        _ => unreachable!("unknown column {column}"),
    }
    wrong
}

#[test]
fn gated_columns_are_exact() {
    let mut failures = Vec::new();
    for key in load() {
        for column in GATES {
            for m in mistakes(&key, column) {
                failures.push(format!("{}: {m}", key.id));
            }
        }
    }
    assert!(failures.is_empty(), "extraction is wrong on gated columns:\n{}", failures.join("\n"));
}

#[test]
fn nothing_from_must_not_list_reaches_the_table() {
    let mut failures = Vec::new();
    for key in load() {
        for banned in &key.must_not_list {
            if has(&key.got.diagnoses, banned) || has(&key.got.medications, banned) {
                failures.push(format!("{}: {banned:?} is in the table", key.id));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn no_identifier_reaches_the_table() {
    let mut failures = Vec::new();
    for key in load() {
        let cells: Vec<String> = key.got.diagnoses.iter().chain(&key.got.medications).cloned().collect();
        for pii in &key.planted {
            if has(&cells, pii) {
                failures.push(format!("{}: {pii:?} is in the table", key.id));
            }
        }
        for cell in &cells {
            assert!(key.deidentified.contains(cell.as_str()), "{}: {cell:?} is not in the de-identified text", key.id);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Prints, per column, how many documents are exactly right, and every
/// mistake. Run with `cargo test --test extraction_documents_test scoreboard -- --nocapture`.
#[test]
fn scoreboard() {
    let keys = load();
    let mut lines = Vec::new();
    for column in COLUMNS {
        let right = keys.iter().filter(|k| mistakes(k, column).is_empty()).count();
        lines.push(format!("{column:<12} {right}/{}", keys.len()));
    }
    let total: usize = COLUMNS
        .iter()
        .map(|c| keys.iter().filter(|k| mistakes(k, c).is_empty()).count())
        .sum();
    println!("\nWord and PDF documents, cells exactly right:\n{}\ntotal        {total}/{}", lines.join("\n"), keys.len() * COLUMNS.len());
    for key in &keys {
        for column in COLUMNS {
            for m in mistakes(key, column) {
                println!("  {}: {m}", key.id);
            }
        }
    }
}
