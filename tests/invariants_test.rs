//! Layer 1 privacy invariants: properties that must hold on every test
//! record, whatever its answer key says about individual identifiers.
//!
//! 1. De-identifying a de-identified record changes nothing: the engine's
//!    output holds nothing it would still replace.
//! 2. De-identifying keeps every line: nothing is dropped or merged.
//! 3. On the development records and documents, where every planted
//!    identifier is removed, the residual scan of the output finds nothing,
//!    and no planted identifier is left for it to find.
//!
//! The answer keys are the standard and are not changed here.

use deid_server::{document, engine, residual, utility};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn json_files(relative: &str) -> Vec<PathBuf> {
    let mut paths: Vec<_> = fs::read_dir(root().join(relative))
        .unwrap_or_else(|e| panic!("{relative}: {e}"))
        .map(|e| e.expect("readable entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    paths.sort();
    paths
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("readable")).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn planted(key: &Value) -> Vec<String> {
    key["planted_pii"]
        .as_array()
        .expect("planted_pii")
        .iter()
        .map(|p| p["text"].as_str().expect("pii text").to_string())
        .collect()
}

/// (id, records read from it, planted identifiers) for one set.
type Set = Vec<(String, Vec<String>, Vec<String>)>;

fn text_records(relative: &str) -> Set {
    json_files(relative)
        .iter()
        .map(|p| {
            let key = read_json(p);
            (
                key["record_id"].as_str().expect("record_id").to_string(),
                vec![key["text"].as_str().expect("text").to_string()],
                planted(&key),
            )
        })
        .collect()
}

fn documents() -> Set {
    let mut set = Set::new();
    for p in json_files("tests/documents/keys") {
        let id = p.file_stem().expect("stem").to_string_lossy().to_string();
        let bytes = |ext: &str| fs::read(root().join(format!("tests/documents/{id}.{ext}"))).expect("document exists");
        let records = if id.starts_with("csv") {
            document::records_from_csv(&bytes("csv")).expect("readable CSV")
        } else {
            vec![document::text_from_docx(&bytes("docx")).expect("readable document")]
        };
        set.push((id, records, planted(&read_json(&p))));
    }
    for p in json_files("tests/documents/pdf/keys") {
        let id = p.file_stem().expect("stem").to_string_lossy().to_string();
        let bytes = fs::read(root().join(format!("tests/documents/pdf/{id}.pdf"))).expect("PDF exists");
        set.push((id, vec![document::text_from_pdf(&bytes).expect("readable PDF")], planted(&read_json(&p))));
    }
    set
}

#[test]
fn de_identifying_twice_changes_nothing() {
    let mut changed = Vec::new();
    let sets = [
        text_records("tests/fixtures"),
        text_records("tests/heldout"),
        text_records("tests/synthea"),
        documents(),
    ];
    for (id, records, _) in sets.iter().flatten() {
        for (i, text) in records.iter().enumerate() {
            let once = engine::process(text);
            if engine::process(&once) != once {
                changed.push(format!("{id} record {}", i + 1));
            }
        }
    }
    assert!(changed.is_empty(), "a second pass changed the output of:\n{}", changed.join("\n"));
}

#[test]
fn every_line_is_kept() {
    let mut changed = Vec::new();
    let sets = [
        text_records("tests/fixtures"),
        text_records("tests/heldout"),
        text_records("tests/synthea"),
        documents(),
    ];
    for (id, records, _) in sets.iter().flatten() {
        for (i, text) in records.iter().enumerate() {
            if !utility::integrity(text, &engine::process(text)).lines_kept {
                changed.push(format!("{id} record {}", i + 1));
            }
        }
    }
    assert!(changed.is_empty(), "lines were dropped or merged in:
{}", changed.join("
"));
}

#[test]
fn nothing_is_left_on_the_development_records() {
    let mut left = Vec::new();
    for (id, records, planted) in text_records("tests/fixtures").iter().chain(&documents()) {
        let outputs: Vec<String> = records.iter().map(|t| engine::process(t)).collect();
        for p in planted {
            if outputs.iter().any(|o| o.contains(p.as_str())) {
                left.push(format!("{id}: planted {p:?} is still in the output"));
            }
        }
        for (i, out) in outputs.iter().enumerate() {
            for f in residual::scan(out) {
                left.push(format!("{id} record {}: residual {} on line {}", i + 1, f.category, f.line));
            }
        }
    }
    assert!(left.is_empty(), "{}", left.join("\n"));
}
