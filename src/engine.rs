//! De-identification engine.
//!
//! Each kind of identifier is handled by its own rule. A rule finds the
//! identifier and replaces it with a placeholder such as `[PHONE]`; everything
//! else in the record is left exactly as written.

use regex::Regex;
use std::sync::LazyLock;

/// US phone numbers written in digits.
///
/// Accepts an optional `+1` / `1` country code, an optional area code written
/// as `(617)` or `617` followed by a separator, then the seven-digit number
/// with a space, dot or hyphen in the middle:
/// `(617) 555-0142`, `415-555-0117`, `206.555.0108`, `+1 303 555 0154`,
/// `555-0198`.
///
/// The 3-then-4 digit shape is what keeps it from touching SSNs (3-2-4),
/// dates, record numbers, doses and blood pressures.
static PHONE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:\+?1[\s.-]?)?(?:\(\d{3}\)\s?|\b\d{3}[\s.-])?\b\d{3}[\s.-]\d{4}\b")
        .expect("phone pattern is valid")
});

/// Email addresses.
static EMAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)*\.[A-Za-z]{2,}\b")
        .expect("email pattern is valid")
});

/// US Social Security numbers: three, two and four digits, separated by
/// hyphens or by spaces (`987-65-4321`, `000 12 3456`).
static SSN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b\d{3}-\d{2}-\d{4}\b|\b\d{3} \d{2} \d{4}\b").expect("ssn pattern is valid")
});

/// Medical record numbers. A bare number is ambiguous, so this rule only fires
/// after a label that says what the number is: `MRN: 00482917`,
/// `record number is 55801276`, `Chart number 60917734`,
/// `chart no. was 4471-208-93`. The label is kept; only the number goes.
static MRN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?P<label>\b(?i:MRN|medical record (?:number|no\.?)|record (?:number|no\.?)|chart (?:number|no\.?)|filed under))",
        r"(?P<join>\s*(?:[:#]|(?i:is|was))?\s*)",
        r"(?P<id>[A-Z]{0,3}-?\d[\d-]{2,}\d)\b",
    ))
    .expect("mrn pattern is valid")
});

/// Insurance, member and policy identifiers, again only after their label:
/// `Insurance ID: HX-4490217`.
static INSURANCE_ID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?P<label>\b(?i:insurance (?:id|number|no\.?)|member (?:id|number|no\.?)|policy (?:id|number|no\.?)))",
        r"(?P<join>\s*[:#]?\s*)",
        r"(?P<id>[A-Z]{0,4}-?\d[\dA-Z-]{2,}[\dA-Z])\b",
    ))
    .expect("insurance id pattern is valid")
});

const MONTH: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|June?|July?|Aug(?:ust)?",
    r"|Sep(?:t(?:ember)?)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)\.?",
);

const ORDINAL_WORD: &str = concat!(
    r"(?:first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth",
    r"|thirteenth|fourteenth|fifteenth|sixteenth|seventeenth|eighteenth|nineteenth|twentieth",
    r"|twenty-(?:first|second|third|fourth|fifth|sixth|seventh|eighth|ninth)|thirtieth|thirty-first)",
);

/// Full calendar dates. HIPAA Safe Harbor treats every date more specific than
/// a year as an identifier, so this is not limited to dates of birth.
///
/// A year, or a month and day without a year, is left alone. Recognized shapes:
/// `03/14/1958`, `3/8/1967`, `1962-05-27`, `09-06-48`, `April 9, 1953`,
/// `March 3rd, '91`, `22 Nov 1991`, `the fourth of July, 1944`.
pub(crate) static DATE: LazyLock<Regex> = LazyLock::new(|| {
    let year = r"(?:\d{4}|'\d{2})";
    let pattern = [
        // 03/14/1958, 3/8/1967
        r"\b\d{1,2}/\d{1,2}/\d{2,4}\b".to_string(),
        // 1962-05-27
        r"\b\d{4}-\d{2}-\d{2}\b".to_string(),
        // 09-06-48, 09-06-1948
        r"\b\d{2}-\d{2}-(?:\d{4}|\d{2})\b".to_string(),
        // April 9, 1953 / March 3rd, '91
        format!(r"\b{MONTH}\s+\d{{1,2}}(?:st|nd|rd|th)?,?\s+{year}\b"),
        // 22 Nov 1991 / 22nd of November, 1991
        format!(r"\b\d{{1,2}}(?:st|nd|rd|th)?\s+(?:of\s+)?{MONTH},?\s+{year}\b"),
        // the fourth of July, 1944 / tenth of October, 1983
        format!(r"\b(?:the\s+)?{ORDINAL_WORD}\s+of\s+{MONTH},?\s+{year}\b"),
        // 12.07.1978
        r"\b\d{1,2}\.\d{1,2}\.\d{4}\b".to_string(),
        // Christmas Eve, 1936: a named day plus a year pins down a date
        format!(
            r"\b(?:Christmas(?: Eve| Day)?|New Year's (?:Eve|Day)|Valentine's Day|Halloween|Independence Day|Thanksgiving(?: Day)?),?\s+(?:of\s+)?{year}\b"
        ),
    ]
    .join("|");
    Regex::new(&pattern).expect("date pattern is valid")
});

