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

/// One capitalized name word: `Carter`, `Abernathy-Cole`, `O'Sullivan`,
/// `DeKalb`, `Adán`. Letters are matched by Unicode class, not just A-Z.
const WORD: &str =
    r"(?:\p{Lu}['’])?\p{Lu}\p{Ll}+(?:\p{Lu}\p{Ll}+)?(?:-\p{Lu}\p{Ll}+)*";
/// An initial with its full stop: `T.`
const INIT: &str = r"[A-Z]\.";
/// `Jr.` and `Sr.` take their full stop; a Roman numeral does not, so the
/// sentence's own full stop after `Whitmore III.` is left in place.
const SUFFIX: &str = r"(?:[ ](?:(?:Jr|Sr)\b\.?|(?:III|II|IV)\b))?";

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
    r"|eldest|youngest|oldest|widow|widower|friend|coworker|landlord",
);

/// Courtesy and professional titles. A name after any of these is removed,
/// including `Dr.`: a doctor named in a record may be a relative, and removing
/// a clinician's name loses no medical content.
const TITLE: &str = r"(?:Mr|Mrs|Ms|Miss|Mx|Dr|Judge|Rev|Fr|Prof|Capt|Sgt|Col|Lt)\.?";

/// Labels of form fields whose value is a person's name.
const NAME_LABEL: &str = concat!(
    r"Patient Name|Full Name|Last Name|First Name|Middle Name|Given Name|Surname|Name|Patient|Pt",
    r"|Client|Member|Subscriber|Insured|Guarantor|Parent/Guardian|Guardian|Emergency Contact",
    r"|Emergency contact|Next of Kin|Next of kin|Spouse|Resident|Re|RE",
);

/// Where a name label may begin: at the start of a line, in the next cell of a
/// table row, after a `|` in a page footer, or after the previous sentence.
const LABEL_START: &str = r"(?:^|[\t|]|[.;][ ])[ \t]*";

/// Letters after a clinician's name: `Samuel Ortega, MD`, `B. Tran, RN`.
/// Bare `PA`, `OD`, `PT` and `OT` are left out: they are also a chest X-ray
/// view, the right eye, and two kinds of therapy.
const CREDENTIAL: &str = concat!(
    r"MD|M\.D\.|DO|D\.O\.|NP|RN|PA-C|LPN|CNA|CNP|APRN|DNP|FNP-C|FNP|PharmD|MBBS|CRNA|LCSW|PhD",
    r"|DDS|DMD|DPM|MSW|CNM",
);

