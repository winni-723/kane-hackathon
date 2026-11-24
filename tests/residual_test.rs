//! Layer 1 measurement of the residual scan (`src/residual.rs`) against the
//! answer keys: of the identifiers the engine left in, how many the scan
//! points at, and how often it points at something that is not one.
//!
//! The answer keys are the standard and are not changed here.

use deid_server::{engine, residual};
use serde_json::Value;
use std::fs;
use std::path::Path;

struct Record {
    id: String,
    text: String,
    planted: Vec<String>,
    must_keep: Vec<String>,
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(|s| s.as_str().map(str::to_string)).collect()).unwrap_or_default()
}

fn load_dir(relative: &str) -> Vec<Record> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    let mut paths: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{relative}: {e}"))
        .map(|e| e.expect("readable entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    paths.sort();
    paths
        .iter()
        .map(|path| {
            let v: Value = serde_json::from_str(&fs::read_to_string(path).expect("readable"))
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            Record {
                id: v["record_id"].as_str().expect("record_id").to_string(),
                text: v["text"].as_str().expect("text").to_string(),
                planted: v["planted_pii"]
                    .as_array()
                    .expect("planted_pii")
                    .iter()
                    .map(|p| p["text"].as_str().expect("pii text").to_string())
                    .collect(),
                must_keep: strings(&v["must_keep"]),
            }
        })
        .collect()
}

/// Byte ranges of every occurrence of `needle` in `hay`.
fn occurrences(hay: &str, needle: &str) -> Vec<(usize, usize)> {
    hay.match_indices(needle).map(|(i, s)| (i, i + s.len())).collect()
}

fn overlaps(a: (usize, usize), b: (usize, usize)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

#[derive(Default)]
struct Tally {
    records: usize,
    left_in: usize,
    pointed_at: usize,
    findings: usize,
    false_alarms: usize,
    medical_flagged: usize,
    details: Vec<String>,
}

fn tally(records: &[Record]) -> Tally {
    let mut t = Tally { records: records.len(), ..Tally::default() };
    for r in records {
        let out = engine::process(&r.text);
        let findings = residual::scan(&out);
        t.findings += findings.len();
        let left: Vec<&String> = r.planted.iter().filter(|p| out.contains(p.as_str())).collect();
        let mut pii_spans = Vec::new();
        for p in &left {
            t.left_in += 1;
            let spans = occurrences(&out, p);
            if spans.iter().any(|&s| findings.iter().any(|f| overlaps(s, (f.start, f.end)))) {
                t.pointed_at += 1;
            } else {
                t.details.push(format!("{}: not pointed at {p:?}", r.id));
            }
            pii_spans.extend(spans);
        }
        let keep_spans: Vec<(usize, usize)> = r.must_keep.iter().flat_map(|k| occurrences(&out, k)).collect();
        for f in &findings {
            let span = (f.start, f.end);
            if !pii_spans.iter().any(|&s| overlaps(s, span)) {
                t.false_alarms += 1;
                t.details.push(format!("{}: false alarm {} {:?}", r.id, f.category, &out[f.start..f.end]));
            }
            if keep_spans.iter().any(|&s| overlaps(s, span)) {
                t.medical_flagged += 1;
                t.details.push(format!("{}: medical text flagged {:?}", r.id, &out[f.start..f.end]));
            }
        }
    }
    t
}

fn print(name: &str, t: &Tally, with_details: bool) {
    println!(
        "{name:<32} records {:>3}  left in by the engine {:>3}, pointed at {:>3}  findings {:>3}, false alarms {:>3}, on medical text {:>2}",
        t.records, t.left_in, t.pointed_at, t.findings, t.false_alarms, t.medical_flagged
    );
    if with_details {
        for d in &t.details {
            println!("    {d}");
        }
    }
}

fn synthea_number(id: &str) -> u32 {
    id.rsplit('s').next().and_then(|n| n.parse().ok()).expect("synthea id ends in sNN")
}

/// The scan never points at the medical content the answer keys say to keep.
#[test]
fn medical_content_is_never_flagged() {
    for dir in ["tests/fixtures", "tests/heldout"] {
        let t = tally(&load_dir(dir));
        assert_eq!(t.medical_flagged, 0, "{dir}:\n{}", t.details.join("\n"));
    }
}

/// Run with `cargo test --test residual_test scoreboard -- --nocapture`.
/// The untouched half of Synthea (s30 onward) is shown as totals only.
#[test]
fn scoreboard() {
    println!();
    print("tests/fixtures", &tally(&load_dir("tests/fixtures")), true);
    print("tests/heldout", &tally(&load_dir("tests/heldout")), true);
    let (tuning, untouched): (Vec<Record>, Vec<Record>) =
        load_dir("tests/synthea").into_iter().partition(|r| synthea_number(&r.id) <= 29);
    print("tests/synthea tuning half", &tally(&tuning), true);
    print("tests/synthea untouched half", &tally(&untouched), false);
}
