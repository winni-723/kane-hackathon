# Document test set

Test documents laid out the way real records are: a letterhead, a page footer
that repeats the patient's name and record number, demographics in a table, a
medication table, a signature block. Everything in them is fictional.

| Path | What it is |
|---|---|
| `source/<id>.json` | The content and layout of one document |
| `<id>.docx` | The document, built by `python scripts/gen-docx.py` |
| `keys/<id>.json` | Its answer key: `planted_pii` must be gone, `must_keep` must survive |

The answer keys follow the HIPAA Safe Harbor list. Besides the kinds used in
`tests/fixtures`, real documents bring kinds those records never had:

| Type | Example | Why it is removed |
|---|---|---|
| `facility_name` | Harlow Valley Medical Center | Places the patient geographically |
| `facility_address`, `facility_phone` | The letterhead | Geography smaller than a state; telephone and fax numbers |
| `provider_name` | Samuel Ortega, MD | Not required by Safe Harbor, removed by common practice |
| `service_date` | Admission, discharge, collection dates | Only the year of a date may remain |
| `other_id` | Lab accession number, the login that printed the page | Any other unique identifying number |

The keys were written before the engine was run on these documents and are not
edited to make a test pass. Direction approved by the user on 2026-11-16
(docx-01 to docx-03 reviewed; docx-04 to docx-06 written the same way).

## First run, before any rule was written for documents (2026-11-16)

Six Word documents: **98 of 132 identifiers removed (74%), medical content
kept 81 of 81.** This number is the honest starting point and is not
overwritten.

| Type | Removed |
|---|---|
| facility_name | 0/10 |
| other_id | 0/3 |
| insurance_id | 1/2 |
| name | 8/14 |
| address | 8/13 |
| provider_name | 10/15 |
| relative_name | 6/9 |
| facility_address | 15/16 |
| dob, email, mrn, phone, facility_phone, service_date, ssn | all removed |

Scores: `cargo test --test documents_test scoreboard -- --nocapture`