/// A street address: a house number, one to four capitalized words, and a
/// street-type word. `48 Willow Bend Road`, `1260 Juniper Court`.
static STREET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b\d{1,6}\s+(?:[A-Z][A-Za-z']*\s+){1,4}",
        r"(?:Street|St|Avenue|Ave|Road|Rd|Lane|Ln|Drive|Dr|Court|Ct|Boulevard|Blvd|Way|Circle|Cir",
        r"|Trail|Trl|Row|Place|Pl|Terrace|Ter|Parkway|Pkwy|Highway|Hwy|Square|Sq",
        // further US Postal Service street suffixes
        r"|Alley|Bend|Bluff|Bridge|Brook|Canyon|Cove|Creek|Crossing|Expressway|Extension|Falls",
        r"|Ferry|Fields?|Flats?|Ford|Forest|Forge|Forks?|Fort|Gardens?|Gateway|Glen|Green|Grove",
        r"|Harbor|Heights|Hills?|Hollow|Island|Junction|Knolls?|Lakes?|Landing|Loop|Manor|Meadows?",
        r"|Mews|Mills?|Mission|Mount|Mountain|Orchard|Oval|Park|Pass|Passage|Path|Pike|Pines?",
        r"|Plains?|Plaza|Point|Port|Prairie|Promenade|Ranch|Rapids|Ridge|Run|Shores?|Springs?",
        r"|Spur|Station|Stream|Summit|Trace|Track|Tunnel|Turnpike|Vale|Valley|View|Village|Vista",
        r"|Walk|Wells?)\b\.?",
    ))
    .expect("street pattern is valid")
});

/// A unit within a building: `Apt 4B`, `Unit 12`, `Suite 300`.
/// Capitalized and followed by a number, so "on the unit as" is not touched.
static UNIT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:Apt|Apartment|Unit|Suite|Ste)\.?\s+#?\d+[A-Za-z]?\b")
        .expect("unit pattern is valid")
});

/// `City, ST` with an optional ZIP code. The state must be a real US postal
/// abbreviation, which keeps this off ordinary comma-separated prose. The
/// state itself is kept: under Safe Harbor a state is not an identifier.
static CITY_STATE_ZIP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:[A-Z][a-z]+ ){0,2}[A-Z][a-z]+,\s+",
        r"(?P<state>AL|AK|AZ|AR|CA|CO|CT|DE|DC|FL|GA|HI|ID|IL|IN|IA|KS|KY|LA|ME|MD|MA|MI|MN|MS|MO",
        r"|MT|NE|NV|NH|NJ|NM|NY|NC|ND|OH|OK|OR|PA|RI|SC|SD|TN|TX|UT|VT|VA|WA|WV|WI|WY)\b",
        r"(?P<zip>\s+\d{5}(?:-\d{4})?\b)?",
    ))
    .expect("city/state/zip pattern is valid")
});

/// A town named right after a street address without a state:
/// `at [ADDRESS] in Dunmore Heights`.
static CITY_AFTER_ADDRESS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?P<lead>\[ADDRESS\](?:,\s*\[ADDRESS\])?\s+in\s+)(?:[A-Z][a-z]+ ){0,2}[A-Z][a-z]+\b")
        .expect("city-after-address pattern is valid")
});

