# Extraction answer keys for Word and PDF documents

One file per test document in `tests/documents/` (`docx-01` to `docx-06`) and
`tests/documents/pdf/` (`pdf-01` to `pdf-05`), saying what the research table
must show for it. Approved by the project owner on 2026-11-23; like the other
answer keys, these are never edited to make a test pass.

The rules are those of `tests/extraction/README.md` (age on the reference date
2026-11-14 when only a date of birth is given; sex from the record's own words;
conditions stated for the patient; drugs taken, given or prescribed;
`must_not_list` for what does not belong to the patient). Two additions:

- A medication written in a table, where name and dose sit in different cells,
  is keyed by its name.
- `notes` says, in words, why something the document mentions is or is not
  keyed: a suspected diagnosis, a symptom, a contrast agent. The research
  table does not show these.

## First run (2026-11-23)

Scores: `cargo test --test extraction_documents_test scoreboard -- --nocapture`.
Measured before any extraction rule was changed for these documents. These
numbers are the honest starting point and are not overwritten.

| Column | Documents exactly right |
|---|---|
| Age | 11/11 |
| Sex | 9/11 |
| Diagnoses | 1/11 |
| Medications | 4/11 |
| **Total** | **25/44** |

The rules were written for free-text notes, where a diagnosis follows words
such as `Diagnosis:` or `diagnosed with`. Documents put them under headings
(`Assessment and Plan`, `Impression`, `Discharge Diagnoses`), in tables, or
after `consistent with`, and give sex in a field such as `DOB / Sex`.
