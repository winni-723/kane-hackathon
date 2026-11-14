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

pub fn process(text: &str) -> String {
    PHONE.replace_all(text, "[PHONE]").into_owned()
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
    fn leaves_other_numbers_alone() {
        for text in [
            "SSN: 987-65-4321",
            "MRN: 00482917",
            "DOB: 03/14/1958",
            "D.O.B. 1962-05-27",
            "Carbidopa-Levodopa 25-100 mg three times daily",
            "Blood pressure 148/92",
            "Metformin 1000 mg twice daily",
            "Westmarch, MA 02199",
            "Old chart no. was 4471-208-93.",
        ] {
            assert_eq!(process(text), text);
        }
    }
}
