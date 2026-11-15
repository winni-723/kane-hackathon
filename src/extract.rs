//! Pulls the research columns out of a record: age, sex, diagnoses and
//! medications.
//!
//! Everything except the age-from-birth-date is read from the de-identified
//! text, so a cell can never hold something the engine masked. The date of
//! birth is the one thing read from the original: it is turned into an age
//! and then dropped, which is what HIPAA Safe Harbor allows (ages under 90
//! may stay; ages of 90 and over are reported as `90+`).
//!
//! A cell is left empty rather than guessed.

use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;

#[derive(Debug, Serialize, PartialEq)]
pub struct Extracted {
    pub age: Option<String>,
    /// `"stated"` when the record wrote the age, `"dob"` when it was worked
    /// out from the date of birth.
    pub age_source: Option<&'static str>,
    pub sex: Option<&'static str>,
    pub diagnoses: Vec<String>,
    pub medications: Vec<String>,
}

/// A calendar day. Ages from a date of birth are counted up to this day.
#[derive(Clone, Copy, Debug)]
pub struct Day {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Day {
    /// Today in UTC, from the system clock.
    pub fn today() -> Day {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Day::from_days_since_epoch((secs / 86_400) as i64)
    }

    // Howard Hinnant's civil-from-days.
    fn from_days_since_epoch(days: i64) -> Day {
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z.rem_euclid(146_097);
        let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        let year = (yoe + era * 400 + if month <= 2 { 1 } else { 0 }) as i32;
        Day { year, month, day }
    }
}

pub fn extract(original: &str, deidentified: &str, today: Day) -> Extracted {
    let (age, age_source) = match stated_age(deidentified) {
        Some(age) => (Some(age), Some("stated")),
        None => match age_from_birth_date(original, today) {
            Some(age) => (Some(age), Some("dob")),
            None => (None, None),
        },
    };
    Extracted {
        age,
        age_source,
        sex: sex(deidentified),
        diagnoses: diagnoses(deidentified),
        medications: medications(deidentified),
    }
}

// --- age ---------------------------------------------------------------------

/// `72-year-old`, `34 y/o`, `Age: 61`, `[NAME], 58,`.
static STATED_AGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(\d{1,3})[- ]years?[- ]old\b",
        r"|\b(\d{1,3})\s?y/?o\b",
        r"|\bage[ds]?:?\s+(\d{1,3})\b",
        r"|\[NAME\], (\d{1,3}),",
    ))
    .expect("stated age pattern is valid")
});

fn report_age(age: u32) -> Option<String> {
    match age {
        0..=89 => Some(age.to_string()),
        90..=120 => Some("90+".to_string()),
        _ => None,
    }
}

fn stated_age(text: &str) -> Option<String> {
    let caps = STATED_AGE.captures(text)?;
    let digits = (1..=4).find_map(|i| caps.get(i))?.as_str();
    report_age(digits.parse().ok()?)
}

/// Words that say the date after them is a date of birth.
static BIRTH_CUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:DOB|D\.O\.B\.?|date of birth|birth ?date|birthday|born)\b")
        .expect("birth cue pattern is valid")
});

/// How far after the cue the date may start: `DOB per wristband: 09-06-48`.
const BIRTH_CUE_REACH: usize = 24;

fn age_from_birth_date(original: &str, today: Day) -> Option<String> {
    for cue in BIRTH_CUE.find_iter(original) {
        let date = crate::engine::DATE.find_at(original, cue.end());
        if let Some(date) = date
            && date.start() - cue.end() <= BIRTH_CUE_REACH
            && let Some(born) = parse_date(date.as_str(), today.year)
        {
            let mut age = today.year - born.year;
            if (born.month, born.day) > (today.month, today.day) {
                age -= 1;
            }
            return u32::try_from(age).ok().and_then(report_age);
        }
    }
    None
}

const MONTH_PREFIXES: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