/// Phone numbers written to dodge a pattern: one digit at a time
/// (`5 5 5 - 0 1 0 9`, `9 1 7 5 5 5 0 1 3 1`) or spelled out
/// (`five five five, zero one four four`). Seven or ten digits only.
static PHONE_SPACED: LazyLock<Regex> = LazyLock::new(|| {
    let digit_word = r"(?:zero|oh|one|two|three|four|five|six|seven|eight|nine)";
    let pattern = [
        // ten single digits, with or without " - " between groups
        r"\b\d(?:[ ](?:-[ ])?\d){9}\b".to_string(),
        // seven single digits, with or without " - " between groups
        r"\b\d(?:[ ](?:-[ ])?\d){6}\b".to_string(),
        // seven or more number words
        format!(r"(?i)\b{digit_word}(?:[ ,]+{digit_word}){{6,}}\b"),
        // number words mixed with digit groups: five oh three, 555, zero one seven five
        format!(r"(?i)\b{digit_word}(?:[ ,]+(?:{digit_word}|\d{{1,4}})){{5,}}\b"),
    ]
    .join("|");
    Regex::new(&pattern).expect("spaced phone pattern is valid")
});

/// A home described by a landmark instead of an address:
/// `the yellow house across from the old Fairmont cannery`,
/// `her apartment over the pharmacy on Kessler Street`,
/// `three doors down from the Lotus Garden restaurant on Ferrow Avenue`.
///
/// It needs a dwelling word followed, in the same clause, by a word that
/// places it relative to something else. "On room air" has the first but not
/// the second, so it is left alone. The whole rest of the clause is removed.
static LANDMARK_HOME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        // Adjectives are only taken after a determiner ("the yellow house"), so
        // ordinary words before the dwelling ("back to her apartment") stay.
        r"(?:(?P<det>\b(?:the|a|an|her|his|their|my|our)[ ])(?:[a-z]+[ ]){0,2}",
        r"|\b(?:one|two|three|four|five|six|several)[ ])?",
        r"\b(?:house|home|apartment|duplex|two-flat|three-flat|townhouse|bungalow|trailer park|trailer",
        r"|room|flat|condo|cottage|cabin|farmhouse|doors?)[ ]",
        r"(?:[a-z]+[ ]){0,2}?",
        r"(?:across from|over|behind|off|above|down from|next to|beside|opposite|near|past",
        r"|on the corner of|at the end of)[ ]",
        r"(?:St\.|[^.;,\n])+",
    ))
    .expect("landmark pattern is valid")
});

/// Someone's "place" described by a landmark: `the cousin's place above the
/// nail salon across from Greeley Park`. It needs an owner in front, so
/// "left in place over the weekend" is not touched.
static LANDMARK_PLACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?P<det>(?:(?:the|a)[ ])?(?:[a-z]+'s|her|his|their|my|our)[ ])place[ ]",
        r"(?:across from|over|behind|off|above|down from|next to|beside|opposite|near|past",
        r"|on the corner of|at the end of)[ ]",
        r"(?:St\.|[^.;,\n])+",
    ))
    .expect("landmark place pattern is valid")
});

/// A form line, `Label: value`. When the label names an identifier and nothing
/// earlier recognized the value, the whole value is replaced. This is what
/// catches an address with an unusual street word or an ID in an unknown
/// format: the label says what it is even when the shape does not.
static FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?m)^(?P<lead>[ \t]*(?P<label>[A-Za-z][A-Za-z0-9 .'/#-]{0,28}?)[ \t]*:[ \t]*)",
        r"(?P<value>[^\r\n]*?[^ \t\r\n])(?P<tail>[ \t]*\r?)$",
    ))
    .expect("field pattern is valid")
});

fn field_placeholder(label: &str) -> Option<&'static str> {
    let normalized: String = label
        .to_lowercase()
        .chars()
        .filter(|c| !matches!(c, '.' | '#' | '\''))
        .collect();
    let normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
    Some(match normalized.as_str() {
        "address" | "street" | "street address" | "mailing address" | "home address"
        | "residence" => "[ADDRESS]",
        "city" | "town" => "[CITY]",
        "zip" | "zip code" | "zipcode" | "postal code" => "[ZIP]",
        "ssn" | "soc sec" | "social security" | "social security number"
        | "social security no" => "[SSN]",
        "drivers" | "drivers license" | "driver license" | "drivers license number" | "dl"
        | "license" | "license number" | "passport" | "passport number" | "passport no" => {
            "[ID_NUMBER]"
        }
        "mrn" | "acct" | "account" | "account number" | "account no" | "chart"
        | "chart number" | "chart no" | "medical record number" | "record number" => "[MRN]",
        "subscriber id" | "member id" | "policy number" | "policy no" | "insurance id"
        | "health plan no" | "health plan number" | "health plan id" | "group number" => {
            "[INSURANCE_ID]"
        }
        "phone" | "tel" | "telephone" | "cell" | "mobile" | "home phone" | "work phone" | "fax"
        | "contact number" => "[PHONE]",
        "email" | "e-mail" => "[EMAIL]",
        "dob" | "birthdate" | "birth date" | "date of birth" => "[DATE]",
        _ => return None,
    })
}

