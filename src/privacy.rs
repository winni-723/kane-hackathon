//! Re-identification risk that is left after de-identification.
//!
//! Residual findings (`src/residual.rs`) are things that may be identifiers
//! themselves. This module looks at quasi-identifiers: facts that identify
//! no one alone but can narrow a person down in combination, such as a very
//! high age, a state, or race and ethnicity. For a set of records it also
//! counts how many records share each age band and sex in the research table.
//!
//! This is a heuristic review aid. It is not an expert determination and not
//! a HIPAA compliance decision; the assumptions it rests on are listed in
//! [`ASSUMPTIONS`] and travel with every assessment.

use crate::extract::Extracted;
use regex::Regex;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::LazyLock;

pub const ASSUMPTIONS: &[&str] = &[
    "Quasi-identifiers are found by fixed patterns in English, US-format text; others (occupation, rare conditions, events) are not looked for.",
    "Group sizes use only the research table's age band (10 years) and sex, counted within this set of records, not within any wider population.",
    "No outside data is consulted, so linkage with other datasets is not assessed.",
    "This is a review aid, not an expert determination or a HIPAA compliance decision.",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuasiIdentifier {
    pub category: &'static str,
    /// `"review"`: Safe Harbor asks for a change. `"note"`: allowed, but it
    /// narrows the group a record belongs to.
    pub level: &'static str,
    /// Line in the de-identified text, counted from 1; 0 for a table value.
    pub line: usize,
    pub action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Assessment {
    pub quasi_identifiers: Vec<QuasiIdentifier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GroupSizes {
    /// Records counted: those with both an age and a sex in the table.
    pub records_counted: usize,
    /// Size of the smallest group sharing an age band and sex (k).
    pub smallest_group: Option<usize>,
    /// Records alone in their group.
    pub records_alone: usize,
}

const OVER_89: &str = "Ages over 89 are to be reported as 90 or older under HIPAA Safe Harbor.";

static AGE_OVER_89: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:9\d|1[0-2]\d)(?:[- ]years?[- ]old|[- ]?yo\b|[- ]?y/o\b|\s*(?:years|yrs)\b)|\bage[ \t]*:?[ \t]*(?:9\d|1[0-2]\d)\b")
        .expect("age pattern is valid")
});

static STATE_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?mi)^[ \t]*state[ \t]*:").expect("state pattern is valid"));

static RACE_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?mi)^[ \t]*(?:race|ethnicity)[ \t]*:").expect("race pattern is valid"));

/// Two-letter state after a masked town, e.g. "[CITY], MA".
static STATE_AFTER_CITY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[CITY\], [A-Z]{2}\b").expect("city-state pattern is valid"));

fn line_of(text: &str, at: usize) -> usize {
    text[..at].matches('\n').count() + 1
}

/// Quasi-identifiers in one de-identified record and its research-table row.
pub fn assess(deidentified: &str, extracted: &Extracted) -> Assessment {
    let mut found = Vec::new();
    for m in AGE_OVER_89.find_iter(deidentified) {
        found.push(QuasiIdentifier { category: "Age over 89", level: "review", line: line_of(deidentified, m.start()), action: OVER_89 });
    }
    // The table shows 90+ for an age the record stated; if the text does not
    // say 90+ too, the exact age was written in a shape the engine leaves
    // (e.g. "94F"). An age from a date of birth needs no flag: the date is gone.
    let stated_over_89 = extracted.age.as_deref() == Some("90+") && extracted.age_source == Some("stated");
    if stated_over_89 && !deidentified.contains("90+") && found.is_empty() {
        found.push(QuasiIdentifier { category: "Age over 89", level: "review", line: 0, action: OVER_89 });
    }
    for m in STATE_LINE.find_iter(deidentified).chain(STATE_AFTER_CITY.find_iter(deidentified)) {
        found.push(QuasiIdentifier {
            category: "State",
            level: "note",
            line: line_of(deidentified, m.start()),
            action: "Allowed under Safe Harbor; keep only if the research needs it.",
        });
    }
    for m in RACE_LINE.find_iter(deidentified) {
        found.push(QuasiIdentifier {
            category: "Race or ethnicity",
            level: "note",
            line: line_of(deidentified, m.start()),
            action: "Allowed under Safe Harbor; with age, sex and place it narrows the group.",
        });
    }
    found.sort_by_key(|q| q.line);
    Assessment { quasi_identifiers: found }
}

