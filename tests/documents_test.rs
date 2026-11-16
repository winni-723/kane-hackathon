//! Layer 1 check of uploaded documents against the answer keys in
//! `tests/documents/keys/`.
//!
//! Each document in `tests/documents/` is laid out like a real one: a
//! letterhead, a page footer repeating the patient's name, tables, a signature
//! block. Its text is read by `document`, de-identified by `engine`, and
//! compared with its answer key.
//!
//! The answer keys are the standard. The engine changes; the keys and the
//! gates do not shrink.

use deid_server::{document, engine};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// (identifier type, difficulty) pairs that must be removed completely.
/// This list only ever grows.
const GATES: &[(&str, &str)] = &[
    ("name", "easy"),
    ("name", "medium"),
    ("name", "hard"),
    ("relative_name", "easy"),
    ("relative_name", "medium"),
    ("relative_name", "hard"),
    ("provider_name", "easy"),
    ("provider_name", "medium"),
    ("provider_name", "hard"),
    ("facility_name", "easy"),
    ("facility_name", "medium"),
    ("facility_name", "hard"),
    ("facility_address", "easy"),
    ("facility_address", "medium"),
    ("facility_address", "hard"),
    ("facility_phone", "easy"),
    ("facility_phone", "medium"),
    ("facility_phone", "hard"),
    ("address", "easy"),
    ("address", "medium"),
    ("address", "hard"),
    ("phone", "easy"),
    ("phone", "medium"),
    ("phone", "hard"),
    ("dob", "easy"),
    ("dob", "medium"),
    ("dob", "hard"),
    ("service_date", "easy"),
    ("service_date", "medium"),
    ("service_date", "hard"),
    ("mrn", "easy"),
    ("mrn", "medium"),
    ("mrn", "hard"),
    ("email", "medium"),
    ("email", "hard"),
    ("insurance_id", "medium"),
    ("insurance_id", "hard"),
    ("ssn", "hard"),
    ("other_id", "easy"),
    ("other_id", "hard"),
];

struct Key {
    id: String,
    format: String,
    difficulty: String,
    planted_pii: Vec<(String, String)>,
    must_keep: Vec<String>,
}

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn load_keys() -> Vec<Key> {
    let dir = root().join("tests/documents/keys");
    let mut paths: Vec<_> = fs::read_dir(&dir)
        .expect("tests/documents/keys exists")
        .map(|e| e.expect("readable entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    paths.sort();
    paths
        .iter()
        .map(|path| {
            let raw = fs::read_to_string(path).expect("readable key");
            let v: Value =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let text = |v: &Value, key: &str| -> String {
                v[key]
                    .as_str()
                    .unwrap_or_else(|| panic!("{}: missing {key}", path.display()))
                    .to_string()
            };
            Key {
                id: text(&v, "record_id"),
                format: text(&v, "format"),
                difficulty: text(&v, "difficulty"),
                planted_pii: v["planted_pii"]
                    .as_array()
                    .expect("planted_pii is an array")
                    .iter()
                    .map(|p| (text(p, "type"), text(p, "text")))
                    .collect(),
                must_keep: v["must_keep"]
                    .as_array()
                    .expect("must_keep is an array")
                    .iter()
                    .map(|s| s.as_str().expect("must_keep entry is a string").to_string())
                    .collect(),
            }
        })
        .collect()
}

/// The plain text read out of a test document.
fn read_document(key: &Key) -> String {
    let path = root().join(format!("tests/documents/{}.{}", key.id, key.format));
    let bytes = fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    match key.format.as_str() {
        "docx" => document::text_from_docx(&bytes)
            .unwrap_or_else(|e| panic!("{}: {e}", path.display())),
        other => panic!("{}: no reader for format {other}", key.id),
    }
}

/// Reading a document must not lose anything: every string an answer key
/// names is in the text handed to the engine. Without this, an identifier
/// could "pass" simply because the reader dropped the part it was in.
#[test]
fn reading_a_document_loses_nothing() {
    let mut missing = Vec::new();
    for key in load_keys() {
        let text = read_document(&key);
        for (kind, pii) in &key.planted_pii {
            if !text.contains(pii.as_str()) {
                missing.push(format!("{}: {kind} \"{pii}\" not in the text read", key.id));
            }
        }
        for keep in &key.must_keep {
            if !text.contains(keep.as_str()) {
                missing.push(format!("{}: \"{keep}\" not in the text read", key.id));
            }
        }
    }
    assert!(missing.is_empty(), "the reader dropped text:\n{}", missing.join("\n"));
}

/// The engine must never remove medical content from a document.
#[test]
fn medical_content_is_always_kept() {
    let mut lost = Vec::new();
    for key in load_keys() {
        let out = engine::process(&read_document(&key));
        for keep in &key.must_keep {
            if !out.contains(keep.as_str()) {
                lost.push(format!("{}: lost \"{keep}\"", key.id));
            }
        }
    }
    assert!(lost.is_empty(), "medical content was removed:\n{}", lost.join("\n"));
}

/// Every gated kind of identifier is fully removed.
#[test]
fn gated_identifiers_are_removed() {
    let mut leaked = Vec::new();
    for key in load_keys() {
        let out = engine::process(&read_document(&key));
        for (kind, pii) in &key.planted_pii {
            let gated = GATES.iter().any(|(k, d)| k == kind && *d == key.difficulty);
            if gated && out.contains(pii.as_str()) {
                leaked.push(format!("{}: {kind} \"{pii}\" still present", key.id));
            }
        }
    }
    assert!(leaked.is_empty(), "gated identifiers leaked:\n{}", leaked.join("\n"));
}

/// Run with: cargo test --test documents_test scoreboard -- --nocapture
#[test]
fn documents_scoreboard() {
    let keys = load_keys();
    let mut by_kind: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let (mut removed, mut planted, mut kept, mut keepable) = (0, 0, 0, 0);
    let mut notes = Vec::new();

    for key in &keys {
        let out = engine::process(&read_document(key));
        for (kind, pii) in &key.planted_pii {
            let entry = by_kind.entry(kind.clone()).or_default();
            entry.1 += 1;
            planted += 1;
            if out.contains(pii.as_str()) {
                notes.push(format!("  leaked  {} ({})  {kind}: \"{pii}\"", key.id, key.difficulty));
            } else {
                entry.0 += 1;
                removed += 1;
            }
        }
        for keep in &key.must_keep {
            keepable += 1;
            if out.contains(keep.as_str()) {
                kept += 1;
            } else {
                notes.push(format!("  LOST    {} ({})  medical: \"{keep}\"", key.id, key.difficulty));
            }
        }
    }

    println!("\ntests/documents ({} documents)", keys.len());
    for (kind, (gone, total)) in &by_kind {
        println!("  {kind:<18} {gone:>3}/{total}");
    }
    println!("  identifiers removed {removed}/{planted}, medical kept {kept}/{keepable}");
    for note in notes {
        println!("{note}");
    }
}