const ORDINAL_WORDS: [&str; 31] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth", "thirteenth", "fourteenth", "fifteenth", "sixteenth", "seventeenth",
    "eighteenth", "nineteenth", "twentieth", "twenty-first", "twenty-second", "twenty-third",
    "twenty-fourth", "twenty-fifth", "twenty-sixth", "twenty-seventh", "twenty-eighth",
    "twenty-ninth", "thirtieth", "thirty-first",
];

/// A two-digit year is in the past: `48` is 1948, `21` is 2021.
fn full_year(digits: &str, this_year: i32) -> Option<i32> {
    let n: i32 = digits.parse().ok()?;
    Some(match digits.len() {
        4 => n,
        2 if 2000 + n > this_year => 1900 + n,
        2 => 2000 + n,
        _ => return None,
    })
}

/// Reads the date shapes the engine masks. Numeric dates are month first, the
/// US order, unless the first number is a four-digit year.
fn parse_date(date: &str, this_year: i32) -> Option<Day> {
    let lower = date.to_lowercase();
    let numbers: Vec<&str> = lower
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .collect();
    let month_name = lower
        .split(|c: char| !c.is_ascii_alphabetic())
        .find_map(|word| MONTH_PREFIXES.iter().position(|m| word.starts_with(m)));

    let (year, month, day) = if let Some(index) = month_name {
        let year = *numbers.last()?;
        let day = match numbers.len() {
            2 => numbers[0].parse().ok()?,
            1 => {
                let word = ORDINAL_WORDS
                    .iter()
                    .rposition(|w| lower.contains(w))?;
                word as u32 + 1
            }
            _ => return None,
        };
        (year, index as u32 + 1, day)
    } else if numbers.len() == 3 && numbers[0].len() == 4 {
        (numbers[0], numbers[1].parse().ok()?, numbers[2].parse().ok()?)
    } else if numbers.len() == 3 {
        (numbers[2], numbers[0].parse().ok()?, numbers[1].parse().ok()?)
    } else {
        return None;
    };

    let year = full_year(year, this_year)?;
    ((1..=12).contains(&month) && (1..=31).contains(&day)).then_some(Day { year, month, day })
}

// --- sex ---------------------------------------------------------------------

/// The word after an age, or a labelled field: `72-year-old woman`,
/// `4-year-old son`, `34 y/o male`, `Sex: F`.
static SEX_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)(?:\d[- ]years?[- ]old|\by/?o)\s+(?:\w+\s+)?",
        r"(woman|female|girl|daughter|lady|man|male|boy|son|gentleman)\b",
        r"|\b(?:sex|gender)\s*:\s*(f|m|female|male|woman|man)\b",
    ))
    .expect("sex word pattern is valid")
});

static PRONOUN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(she|her|hers|herself)|(he|him|his|himself))\b")
        .expect("pronoun pattern is valid")
});

fn sex(text: &str) -> Option<&'static str> {
    if let Some(caps) = SEX_WORD.captures(text) {
        let word = caps.get(1).or(caps.get(2))?.as_str().to_lowercase();
        let female = ["woman", "female", "girl", "daughter", "lady", "f"];
        return Some(if female.contains(&word.as_str()) { "F" } else { "M" });
    }
    // Pronouns also point at relatives ("her nephew ... his cell"), so one
    // side has to clearly outnumber the other.
    let (mut female, mut male) = (0u32, 0u32);
    for caps in PRONOUN.captures_iter(text) {
        if caps.get(1).is_some() {
            female += 1;
        } else {
            male += 1;
        }
    }
    if female >= 2 && female >= 2 * male {
        Some("F")
    } else if male >= 2 && male >= 2 * female {
        Some("M")
    } else {
        None
    }
}

// --- diagnoses ---------------------------------------------------------------

/// A field that names the diagnosis: `Diagnosis: Migraine without aura.`
static DIAGNOSIS_FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(?:final |primary |principal )?diagnos[ie]s|impression|dx)\s*:\s*([^.;\n]+)")
        .expect("diagnosis field pattern is valid")
});