/// A comma-separated piece of an address line that needs nothing more done:
/// a placeholder, or a state with or without its masked ZIP.
static ADDRESS_PIECE_DONE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\[[A-Z_]+\]|[A-Z]{2}(?: \[ZIP\])?)$").expect("address piece pattern is valid")
});

fn mask_field(caps: &regex::Captures) -> String {
    let value = &caps["value"];
    match field_placeholder(&caps["label"]) {
        // An address line that was only partly recognized, e.g. the unit but
        // not a street with an unusual suffix: finish the remaining pieces.
        Some("[ADDRESS]") if value.contains('[') => {
            let pieces: Vec<&str> = value
                .split(", ")
                .map(|piece| {
                    if ADDRESS_PIECE_DONE.is_match(piece.trim()) {
                        piece
                    } else {
                        "[ADDRESS]"
                    }
                })
                .collect();
            format!("{}{}{}", &caps["lead"], pieces.join(", "), &caps["tail"])
        }
        // A value that already holds a placeholder was handled by a more precise rule.
        Some(placeholder) if !value.contains('[') => {
            format!("{}{placeholder}{}", &caps["lead"], &caps["tail"])
        }
        _ => caps[0].to_string(),
    }
}

fn mask_city_state_zip(caps: &regex::Captures) -> String {
    let zip = if caps.name("zip").is_some() { " [ZIP]" } else { "" };
    format!("[CITY], {}{zip}", &caps["state"])
}

