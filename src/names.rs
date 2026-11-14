//! Names of patients and of the people around them.
//!
//! A name has no fixed shape, so it cannot be matched on its own the way a
//! phone number can. This works in two passes:
//!
//! 1. **Find** names from the context that introduces them: a form label
//!    (`Name: Eleanor Carter`), a title (`Ms. Carter`), a family word
//!    (`her daughter Mary`), or an introducing phrase (`Spoke with Naomi
//!    Feldstein`, `goes by June`).
//! 2. **Remove** every later mention of each name found, including a first
//!    name or surname used alone (`If Grace cannot be reached`,
//!    `Hector's date of birth`).
//!
//! A capitalized word that is never introduced as a person is left alone, which
//! is what keeps `Parkinson's disease` or `Montreal Cognitive Assessment` intact.

use regex::Regex;
use std::sync::LazyLock;

/// One capitalized name word, optionally hyphenated: `Carter`, `Abernathy-Cole`.
const WORD: &str = r"[A-Z][a-z]+(?:-[A-Z][a-z]+)*";
/// An initial with its full stop: `T.`
const INIT: &str = r"[A-Z]\.";
const SUFFIX: &str = r"(?:[ ](?:Jr|Sr|II|III)\.?)?";

/// One to three name words with optional initials: `Mary`, `Eleanor Carter`,
/// `Dorothea M. Vance`, `James T. Holloway Jr.`
fn name() -> String {
    format!(r"(?:{INIT}[ ]){{0,2}}{WORD}(?:[ ]{INIT})?(?:[ ]{WORD}){{0,2}}{SUFFIX}")
}

/// A name that is unlikely to be an ordinary capitalized word: at least two
/// name words, or initials followed by a surname (`J. R. Abernathy-Cole`).
fn strong_name() -> String {
    format!(
        r"(?:(?:{INIT}[ ]){{1,2}}{WORD}|(?:{INIT}[ ]){{0,2}}{WORD}(?:[ ]{INIT})?(?:[ ]{WORD}){{1,2}}){SUFFIX}"
    )
}

const KIN: &str = concat!(
    r"daughter-in-law|son-in-law|mother-in-law|father-in-law|sister-in-law|brother-in-law",
    r"|granddaughter|grandson|grandmother|grandfather|stepmother|stepfather|stepson|stepdaughter",
    r"|daughter|son|wife|husband|brother|sister|mother|father|nephew|niece|uncle|aunt|cousin",
    r"|partner|fiancee|fiance|girlfriend|boyfriend|neighbor|roommate|caregiver|guardian",
);

/// Patterns that introduce a person. Each captures the name as `n`.
static INTRODUCTIONS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    let name = name();
    let strong = strong_name();
    [
        // Name: Eleanor Carter / Emergency Contact: Anil Raman (husband)
        format!(
            r"(?m)^[ \t]*(?:Patient Name|Name|Patient|Parent/Guardian|Guardian|Emergency Contact|Next of Kin|Spouse)[ \t]*:[ \t]*(?P<n>{name})"
        ),
        // Ms. Carter / Mr. Holloway
        format!(r"\b(?:Mr|Mrs|Ms|Miss|Mx)\.?[ ](?P<n>{name})"),
        // her daughter Mary / his son, Peter Kowalczyk / Mother, Farah Haddad
        format!(r"\b(?i:{KIN}),?[ ](?P<n>{name})"),
        // Spoke with Naomi Feldstein / goes by June / first name Malik
        format!(
            r#"\b(?:Spoke with|Met with|Visited|first name|legal name|goes by|called|name is)[ ]"?(?P<n>{name})"#
        ),
        // whom staff know as Dottie / known on the unit as "Big Jim"
        format!(r#"\bknow[n]?(?:[ ][a-z]+){{0,4}}[ ]as[ ]"?(?P<n>{name})"#),
        // Patient is Hector Salazar
        format!(r"\b(?:Patient|Pt) is[ ](?P<n>{strong})"),
        // Grace Whitfield is a 72-year-old woman
        format!(r"(?P<n>{strong})[ ]is[ ]an?[ ]\d"),
        // Samuel Oyelaran, 58, is scheduled
        format!(r"(?P<n>{strong}),[ ]\d{{1,3}},[ ]"),
        // This 81-year-old man, Bernard Kowalczyk, was admitted
        format!(
            r"\b(?:man|woman|male|female|boy|girl|gentleman|lady|veteran|patient),[ ](?P<n>{strong}),"
        ),
        // J. R. Abernathy-Cole (goes by June) / (James T. Holloway Jr.)
        format!(r"(?P<n>{strong})[ ]\((?:goes by|known as|aka|legal name)\b"),
        format!(r"\((?P<n>{strong})\)"),
    ]
    .iter()
    .map(|p| Regex::new(p).expect("introduction pattern is valid"))
    .collect()
});

