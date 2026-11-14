//! Layer 1 check of the engine against the answer keys in `tests/fixtures/`.
//!
//! The answer keys are the standard. Nothing here may be loosened to make a
//! test pass: the engine changes, the fixtures and the gates do not shrink.

use deid_server::engine;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// PII kinds the engine is required to remove completely, per difficulty.
/// Every planted item of a gated (type, difficulty) pair must be gone from the
/// output of every record of that difficulty.
///
/// This list only ever grows. Add a pair when the engine handles it; never
/// remove one.
const GATES: &[(&str, &str)] = &[
    ("phone", "easy"),
    ("phone", "medium"),
    ("email", "easy"),
    ("email", "medium"),
    ("email", "hard"),
    ("ssn", "easy"),
    ("ssn", "medium"),
    ("ssn", "hard"),
    ("mrn", "easy"),
    ("mrn", "medium"),
    ("mrn", "hard"),
    ("insurance_id", "easy"),
    ("dob", "easy"),
    ("dob", "medium"),
    ("dob", "hard"),
    ("address", "easy"),
    ("address", "medium"),
    ("address", "hard"),
    ("phone", "hard"),
    ("name", "easy"),
    ("name", "medium"),
    ("name", "hard"),
    ("relative_name", "easy"),
    ("relative_name", "medium"),
    ("relative_name", "hard"),
];

struct Pii {
    kind: String,
    text: String,
}

struct Fixture {
    id: String,
    difficulty: String,
    text: String,
    planted_pii: Vec<Pii>,
    must_keep: Vec<String>,
}

fn field<'a>(v: &'a Value, key: &str, file: &Path) -> &'a str {
    v[key]
        .as_str()
        .unwrap_or_else(|| panic!("{}: missing string field {key}", file.display()))
}

