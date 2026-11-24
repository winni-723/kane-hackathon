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

/// Web addresses (HIPAA Safe Harbor lists URLs among the identifiers): a
/// scheme or `www.` and everything up to the next space, less any closing
/// punctuation that ends the sentence rather than the address.
static URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:https?://|www\.)[^\s<>"'()\[\]]*[^\s<>"'()\[\].,;:!?]"#).expect("url pattern is valid")
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
        r"(?P<label>\b(?i:insurance (?:id|number|no\.?)|member (?:id|number|no\.?)|policy (?:id|number|no\.?)",
        r"|(?:medicare|medicaid|subscriber) (?:id|number|no\.?)))",
        r"(?P<join>\s*[:#]?\s*)",
        r"(?P<id>[A-Z]{0,4}-?\d[\dA-Z-]{2,}[\dA-Z])\b",
    ))
    .expect("insurance id pattern is valid")
});

/// Numbers that identify one order, specimen or visit, after their label:
/// `Accession: RA-20261109-0457`, `Acc# 88213`, `Lab accession L-2026-448817`,
/// `Encounter number 5530021`. The value must contain a digit, so
/// "accession of fluid" is not touched.
static OTHER_ID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?P<label>\b(?i:accession(?: number| no\.?)?|acc\.?[ ]?#|specimen (?:id|number|no\.?)",
        r"|(?:encounter|visit|order) (?:id|number|no\.?)|(?:encounter|visit|order)[ ]?#",
        r"|npi|clia|rx[ ]?#|rx (?:number|no\.?)|prescription (?:#|number|no\.?)))",
        r"(?P<join>\s*[:#]?\s*)",
        r"(?P<id>[A-Z]{0,4}-?\d[\dA-Z-]{2,}[\dA-Z])\b",
    ))
    .expect("other id pattern is valid")
});

/// Column headers whose cells below hold one identifying number each:
/// `Rx #`, `Accession`, `Order #`, `NPI`.
static ID_COLUMN_HEADER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?i:rx[ ]?#|rx (?:number|no\.?)|prescription (?:#|number|no\.?)|accession(?: number| no\.?| #)?",
        r"|acc\.?[ ]?#|specimen (?:id|#|number|no\.?)|order (?:id|#|number|no\.?)|order#|npi)$",
    ))
    .expect("id column header pattern is valid")
});

/// What a cell must look like to be taken for an identifying number.
static ID_CELL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Z]{0,4}-?\d[\dA-Z-]{2,}[\dA-Z]$").expect("id cell pattern is valid"));

/// Masks identifying numbers in a table column headed by their label:
///
/// ```text
/// Medication  Directions  Prescriber  Rx #     Last filled
/// Warfarin    ...         ...         6610490  10/30/2026
/// ```
///
/// A table row is a line of tab-separated cells. Only rows with exactly as
/// many cells as the header are read, so a row that lost an empty cell never
/// has the wrong column masked. The table ends at the first line without a
/// tab.
fn mask_id_columns(text: &str) -> String {
    let mut columns: Vec<usize> = Vec::new();
    let mut width = 0;
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        let body = line.trim_end_matches(['\r', '\n']);
        let cells: Vec<&str> = body.split('\t').collect();
        if cells.len() < 2 {
            columns.clear();
            out.push_str(line);
            continue;
        }
        let headers: Vec<usize> = cells
            .iter()
            .enumerate()
            .filter(|(_, c)| ID_COLUMN_HEADER.is_match(c.trim()))
            .map(|(i, _)| i)
            .collect();
        if !headers.is_empty() {
            columns = headers;
            width = cells.len();
            out.push_str(line);
            continue;
        }
        if columns.is_empty() || cells.len() != width {
            out.push_str(line);
            continue;
        }
        let masked: Vec<&str> = cells
            .iter()
            .enumerate()
            .map(|(i, c)| if columns.contains(&i) && ID_CELL.is_match(c.trim()) { "[ID_NUMBER]" } else { c })
            .collect();
        out.push_str(&masked.join("\t"));
        out.push_str(&line[body.len()..]);
    }
    out
}

/// The login of whoever printed or entered the page: `Printed 11/03/2026 09:14
/// by fdesk02`. The login must contain a digit or a separator, so a person's
/// name after "by" is left to the name rules.
static USER_LOGIN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?P<lead>\b(?:Printed|Entered|Generated|Exported|Transcribed|Scanned)\b[^\n|]{0,40}?\bby[ ])",
        r"(?P<id>[A-Za-z]+[\d._][A-Za-z\d._-]*)",
    ))
    .expect("user login pattern is valid")
});

