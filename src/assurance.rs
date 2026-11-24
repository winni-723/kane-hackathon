//! The assurance report for one set of records: what was found and replaced,
//! what still looks like an identifier, the re-identification risk left, and
//! how much research value remains, kept as separate sections.
//!
//! The report holds counts, categories and line numbers, never record text,
//! so it can be shared without the records. It carries no overall privacy
//! score, and its review status starts as not reviewed: a person decides on
//! release. Software test results are not evidence about a dataset and are
//! not part of it.

use crate::extract::{self, Day, Extracted};
use crate::{engine, privacy, residual, utility};
use regex::Regex;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::LazyLock;

pub const REPORT_FORMAT: &str = "deid-assurance-report/1";

/// The replacement rules in `src/engine.rs` and `src/names.rs`. Raise it
/// whenever those rules change what they replace.
pub const RULES_VERSION: &str = "2026-11-24";

pub const DISCLAIMER: &str = "This report describes automated checks on this set of records. \
It is not a HIPAA compliance determination or an expert determination. \
A person must review the records and the findings before release.";

const RESIDUAL_NOTE: &str = "An empty list means nothing of these shapes was found, not that every identifier is gone. \
Names written in lower case, or with no title or relation word before them, are not seen by this scan.";

#[derive(Debug, Serialize)]
pub struct Report {
    pub report_format: &'static str,
    pub generated_on: String,
    pub engine: EngineInfo,
    pub dataset: DatasetInfo,
    /// Identifiers replaced, by placeholder, over all records.
    pub replaced: BTreeMap<String, usize>,
    pub residual: Residual,
    pub privacy: Privacy,
    pub utility: Utility,
    pub review: Review,
    pub disclaimer: &'static str,
}

#[derive(Debug, Serialize)]
pub struct EngineInfo {
    pub version: &'static str,
    pub rules_version: &'static str,
    pub method: &'static str,
}

#[derive(Debug, Serialize)]
pub struct DatasetInfo {
    pub records: usize,
}

#[derive(Debug, Serialize)]
pub struct Located<T> {
    /// Record number in the set, counted from 1.
    pub record: usize,
    #[serde(flatten)]
    pub item: T,
}

#[derive(Debug, Serialize)]
pub struct Residual {
    pub high: usize,
    pub medium: usize,
    pub findings: Vec<Located<ResidualFinding>>,
    pub note: &'static str,
}

/// A residual finding without its byte range: line numbers are enough to
/// review it, and the report stays free of anything tied to the text.
#[derive(Debug, Serialize)]
pub struct ResidualFinding {
    pub category: &'static str,
    pub severity: residual::Severity,
    pub line: usize,
    pub action: &'static str,
}

#[derive(Debug, Serialize)]
pub struct Privacy {
    pub quasi_identifiers: Vec<Located<privacy::QuasiIdentifier>>,
    /// Only for a set of more than one record.
    pub group_sizes: Option<privacy::GroupSizes>,
    pub assumptions: &'static [&'static str],
}

#[derive(Debug, Serialize)]
pub struct Utility {
    pub completeness: utility::Completeness,
    pub records_with_every_line_kept: usize,
    /// The lowest share of original text kept in any record, as a percent.
    pub lowest_text_kept_percent: u32,
}

#[derive(Debug, Serialize)]
pub struct Review {
    pub status: &'static str,
    pub release: &'static str,
}

static PLACEHOLDER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([A-Z_]+)\]").expect("placeholder pattern is valid"));

