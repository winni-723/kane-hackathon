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
    let digits = match STATED_AGE.captures(text) {
        Some(caps) => (1..=4).find_map(|i| caps.get(i))?.as_str().to_string(),
        None => age_and_sex(text)?.0,
    };
    report_age(digits.parse().ok()?)
}

/// The shorthand that opens many emergency notes: `47M`, `62F`, `81yoF`.
static AGE_SEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:^|[\s(,;])(\d{1,3})(?:yo)?([MF]) [A-Za-z]").expect("age-sex pattern is valid")
});

/// Words before a number-and-letter that make it a temperature: `Temp 102F`.
const TEMPERATURE_WORDS: [&str; 6] = ["temp", "temperature", "fever", "tmax", "t", "of"];

fn age_and_sex(text: &str) -> Option<(String, &'static str)> {
    AGE_SEX.captures_iter(text).find_map(|caps| {
        let digits = caps.get(1)?;
        let before = text[..digits.start()]
            .split(|c: char| !c.is_ascii_alphabetic())
            .rfind(|w| !w.is_empty())
            .unwrap_or_default()
            .to_lowercase();
        if TEMPERATURE_WORDS.contains(&before.as_str()) {
            return None;
        }
        Some((digits.as_str().to_string(), if &caps[2] == "F" { "F" } else { "M" }))
    })
}

/// Words that say the date after them is a date of birth.
static BIRTH_CUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:DOB|D\.O\.B\.?|date of birth|birth ?date|birthday|born)\b")
        .expect("birth cue pattern is valid")
});

/// How far after the cue the date may start: `DOB per wristband: 09-06-48`.
const BIRTH_CUE_REACH: usize = 24;

/// A date written as eight digits, as database exports do: `19900602`.
/// Only read as a date directly after a birth cue.
static COMPACT_DATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[\s:]{0,3}((?:18|19|20)\d{2})(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])\b")
        .expect("compact date pattern is valid")
});

fn age_from_birth_date(original: &str, today: Day) -> Option<String> {
    for cue in BIRTH_CUE.find_iter(original) {
        let compact = COMPACT_DATE.captures(&original[cue.end()..]).and_then(|caps| {
            Some(Day {
                year: caps[1].parse().ok()?,
                month: caps[2].parse().ok()?,
                day: caps[3].parse().ok()?,
            })
        });
        let written = || {
            crate::engine::DATE
                .find_at(original, cue.end())
                .filter(|date| date.start() - cue.end() <= BIRTH_CUE_REACH)
                .and_then(|date| parse_date(date.as_str(), today.year))
        };
        if let Some(born) = compact.or_else(written) {
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

    // `Christmas Eve, 1936`: a named day with a fixed date. Thanksgiving moves
    // from year to year and is left out.
    let holiday = [
        ("christmas eve", 12, 24),
        ("christmas", 12, 25),
        ("new year's eve", 12, 31),
        ("new year's day", 1, 1),
        ("valentine's day", 2, 14),
        ("halloween", 10, 31),
        ("independence day", 7, 4),
    ]
    .into_iter()
    .find(|(name, _, _)| lower.contains(name));

    let (year, month, day) = if let Some((_, month, day)) = holiday {
        (*numbers.last()?, month, day)
    } else if let Some(index) = month_name {
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
/// `4-year-old son`, `34 y/o male`, `Sex: F`, and the fields of a document's
/// table: `Sex<tab>Female`, `DOB / Sex<tab>[DATE] / F`.
static SEX_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)(?:\d[- ]years?[- ]old|\by/?o)\s+(?:\w+\s+)?",
        r"(woman|female|girl|daughter|lady|man|male|boy|son|gentleman)\b",
        r"|\b(?:sex|gender)(?:[ \t]*:|\t)[ \t]*(f|m|female|male|woman|man)\b",
        r"|\b(?:dob|date of birth) ?/ ?(?:sex|gender)(?:[ \t]*:|\t)[ \t]*\S+ ?/ ?(f|m|female|male)\b",
    ))
    .expect("sex word pattern is valid")
});

static PATIENT_TITLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?mi)^(?:re|patient(?: name)?|pt|name)\s*:\s*(mr|mrs|ms|miss)\b")
        .expect("patient title pattern is valid")
});