/// What a healthcare organization calls itself. A run of capitalized words
/// ending in one of these is the name of a facility.
const FACILITY_KIND: &str = concat!(
    r"Medical Cent(?:er|re)|Health Cent(?:er|re)|Health System|Health Clinic|Healthcare|Health Care",
    r"|Hospital|Clinic|Pharmacy|Drugstore|Associates|Family Practice|Family Medicine|Medical Group",
    r"|Nursing Home|Skilled Nursing|Rehabilitation Cent(?:er|re)|Rehabilitation Hospital",
    r"|Imaging Cent(?:er|re)|Urgent Care|Surgery Cent(?:er|re)|Surgical Cent(?:er|re)|Laboratories|Laboratory",
    r"|Hospice|Infirmary|Assisted Living|Senior Living|Memory Care",
);

/// A facility name: `Harlow Valley Medical Center`, `St. Brendan Regional
/// Hospital`, `Cardiology Associates of Tilden`, `Maplewood Gardens Skilled
/// Nursing and Rehabilitation`.
static FACILITY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        concat!(
            r"(?P<pre>(?:(?:St|Mt|Ft)\.[ ])?(?:[A-Z][A-Za-z'’]+(?:-[A-Z][a-z]+)*[ ]){{1,5}})",
            r"(?P<kind>{kind})\b",
            r"(?P<post>(?:[ ](?:and|&)[ ][A-Z][a-z]+)?(?:[ ]of[ ](?P<place>[A-Z][a-z]+(?:[ ][A-Z][a-z]+){{0,2}}))?)",
        ),
        kind = FACILITY_KIND
    ))
    .expect("facility pattern is valid")
});

/// Words that begin a sentence in front of a facility name without being part
/// of it: `Called Ridgeline Pharmacy`.
const BEFORE_A_FACILITY: &[&str] = &[
    "At", "To", "From", "In", "Via", "Per", "With", "And", "Or", "By", "For", "Her", "His", "Their",
    "My", "Our", "Your", "The", "This", "That", "A", "An", "Called", "Seen", "Transfer",
    "Transferred", "Admitted", "Discharged", "Referred", "Visited", "Contacted", "Notified",
    "Returned", "Presented", "Sent", "Faxed",
];

/// Words that describe a kind of department or service. `Nephrology Clinic`
/// and `Emergency Department` say nothing about where the patient is, so a
/// facility name made only of these is left alone.
const GENERIC_FACILITY_WORDS: &[&str] = &[
    "Regional", "Memorial", "General", "Community", "University", "County", "City", "State",
    "National", "Central", "Children's", "Childrens", "Women's", "Womens", "Veterans", "VA",
    "Medical", "Health", "Family", "Emergency", "Outpatient", "Inpatient", "Primary", "Urgent",
    "Skilled", "Chest", "Heart", "Eye", "Pain", "Sleep", "Memory", "Wound", "Cancer", "Dental",
    "Clinical", "Specialty", "Cardiology", "Nephrology", "Neurology", "Oncology", "Endocrinology",
    "Pulmonology", "Pulmonary", "Gastroenterology", "GI", "Urology", "Dermatology",
    "Rheumatology", "Hematology", "Psychiatry", "Psychiatric", "Pediatric", "Pediatrics",
    "Geriatric", "Geriatrics", "Orthopedic", "Orthopedics", "Orthopaedic", "Surgical", "Surgery",
    "Radiology", "Imaging", "Obstetrics", "Gynecology", "ENT", "Ophthalmology", "Podiatry",
    "Allergy", "Asthma", "Diabetes", "Dialysis", "Transplant", "Vascular", "Cardiac", "Renal",
    "Thoracic", "Spine", "Sports", "Medicine", "Internal", "Behavioral", "Mental", "Infectious",
    "Disease", "Anticoagulation", "Coumadin", "Lipid", "Hypertension", "Fracture", "Breast",
    "Liver", "Stroke", "Epilepsy", "Headache", "Movement", "Disorders", "Rehabilitation",
    "Physical", "Therapy", "Occupational", "Nursing", "Care", "Free", "Mobile", "Student",
    "Employee", "Travel", "Teaching", "Retail", "Local", "Outside", "Home", "Senior", "Assisted",
    "Living", "Follow-up", "Discharge", "Admission", "Hospital", "Clinic", "Pharmacy",
    "Reference", "Pathology", "Microbiology", "Chemistry", "Toxicology", "Blood", "Core",
];