/// Phrases that introduce the condition being treated:
/// `follow-up of atrial fibrillation`, `admitted with pneumonia`.
static DIAGNOSIS_CUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(?:follow-up (?:of|for)|management of|admitted with|diagnosed with|treated for",
        r"|history (?:of|includes)|chemotherapy for|for symptomatic|secondary to)\s+",
        r"([^.;,\n]+)",
    ))
    .expect("diagnosis cue pattern is valid")
});

/// `Stage 4 chronic kidney disease secondary to IgA nephropathy`: the
/// condition before the phrase is a diagnosis too.
static SECONDARY_TO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:^|[.;\n]\s*)([A-Z][^.;,\n]*?) secondary to ")
        .expect("secondary-to pattern is valid")
});

/// Where a condition's name stops inside a clause.
static CONDITION_END: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i) (?:controlled|managed|treated|despite|since|on|with|and|who|which|that|but|was|is) ")
        .expect("condition end pattern is valid")
});

fn clean_condition(raw: &str) -> Option<String> {
    let padded = format!("{raw} ");
    let end = CONDITION_END.find(&padded).map_or(raw.len(), |m| m.start());
    let text = raw[..end.min(raw.len())].trim();
    let first = text.split_whitespace().next()?.to_lowercase();
    // A possessive or an article here means the phrase is not a diagnosis
    // name: "admitted for his third exacerbation".
    let not_a_name = ["his", "her", "their", "a", "an", "the", "this", "that"];
    let words = text.split_whitespace().count();
    (!not_a_name.contains(&first.as_str()) && words <= 8 && !text.contains('[')).then(|| text.to_string())
}

fn push_unique(list: &mut Vec<String>, item: String) {
    let lower = item.to_lowercase();
    if !list.iter().any(|have| have.to_lowercase().contains(&lower)) {
        list.retain(|have| !lower.contains(&have.to_lowercase()));
        list.push(item);
    }
}

fn diagnoses(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for caps in DIAGNOSIS_FIELD.captures_iter(text) {
        let value = caps[1].trim();
        if !value.is_empty() && !value.contains('[') {
            push_unique(&mut found, value.to_string());
        }
    }
    for caps in SECONDARY_TO.captures_iter(text) {
        if let Some(condition) = clean_condition(&caps[1]) {
            push_unique(&mut found, condition);
        }
    }
    for caps in DIAGNOSIS_CUE.captures_iter(text) {
        let whole = caps.get(0).expect("group 0 always exists");
        // A relative's condition is not the patient's.
        let before = &text[..whole.start()].to_lowercase();
        if before.trim_end().ends_with("family") {
            continue;
        }
        if let Some(condition) = clean_condition(&caps[1]) {
            push_unique(&mut found, condition);
        }
    }
    found
}

// --- medications -------------------------------------------------------------

/// A drug name followed by a dose and how it is taken:
/// `Furosemide 40 mg daily`, `Carbidopa-Levodopa 25-100 mg three times daily`,
/// `Albuterol inhaler 2 puffs every 4 hours as needed`, `Ceftriaxone 1 g IV daily`.
static MEDICATION: LazyLock<Regex> = LazyLock::new(|| {
    let name = r"[A-Z][a-z]{3,}(?:-[A-Z][a-z]+)?(?: (?:acid|inhaler|nebulizers?|patch|drops|cream))?";
    let dose = r"\d[\d.,-]*\s?(?:mg|mcg|g|mL|units?|puffs?|IU|tablets?|capsules?)\b";
    let how = concat!(
        r"(?:\s(?:IV|PO|IM|subcutaneously|orally|topically|daily|nightly|weekly|monthly|once|twice",
        r"|three|four|times|every|\d{1,2}|hours?|days?|as needed|at bedtime|in the morning|a day",
        r"|chewed)\b)*",
    );
    Regex::new(&format!(r"\b({name}) ({dose})({how})")).expect("medication pattern is valid")
});