/// `Reviewed with patient and his son`: the pronoun right after "patient
/// and" can only be the patient's.
static PATIENT_AND_PRONOUN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:the )?patient and (?:(her)|(his))\b").expect("patient pronoun pattern is valid")
});

static PRONOUN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(she|her|hers|herself)|(he|him|his|himself))\b")
        .expect("pronoun pattern is valid")
});

fn sex(text: &str) -> Option<&'static str> {
    if let Some(caps) = SEX_WORD.captures(text) {
        let word = caps.get(1).or(caps.get(2)).or(caps.get(3))?.as_str().to_lowercase();
        let female = ["woman", "female", "girl", "daughter", "lady", "f"];
        return Some(if female.contains(&word.as_str()) { "F" } else { "M" });
    }
    if let Some((_, sex)) = age_and_sex(text) {
        return Some(sex);
    }
    // A title in the field that names the patient: `Re: Mr. [NAME]`. Titles
    // elsewhere in a record often belong to relatives and are not used.
    if let Some(caps) = PATIENT_TITLE.captures(text) {
        return Some(if caps[1].eq_ignore_ascii_case("mr") { "M" } else { "F" });
    }
    if let Some(caps) = PATIENT_AND_PRONOUN.captures(text) {
        return Some(if caps.get(1).is_some() { "F" } else { "M" });
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

/// An assessment line, including the `A:` of a SOAP note. It may name the
/// diagnosis (`A: Preeclampsia with severe features.`) or only say how the
/// patient is doing (`Assessment: PHQ-9 score 14, improved from 19.`).
static ASSESSMENT_FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^(?:Assessment|A)\s*:\s*([^.;\n]+)").expect("assessment field pattern is valid")
});

/// Words that make an assessment a progress report rather than a diagnosis.
const PROGRESS_WORDS: [&str; 14] = [
    "control", "controlled", "suboptimal", "improved", "improving", "stable", "worsening",
    "worse", "unchanged", "well", "normal", "resolved", "tolerating", "continue",
];

/// Phrases that introduce the condition being treated:
/// `follow-up of atrial fibrillation`, `admitted with pneumonia`.
static DIAGNOSIS_CUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(?:follow-up (?:of|for)|management of|admitted with|diagnosed with|treated for",
        r"|(?:history|hx) (?:of|includes)|for symptomatic|secondary to",
        // the reason for an operation or a course of treatment
        r"|(?:chemotherapy|radiotherapy|\w+(?:ectomy|otomy|ostomy|plasty)|resection) for(?: symptomatic)?)\s+",
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
    Regex::new(r"(?i) (?:controlled|managed|treated|despite|since|on|with|and|who|which|that|but|was|is|per) ")
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

/// A document's heading for its diagnoses, alone on its line:
/// `Discharge Diagnoses`, `Secondary Diagnoses`, `Impression`.
static DIAGNOSIS_HEADING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?mi)^(?:(?:principal|primary|secondary|discharge|final|admitting) )?(?:diagnos[ie]s|impression)[ \t]*$")
        .expect("diagnosis heading pattern is valid")
});

/// `1. ` at the start of a numbered item.
static ITEM_NUMBER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:^|\s)\d{1,2}\.(?:\s+|$)").expect("item number pattern is valid")
});

/// The conditions under a diagnosis heading. The section is the next line,
/// joined with the lines after it while an item is cut off by the page's
/// wrapping (`... COPD. 3.` / `Paroxysmal atrial fibrillation ...`). Items
/// are numbered, or else one sentence each; a condition ends at its first
/// comma (`Community-acquired pneumonia, right lower lobe`).
fn diagnoses_under_heading(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for heading in DIAGNOSIS_HEADING.find_iter(text) {
        let mut lines = text[heading.end()..].lines().skip(1);
        let Some(first) = lines.next() else { continue };
        let mut body = first.trim().to_string();
        while !body.ends_with('.') || ITEM_NUMBER.find_iter(&body).last().is_some_and(|m| m.end() == body.len()) {
            match lines.next() {
                Some(next) if !next.trim().is_empty() => body = format!("{body} {}", next.trim()),
                _ => break,
            }
        }
        let items: Vec<&str> = if ITEM_NUMBER.is_match(&body) {
            ITEM_NUMBER.split(&body).collect()
        } else {
            body.split(". ").collect()
        };
        for item in items {
            let name = item.split([',', '.']).next().unwrap_or_default().trim();
            if let Some(condition) = clean_condition(name) {
                push_unique(&mut found, condition);
            }
        }
    }
    found
}