/// Short forms of given names. When a record names `Thomas`, a later `Tom` is
/// the same person even though nothing introduces him. Short forms that are
/// also ordinary words in a clinical note (`Will`, `Ray` as in X-Ray, `Drew`,
/// `Rich`, `Art`, `Al`) are left out.
const SHORT_FORMS: &[(&str, &[&str])] = &[
    ("Thomas", &["Tom", "Tommy"]),
    ("William", &["Bill", "Billy", "Liam"]),
    ("Robert", &["Bob", "Bobby", "Rob", "Robbie"]),
    ("Richard", &["Rick", "Ricky", "Dick"]),
    ("James", &["Jim", "Jimmy", "Jamie"]),
    ("John", &["Johnny", "Jack"]),
    ("Joseph", &["Joe", "Joey"]),
    ("Michael", &["Mike", "Mikey"]),
    ("Charles", &["Charlie", "Chuck"]),
    ("Edward", &["Ed", "Eddie", "Ted"]),
    ("Theodore", &["Ted", "Theo"]),
    ("Daniel", &["Dan", "Danny"]),
    ("Matthew", &["Matt"]),
    ("Christopher", &["Chris"]),
    ("Anthony", &["Tony"]),
    ("Andrew", &["Andy"]),
    ("Steven", &["Steve"]),
    ("Stephen", &["Steve"]),
    ("Kenneth", &["Ken", "Kenny"]),
    ("Timothy", &["Tim"]),
    ("Ronald", &["Ron", "Ronnie"]),
    ("Donald", &["Donnie"]),
    ("Benjamin", &["Ben", "Benny"]),
    ("Samuel", &["Sam", "Sammy"]),
    ("Nicholas", &["Nick"]),
    ("Patrick", &["Pat", "Paddy"]),
    ("Gregory", &["Greg"]),
    ("Lawrence", &["Larry"]),
    ("Gerald", &["Gerry", "Jerry"]),
    ("Frederick", &["Fred", "Freddie"]),
    ("Alexander", &["Alex"]),
    ("Jonathan", &["Jon"]),
    ("Joshua", &["Josh"]),
    ("Harold", &["Hal", "Harry"]),
    ("Henry", &["Hank", "Harry"]),
    ("Walter", &["Walt", "Wally"]),
    ("Albert", &["Bert"]),
    ("Leonard", &["Len", "Lenny", "Leo"]),
    ("Vincent", &["Vince", "Vinny"]),
    ("Philip", &["Phil"]),
    ("Phillip", &["Phil"]),
    ("Douglas", &["Doug"]),
    ("Peter", &["Pete"]),
    ("Nathaniel", &["Nate", "Nat"]),
    ("Nathan", &["Nate"]),
    ("Zachary", &["Zach", "Zack"]),
    ("Margaret", &["Peggy", "Maggie", "Meg", "Marge"]),
    ("Elizabeth", &["Liz", "Lizzie", "Beth", "Betty", "Betsy", "Eliza"]),
    ("Katherine", &["Kate", "Katie", "Kathy", "Kat"]),
    ("Catherine", &["Cathy", "Kate", "Katie"]),
    ("Kathleen", &["Kathy", "Katie"]),
    ("Patricia", &["Pat", "Patty", "Trish", "Tricia"]),
    ("Jennifer", &["Jen", "Jenny"]),
    ("Barbara", &["Barb"]),
    ("Susan", &["Sue", "Susie"]),
    ("Deborah", &["Deb", "Debbie"]),
    ("Dorothy", &["Dottie"]),
    ("Rebecca", &["Becky", "Becca"]),
    ("Jessica", &["Jess", "Jessie"]),
    ("Christine", &["Chris", "Chrissy"]),
    ("Christina", &["Chris", "Tina"]),
    ("Victoria", &["Vicky", "Tori"]),
    ("Virginia", &["Ginny"]),
    ("Jacqueline", &["Jackie"]),
    ("Cynthia", &["Cindy"]),
    ("Pamela", &["Pam"]),
    ("Sandra", &["Sandy"]),
    ("Kimberly", &["Kim"]),
    ("Stephanie", &["Steph"]),
    ("Samantha", &["Sam"]),
    ("Abigail", &["Abby"]),
    ("Eleanor", &["Ellie", "Nora"]),
    ("Josephine", &["Josie"]),
    ("Gertrude", &["Trudy"]),
    ("Mildred", &["Millie"]),
];