/// A drug given through a device on a schedule, with no dose written:
/// `Albuterol nebulizers every 4 hours`.
static MEDICATION_BY_DEVICE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b[A-Z][a-z]{3,} (?:inhaler|nebulizers?) every \d{1,2} hours(?: as needed)?")
        .expect("device medication pattern is valid")
});

/// A drug named by how it was given: `Heparin bolus`, `ABVD chemotherapy`.
static MEDICATION_BY_FORM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:[A-Z][a-z]{3,} (?:bolus|infusion|drip)|[A-Z]{3,6} chemotherapy)\b")
        .expect("form medication pattern is valid")
});

/// Capitalized words that take a number and a unit but are not drugs.
const NOT_A_DRUG: [&str; 12] = [
    "Glucose", "Creatinine", "Hemoglobin", "Potassium", "Sodium", "Calcium", "Cholesterol",
    "Weight", "Height", "Dose", "Total", "Protein",
];

fn medications(text: &str) -> Vec<String> {
    let mut found: Vec<(usize, String)> = Vec::new();
    for caps in MEDICATION.captures_iter(text) {
        let whole = caps.get(0).expect("group 0 always exists");
        let dose = caps.get(2).expect("dose group always matches");
        // `152 mg/dL` is a lab value, not a dose.
        let is_lab_value = text[dose.end()..].starts_with('/');
        let name = caps[1].split(' ').next().unwrap_or_default();
        if !is_lab_value && !NOT_A_DRUG.contains(&name) {
            found.push((whole.start(), whole.as_str().trim().to_string()));
        }
    }
    for m in MEDICATION_BY_DEVICE.find_iter(text).chain(MEDICATION_BY_FORM.find_iter(text)) {
        found.push((m.start(), m.as_str().to_string()));
    }
    found.sort();

    // One row per drug: the first mention with a dose stands for the drug.
    let mut list: Vec<String> = Vec::new();
    for (_, item) in found {
        let name = item.split(' ').next().unwrap_or_default().to_string();
        if !list.iter().any(|have| have.split(' ').next() == Some(name.as_str())) {
            list.push(item);
        }
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    const TODAY: Day = Day { year: 2026, month: 11, day: 14 };

    fn born(date: &str) -> Option<String> {
        age_from_birth_date(&format!("DOB: {date}"), TODAY)
    }

    #[test]
    fn age_from_every_date_shape() {
        assert_eq!(born("03/14/1958").as_deref(), Some("68"));
        assert_eq!(born("1962-05-27").as_deref(), Some("64"));
        assert_eq!(born("09-06-48").as_deref(), Some("78"));
        assert_eq!(born("April 9, 1953").as_deref(), Some("73"));
        assert_eq!(born("March 3rd, '91").as_deref(), Some("35"));
        assert_eq!(born("the fourth of July, 1944").as_deref(), Some("82"));
    }

    #[test]
    fn birthday_not_yet_reached_this_year() {
        assert_eq!(born("11/14/1990").as_deref(), Some("36"));
        assert_eq!(born("11/15/1990").as_deref(), Some("35"));
        assert_eq!(born("22 Nov 1991").as_deref(), Some("34"));
    }

    #[test]
    fn ninety_and_over_is_not_reported_exactly() {
        assert_eq!(born("01/01/1930").as_deref(), Some("90+"));
        assert_eq!(stated_age("a 94-year-old woman").as_deref(), Some("90+"));
        assert_eq!(stated_age("an 89-year-old man").as_deref(), Some("89"));
    }

    #[test]
    fn a_date_that_is_not_a_birth_date_gives_no_age() {
        assert_eq!(age_from_birth_date("Seen on 03/14/2024 for cough.", TODAY), None);
        assert_eq!(age_from_birth_date("DOB: [DATE]", TODAY), None);
    }

    #[test]
    fn stated_age_wins_over_birth_date() {
        let e = extract(
            "A 72-year-old woman, born April 9, 1953.",
            "A 72-year-old woman, born [DATE].",
            TODAY,
        );
        assert_eq!((e.age.as_deref(), e.age_source), (Some("72"), Some("stated")));
    }

    #[test]
    fn sex_from_words_and_pronouns() {
        assert_eq!(sex("a 72-year-old woman"), Some("F"));
        assert_eq!(sex("her 4-year-old son [NAME]"), Some("M"));
        assert_eq!(sex("Sex: F"), Some("F"));
        assert_eq!(sex("He reports pain. His wife drove him."), Some("M"));
        // One pronoun each way is not enough to decide.
        assert_eq!(sex("Her nephew translates; call his cell."), None);
        assert_eq!(sex("Name: [NAME]\nDiagnosis: Migraine."), None);
    }

    #[test]
    fn diagnoses_from_fields_and_phrases() {
        assert_eq!(diagnoses("Diagnosis: Migraine without aura."), ["Migraine without aura"]);
        assert_eq!(diagnoses("seen for follow-up of atrial fibrillation."), ["atrial fibrillation"]);
        assert_eq!(
            diagnoses("Past history includes hypertension controlled on Amlodipine 5 mg daily."),
            ["hypertension"]
        );
        assert_eq!(
            diagnoses("Stage 4 chronic kidney disease secondary to IgA nephropathy; eGFR is 22."),
            ["Stage 4 chronic kidney disease", "IgA nephropathy"]
        );
    }

    #[test]
    fn not_the_patients_condition() {
        assert!(diagnoses("has a family history of early coronary artery disease.").is_empty());
        assert!(diagnoses("was admitted for his third exacerbation this year.").is_empty());
        assert!(diagnoses("Assessment: PHQ-9 score 14, improved from 19.").is_empty());
        assert!(diagnoses("Chief Complaint: Shortness of breath on exertion.").is_empty());
    }

    #[test]
    fn medications_with_doses() {
        assert_eq!(
            medications("Medications: Furosemide 40 mg daily; Lisinopril 10 mg daily."),
            ["Furosemide 40 mg daily", "Lisinopril 10 mg daily"]
        );
        assert_eq!(
            medications("Carbidopa-Levodopa 25-100 mg three times daily."),
            ["Carbidopa-Levodopa 25-100 mg three times daily"]
        );
        assert_eq!(
            medications("Albuterol inhaler 2 puffs every 4 hours as needed; with Folic acid 1 mg daily."),
            ["Albuterol inhaler 2 puffs every 4 hours as needed", "Folic acid 1 mg daily"]
        );
        assert_eq!(medications("Ceftriaxone 1 g IV daily"), ["Ceftriaxone 1 g IV daily"]);
        assert_eq!(medications("Aspirin 325 mg chewed, Heparin bolus given"), ["Aspirin 325 mg chewed", "Heparin bolus"]);
    }

    #[test]
    fn a_medication_ends_at_the_last_whole_schedule_word() {
        assert_eq!(
            medications("Continue Warfarin 5 mg daily and Metoprolol 25 mg twice daily."),
            ["Warfarin 5 mg daily", "Metoprolol 25 mg twice daily"]
        );
        assert_eq!(medications("Start Sertraline 50 mg daily at home."), ["Sertraline 50 mg daily"]);
    }

    #[test]
    fn one_row_per_drug() {
        assert_eq!(
            medications("on Amlodipine 5 mg daily. Hold Amlodipine on the morning of surgery."),
            ["Amlodipine 5 mg daily"]
        );
    }

    #[test]
    fn lab_values_and_unneeded_drugs_are_not_medications() {
        assert!(medications("Glucose 152 mg/dL, LDL cholesterol is 172 mg/dL.").is_empty());
        assert!(medications("Troponin I is 3.2 ng/mL. eGFR is 22 mL/min.").is_empty());
        assert!(medications("Kayexalate not required; potassium is 4.9.").is_empty());
    }

    #[test]
    fn today_is_a_real_date() {
        let d = Day::from_days_since_epoch(20_771);
        assert_eq!((d.year, d.month, d.day), (2026, 11, 14));
    }
}