/// A fax banner, the line a fax machine prints across the top of each page:
/// `11/05/2026 14:22 FROM: Pinecrest Family Practice FAX 541-555-0177 TO:
/// Sutter Ridge Cardiology P.001/001`. The parties named after `FROM:` and
/// `TO:` are practices, whatever kind of word ends their name. Only the
/// capitals form a fax machine prints is read, and only on a line that says
/// FAX, so a letter's `To: Mom` is left alone.
static FAX_PARTY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:FROM|TO):[ ]+(?P<n>[A-Z][a-z][A-Za-z'’-]*(?:[ ](?:&[ ])?[A-Z][a-z][A-Za-z'’-]*){0,5})")
        .expect("fax party pattern is valid")
});

/// Removes facility names. Returns the text and the proper names found in
/// them (`Harlow Valley`, `Tilden`), which place the patient just as well when
/// they are written alone later in the record.
fn mask_facilities(text: &str) -> (String, Vec<String>) {
    let mut places: Vec<String> = Vec::new();
    let out = FACILITY.replace_all(text, |caps: &regex::Captures| {
        let pre = &caps["pre"];
        let mut words: Vec<&str> = pre.split(' ').filter(|w| !w.is_empty()).collect();
        let mut skipped = 0;
        while words.first().is_some_and(|w| BEFORE_A_FACILITY.contains(w)) {
            skipped += words[0].len() + 1;
            words.remove(0);
        }
        let proper: Vec<&str> = words
            .iter()
            .copied()
            .take_while(|w| !GENERIC_FACILITY_WORDS.contains(w))
            .collect();
        let place = caps.name("place").map(|m| m.as_str());
        let names_a_place = words.iter().any(|w| !GENERIC_FACILITY_WORDS.contains(w));
        if !names_a_place && place.is_none() {
            return caps[0].to_string();
        }
        if !proper.is_empty() {
            places.push(proper.join(" "));
        }
        if let Some(place) = place {
            places.push(place.to_string());
        }
        format!("{}[FACILITY]", &pre[..skipped])
    });
    let out = out
        .split_inclusive('\n')
        .map(|line| {
            if !line.contains("FAX") {
                return line.to_string();
            }
            FAX_PARTY
                .replace_all(line, |caps: &regex::Captures| {
                    let party = &caps["n"];
                    let words: Vec<&str> = party.split(' ').filter(|w| *w != "&").collect();
                    if words.iter().all(|w| GENERIC_FACILITY_WORDS.contains(w)) {
                        return caps[0].to_string();
                    }
                    let proper: Vec<&str> =
                        words.iter().copied().take_while(|w| !GENERIC_FACILITY_WORDS.contains(w)).collect();
                    if !proper.is_empty() {
                        places.push(proper.join(" "));
                    }
                    caps[0].replacen(party, "[FACILITY]", 1)
                })
                .into_owned()
        })
        .collect::<String>();
    places.retain(|p| p.len() >= 5);
    places.sort_by_key(|p| std::cmp::Reverse(p.len()));
    places.dedup();
    (out, places)
}

