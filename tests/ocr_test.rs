//! Layer 1 check of reading scanned PDFs by OCR, against the answer keys in
//! `tests/documents/ocr/keys/`.
//!
//! A scan's key is the key of the document it is a scan of: pdf-06 is pdf-03
//! printed and scanned, so what must be removed and kept is the same. The
//! keys are the standard and are not changed to make a test pass. An
//! identifier OCR misreads counts as "not read", never as removed.
//!
//! The models are read from `web/public/models/`, the files the page serves.
#![cfg(feature = "ocr")]

use deid_server::{document, engine, ocr};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

struct Key {
    id: String,
    planted_pii: Vec<(String, String)>,
    must_keep: Vec<String>,
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn keys() -> Vec<Key> {
    let mut paths: Vec<_> = fs::read_dir(root().join("tests/documents/ocr/keys"))
        .expect("tests/documents/ocr/keys exists")
        .map(|e| e.expect("readable entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    paths.sort();
    paths
        .iter()
        .map(|path| {
            let v: Value = serde_json::from_str(&fs::read_to_string(path).expect("readable key"))
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let s = |v: &Value, k: &str| v[k].as_str().unwrap_or_else(|| panic!("{}: {k}", path.display())).to_string();
            Key {
                id: s(&v, "record_id"),
                planted_pii: v["planted_pii"].as_array().unwrap().iter().map(|p| (s(p, "type"), s(p, "text"))).collect(),
                must_keep: v["must_keep"].as_array().unwrap().iter().map(|k| k.as_str().unwrap().to_string()).collect(),
            }
        })
        .collect()
}

/// The text OCR reads from each scan, read once for all the tests.
fn read(id: &str) -> String {
    static TEXTS: OnceLock<Vec<(String, String)>> = OnceLock::new();
    let texts = TEXTS.get_or_init(|| {
        let model = |name: &str| fs::read(root().join("web/public/models").join(name)).expect("model file");
        let reader = ocr::Reader::new(model("text-detection.onnx"), model("text-recognition.onnx")).expect("models load");
        keys()
            .iter()
            .map(|k| {
                let pdf = fs::read(root().join(format!("tests/documents/pdf/{}.pdf", k.id))).expect("scan exists");
                (k.id.clone(), reader.text_from_scan(&pdf).unwrap_or_else(|e| panic!("{}: {e}", k.id)))
            })
            .collect()
    });
    texts.iter().find(|(k, _)| k == id).map(|(_, t)| t.clone()).expect("scan was read")
}

/// The text reader still refuses a scan: reading one by OCR is the visitor's
/// choice, never automatic.
#[test]
fn the_text_reader_still_refuses_a_scan() {
    for key in keys() {
        let pdf = fs::read(root().join(format!("tests/documents/pdf/{}.pdf", key.id))).unwrap();
        assert_eq!(document::text_from_pdf(&pdf), Err(document::DocumentError::Scanned), "{}", key.id);
    }
}

/// OCR must not lose or misread anything the key names, so that an identifier
/// cannot "pass" by being garbled.
#[test]
fn reading_a_scan_loses_nothing() {
    let mut missing = Vec::new();
    for key in keys() {
        let text = read(&key.id);
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
    assert!(missing.is_empty(), "OCR misread:\n{}", missing.join("\n"));
}

#[test]
fn medical_content_is_always_kept() {
    let mut lost = Vec::new();
    for key in keys() {
        let out = engine::process(&read(&key.id));
        lost.extend(key.must_keep.iter().filter(|k| !out.contains(k.as_str())).map(|k| format!("{}: \"{k}\"", key.id)));
    }
    assert!(lost.is_empty(), "medical content removed:\n{}", lost.join("\n"));
}

/// Every identifier in a scan's key is removed. Since 2026-11-26.
#[test]
fn every_identifier_is_removed() {
    let mut leaked = Vec::new();
    for key in keys() {
        let out = engine::process(&read(&key.id));
        leaked.extend(
            key.planted_pii.iter().filter(|(_, t)| out.contains(t.as_str())).map(|(k, t)| format!("{}: {k} \"{t}\"", key.id)),
        );
    }
    assert!(leaked.is_empty(), "identifiers left in:\n{}", leaked.join("\n"));
}

/// Run with: cargo test --test ocr_test scoreboard -- --nocapture
#[test]
fn ocr_scoreboard() {
    let (mut removed, mut planted, mut unread, mut kept, mut keepable) = (0, 0, 0, 0, 0);
    for key in keys() {
        let text = read(&key.id);
        let out = engine::process(&text);
        for (kind, pii) in &key.planted_pii {
            planted += 1;
            if !text.contains(pii.as_str()) {
                unread += 1;
                println!("  NOT READ {}  {kind}: \"{pii}\"", key.id);
            } else if out.contains(pii.as_str()) {
                println!("  leaked   {}  {kind}: \"{pii}\"", key.id);
            } else {
                removed += 1;
            }
        }
        for keep in &key.must_keep {
            keepable += 1;
            if out.contains(keep.as_str()) {
                kept += 1;
            } else {
                println!("  LOST     {}  medical: \"{keep}\"", key.id);
            }
        }
    }
    println!(
        "\ntests/documents/ocr: identifiers removed {removed}/{planted} (not read by OCR: {unread}), medical kept {kept}/{keepable}"
    );
}