/// The patient introduced by age with their conditions:
/// `85-year-old woman with Alzheimer disease, atrial fibrillation and
/// chronic kidney disease stage 3 who fell`.
static AGED_PATIENT_WITH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\d[- ]years?[- ]old (?:woman|man|female|male|lady|gentleman|girl|boy|patient) with ",
        r"([^.;\n]+)",
    ))
    .expect("aged patient pattern is valid")
});

/// Where the list of conditions after "with" stops.
static WITH_LIST_END: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i),? (?:who|here|presenting|presents|presented|over|for|on|since|that|after) ")
        .expect("list end pattern is valid")
});

/// Words that make an item a symptom or a finding, not a diagnosis.
const SYMPTOM_WORDS: [&str; 16] = [
    "pain", "ache", "fever", "cough", "dyspnea", "breath", "nausea", "vomiting", "wheezing",
    "fatigue", "leukocytosis", "swelling", "rash", "bleeding", "dizziness", "headache",
];

fn diagnoses_after_age(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for caps in AGED_PATIENT_WITH.captures_iter(text) {
        let list = &caps[1];
        let list = &list[..WITH_LIST_END.find(list).map_or(list.len(), |m| m.start())];
        for item in list.split(", and ").flat_map(|s| s.split(", ")).flat_map(|s| s.split(" and ")) {
            let item = item.trim().trim_end_matches(',');
            let lower = item.to_lowercase();
            let symptom = lower.split(|c: char| !c.is_ascii_alphabetic()).any(|w| SYMPTOM_WORDS.contains(&w));
            if !symptom && let Some(condition) = clean_condition(item) {
                push_unique(&mut found, condition);
            }
        }
    }
    found
}

fn diagnoses(text: &str) -> Vec<String> {
    let mut found = diagnoses_under_heading(text);
    for condition in diagnoses_after_age(text) {
        push_unique(&mut found, condition);
    }
    for caps in DIAGNOSIS_FIELD.captures_iter(text) {
        let value = caps[1].trim();
        if !value.is_empty() && !value.contains('[') {
            push_unique(&mut found, value.to_string());
        }
    }
    for caps in ASSESSMENT_FIELD.captures_iter(text) {
        let value = caps[1].trim();
        let lower = value.to_lowercase();
        let describes_progress = value.chars().any(|c| c.is_ascii_digit())
            || PROGRESS_WORDS.iter().any(|w| lower.split(|c: char| !c.is_ascii_alphabetic()).any(|x| x == *w));
        if !value.is_empty() && !describes_progress && !value.contains('[') {
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
    let name = concat!(
        // a vitamin is named by a letter: Vitamin D, Vitamin B12
        r"(?:Vitamin [A-EK]\d{0,2}|",
        // the second half of a combination may be lower case: Budesonide-formoterol
        r"[A-Z][a-z]{3,}(?:-[A-Za-z][a-z]+)?",
        // the second word of a drug's name, or the device it comes in
        r"(?: (?:acid|sulfate|chloride|sodium|succinate|tartrate|citrate|carbonate|phosphate",
        r"|hydrochloride|acetate|gluconate|oxide|inhaler|nebulizers?|patch|drops|cream))?",
        // an extended-release form: Diltiazem ER
        r"(?: (?:ER|XR|SR|CR|XL|LA))?)",
    );
    let dose = concat!(
        // 160/4.5 mcg: the two strengths of a combination
        r"(?:\d[\d.,/-]*\s?(?:mg|mcg|g|mL|mEq|units?|puffs?|IU|tablets?|capsules?)\b",
        // a strength written as a percentage needs the form to be a drug
        r"|\d[\d.]*\s?% (?:cream|ointment|gel|lotion|solution|drops)\b)",
    );
    let how = concat!(
        r"(?:\s(?:IV|PO|IM|subcutaneously|orally|topically|sublingual|sublingually|daily|nightly|weekly|monthly|once|twice",
        r"|three|four|times|every|\d{1,2}|hours?|days?|weeks?|months?|puffs?|as needed|at bedtime",
        r"|in the morning|a day",
        r"|chewed)\b)*",
    );
    // `reduce Amlodipine to 2.5 mg daily`. In a document's table the dose is
    // in the next cell (`Metformin<tab>1000 mg<tab>twice daily`), and in a
    // wrapped paragraph it may start the next line.
    // A change of dose names both: `Levothyroxine from 75 mcg to 88 mcg daily`.
    let change = r"(?:from \d[\d.]*\s?(?:mg|mcg|g|units?) )?";
    Regex::new(&format!(r"\b({name})[ \t\n]{change}(?:to )?({dose})({how})")).expect("medication pattern is valid")
});

/// A drug given through a device on a schedule, with no dose written:
/// `Albuterol nebulizers every 4 hours`.
static MEDICATION_BY_DEVICE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b[A-Z][a-z]{3,} (?:inhaler|nebulizers?) every \d{1,2} hours(?: as needed)?")
        .expect("device medication pattern is valid")
});