fn whole_phrase(phrase: &str) -> Regex {
    Regex::new(&format!(r"\b{}\b", regex::escape(phrase))).expect("escaped phrase is a valid pattern")
}

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
        // Spaces only: a tab or a line break is a boundary between fields.
        r"\b\d{1,6}[ ]+(?:[A-Z][A-Za-z']*[ ]+){1,4}",
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
    Regex::new(r"\b(?:Apt|Apartment|Unit|Suite|Ste|Room|Rm)\.?[ ]+#?\d+[A-Za-z]?\b")
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
        // The label ends in a colon, or in the tab that separates two table cells.
        r"(?m)^(?P<lead>[ \t]*(?P<label>[A-Za-z][A-Za-z0-9 .'/#-]{0,28}?)(?:[ \t]*:[ \t]*|\t))",
        r"(?P<value>[^\r\n\t]*?[^ \t\r\n])(?P<tail>[ ]*(?:\t[^\r\n]*)?\r?)$",
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
    let out = URL.replace_all(text, "[URL]");
    let out = EMAIL.replace_all(&out, "[EMAIL]");
    let out = SSN.replace_all(&out, "[SSN]");
    let out = MRN.replace_all(&out, "${label}${join}[MRN]");
    let out = INSURANCE_ID.replace_all(&out, "${label}${join}[INSURANCE_ID]");
    let out = OTHER_ID.replace_all(&out, "${label}${join}[ID_NUMBER]");
    let out = mask_id_columns(&out);
    let out = DATE.replace_all(&out, "[DATE]");
    let out = PHONE.replace_all(&out, "[PHONE]");
    let out = PHONE_SPACED.replace_all(&out, "[PHONE]");
    let out = USER_LOGIN.replace_all(&out, "${lead}[ID_NUMBER]");
    let out = LANDMARK_HOME.replace_all(&out, "${det}[LOCATION]");
    let out = LANDMARK_PLACE.replace_all(&out, "${det}[LOCATION]");
    // Before the street rule, so "Old Mill Pike ... Clinic" is read as a whole.
    let (out, places) = mask_facilities(&out);
    let out = STREET.replace_all(&out, "[ADDRESS]");
    let out = UNIT.replace_all(&out, "[ADDRESS]");
    // A town written with its state and ZIP is a town wherever else it appears.
    let towns: Vec<String> = CITY_STATE_ZIP
        .captures_iter(&out)
        .filter(|caps| caps.name("zip").is_some())
        .filter_map(|caps| caps[0].split(',').next().map(str::to_string))
        .collect();
    let out = CITY_STATE_ZIP.replace_all(&out, |caps: &regex::Captures| {
        // "Whitcombe, PA-C" is a credential, not Pennsylvania.
        let whole = caps.get(0).expect("group 0 always matches");
        if out[whole.end()..].starts_with('-') {
            return caps[0].to_string();
        }
        // "Samuel Ortega, MD" is a physician and "Silver Spring, MD" a town.
        // Two words before "MD" with no ZIP after it are left to the name
        // rules unless a preposition says it is a place.
        let before = &out[..whole.start()];
        let placed = ["in ", "at ", "from ", "near ", "to ", "of "]
            .iter()
            .any(|word| before.ends_with(word));
        if &caps["state"] == "MD"
            && caps.name("zip").is_none()
            && whole.as_str().split(',').next().is_some_and(|town| town.contains(' '))
            && !placed
        {
            return caps[0].to_string();
        }
        mask_city_state_zip(caps)
    });
    let out = CITY_AFTER_ADDRESS.replace_all(&out, "${lead}[CITY]");
    let mut out = out.into_owned();
    for town in &towns {
        out = whole_phrase(town).replace_all(&out, "[CITY]").into_owned();
    }
    for place in &places {
        out = whole_phrase(place).replace_all(&out, "[LOCATION]").into_owned();
    }
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
    fn masks_web_addresses() {
        assert_eq!(
            process("See www.example.org/portal for results."),
            "See [URL] for results."
        );
        assert_eq!(
            process("Portal: https://portal.example.com/p?id=4417&u=a.b@example.com"),
            "Portal: [URL]"
        );
        assert_eq!(process("(http://example.net/x)"), "([URL])");
    }

    #[test]
    fn web_address_rule_leaves_medical_text_alone() {
        for text in [
            "Metformin 0.5 mg q.d. with meals.",
            "Hgb 13.2 g/dL, eGFR 58 mL/min/1.73m2.",
            "Follow up p.r.n. at the www clinic desk.",
        ] {
            assert_eq!(process(text), text);
        }
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

    #[test]
    fn masks_facility_names_and_the_place_inside_them() {
        assert_eq!(
            process("Harlow Valley Medical Center\nSeen in Harlow Valley last week."),
            "[FACILITY]\nSeen in [LOCATION] last week."
        );
        assert_eq!(process("St. Brendan Regional Hospital - Emergency Department"), "[FACILITY] - Emergency Department");
        assert_eq!(process("Referred to Cardiology Associates of Tilden."), "Referred to [FACILITY].");
        assert_eq!(
            process("Maplewood Gardens Skilled Nursing and Rehabilitation"),
            "[FACILITY]"
        );
        assert_eq!(process("Called Ridgeline Pharmacy today."), "Called [FACILITY] today.");
    }

    #[test]
    fn leaves_departments_and_services_alone() {
        for text in [
            "Nephrology Clinic",
            "Seen in the Emergency Department and then Pulmonary Clinic.",
            "Referred to Cardiac Rehabilitation and Physical Therapy.",
            "Follow up at the Anticoagulation Clinic in 2 weeks.",
            "Hospital Course",
            "Her mother called the clinic. Sent to the pharmacy.",
            "Started Peritoneal Dialysis in March.",
            "Sent to Reference Laboratory. Clinical Laboratory results below.",
            "Laboratory director signed the report.",
            "Specialty: Cardiology",
        ] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn masks_laboratories_and_fax_parties() {
        assert_eq!(process("Brightwater Regional Laboratory\nResults"), "[FACILITY]\nResults");
        assert_eq!(
            process("14:22 FROM: Pinecrest Family Practice FAX x TO: Sutter Ridge Cardiology P.001/001\nSutter Ridge called."),
            "14:22 FROM: [FACILITY] FAX x TO: [FACILITY] P.001/001\n[LOCATION] called."
        );
    }

    #[test]
    fn masks_npi_clia_and_rx_numbers() {
        assert_eq!(process("Lena Okonkwo, NP\tNPI: 1093847562"), "[NAME], NP\tNPI: [ID_NUMBER]");
        assert_eq!(process("Director. CLIA 29D0998877."), "Director. CLIA [ID_NUMBER].");
        assert_eq!(process("Refill Rx #6610482 today."), "Refill Rx #[ID_NUMBER] today.");
    }

    #[test]
    fn masks_an_id_column_under_its_header() {
        assert_eq!(
            process("Medication\tRx #\tQty\nWarfarin 5 mg\t6610490\t30\nAspirin 81 mg\t6608817\t90\nEnd of list"),
            "Medication\tRx #\tQty\nWarfarin 5 mg\t[ID_NUMBER]\t30\nAspirin 81 mg\t[ID_NUMBER]\t90\nEnd of list"
        );
    }

    #[test]
    fn leaves_other_columns_and_misaligned_rows_alone() {
        for text in [
            // Values in the other columns, a cell that is not a number, a
            // row with a cell missing, and a table that never had the header.
            "Test\tAccession\tResult\nTSH\tpending\t6.8 mIU/L\nFree T4\t0.8 ng/dL",
            "Test\tResult\tRange\nGlucose\t1680\t70-99",
            "No NPI on file. Rx count 4.",
        ] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn leaves_to_and_from_alone_off_a_fax_banner() {
        for text in [
            "TO: Cardiology FAX x",
            "To: Mom. From: Dr. office",
            "TO: Sutter Ridge Cardiology",
        ] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn masks_labelled_order_numbers_and_logins() {
        assert_eq!(process("Accession: RA-20261109-0457"), "Accession: [ID_NUMBER]");
        assert_eq!(process("Lab accession L-2026-448817."), "Lab accession [ID_NUMBER].");
        assert_eq!(process("Acc# 88213"), "Acc# [ID_NUMBER]");
        assert_eq!(
            process("Printed 11/03/2026 09:14 by fdesk02 | Page 1"),
            "Printed [DATE] 09:14 by [ID_NUMBER] | Page 1"
        );
        assert_eq!(process("Medicare ID\t1EG4-TE5-MK73"), "Medicare ID\t[INSURANCE_ID]");
        for text in ["No accession of fluid.", "Order 2 units of blood.", "Visit 3 of 6 completed."] {
            assert_eq!(process(text), text);
        }
    }

    #[test]
    fn reads_table_rows_as_labelled_fields() {
        assert_eq!(process("Address\t9 Odd Name Qtr\nSex\tFemale"), "Address\t[ADDRESS]\nSex\tFemale");
        assert_eq!(process("TSH\t6.8 mIU/L\t0.4-4.0"), "TSH\t6.8 mIU/L\t0.4-4.0");
        // A time and the next cell are not a street address.
        assert_eq!(process("9:30 AM\tDr. Samir Haddad"), "9:30 AM\tDr. [NAME]");
    }

    #[test]
    fn a_town_given_with_its_zip_is_removed_wherever_it_appears() {
        assert_eq!(
            process("Home: 12 Elm Street, Millbrook, OH 43760. She has lived in Millbrook all her life."),
            "Home: [ADDRESS], [CITY], OH [ZIP]. She has lived in [CITY] all her life."
        );
    }

    #[test]
    fn tells_a_credential_from_a_state() {
        assert_eq!(process("Seen by Daniel Whitcombe, PA-C, today."), "Seen by [NAME], PA-C, today.");
        assert_eq!(process("Signed by Samuel Ortega, MD on Monday."), "Signed by [NAME], MD on Monday.");
        assert_eq!(process("Moved from Silver Spring, MD last year."), "Moved from [CITY], MD last year.");
        assert_eq!(process("Lives in Baltimore, MD."), "Lives in [CITY], MD.");
    }
}