/// Patterns that introduce a person. Each captures the name as `n`.
static INTRODUCTIONS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    let name = name();
    let strong = strong_name();
    [
        // Name: Eleanor Carter / Emergency Contact: Anil Raman (husband)
        format!(r"(?m){LABEL_START}(?:{NAME_LABEL})[ \t]*:[ \t]*(?:{TITLE}[ ])?(?P<n>{name})"),
        // The same label and its value in two table cells. A table header such
        // as "Name | Dose" has the same shape, so a full name is required.
        format!(r"(?m){LABEL_START}(?:{NAME_LABEL})\t(?:{TITLE}[ ])?(?P<n>{strong})"),
        // A contact line: Daughter: Siobhan Reyes-Whitaker, cell [PHONE]
        // The phone number is what tells it from "Mother: Breast Cancer".
        format!(r"\b(?i:{KIN}):[ ](?P<n>{strong})[^\r\n\t]{{0,30}}\[PHONE\]"),
        // Samuel Ortega, MD / B. Tran, RN | Page 1 / Renee Castillo, DO
        // Not "Metoprolol Succinate, DO NOT CRUSH": no capital may follow.
        format!(r"(?m)(?P<n>{strong}),[ ](?:{CREDENTIAL})(?:[,.;:)\t|]|[ ][^A-Z\r\n]|[ ]*\r?$)"),
        // A page headed by the name and then the birth date:
        //   Aisha Rahman    Date of birth: [DATE]
        format!(r"(?m)^(?P<n>{strong})[ \t,|]+(?:Date of [Bb]irth|DOB|D\.O\.B\.)"),
        // prefers "Yuki"
        format!(r#"\b(?:prefers|preferred name(?: is)?:?)[ ]"(?P<n>{WORD})""#),
        // A record headed by the person's name, underlined, then demographics:
        //   Andy Howe
        //   =========
        //   Race: ...
        format!(
            r"(?m)^(?P<n>{strong})[ \t]*\r?\n={{2,}}[^\r\n]*\r?\n(?:Race|Ethnicity|Gender|Sex|Age|Birth|DOB)\b"
        ),
        // Ms. Carter / Mr. Holloway / Mrs. Judge Fenwick / Dr. Priscilla Fenwick-Araujo
        format!(r"\b(?:{TITLE}[ ]){{1,2}}(?P<n>{name})"),
        // her daughter Mary / his son, Peter Kowalczyk / Mother, Farah Haddad
        format!(r"\b(?i:{KIN}),?[ ](?P<n>{name})"),
        // Spoke with Naomi Feldstein / goes by June / first name Malik
        format!(
            r#"\b(?:Spoke with|Met with|Visited|first name|legal name|given name is|goes by|answers to|called|calls|nicknamed|name is|reads)[ ]"?(?P<n>{name})"#
        ),
        // reviewed with Margaret and with Declan, who will help
        format!(r"\bwith[ ](?P<n>{name}),[ ]who\b"),
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
            r"\b(?i:man|woman|male|female|boy|girl|gentleman|lady|veteran|patient),[ ](?P<n>{strong}),"
        ),
        // J. R. Abernathy-Cole (goes by June) / (James T. Holloway Jr.) / (Tran Thi Bao, you saw me)
        format!(r"(?P<n>{strong})[ ]\((?:goes by|known as|aka|legal name)\b"),
        format!(r"\((?P<n>{strong})[,)]"),
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

/// `Resident: O'Connell-Reyes, Margaret` or, in a table, `Patient | NAKAMURA,
/// Yuki A.`: after a name label the surname-first order needs no further proof.
static LABELLED_SURNAME_FIRST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?m){LABEL_START}(?:{NAME_LABEL})(?:[ \t]*:[ \t]*|\t)(?P<s>{WORD}|[A-Z][A-Z'’-]{{2,}}),[ ](?P<g>{WORD})"
    ))
    .expect("labelled surname-first pattern is valid")
});

/// A nickname in quotation marks right after a name: `Margaret ("Peggy")`.
static QUOTED_NICKNAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r#"(?P<prev>{WORD})[ ]\(?["“](?P<n>{WORD})["”]"#))
        .expect("quoted nickname pattern is valid")
});

static NAME_WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!("^{WORD}$")).expect("word pattern is valid"));