/// A drug named by how it was given: `Heparin bolus`, `ABVD chemotherapy`.
static MEDICATION_BY_FORM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:[A-Z][a-z]{3,} (?:bolus|infusion|drip|PCA)|[A-Z]{3,6} chemotherapy)\b")
        .expect("form medication pattern is valid")
});

/// Drugs said to be given, with no dose written:
/// `treated with intravenous Ceftriaxone and Azithromycin`.
static MEDICATION_GIVEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:treated with|started on|given)",
        r"(?: (?:intravenous|IV|oral|inhaled|nebulized|subcutaneous))?",
        r" ([A-Z][a-z]{3,}(?:-[a-z]+)?)(?: and ([A-Z][a-z]{3,}(?:-[a-z]+)?))?\b",
    ))
    .expect("given medication pattern is valid")
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
            found.push((whole.start(), one_line(whole.as_str())));
        }
    }
    for m in MEDICATION_BY_DEVICE.find_iter(text).chain(MEDICATION_BY_FORM.find_iter(text)) {
        found.push((m.start(), one_line(m.as_str())));
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
    // A drug given with no dose is listed by name, unless it is already
    // there with one.
    for caps in MEDICATION_GIVEN.captures_iter(text) {
        for name in [caps.get(1), caps.get(2)].into_iter().flatten().map(|m| m.as_str()) {
            if !NOT_A_DRUG.contains(&name) && !list.iter().any(|have| have.split(' ').next() == Some(name)) {
                list.push(name.to_string());
            }
        }
    }
    list
}