fn load_dir(relative: &str) -> Vec<Fixture> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    let mut paths: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{relative}: {e}"))
        .map(|e| e.expect("readable entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    paths.sort();

    let fixtures: Vec<Fixture> = paths
        .iter()
        .map(|path| {
            let raw = fs::read_to_string(path).expect("readable fixture");
            let v: Value = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            Fixture {
                id: field(&v, "record_id", path).to_string(),
                difficulty: field(&v, "difficulty", path).to_string(),
                text: field(&v, "text", path).to_string(),
                planted_pii: v["planted_pii"]
                    .as_array()
                    .expect("planted_pii is an array")
                    .iter()
                    .map(|p| Pii {
                        kind: field(p, "type", path).to_string(),
                        text: field(p, "text", path).to_string(),
                    })
                    .collect(),
                must_keep: v["must_keep"]
                    .as_array()
                    .expect("must_keep is an array")
                    .iter()
                    .map(|s| s.as_str().expect("must_keep entry is a string").to_string())
                    .collect(),
            }
        })
        .collect();

    fixtures
}

/// The 20 records the engine is developed against and gated on.
fn load_fixtures() -> Vec<Fixture> {
    let fixtures = load_dir("tests/fixtures");
    assert_eq!(fixtures.len(), 20, "expected 20 fixtures");
    fixtures
}

/// The engine must never remove medical content, on any record, at any stage.
#[test]
fn medical_content_is_always_kept() {
    let mut lost = Vec::new();
    for f in load_fixtures() {
        let out = engine::process(&f.text);
        for keep in &f.must_keep {
            if !out.contains(keep.as_str()) {
                lost.push(format!("{}: lost \"{keep}\"", f.id));
            }
        }
    }
    assert!(lost.is_empty(), "medical content was removed:\n{}", lost.join("\n"));
}

/// Every gated kind of PII is fully removed.
#[test]
fn gated_pii_is_removed() {
    let mut leaked = Vec::new();
    for f in load_fixtures() {
        let out = engine::process(&f.text);
        for pii in &f.planted_pii {
            let gated = GATES
                .iter()
                .any(|(kind, difficulty)| *kind == pii.kind && *difficulty == f.difficulty);
            if gated && out.contains(pii.text.as_str()) {
                leaked.push(format!("{}: {} \"{}\" still present", f.id, pii.kind, pii.text));
            }
        }
    }
    assert!(leaked.is_empty(), "gated PII leaked:\n{}", leaked.join("\n"));
}

/// Prints how much planted PII is removed, by kind and difficulty.
/// Run with: cargo test --test fixtures_test scoreboard -- --nocapture
#[test]
fn scoreboard() {
    print_scoreboard("tests/fixtures (development set)", &load_fixtures(), true);
    println!("gated: {GATES:?}");
}

/// Prints the same table for records the engine was NOT developed against.
/// Nothing here is asserted: these sets measure, they do not gate.
/// Run with: cargo test --test fixtures_test heldout_scoreboard -- --nocapture
#[test]
fn heldout_scoreboard() {
    let exists = |dir: &str| Path::new(env!("CARGO_MANIFEST_DIR")).join(dir).is_dir();
    if exists("tests/heldout") {
        print_scoreboard("tests/heldout", &load_dir("tests/heldout"), true);
    }
    if exists("tests/synthea") {
        // The engine may be tuned against the first half. The second half is
        // never inspected record by record: only its totals are printed, so it
        // stays an honest measure of how the rules generalize.
        let (tuning, untouched): (Vec<Fixture>, Vec<Fixture>) = load_dir("tests/synthea")
            .into_iter()
            .partition(|f| synthea_number(&f.id) <= SYNTHEA_TUNING_LAST);
        print_scoreboard("tests/synthea tuning half", &tuning, true);
        print_scoreboard("tests/synthea untouched half", &untouched, false);
    }
}

/// Synthea records `easy-s01` to `easy-s29` may be used to tune the engine;
/// `easy-s30` onward are the untouched half.
const SYNTHEA_TUNING_LAST: u32 = 29;

fn synthea_number(id: &str) -> u32 {
    id.rsplit('s').next().and_then(|n| n.parse().ok()).unwrap_or(0)
}

fn print_scoreboard(title: &str, fixtures: &[Fixture], details: bool) {
    // (kind, difficulty) -> (removed, total)
    let mut table: BTreeMap<(String, String), (usize, usize)> = BTreeMap::new();
    let mut remaining = Vec::new();
    let mut lost = Vec::new();
    let mut keep_total = 0;
    for f in fixtures {
        let out = engine::process(&f.text);
        for keep in &f.must_keep {
            keep_total += 1;
            if !out.contains(keep.as_str()) {
                lost.push(format!("{} \"{keep}\"", f.id));
            }
        }
        for pii in &f.planted_pii {
            let cell = table
                .entry((pii.kind.clone(), f.difficulty.clone()))
                .or_insert((0, 0));
            cell.1 += 1;
            if out.contains(pii.text.as_str()) {
                remaining.push(format!("{} {} \"{}\"", f.id, pii.kind, pii.text));
            } else {
                cell.0 += 1;
            }
        }
    }

    println!("\n== {title}: {} records", fixtures.len());
    println!("PII removed, by kind and difficulty (removed/total):");
    println!("{:<14} {:>8} {:>8} {:>8}", "kind", "easy", "medium", "hard");
    let kinds: Vec<String> = {
        let mut k: Vec<String> = table.keys().map(|(kind, _)| kind.clone()).collect();
        k.dedup();
        k
    };
    let (mut removed_all, mut total_all) = (0, 0);
    for kind in &kinds {
        let cell = |d: &str| match table.get(&(kind.clone(), d.to_string())) {
            Some((r, t)) => format!("{r}/{t}"),
            None => "-".to_string(),
        };
        println!("{:<14} {:>8} {:>8} {:>8}", kind, cell("easy"), cell("medium"), cell("hard"));
    }
    for (r, t) in table.values() {
        removed_all += r;
        total_all += t;
    }
    println!("total removed: {removed_all}/{total_all}");
    println!("medical content kept: {}/{keep_total}", keep_total - lost.len());
    if !lost.is_empty() {
        println!("\nmedical content LOST ({}):", lost.len());
        for line in &lost {
            println!("  {line}");
        }
    }
    if !remaining.is_empty() && !details {
        println!("\nstill present: {} (not listed for this set)", remaining.len());
    }
    if !remaining.is_empty() && details {
        println!("\nstill present ({}):", remaining.len());
        for line in &remaining {
            println!("  {line}");
        }
    }
}