/// A research-table age as a number; the table writes 90 and over as "90+".
fn table_age(age: &str) -> Option<u32> {
    if age == "90+" { Some(90) } else { age.parse().ok() }
}

/// How many records share each (age band, sex) in a set's research table.
pub fn group_sizes(rows: &[Extracted]) -> GroupSizes {
    let mut groups: BTreeMap<(u32, &str), usize> = BTreeMap::new();
    for row in rows {
        let age = row.age.as_deref().and_then(table_age);
        if let (Some(age), Some(sex)) = (age, row.sex) {
            // 90 and over is one band, as Safe Harbor reports it.
            *groups.entry((age.min(90) / 10, sex)).or_default() += 1;
        }
    }
    GroupSizes {
        records_counted: groups.values().sum(),
        smallest_group: groups.values().copied().min(),
        records_alone: groups.values().filter(|&&n| n == 1).count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(age: Option<&str>, sex: Option<&'static str>) -> Extracted {
        Extracted { age: age.map(str::to_string), age_source: None, sex, diagnoses: vec![], medications: vec![] }
    }

    fn categories(text: &str, extracted: &Extracted) -> Vec<(&'static str, usize)> {
        assess(text, extracted).quasi_identifiers.iter().map(|q| (q.category, q.line)).collect()
    }

    #[test]
    fn ages_over_89_are_flagged_for_review() {
        let none = row(None, None);
        assert_eq!(categories("A 92-year-old woman with CHF.", &none), vec![("Age over 89", 1)]);
        assert_eq!(categories("Note\nAge: 101", &none), vec![("Age over 89", 2)]);
        assert_eq!(categories("Pt is 95 yo.", &none), vec![("Age over 89", 1)]);
        // The table says 90+ from a shape the engine leaves in the text.
        let stated = Extracted { age_source: Some("stated"), ..row(Some("90+"), None) };
        assert_eq!(categories("94F with CHF.", &stated), vec![("Age over 89", 0)]);
        // Already 90+ in the text, or worked out from a removed date of birth.
        assert_eq!(categories("A 90+-year-old woman.", &stated), vec![]);
        let from_dob = Extracted { age_source: Some("dob"), ..row(Some("90+"), None) };
        assert_eq!(categories("DOB: [DATE]", &from_dob), vec![]);
    }

    #[test]
    fn ages_under_90_and_clinical_numbers_are_not_flagged() {
        let none = row(Some("89"), None);
        assert_eq!(categories("An 89-year-old man. HR 96, SpO2 94%. Glucose 110 mg/dL.", &none), vec![]);
    }

    #[test]
    fn state_and_race_are_noted() {
        let text = "Address: [ADDRESS]\n[CITY], MA [ZIP]\nRace:      White\nState:     Massachusetts";
        assert_eq!(
            categories(text, &row(None, None)),
            vec![("State", 2), ("Race or ethnicity", 3), ("State", 4)]
        );
        assert!(assess(text, &row(None, None)).quasi_identifiers.iter().all(|q| q.level == "note"));
    }

    #[test]
    fn group_sizes_count_age_band_and_sex() {
        let rows = [
            row(Some("41"), Some("F")),
            row(Some("47"), Some("F")),
            row(Some("63"), Some("M")),
            row(Some("95"), Some("F")),
            // as the table writes it; one band with the 95 above
            row(Some("90+"), Some("F")),
            row(None, Some("M")),
        ];
        let g = group_sizes(&rows);
        assert_eq!(g.records_counted, 5);
        assert_eq!(g.smallest_group, Some(1));
        assert_eq!(g.records_alone, 1);
        assert_eq!(group_sizes(&[]).smallest_group, None);
    }
}