/// De-identifies `originals` and reports on the result. `today` is the day
/// ages from a date of birth are counted to and the report's date.
pub fn report(originals: &[String], today: Day) -> Report {
    let outputs: Vec<String> = originals.iter().map(|t| engine::process(t)).collect();
    let rows: Vec<Extracted> = originals.iter().zip(&outputs).map(|(o, d)| extract::extract(o, d, today)).collect();

    let mut replaced = BTreeMap::new();
    let mut findings = Vec::new();
    let mut quasi = Vec::new();
    let mut lines_kept = 0;
    let mut lowest_kept = 100;
    for (i, ((original, out), row)) in originals.iter().zip(&outputs).zip(&rows).enumerate() {
        let record = i + 1;
        for caps in PLACEHOLDER.captures_iter(out) {
            *replaced.entry(caps[1].to_string()).or_insert(0) += 1;
        }
        for f in residual::scan(out) {
            let item = ResidualFinding { category: f.category, severity: f.severity, line: f.line, action: f.action };
            findings.push(Located { record, item });
        }
        for q in privacy::assess(out, row).quasi_identifiers {
            quasi.push(Located { record, item: q });
        }
        let integrity = utility::integrity(original, out);
        lines_kept += usize::from(integrity.lines_kept);
        lowest_kept = lowest_kept.min(integrity.text_kept_percent);
    }
    let count = |s: residual::Severity| findings.iter().filter(|f| f.item.severity == s).count();

    Report {
        report_format: REPORT_FORMAT,
        generated_on: format!("{:04}-{:02}-{:02}", today.year, today.month, today.day),
        engine: EngineInfo {
            version: env!("CARGO_PKG_VERSION"),
            rules_version: RULES_VERSION,
            method: "Each identifier is replaced by a placeholder naming its category, e.g. [NAME].",
        },
        dataset: DatasetInfo { records: originals.len() },
        replaced,
        residual: Residual { high: count(residual::Severity::High), medium: count(residual::Severity::Medium), findings, note: RESIDUAL_NOTE },
        privacy: Privacy {
            quasi_identifiers: quasi,
            group_sizes: (originals.len() > 1).then(|| privacy::group_sizes(&rows)),
            assumptions: privacy::ASSUMPTIONS,
        },
        utility: Utility {
            completeness: utility::completeness(&rows),
            records_with_every_line_kept: lines_kept,
            lowest_text_kept_percent: lowest_kept,
        },
        review: Review { status: "not reviewed", release: "not approved" },
        disclaimer: DISCLAIMER,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: Day = Day { year: 2026, month: 11, day: 24 };

    fn record() -> String {
        "Name: Eleanor Carter\nDOB: 03/14/1958\nPhone: (617) 555-0142\nAddress: 48 Willow Bend Road, Westmarch, MA 02199\n\
         Diagnosis: Congestive heart failure.\nMedications: Furosemide 40 mg daily."
            .to_string()
    }

    #[test]
    fn a_report_counts_what_was_replaced() {
        let r = report(&[record()], DAY);
        assert_eq!(r.dataset.records, 1);
        assert_eq!(r.replaced.get("NAME"), Some(&1));
        assert_eq!(r.replaced.get("PHONE"), Some(&1));
        assert_eq!(r.residual.high, 0);
        assert_eq!(r.review.status, "not reviewed");
        assert!(r.privacy.group_sizes.is_none(), "one record has no groups to count");
        assert_eq!(r.utility.completeness.rows, 1);
        assert_eq!(r.utility.records_with_every_line_kept, 1);
        assert_eq!(r.generated_on, "2026-11-24");
    }

    #[test]
    fn a_report_holds_no_record_text() {
        let json = serde_json::to_string(&report(&[record(), record()], DAY)).expect("serializes");
        for identifier in ["Eleanor", "Carter", "03/14/1958", "555-0142", "Willow Bend", "Westmarch", "02199"] {
            assert!(!json.contains(identifier), "{identifier} is in the report");
        }
        // Nor the medical content: the report is about the records, not a copy.
        assert!(!json.contains("Furosemide"));
    }

    #[test]
    fn residual_findings_are_reported_by_record_and_line() {
        let r = report(&["Seen today.".to_string(), "Seen today.\nSee www.example.org/p for the portal.".to_string()], DAY);
        assert_eq!((r.residual.high, r.residual.medium), (1, 0));
        assert_eq!(r.residual.findings.len(), 1);
        for f in &r.residual.findings {
            assert_eq!(f.record, 2);
            assert_eq!(f.item.line, 2);
        }
        assert!(r.privacy.group_sizes.is_some());
    }
}
