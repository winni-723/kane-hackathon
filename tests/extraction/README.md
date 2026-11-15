# Extraction answer keys

One file per record in `tests/fixtures/`, saying what the research table must
show for that record. Approved by the project owner on 2026-11-14; like the
other answer keys, these are never edited to make a test pass.

- `age`: the age written in the record (`age_source: "stated"`), or, when the
  record gives only a date of birth, the age on the reference date
  **2026-11-14** (`age_source: "dob"`). `null` when the record has neither.
- `sex`: `F` or `M` from the record's own words or pronouns; `null` when the
  only clue would be the patient's name.
- `diagnoses`: conditions stated for the patient. A table cell passes when it
  contains the string. Nothing is inferred from lab values.
- `medications`: drugs the patient takes, was given or was prescribed, as
  written.
- `must_not_list`: things the record mentions that do not belong to the
  patient (ruled out, not needed, family history). They must not appear in
  the diagnoses or medications cells.