/// A medication as one line with single spaces: a table row's tabs and a
/// wrapped paragraph's line break become spaces. A bare number left at the
/// end is the next cell (`25 mg<tab>1 tablet`), not part of the schedule.
fn one_line(raw: &str) -> String {
    let mut words: Vec<&str> = raw.split_whitespace().collect();
    while words.len() > 1 && words.last().is_some_and(|w| w.chars().all(|c| c.is_ascii_digit())) {
        words.pop();
    }
    words.join(" ")
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
    fn age_from_an_eight_digit_birth_date() {
        assert_eq!(born("19900602").as_deref(), Some("36"));
        assert_eq!(age_from_birth_date("Birth Date: 19381225", TODAY).as_deref(), Some("87"));
        // Eight digits that are not right after a birth cue are not a date.
        assert_eq!(age_from_birth_date("MRN: 19900602", TODAY), None);
        assert_eq!(age_from_birth_date("DOB unknown. Chart 19900602", TODAY), None);
    }

    #[test]
    fn medications_with_combined_strengths_and_long_intervals() {
        assert_eq!(
            medications("Continue Adalimumab 40 mg every 2 weeks. Budesonide-formoterol 160/4.5 mcg 2 puffs twice daily."),
            ["Adalimumab 40 mg every 2 weeks", "Budesonide-formoterol 160/4.5 mcg 2 puffs twice daily"]
        );
        // A lab value with a slash in its unit is still not a dose.
        assert!(medications("Glucose 152 mg/dL. Cholesterol 210 mg/dL.").is_empty());
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
    fn sex_from_a_table_field() {
        assert_eq!(sex("Date of Birth\t[DATE]\tSex\tFemale"), Some("F"));
        assert_eq!(sex("DOB / Sex\t[DATE] / F\tCollected\t[DATE]"), Some("F"));
        assert_eq!(sex("DOB/Sex: [DATE]/M"), Some("M"));
        // The word alone, with no value after it, says nothing.
        assert_eq!(sex("Safe sex practices were discussed."), None);
        assert_eq!(sex("Sex\tnot recorded"), None);
    }

    #[test]
    fn sex_from_the_pronoun_after_patient_and() {
        assert_eq!(sex("Reviewed with patient and his son [NAME]."), Some("M"));
        assert_eq!(sex("Discussed with the patient and her husband."), Some("F"));
        // A relative's pronoun elsewhere does not count.
        assert_eq!(sex("Called the patient's daughter and her husband."), None);
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
    fn diagnoses_under_a_heading() {
        assert_eq!(
            diagnoses("Principal Diagnosis\nCommunity-acquired pneumonia, right lower lobe.\nSecondary Diagnoses\nType 2 diabetes mellitus. Hypertension.\nHospital Course\nShe improved."),
            ["Community-acquired pneumonia", "Type 2 diabetes mellitus", "Hypertension"]
        );
        // Numbered items, one cut off by the page's wrapping.
        assert_eq!(
            diagnoses("Discharge Diagnoses\n1. Pneumonia. 2. Acute exacerbation of COPD. 3.\nParoxysmal atrial fibrillation, now rate controlled.\nHospital Course"),
            ["Pneumonia", "Acute exacerbation of COPD", "Paroxysmal atrial fibrillation"]
        );
        assert_eq!(
            diagnoses("Impression\n1. Acute appendicitis. 2. Mild\ncentrilobular emphysema.\nFindings were called to Dr. [NAME]."),
            ["Acute appendicitis", "Mild centrilobular emphysema"]
        );
        // The word inside a sentence is not a heading.
        assert!(diagnoses("My impression\nis that she is improving.").is_empty());
    }

    #[test]
    fn conditions_after_the_patients_age() {
        assert_eq!(
            diagnoses("[NAME] is an 85-year-old woman with Alzheimer disease, atrial fibrillation and chronic kidney disease stage 3 who fell."),
            ["Alzheimer disease", "atrial fibrillation", "chronic kidney disease stage 3"]
        );
        assert_eq!(diagnoses("35-year-old woman with Hashimoto thyroiditis, here for follow-up."), ["Hashimoto thyroiditis"]);
        assert_eq!(diagnoses("a 67-year-old man with COPD on home oxygen 2 L who presented"), ["COPD"]);
        // Symptoms and findings are not diagnoses.
        assert!(diagnoses("66-year-old man with exertional chest pain over the past two months.").is_empty());
        assert!(diagnoses("51-year-old man with right lower quadrant pain for 2 days, fever and leukocytosis.").is_empty());
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
    fn age_and_sex_shorthand() {
        assert_eq!(stated_age("47M BIBA after a seizure.").as_deref(), Some("47"));
        assert_eq!(sex("47M BIBA after a seizure."), Some("M"));
        assert_eq!(sex("Seen today: 62F with chest pain."), Some("F"));
        // a temperature, a distance and a staging code are not an age
        assert_eq!(stated_age("Temp 102F and rising."), None);
        assert_eq!(stated_age("fever of 102.4 F and right ear pain"), None);
        assert_eq!(stated_age("Ambulated 200m with assistance. Stage T2N1M0 disease."), None);
    }

    #[test]
    fn age_from_a_named_day() {
        assert_eq!(born("Christmas Eve, 1936").as_deref(), Some("89"));
        assert_eq!(born("Halloween, 1936").as_deref(), Some("90+"));
        assert_eq!(born("Thanksgiving, 1950"), None);
    }

    #[test]
    fn sex_from_a_title_only_in_the_patient_field() {
        assert_eq!(sex("REFERRAL\nRe: Mr. [NAME]\nDate of birth: [DATE]"), Some("M"));
        assert_eq!(sex("Patient: Mrs. [NAME]"), Some("F"));
        // a relative's title says nothing about the patient
        assert_eq!(sex("Discussed with the son, Mr. [NAME]."), None);
    }

    #[test]
    fn an_assessment_is_a_diagnosis_only_when_it_names_one() {
        assert_eq!(
            diagnoses("Assessment: Gastroesophageal reflux disease.\nRx: Omeprazole 20 mg daily."),
            ["Gastroesophageal reflux disease"]
        );
        assert_eq!(
            diagnoses("O: BP 162/104.\nA: Preeclampsia with severe features.\nP: Admit."),
            ["Preeclampsia with severe features"]
        );
        assert!(diagnoses("Assessment: Glycemic control suboptimal. Add Empagliflozin 10 mg daily.").is_empty());
        assert!(diagnoses("Assessment: Stable, tolerating diet.").is_empty());
    }

    #[test]
    fn the_reason_for_an_operation_and_a_history() {
        assert_eq!(
            diagnoses("post-op day 2 from a right hemicolectomy for adenocarcinoma of the ascending colon."),
            ["adenocarcinoma of the ascending colon"]
        );
        assert_eq!(diagnoses("Hx of traumatic brain injury per chart."), ["traumatic brain injury"]);
        assert!(diagnoses("Referred for Physical Therapy after surgery.").is_empty());
    }

    #[test]
    fn more_ways_a_medication_is_written() {
        assert_eq!(
            medications("Plan: Ferrous sulfate 325 mg twice daily; recheck in 6 weeks."),
            ["Ferrous sulfate 325 mg twice daily"]
        );
        assert_eq!(
            medications("add Triamcinolone 0.1% cream twice daily."),
            ["Triamcinolone 0.1% cream twice daily"]
        );
        assert_eq!(medications("reduce Amlodipine to 2.5 mg daily"), ["Amlodipine to 2.5 mg daily"]);
        assert_eq!(medications("Pain controlled on Hydromorphone PCA; tolerating liquids."), ["Hydromorphone PCA"]);
        // a stopped drug and a percentage that is not a strength
        assert!(medications("stop Diphenhydramine; SpO2 95% on room air; HbA1c 7.8%.").is_empty());
    }

    #[test]
    fn a_dose_in_the_next_cell_or_on_the_next_line() {
        assert_eq!(
            medications("Medication\tDose\tFrequency\nMetformin\t1000 mg\ttwice daily\tcontinue"),
            ["Metformin 1000 mg twice daily"]
        );
        assert_eq!(medications("Lisinopril 20 mg daily, Atorvastatin\n40 mg daily."), [
            "Lisinopril 20 mg daily",
            "Atorvastatin 40 mg daily"
        ]);
        // The next cell's number is not part of the schedule.
        assert_eq!(medications("Furosemide 20 mg\t1 tablet by mouth"), ["Furosemide 20 mg"]);
        // A drug mentioned twice is one row, however its dose was spaced.
        assert_eq!(medications("Prednisone 40 mg by mouth.\nPrednisone\t40 mg daily"), ["Prednisone 40 mg"]);
        // A lab table is not a medication list.
        assert!(medications("Test\tResult\nGlucose\t168 mg/dL\tH").is_empty());
    }

    #[test]
    fn release_forms_vitamins_routes_and_dose_changes() {
        assert_eq!(medications("Diltiazem ER 120 mg daily (new)."), ["Diltiazem ER 120 mg daily"]);
        assert_eq!(medications("Vitamin D 2000 IU daily to continue."), ["Vitamin D 2000 IU daily"]);
        assert_eq!(
            medications("given Nitroglycerin 0.4 mg sublingual as needed."),
            ["Nitroglycerin 0.4 mg sublingual as needed"]
        );
        assert_eq!(
            medications("Increase Levothyroxine from 75 mcg to 88 mcg daily, taken fasting."),
            ["Levothyroxine from 75 mcg to 88 mcg daily"]
        );
        // `ER` alone after a word is not a release form: the emergency room.
        assert!(medications("Seen in the ER 2 days ago.").is_empty());
    }

    #[test]
    fn drugs_given_with_no_dose() {
        assert_eq!(
            medications("She was treated with intravenous Ceftriaxone and Azithromycin and improved."),
            ["Ceftriaxone", "Azithromycin"]
        );
        // With a dose elsewhere, the drug is listed once, with its dose.
        assert_eq!(medications("Started on Apixaban. Apixaban 5 mg twice daily (new)."), ["Apixaban 5 mg twice daily"]);
        // A name is never a drug: it is a tag by now.
        assert!(medications("She was given [NAME]'s number.").is_empty());
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