/// Capitalized words that can follow an introducing phrase without being a name.
const NOT_A_NAME: &[&str] = &[
    "The", "A", "An", "He", "She", "His", "Her", "Him", "They", "Their", "Patient", "Pt", "Mr",
    "Mrs", "Ms", "Miss", "Mx", "Dr", "Judge", "Rev", "Fr", "Prof", "Capt", "Sgt", "Col", "Lt",
    "Mother", "Father", "Son", "Daughter", "Wife", "Husband",
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

/// Matches a name word on its own, and as a family plural (`the Whitmores`).
fn whole_word(word: &str) -> Regex {
    Regex::new(&format!(r"\b{}(?:e?s)?\b", regex::escape(word)))
        .expect("escaped word is a valid pattern")
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

    for caps in LABELLED_SURNAME_FIRST.captures_iter(text) {
        let surname = &caps["s"];
        if surname.chars().any(|c| c.is_lowercase()) {
            add_word(&mut words, surname);
        } else {
            phrases.push(surname.to_string());
            add_word(&mut words, &title_case(surname));
        }
        add_word(&mut words, &caps["g"]);
    }

    for caps in QUOTED_NICKNAME.captures_iter(text) {
        if words.iter().any(|w| w == &caps["prev"]) {
            add_word(&mut words, &caps["n"]);
        }
    }

    for (full, short_forms) in SHORT_FORMS {
        if words.iter().any(|w| w == full) {
            for short in *short_forms {
                add_word(&mut words, short);
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
    fn apostrophes_accents_and_inner_capitals() {
        assert_eq!(
            mask("Patient, Margaret O'Sullivan, lives with her husband. Reviewed with Margaret."),
            "Patient, [NAME], lives with her husband. Reviewed with [NAME]."
        );
        assert_eq!(
            mask("Adán Arroyo\n==á========\nRace:                White"),
            "[NAME]\n==á========\nRace:                White"
        );
        assert_eq!(
            mask("Wallet ID reads DeKalb R. Whitmore III. The Whitmores rent upstairs."),
            "Wallet ID reads [NAME]. The [NAME] rent upstairs."
        );
    }

    #[test]
    fn more_ways_a_person_is_introduced() {
        assert_eq!(
            mask("Last Name: Brennan\nFirst Name: Oliver\nNext of Kin: Siobhan Brennan (spouse)"),
            "Last Name: [NAME]\nFirst Name: [NAME]\nNext of Kin: [NAME] (spouse)"
        );
        assert_eq!(mask("His eldest, Fatima, phoned."), "His eldest, [NAME], phoned.");
        assert_eq!(
            mask("Her niece, Dr. Priscilla Fenwick-Araujo, holds power of attorney."),
            "Her niece, Dr. [NAME], holds power of attorney."
        );
        assert_eq!(
            mask("her given name is Wilhelmina, though she answers to Billie."),
            "her given name is [NAME], though she answers to [NAME]."
        );
    }

    #[test]
    fn an_underlined_heading_is_not_a_name_without_demographics() {
        let text = "Discharge Summary\n=================\nAdmitted with pneumonia.";
        assert_eq!(mask(text), text);
    }

    #[test]
    fn introducing_words_before_medical_terms_are_not_names() {
        for text in [
            "Referred for Physical Therapy after surgery.",
            "prescribed for Childhood asthma (disorder)",
            "Loaded with Levetiracetam 1500 mg IV.",
            "Start Magnesium sulfate 4 g IV loading dose.",
            "Evaluation for implantable cardioverter-defibrillator.",
        ] {
            assert_eq!(mask(text), text);
        }
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

    #[test]
    fn names_in_table_cells_and_page_footers() {
        assert_eq!(mask("Patient Name\tDorothy Kessler\tMRN\tx"), "Patient Name\t[NAME]\tMRN\tx");
        assert_eq!(
            mask("Patient\tNAKAMURA, Yuki A. (prefers \"Yuki\")\nYuki says"),
            "Patient\t[NAME], [NAME] A. (prefers \"[NAME]\")\n[NAME] says"
        );
        assert_eq!(
            mask("Transfer form | Resident: O'Connell-Reyes, Margaret \"Peggy\" | x\nPeggy fell."),
            "Transfer form | Resident: [NAME], [NAME] \"[NAME]\" | x\n[NAME] fell."
        );
        assert_eq!(mask("Aisha Rahman    Date of birth: x"), "[NAME]    Date of birth: x");
        assert_eq!(
            mask("Mobile: x. Emergency contact: Farid Rahman (father), y"),
            "Mobile: x. Emergency contact: [NAME] (father), y"
        );
    }

    #[test]
    fn clinicians_named_by_their_credentials() {
        assert_eq!(mask("Electronically signed by Samuel Ortega, MD on x"), "Electronically signed by [NAME], MD on x");
        assert_eq!(mask("Completed x by B. Tran, RN | Page 1"), "Completed x by [NAME], RN | Page 1");
        assert_eq!(mask("Renee Castillo, DO\nPine Hollow"), "[NAME], DO\nPine Hollow");
    }

    #[test]
    fn a_short_form_of_a_name_already_found() {
        assert_eq!(
            mask("RE: Thomas Abernathy\nTom works nights."),
            "RE: [NAME]\n[NAME] works nights."
        );
        // Nothing names a Thomas here, so Tom is not assumed to be a person.
        assert_eq!(mask("Tom Collins mix was denied."), "Tom Collins mix was denied.");
    }

    #[test]
    fn contact_lines_need_a_phone_number() {
        assert_eq!(
            mask("Daughter: Siobhan Reyes-Whitaker, cell [PHONE]"),
            "Daughter: [NAME], cell [PHONE]"
        );
    }

    #[test]
    fn new_rules_leave_medical_text_alone() {
        for text in [
            "Mother: Breast Cancer at 52. Father: Coronary Artery Disease.",
            "Name\tDose\tFrequency",
            "Metoprolol Succinate, DO NOT CRUSH.",
            "Chest Film, PA and lateral views.",
            "She prefers Tylenol to Ibuprofen.",
            "Test\tResult\nPatient\tStable",
            "Timolol Eye Drops, OD twice daily.",
        ] {
            assert_eq!(mask(text), text);
        }
    }
}
