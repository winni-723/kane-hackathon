//! A second look at a record after de-identification.
//!
//! The engine replaces what its rules recognize. This scan reads the
//! de-identified text with different, looser patterns and lists anything that
//! still looks like an identifier, so a person can review it before release.
//! It finds shapes, not meanings: an empty list means nothing of these shapes
//! was found, not that every identifier is gone. A name written in lower case,
//! or one with no title or relation word before it, is not seen here.
//!
//! A finding carries where it is and what kind it may be, never the text
//! itself, so a report built from findings holds no identifier.

use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// The shape of a direct identifier: review before release.
    High,
    /// Could be an identifier or ordinary clinical text: check it.
    Medium,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    /// What it may be, e.g. "Phone-like number".
    pub category: &'static str,
    pub severity: Severity,
    /// Line in the de-identified text, counted from 1.
    pub line: usize,
    /// Byte range in the de-identified text, so the page can point at it.
    pub start: usize,
    pub end: usize,
    pub action: &'static str,
}

struct Rule {
    category: &'static str,
    severity: Severity,
    action: &'static str,
    pattern: LazyLock<Regex>,
}

const REMOVE: &str = "Remove or replace it before release.";
const CHECK: &str = "Check whether it identifies the patient; remove it if so.";

macro_rules! rule {
    ($category:expr, $severity:ident, $action:expr, $pattern:expr) => {
        Rule {
            category: $category,
            severity: Severity::$severity,
            action: $action,
            pattern: LazyLock::new(|| Regex::new($pattern).expect("residual pattern is valid")),
        }
    };
}

/// In priority order: where two matches overlap, the earlier rule's is kept.
static RULES: [Rule; 11] = [
    rule!("Email address", High, REMOVE, r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}"),
    rule!("Web address", High, REMOVE, r"\b(?:https?://|www\.)[^\s)]+"),
    rule!("SSN-like number", High, REMOVE, r"\b\d{3}-\d{2}-\d{4}\b"),
    rule!("Phone-like number", High, REMOVE, r"(?:\(\d{3}\)\s?|\b\d{3}[-. ]?)\d{3}[-. ]\d{4}\b"),
    rule!("Date", High, REMOVE, r"\b(?:\d{1,2}[/-]\d{1,2}[/-]\d{2,4}|\d{4}-\d{2}-\d{2})\b"),
    rule!("ID-like number", High, REMOVE, r"\b[A-Z]{0,4}-?\d{6,}[A-Z]?\b"),
    rule!("Name after a title", High, REMOVE, r"\b(?:Mr|Mrs|Ms|Miss|Dr)\.? [A-Z][a-z]+"),
    rule!(
        "Date",
        Medium,
        CHECK,
        r"\b(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Sept|Oct|Nov|Dec)[a-z]*\.? \d{1,2}(?:st|nd|rd|th)?\b"
    ),
    rule!("Possible ZIP code", Medium, CHECK, r"\b\d{5}(?:-\d{4})?\b"),
    rule!(
        "Possible relative's name",
        Medium,
        CHECK,
        r"\b(?:wife|husband|son|daughter|mother|father|sister|brother|spouse|partner|friend|neighbor|niece|nephew|aunt|uncle|grandson|granddaughter|caregiver),? [A-Z][a-z]+"
    ),
    rule!("Possible name", Medium, CHECK, r"\b(?:patient|named|called) [A-Z][a-z]+ [A-Z][a-z]+"),
];

/// Matches that are ordinary clinical text after all: the digits of a
/// decimal dose ("0.000813 MG/HR"), an FDA application number ("NDA020800"),
/// or a relation word followed by a title whose name is already a placeholder
/// ("his son, Mr. [NAME]").
fn not_an_identifier(text: &str, start: usize, end: usize) -> bool {
    let found = &text[start..end];
    let before = text[..start].chars().next_back();
    let starts_with_digit = found.starts_with(|c: char| c.is_ascii_digit());
    (starts_with_digit && before.is_some_and(|c| c == '.' || c.is_ascii_digit()))
        || ["NDA", "ANDA", "BLA"].iter().any(|p| found.strip_prefix(p).is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit())))
        || found.rsplit(' ').next().is_some_and(|last| ["Mr", "Mrs", "Ms", "Miss", "Dr"].contains(&last))
}

/// Lists what in `deidentified` still looks like an identifier, in text order.
pub fn scan(deidentified: &str) -> Vec<Finding> {
    let mut taken: Vec<(usize, usize)> = Vec::new();
    let mut findings = Vec::new();
    for rule in &RULES {
        for m in rule.pattern.find_iter(deidentified) {
            let (start, end) = (m.start(), m.end());
            if not_an_identifier(deidentified, start, end) || taken.iter().any(|&(s, e)| start < e && s < end) {
                continue;
            }
            taken.push((start, end));
            findings.push(Finding {
                category: rule.category,
                severity: rule.severity,
                line: deidentified[..start].matches('\n').count() + 1,
                start,
                end,
                action: rule.action,
            });
        }
    }
    findings.sort_by_key(|f| f.start);
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(text: &str) -> Vec<(&'static str, &str)> {
        scan(text).iter().map(|f| (f.category, &text[f.start..f.end])).collect()
    }

    #[test]
    fn identifier_shapes_are_found() {
        let text = "Call (617) 555-0142 or write to jo@example.com.\nSSN 987-65-4321, seen 03/14/2026.\nDr. Okafor and his son Mark visited.";
        assert_eq!(
            found(text),
            vec![
                ("Phone-like number", "(617) 555-0142"),
                ("Email address", "jo@example.com"),
                ("SSN-like number", "987-65-4321"),
                ("Date", "03/14/2026"),
                ("Name after a title", "Dr. Okafor"),
                ("Possible relative's name", "son Mark"),
            ]
        );
    }

    #[test]
    fn line_numbers_count_from_one() {
        let findings = scan("first\nsecond 02199\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].line, 2);
        assert_eq!(findings[0].severity, Severity::Medium);
    }

    #[test]
    fn placeholders_and_clinical_text_are_not_findings() {
        let text = "Name: [NAME]\nPhone: [PHONE]\nDOB: [DATE]\nDr. [NAME] saw the patient.\n\
                    BP 128/82, HR 76. Metformin 500 mg twice daily; 1/2 tablet at night.\n\
                    Hemoglobin A1c 7.2%. Follow-up in 3 months. Type 2 diabetes mellitus.";
        assert_eq!(found(text), vec![]);
    }

    #[test]
    fn overlapping_matches_are_reported_once() {
        // The phone rule is ahead of the ID rule, so this is one phone.
        assert_eq!(found("617-555-0142"), vec![("Phone-like number", "617-555-0142")]);
        assert_eq!(found("Drivers: S99954805"), vec![("ID-like number", "S99954805")]);
        assert_eq!(found("Born 1971-01-24"), vec![("Date", "1971-01-24")]);
    }

    #[test]
    fn doses_drug_codes_and_titles_before_a_placeholder_are_not_findings() {
        assert_eq!(found("levonorgestrel 0.000813 MG/HR"), vec![]);
        assert_eq!(found("NDA020800 0.3 ML Epinephrine"), vec![]);
        assert_eq!(found("her son, Mr. [NAME], and her niece, Dr. [NAME]"), vec![]);
        assert_eq!(found("Patient Portal Message"), vec![]);
    }

    #[test]
    fn a_finding_does_not_carry_the_text() {
        let json = serde_json::to_string(&scan("Phone 617-555-0142")).expect("serializes");
        assert!(!json.contains("555-0142"), "{json}");
    }
}
