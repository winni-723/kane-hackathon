//! Layer 1 check of the research-table extraction against the answer keys in
//! `tests/extraction/`.
//!
//! The answer keys are the standard. Nothing here may be loosened to make a
//! test pass: the extraction changes, the keys and the gates do not shrink.

use deid_server::engine;
use deid_server::extract::{self, Day, Extracted};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

/// Ages worked out from a date of birth are counted up to this day, the one
/// the answer keys were written for.
const REFERENCE_DAY: Day = Day { year: 2026, month: 11, day: 14 };

/// (column, difficulty) pairs the extraction must get completely right on
/// every development record. This list only ever grows.
const GATES: &[(&str, &str)] = &[
    ("age", "easy"),
    ("age", "medium"),
    ("age", "hard"),
    ("sex", "easy"),
    ("sex", "medium"),
    ("diagnoses", "easy"),
    ("diagnoses", "medium"),
    ("medications", "easy"),
    ("medications", "medium"),
    ("medications", "hard"),
];

struct Key {
    id: String,
    difficulty: String,
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

fn load() -> Vec<Key> {
    load_from("tests/extraction", "tests/fixtures", 20)
}

fn load_from(key_dir: &str, record_dir: &str, expected: usize) -> Vec<Key> {
    let mut keys = Vec::new();
    let mut paths: Vec<_> = fs::read_dir(key_dir)
        .unwrap_or_else(|e| panic!("{key_dir}: {e}"))
        .map(|e| e.expect("readable entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    paths.sort();
    for path in paths {
        let key: Value = serde_json::from_str(&fs::read_to_string(&path).expect("readable key"))
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let id = key["record_id"].as_str().expect("record_id").to_string();
        let record: Value = serde_json::from_str(
            &fs::read_to_string(format!("{record_dir}/{id}.json"))
                .unwrap_or_else(|e| panic!("no record for extraction key {id}: {e}")),
        )
        .expect("record is valid JSON");
        let text = record["text"].as_str().expect("text");
        let deidentified = engine::process(text);
        let got = extract::extract(text, &deidentified, REFERENCE_DAY);
        keys.push(Key {
            difficulty: record["difficulty"].as_str().expect("difficulty").to_string(),
            age: opt(&key["age"]),
            age_source: opt(&key["age_source"]),
            sex: opt(&key["sex"]),
            diagnoses: strings(&key["diagnoses"]),
            medications: strings(&key["medications"]),
            must_not_list: strings(&key["must_not_list"]),
            planted: record["planted_pii"]
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
    assert_eq!(keys.len(), expected, "one extraction key per record in {record_dir}");
    keys
}

fn has(cells: &[String], wanted: &str) -> bool {
    let wanted = wanted.to_lowercase();
    cells.iter().any(|c| c.to_lowercase().contains(&wanted))
}

/// What one record got wrong in one column, as text for the failure message.
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
            // A cell that matches nothing in the key is a wrong entry.
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

const COLUMNS: [&str; 4] = ["age", "sex", "diagnoses", "medications"];

#[test]
fn gated_columns_are_exact() {
    let mut failures = Vec::new();
    for key in load() {
        for column in COLUMNS {
            if GATES.contains(&(column, key.difficulty.as_str())) {
                for m in mistakes(&key, column) {
                    failures.push(format!("{}: {m}", key.id));
                }
            }
        }
    }
    assert!(failures.is_empty(), "extraction is wrong on gated columns:\n{}", failures.join("\n"));
}

/// Things the record mentions that are not the patient's (ruled out, not
/// needed, family history) never reach the table. Not tied to a gate.
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

/// The table is built from de-identified text, so no planted identifier may
/// appear in any cell.
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

#[test]
fn extraction_scoreboard() {
    print_scoreboard("tests/extraction", &load());
}

/// Records the extraction rules were not written against. Measured, never
/// gated: a miss here is a number to report, not a failing test.
#[test]
fn heldout_extraction_scoreboard() {
    print_scoreboard(
        "tests/extraction-heldout",
        &load_from("tests/extraction-heldout", "tests/heldout", 10),
    );
}

fn print_scoreboard(title: &str, keys: &[Key]) {
    let mut board: BTreeMap<(&str, String), (u32, u32)> = BTreeMap::new();
    let mut details = Vec::new();
    for key in keys {
        for column in COLUMNS {
            let wrong = mistakes(key, column);
            let cell = board.entry((column, key.difficulty.clone())).or_default();
            cell.1 += 1;
            if wrong.is_empty() {
                cell.0 += 1;
            }
            for m in wrong {
                details.push(format!("  {}: {m}", key.id));
            }
        }
    }
    println!("\nResearch table, records with the whole column right ({title}):");
    println!("  {:<12} {:>6} {:>7} {:>6}", "column", "easy", "medium", "hard");
    let mut total = (0, 0);
    for column in COLUMNS {
        let cell = |d: &str| {
            let (ok, all) = board.get(&(column, d.to_string())).copied().unwrap_or_default();
            format!("{ok}/{all}")
        };
        println!("  {:<12} {:>6} {:>7} {:>6}", column, cell("easy"), cell("medium"), cell("hard"));
        for d in ["easy", "medium", "hard"] {
            let (ok, all) = board.get(&(column, d.to_string())).copied().unwrap_or_default();
            total = (total.0 + ok, total.1 + all);
        }
    }
    println!("  extraction: {}/{} cells right", total.0, total.1);
    if !details.is_empty() {
        println!("  wrong cells:\n{}", details.join("\n"));
    }
}