pub fn process(text: &str) -> String {
    // Order matters: the most specific shapes go first so a later, looser rule
    // never sees part of an identifier that has already been replaced.
    let out = EMAIL.replace_all(text, "[EMAIL]");
    let out = SSN.replace_all(&out, "[SSN]");
    let out = MRN.replace_all(&out, "${label}${join}[MRN]");
    let out = INSURANCE_ID.replace_all(&out, "${label}${join}[INSURANCE_ID]");
    let out = DATE.replace_all(&out, "[DATE]");
    let out = PHONE.replace_all(&out, "[PHONE]");
    let out = PHONE_SPACED.replace_all(&out, "[PHONE]");
    let out = LANDMARK_HOME.replace_all(&out, "${det}[LOCATION]");
    let out = LANDMARK_PLACE.replace_all(&out, "${det}[LOCATION]");
    let out = STREET.replace_all(&out, "[ADDRESS]");
    let out = UNIT.replace_all(&out, "[ADDRESS]");
    let out = CITY_STATE_ZIP.replace_all(&out, mask_city_state_zip);
    let out = CITY_AFTER_ADDRESS.replace_all(&out, "${lead}[CITY]");
    let out = FIELD.replace_all(&out, mask_field);
    crate::names::mask(&out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_without_identifiers_is_unchanged() {
        assert_eq!(process("hello test"), "hello test");
        assert_eq!(process("line one\nline two"), "line one\nline two");
        assert_eq!(process(""), "");
    }

    #[test]
    fn masks_common_phone_formats() {
        for phone in [
            "(617) 555-0142",
            "415-555-0117",
            "206.555.0108",
            "+1 303 555 0154",
            "(518) 555 0118",
            "555-0198",
            "555.0177",
        ] {
            let out = process(&format!("Phone: {phone} (home)"));
            assert_eq!(out, "Phone: [PHONE] (home)", "format: {phone}");
        }
    }

    #[test]
    fn masks_every_phone_in_a_sentence() {
        let out = process("Call (312) 555-0165 or (312) 555-0190 after 5 pm.");
        assert_eq!(out, "Call [PHONE] or [PHONE] after 5 pm.");
    }

    #[test]
    fn masks_emails() {
        assert_eq!(
            process("Email: marcus.delgado@example.com"),
            "Email: [EMAIL]"
        );
        assert_eq!(
            process("results by email at bao.ng88@example.com or on his cell"),
            "results by email at [EMAIL] or on his cell"
        );
        assert_eq!(process("email s.oyelaran@example.net."), "email [EMAIL].");
    }

    #[test]
    fn masks_ssns() {
        assert_eq!(process("SSN: 987-65-4321"), "SSN: [SSN]");
        assert_eq!(
            process("social security number 987-65-4329 for enrollment"),
            "social security number [SSN] for enrollment"
        );
        assert_eq!(process("his SSN as 000 12 3456."), "his SSN as [SSN].");
    }

    #[test]
    fn masks_record_numbers_after_a_label() {
        assert_eq!(process("MRN: 00482917"), "MRN: [MRN]");
        assert_eq!(
            process("his record number is 55801276."),
            "his record number is [MRN]."
        );
        assert_eq!(process("Chart number 60917734."), "Chart number [MRN].");
        assert_eq!(
            process("Old chart no. was 4471-208-93."),
            "Old chart no. was [MRN]."
        );
    }

    #[test]
    fn masks_insurance_ids_after_a_label() {
        assert_eq!(
            process("Insurance ID: HX-4490217"),
            "Insurance ID: [INSURANCE_ID]"
        );
    }

    #[test]
    fn labels_without_a_number_are_left_alone() {
        for text in [
            "MRN is not on file.",
            "The chart number was requested from the referring clinic.",
            "Insurance ID pending verification.",
        ] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn masks_phones_written_digit_by_digit_or_in_words() {
        assert_eq!(
            process("Rhea's line: 5 5 5 - 0 1 0 9. Born later."),
            "Rhea's line: [PHONE]. Born later."
        );
        assert_eq!(
            process("text only, 9 1 7 5 5 5 0 1 3 1."),
            "text only, [PHONE]."
        );
        assert_eq!(
            process("ring his cell, five five five, zero one four four, if anything changes"),
            "ring his cell, [PHONE], if anything changes"
        );
    }

    #[test]
    fn leaves_ordinary_number_words_and_spaced_digits_alone() {
        for text in [
            "Pain is 3 out of 10 on Oxycodone 5 mg every 6 hours as needed.",
            "two classmates were recently sick",
            "one or two episodes, three to four times a week",
            "GOLD stage 4, mMRC dyspnea grade 4.",
        ] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn masks_homes_described_by_a_landmark() {
        assert_eq!(
            process("the patient lives in the yellow house across from the old Fairmont cannery, second floor."),
            "the patient lives in the [LOCATION], second floor."
        );
        assert_eq!(
            process("walking back to her apartment over the pharmacy on Kessler Street."),
            "walking back to her [LOCATION]."
        );
        assert_eq!(
            process("in the blue duplex behind St. Brendan's church in Calloway Flats. Best contact"),
            "in the [LOCATION]. Best contact"
        );
        assert_eq!(
            process("They live three doors down from the Lotus Garden restaurant on Ferrow Avenue. Next."),
            "They live [LOCATION]. Next."
        );
    }

    #[test]
    fn leaves_dwelling_words_in_medical_text_alone() {
        for text in [
            "SpO2 95% on room air, respiratory rate 28.",
            "oxygen was weaned to room air by day 3.",
            "3 L/min home oxygen",
            "Rosa offered her ground-floor home instead.",
            "discharged to the home of his son",
        ] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn masks_labelled_fields_whatever_the_value_looks_like() {
        for (input, expected) in [
            ("Address:             139 Krajcik Promenade", "Address:             [ADDRESS]"),
            ("Address: 549 Herman Underpass Unit 54", "Address: [ADDRESS]"),
            ("City:                New Bedford", "City:                [CITY]"),
            ("Zip:                 02746", "Zip:                 [ZIP]"),
            ("Drivers:             S99998773", "Drivers:             [ID_NUMBER]"),
            ("Passport:            X81836749X", "Passport:            [ID_NUMBER]"),
            ("Acct #: 40077213", "Acct #: [MRN]"),
            ("Subscriber ID: QRT-88120457", "Subscriber ID: [INSURANCE_ID]"),
            ("Health plan no.: 7731-0092-6", "Health plan no.: [INSURANCE_ID]"),
            ("Soc Sec: 900-31-4477", "Soc Sec: [SSN]"),
        ] {
            assert_eq!(process(input), expected);
        }
    }

    #[test]
    fn labelled_fields_keep_what_a_precise_rule_already_did() {
        assert_eq!(
            process("Address: 48 Willow Bend Road, Westmarch, MA 02199"),
            "Address: [ADDRESS], [CITY], MA [ZIP]"
        );
        assert_eq!(process("State:               Massachusetts"), "State:               Massachusetts");
    }

    #[test]
    fn medical_form_lines_are_not_identifier_fields() {
        for text in [
            "Diagnosis: Iron deficiency anemia.",
            "Medications: Furosemide 40 mg daily; Lisinopril 10 mg daily.",
            "Results: Hemoglobin 9.8 g/dL, ferritin 6 ng/mL, TSH 2.1 mIU/L.",
            "O: BP 162/104 on two readings. Urine protein 2+. Fetal heart rate 148.",
            "Gender:              M",
        ] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn masks_more_date_and_phone_spellings() {
        assert_eq!(process("Birth date on license: 12.07.1978."), "Birth date on license: [DATE].");
        assert_eq!(process("Born on Christmas Eve, 1936."), "Born on [DATE].");
        assert_eq!(
            process("reached on her mobile, 4 1 2 - 5 5 5 - 0 1 6 2. She says"),
            "reached on her mobile, [PHONE]. She says"
        );
        assert_eq!(
            process("my number changed to five oh three, 555, zero one seven five."),
            "my number changed to [PHONE]."
        );
    }

    #[test]
    fn a_place_needs_an_owner_to_count_as_a_home() {
        assert_eq!(
            process("at the cousin's place above the nail salon across from Greeley Park until the 1st."),
            "at the cousin's [LOCATION]."
        );
        let text = "The catheter was left in place over the weekend.";
        assert_eq!(process(text), text);
    }

    #[test]
    fn masks_full_dates() {
        for (input, expected) in [
            ("DOB: 03/14/1958", "DOB: [DATE]"),
            ("born on 3/8/1967.", "born on [DATE]."),
            ("D.O.B. 1962-05-27", "D.O.B. [DATE]"),
            ("DOB per wristband: 09-06-48.", "DOB per wristband: [DATE]."),
            ("born on April 9, 1953 and lives", "born on [DATE] and lives"),
            ("birthday as March 3rd, '91 and", "birthday as [DATE] and"),
            ("Born 22 Nov 1991.", "Born [DATE]."),
            ("DOB: the fourth of July, 1944.", "DOB: [DATE]."),
        ] {
            assert_eq!(process(input), expected);
        }
    }

    #[test]
    fn leaves_partial_dates_and_date_like_values_alone() {
        for text in [
            "Symptoms began in March.",
            "Diagnosed in 2019.",
            "June returns after cycle 3.",
            "Blood pressure 148/92",
            "Recheck INR in 4 weeks.",
            "scored 19 out of 30",
        ] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn masks_street_addresses() {
        assert_eq!(
            process("Address: 48 Willow Bend Road, Westmarch, MA 02199"),
            "Address: [ADDRESS], [CITY], MA [ZIP]"
        );
        assert_eq!(
            process("Address: 1260 Juniper Court, Apt 4B, Alder Creek, CA 94016"),
            "Address: [ADDRESS], [ADDRESS], [CITY], CA [ZIP]"
        );
        assert_eq!(
            process("lives with him at 902 Alameda Verde Avenue, Santa Rilla, CA."),
            "lives with him at [ADDRESS], [CITY], CA."
        );
        assert_eq!(
            process("lives alone at 18 Larkspur Lane in Dunmore Heights."),
            "lives alone at [ADDRESS] in [CITY]."
        );
    }

    #[test]
    fn leaves_address_like_prose_alone() {
        for text in [
            "known on the unit as a careful patient",
            "Hodgkin lymphoma, stage IIA.",
            "ST elevation in leads II, III and aVF.",
            "Ceftriaxone 1 g IV daily",
            "ice 20 minutes three times daily",
            "Advised a Mediterranean diet",
            "Follow-up in Cardiology Clinic",
        ] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn leaves_other_numbers_alone() {
        for text in [
            "Carbidopa-Levodopa 25-100 mg three times daily",
            "Blood pressure 148/92",
            "Metformin 1000 mg twice daily",
            "PHQ-9 score 14, improved from 19.",
            "PCL-5 score is 52.",
            "eGFR is 22 mL/min",
            "Troponin I is 3.2 ng/mL",
            "ESR is 42 mm/hr",
        ] {
            assert_eq!(process(text), text);
        }
    }
}
