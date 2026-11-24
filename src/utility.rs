//! How much research value is left after de-identification.
//!
//! Integrity: each record still has its lines and its non-identifying text.
//! Completeness: how many rows of the research table have each column.
//! These are measured without an answer key, so they show what was kept,
//! not whether every medical fact survived; the answer-key tests in `tests/`
//! check that.

use crate::extract::Extracted;
use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;

static PLACEHOLDER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[[A-Z_]+\]").expect("placeholder pattern is valid"));

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Integrity {
    /// Same number of lines before and after: nothing was dropped or merged.
    pub lines_kept: bool,
    /// Placeholders put in the text.
    pub placeholders: usize,
    /// Share of the original characters still in the text, outside
    /// placeholders, as a whole percent.
    pub text_kept_percent: u32,
}

pub fn integrity(original: &str, deidentified: &str) -> Integrity {
    let placeholders: Vec<_> = PLACEHOLDER.find_iter(deidentified).collect();
    let placeholder_chars: usize = placeholders.iter().map(|m| m.as_str().chars().count()).sum();
    let kept = deidentified.chars().count().saturating_sub(placeholder_chars);
    let total = original.chars().count();
    Integrity {
        lines_kept: original.lines().count() == deidentified.lines().count(),
        placeholders: placeholders.len(),
        text_kept_percent: (kept * 100).checked_div(total).map_or(100, |p| p.min(100) as u32),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Completeness {
    pub rows: usize,
    pub age: usize,
    pub sex: usize,
    pub diagnoses: usize,
    pub medications: usize,
}

/// Rows of the research table that have a value in each column.
pub fn completeness(rows: &[Extracted]) -> Completeness {
    Completeness {
        rows: rows.len(),
        age: rows.iter().filter(|r| r.age.is_some()).count(),
        sex: rows.iter().filter(|r| r.sex.is_some()).count(),
        diagnoses: rows.iter().filter(|r| !r.diagnoses.is_empty()).count(),
        medications: rows.iter().filter(|r| !r.medications.is_empty()).count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrity_counts_placeholders_and_kept_text() {
        let original = "Name: Eleanor Carter\nDiagnosis: CHF";
        let out = "Name: [NAME]\nDiagnosis: CHF";
        let i = integrity(original, out);
        assert!(i.lines_kept);
        assert_eq!(i.placeholders, 1);
        // 21 of 35 characters remain outside the placeholder.
        assert_eq!(i.text_kept_percent, 60);
    }

    #[test]
    fn a_dropped_line_is_seen() {
        assert!(!integrity("a\nb\nc", "a\nc").lines_kept);
        assert_eq!(integrity("", "").text_kept_percent, 100);
    }

    #[test]
    fn completeness_counts_filled_columns() {
        let row = |age: Option<&str>, meds: &[&str]| Extracted {
            age: age.map(str::to_string),
            age_source: None,
            sex: Some("F"),
            diagnoses: vec![],
            medications: meds.iter().map(|m| m.to_string()).collect(),
        };
        let c = completeness(&[row(Some("40"), &["Metformin 500 mg"]), row(None, &[])]);
        assert_eq!(c, Completeness { rows: 2, age: 1, sex: 2, diagnoses: 0, medications: 1 });
    }
}