/// A surname spelled out letter by letter: `last name spelled O-S-E-I`.
static SPELLED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bspelled[ ](?P<s>(?:[A-Z]-)+[A-Z])\b").expect("spelled pattern is valid")
});

/// `NGUYEN, Thi Lan`: an upper-case surname, a comma, then given names.
static SURNAME_FIRST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"\b(?P<s>[A-Z]{{3,}}),[ ](?P<g>{WORD}(?:[ ]{WORD})?)\b"))
        .expect("surname-first pattern is valid")
});

static NAME_WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!("^{WORD}$")).expect("word pattern is valid"));

/// Capitalized words that can follow an introducing phrase without being a name.
const NOT_A_NAME: &[&str] = &[
    "The", "A", "An", "He", "She", "His", "Her", "Him", "They", "Their", "Patient", "Pt", "Mr",
    "Mrs", "Ms", "Miss", "Mx", "Dr", "Mother", "Father", "Son", "Daughter", "Wife", "Husband",
    "Brother", "Sister", "Emergency", "Contact", "Name", "Phone", "Address", "Date", "Birth",
    "No", "Yes", "Not", "None", "Unknown", "Jr", "Sr", "In", "On", "At", "Is", "Was", "Has",
    "And", "Or", "With", "Will", "Can",
];

