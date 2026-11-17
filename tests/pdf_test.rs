//! Layer 1 check of PDF uploads against the answer keys in
//! `tests/documents/pdf/keys/`.
//!
//! The PDFs are made the ways real ones are (Chrome printing a page, Word
//! saving as PDF, form software placing text, a scanner), because each lays
//! text out differently. Their text is read by `document::text_from_pdf`,
//! de-identified by `engine`, and compared with the answer key.
//!
//! The answer keys are the standard. The engine and the reader change; the
//! keys do not. An identifier the reader garbles counts as "not read", never
//! as removed.

use deid_server::{document, engine};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

struct Key {
    id: String,
    difficulty: String,
    planted_pii: Vec<(String, String)>,
    must_keep: Vec<String>,
}

fn dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/documents/pdf")
}

fn load_keys() -> Vec<Key> {
    let mut paths: Vec<_> = fs::read_dir(dir().join("keys"))
        .expect("tests/documents/pdf/keys exists")
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

fn read_pdf(id: &str) -> Result<String, document::DocumentError> {
    let path = dir().join(format!("{id}.pdf"));
    document::text_from_pdf(&fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
}

fn read(key: &Key) -> String {
    read_pdf(&key.id).unwrap_or_else(|e| panic!("{}: {e}", key.id))
}

/// Reading a PDF must not lose or garble anything: every string an answer key
/// names is in the text handed to the engine. Without this, an identifier
/// could "pass" because the reader broke it apart, not because it was removed.
#[test]
fn reading_a_pdf_loses_nothing() {
    let mut missing = Vec::new();
    for key in load_keys() {
        let text = read(&key);
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

/// The engine must never remove medical content from a PDF.
#[test]
fn medical_content_is_always_kept() {
    let mut lost = Vec::new();
    for key in load_keys() {
        let out = engine::process(&read(&key));
        for keep in &key.must_keep {
            if !out.contains(keep.as_str()) {
                lost.push(format!("{}: lost \"{keep}\"", key.id));
            }
        }
    }
    assert!(lost.is_empty(), "medical content was removed:\n{}", lost.join("\n"));
}

/// A scanned page holds no text. The site must say so, not show an empty
/// result that looks like a successful de-identification.
#[test]
fn a_scanned_pdf_is_refused() {
    assert_eq!(read_pdf("pdf-06"), Err(document::DocumentError::Scanned));
}

/// Run with: cargo test --test pdf_test scoreboard -- --nocapture
///
/// An identifier counts as removed only if the reader delivered it to the
/// engine and the engine took it out. One the reader garbled counts as
/// "not read", never as removed.
#[test]
fn pdf_scoreboard() {
    let keys = load_keys();
    let mut by_kind: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let (mut removed, mut planted, mut kept, mut keepable, mut unread) = (0, 0, 0, 0, 0);
    let mut notes = Vec::new();

    for key in &keys {
        let text = read(key);
        let out = engine::process(&text);
        for (kind, pii) in &key.planted_pii {
            let entry = by_kind.entry(kind.clone()).or_default();
            entry.1 += 1;
            planted += 1;
            if !text.contains(pii.as_str()) {
                unread += 1;
                notes.push(format!("  NOT READ {} ({})  {kind}: \"{pii}\"", key.id, key.difficulty));
            } else if out.contains(pii.as_str()) {
                notes.push(format!("  leaked   {} ({})  {kind}: \"{pii}\"", key.id, key.difficulty));
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
                notes.push(format!("  LOST     {} ({})  medical: \"{keep}\"", key.id, key.difficulty));
            }
        }
    }

    println!("\ntests/documents/pdf ({} documents)", keys.len());
    for (kind, (gone, total)) in &by_kind {
        println!("  {kind:<18} {gone:>3}/{total}");
    }
    println!(
        "  identifiers removed {removed}/{planted} (not read by the reader: {unread}), medical kept {kept}/{keepable}"
    );
    for note in notes {
        println!("{note}");
    }
}