fn title_case(upper: &str) -> String {
    let mut chars = upper.chars();
    match chars.next() {
        Some(first) => first.to_string() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}

fn whole_word(word: &str) -> Regex {
    Regex::new(&format!(r"\b{}\b", regex::escape(word))).expect("escaped word is a valid pattern")
}

/// Everything in `text` that should be removed as a name: whole phrases first,
/// then the individual name words inside them.
struct Found {
    /// Literal strings removed wherever they appear, longest first.
    phrases: Vec<String>,
    /// Single name words removed wherever they appear as a whole word.
    words: Vec<String>,
}

fn find(text: &str) -> Found {
    let mut phrases: Vec<String> = Vec::new();
    let mut words: Vec<String> = Vec::new();

    let add_word = |words: &mut Vec<String>, w: &str| {
        if NAME_WORD.is_match(w) && !NOT_A_NAME.contains(&w) && !words.iter().any(|x| x == w) {
            words.push(w.to_string());
        }
    };

    for pattern in INTRODUCTIONS.iter() {
        for caps in pattern.captures_iter(text) {
            let phrase = caps["n"].trim();
            let tokens: Vec<&str> = phrase.split(' ').collect();
            let before = words.len();
            for token in &tokens {
                add_word(&mut words, token);
            }
            // Keep the phrase only if it contributed or repeats a real name word.
            let has_name_word = tokens
                .iter()
                .any(|t| words.iter().any(|w| w == t));
            if has_name_word || words.len() > before {
                if tokens.len() > 1 && !phrases.iter().any(|p| p == phrase) {
                    phrases.push(phrase.to_string());
                }
                // James T. Holloway -> also "J.T." and "J. T."
                if tokens.len() >= 3
                    && tokens[1].len() == 2
                    && tokens[1].ends_with('.')
                    && let Some(first) = tokens[0].chars().next()
                {
                    let middle = &tokens[1][..1];
                    phrases.push(format!("{first}.{middle}."));
                    phrases.push(format!("{first}. {middle}."));
                }
            }
        }
    }

    for caps in SPELLED.captures_iter(text) {
        let spelled = &caps["s"];
        phrases.push(spelled.to_string());
        add_word(&mut words, &title_case(&spelled.replace('-', "")));
    }

    for caps in SURNAME_FIRST.captures_iter(text) {
        let surname = title_case(&caps["s"]);
        // Only when the same surname also appears in ordinary capitalization,
        // so "COPD, Albuterol" is never read as a name.
        if whole_word(&surname).is_match(text) {
            phrases.push(caps[0].to_string());
            phrases.push(caps["s"].to_string());
            add_word(&mut words, &surname);
            for given in caps["g"].split(' ') {
                add_word(&mut words, given);
            }
        }
    }

    phrases.sort_by_key(|p| std::cmp::Reverse(p.len()));
    phrases.dedup();
    Found { phrases, words }
}

/// Replaces every name found in `text` with `[NAME]`.
pub fn mask(text: &str) -> String {
    let found = find(text);
    let mut out = text.to_string();
    for phrase in &found.phrases {
        out = out.replace(phrase.as_str(), "[NAME]");
    }
    for word in &found.words {
        out = whole_word(word).replace_all(&out, "[NAME]").into_owned();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labelled_names() {
        assert_eq!(mask("Name: Eleanor Carter\nDOB: x"), "Name: [NAME]\nDOB: x");
        assert_eq!(mask("Patient Name: Priya Raman"), "Patient Name: [NAME]");
        assert_eq!(
            mask("Emergency Contact: Anil Raman (husband), x"),
            "Emergency Contact: [NAME] (husband), x"
        );
        assert_eq!(
            mask("Parent/Guardian: Chinedu Okafor\nGuardian Phone: x"),
            "Parent/Guardian: [NAME]\nGuardian Phone: x"
        );
    }

    #[test]
    fn a_label_does_not_swallow_the_next_line() {
        assert_eq!(
            mask("Patient: Amara Okafor\nDate of Birth: x"),
            "Patient: [NAME]\nDate of Birth: x"
        );
    }

    #[test]
    fn names_introduced_in_a_sentence_and_repeated_later() {
        assert_eq!(
            mask("Grace Whitfield is a 72-year-old woman. If Grace cannot be reached, call."),
            "[NAME] is a 72-year-old woman. If [NAME] cannot be reached, call."
        );
        assert_eq!(
            mask("Patient is Hector Salazar, a 45-year-old man. Hector's record is here."),
            "Patient is [NAME], a 45-year-old man. [NAME]'s record is here."
        );
        assert_eq!(
            mask("Samuel Oyelaran, 58, is scheduled. Samuel was born in spring."),
            "[NAME], 58, is scheduled. [NAME] was born in spring."
        );
        assert_eq!(
            mask("This 81-year-old man, Bernard Kowalczyk, was admitted."),
            "This 81-year-old man, [NAME], was admitted."
        );
    }

    #[test]
    fn relatives() {
        assert_eq!(
            mask("Her daughter Mary called. His son, Peter Kowalczyk, came. Peter's phone."),
            "Her daughter [NAME] called. His son, [NAME], came. [NAME]'s phone."
        );
        assert_eq!(
            mask("Mother, Farah Haddad, brings in her 4-year-old son Zayd. Father Karim is away."),
            "Mother, [NAME], brings in her 4-year-old son [NAME]. Father [NAME] is away."
        );
    }

    #[test]
    fn titles_and_initials() {
        assert_eq!(
            mask("Visited Ms. Carter today. E. Carter is recovering. Mrs. Carter's neighbor."),
            "Visited Ms. [NAME] today. E. [NAME] is recovering. Mrs. [NAME]'s neighbor."
        );
    }

    #[test]
    fn nicknames_and_spelled_surnames() {
        assert_eq!(
            mask("staff know as Dottie (legal name Dorothea M. Vance). D. Vance's son."),
            "staff know as [NAME] (legal name [NAME]). D. [NAME]'s son."
        );
        assert_eq!(
            mask("first name Malik, last name spelled O-S-E-I. Called Mal by Mrs. Osei."),
            "first name [NAME], last name spelled [NAME]. Called Mal by Mrs. [NAME]."
        );
    }

    #[test]
    fn eponyms_and_other_capitalized_terms_are_not_names() {
        for text in [
            "Diagnosis: Parkinson's disease, Hoehn and Yahr stage 2.",
            "scored 19 out of 30 on the Montreal Cognitive Assessment.",
            "Columbia Suicide Severity Rating Scale negative.",
            "Hodgkin lymphoma, stage IIA. Deauville score of 2.",
            "Homans sign negative. Troponin I is 3.2 ng/mL.",
            "Advised a Mediterranean diet. Started Atorvastatin 20 mg nightly.",
            "The son reports she got lost. Her mother called the clinic.",
            "Patient is alert and oriented. Patient is Afebrile.",
        ] {
            assert_eq!(mask(text), text);
        }
    }
}
