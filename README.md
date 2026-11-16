# Medical Record De-identifier

[![CI](https://github.com/winni-723/kane-hackathon/actions/workflows/ci.yml/badge.svg)](https://github.com/winni-723/kane-hackathon/actions/workflows/ci.yml)

Paste a medical record, get it back with the personal identifiers removed and
the medical content intact, then download the result.

Built for the Kane CLI Online Hackathon with Claude Code as the coding agent.
Every engine change in this repo went through the same loop: the agent edits
the code, an answer key decides whether the edit is right, Kane CLI confirms it
in a real browser, and the agent reads both results before it is allowed to
commit.

```text
Name: Eleanor Carter                    Name: [NAME]
DOB: 03/14/1958                         DOB: [DATE]
Phone: (617) 555-0142            ->     Phone: [PHONE]
Address: 48 Willow Bend Road,           Address: [ADDRESS],
  Westmarch, MA 02199                     [CITY], MA [ZIP]
Diagnosis: Congestive heart failure     Diagnosis: Congestive heart failure
Medications: Furosemide 40 mg daily     Medications: Furosemide 40 mg daily
```

## Try it

**Live: https://medical-record-deidentifier.onrender.com**

Press **Load sample record**, then **Process**. You can also paste a record or
choose a `.txt` or Word (`.docx`) file; more sample records are in
`tests/fixtures/` (the `text` field of each file), and Word documents laid out
like real ones (letterhead, page footer, tables) are in `tests/documents/`.

For several records at once, press **Load sample batch** (or choose more than
one `.txt` file), then **Process**: you get a research table, one row per
record, that downloads as CSV.

| Record | Age | Sex | Diagnoses | Medications |
|---|---|---|---|---|
| Record 1 | 72 | F | atrial fibrillation | Warfarin 5 mg daily; Metoprolol 25 mg twice daily |
| Record 2 | 81 | M | community-acquired pneumonia | Ceftriaxone 1 g IV daily; Azithromycin 500 mg daily; Amoxicillin-Clavulanate 875 mg twice daily |
| Record 3 | 34 | M | post-traumatic stress disorder | Sertraline 50 mg daily; Prazosin 1 mg at bedtime |

The site runs on a free instance, so the
first request after a quiet spell can take up to a minute.

## Run it

Requirements: Rust (stable) and Node.js 18+.

```bash
npm --prefix web install && npm --prefix web run build && cargo run
```

Then open http://127.0.0.1:3000, paste a record, and press **Process**.
Sample records are in `tests/fixtures/` (the `text` field of each file).

Or with Docker, no Rust or Node needed:

```bash
docker build -t deid . && docker run --rm -p 8080:8080 deid
```

Then open http://127.0.0.1:8080. CI builds this image on every push and sends
a record through it.

## What it removes

| Identifier | Placeholder | How it is found |
|---|---|---|
| Names of patients and relatives | `[NAME]` | From the context that introduces them, then every later mention |
| Phone numbers | `[PHONE]` | Digit patterns, including digit-by-digit and spelled-out numbers |
| Dates | `[DATE]` | Eight written forms of a full calendar date |
| Street addresses, towns, ZIP codes | `[ADDRESS]` `[CITY]` `[ZIP]` | Street-type words, `City, ST ZIP`, unit numbers |
| Homes described by a landmark | `[LOCATION]` | A dwelling word plus a word placing it relative to something |
| Email addresses | `[EMAIL]` | Pattern |
| Social Security numbers | `[SSN]` | Pattern |
| Record numbers, insurance IDs | `[MRN]` `[INSURANCE_ID]` | Only after a label naming them |
| Clinicians' names | `[NAME]` | A title (`Dr.`) or credentials after the name (`, MD`, `, RN`) |
| Hospitals, clinics, pharmacies | `[FACILITY]` | Capitalized words ending in a facility word; the place name inside is then removed wherever it appears |
| Accession, order and visit numbers, staff logins | `[ID_NUMBER]` | Only after a label naming them |

The list follows HIPAA Safe Harbor. Departments and services
(`Nephrology Clinic`, `Emergency Department`) are kept: they say nothing about
where the patient is.

Ages, states, diagnoses, doses, lab values and scores are kept. Terms that
only look like identifiers are kept too: `Parkinson's disease`,
`Montreal Cognitive Assessment`, `Blood pressure 148/92`.

The engine is rule-based Rust (`src/engine.rs`, `src/names.rs`). Nothing is
sent anywhere: the record goes to the local server and comes back.

## The research table

`src/extract.rs` reads four columns from each record: age, sex, diagnoses and
medications.

- The columns are read from the **de-identified** text, so a cell cannot hold
  anything the engine masked. A layer 1 test checks that no planted identifier
  is in any cell. Rows are numbered `Record 1, 2, 3`; file names are not shown,
  because a file name is often the patient's name.
- The one thing read from the original is the date of birth. When a record
  does not state the age, the age is worked out from the date of birth and the
  date itself is dropped. Such ages are marked `*` and are as of today, not as
  of the visit.
- Ages of 90 and over are shown as `90+`, as HIPAA Safe Harbor requires.
- A cell is left empty rather than guessed. Sex is never guessed from a name.
- Things a record mentions that are not the patient's (a ruled-out finding, a
  drug that was not needed, family history) must not reach the table.

`tests/extraction/` holds an answer key for each of the 20 development
records. Whole columns right, by difficulty:

| Column | Easy | Medium | Hard |
|---|---|---|---|
| Age | 7/7 | 7/7 | 6/6 |
| Sex | 7/7 | 7/7 | 5/6 |
| Diagnoses | 7/7 | 7/7 | 5/6 |
| Medications | 7/7 | 7/7 | 6/6 |

The two misses are left as they are: a record whose only pronouns are one
`her` and one `his`, and a diagnosis introduced by nothing but `has`.

Those numbers are from the records the rules were written against. The 10
hand-written held-out records have their own extraction keys
(`tests/extraction-heldout/`), fixed after the first run and before any rule
was changed for them:

| Cells right | First run, untuned | After the loop |
|---|---|---|
| Development (`tests/extraction`), 80 cells | 78 | 78 |
| Held-out (`tests/extraction-heldout`), 40 cells | 25 (63%) | 37 (93%) |

On the first run, by column: age 8/10, sex 8/10, diagnoses 4/10, medications
5/10. Every one of the 15 wrong cells was empty or incomplete; none held a
wrong value. The first-run column is the measure of how the extraction
carries over to new documents. The second is not: the rules were then
tuned on those records.

Three held-out diagnoses are still missed and left that way, because the
rule that would catch them would also list symptoms and treatment reasons:
a seizure named only as the event that brought the patient in, a diagnosis
stated as a bare sentence (`Contact dermatitis, recurrent.`), and the
reason given after a drug (`Clarithromycin ... for H. pylori`).

## How it is verified

### The answer keys

`tests/fixtures/` holds 20 invented medical records (7 easy, 7 medium, 6 hard).
Each one lists `planted_pii`, the exact strings that must be gone, and
`must_keep`, the exact strings that must survive. The answer keys were written
before the engine and are never edited to make a test pass.

### Two layers

```bash
bash scripts/verify.sh
```

**Layer 1** runs in seconds: `cargo build`, `cargo clippy`, `cargo test`, `tsc`.
The answer-key tests in `tests/fixtures_test.rs` run the engine on all 20
records and fail if any `must_keep` string is lost or any gated identifier
survives.

**Layer 2** is Kane CLI. For each record and each kind of identifier in it,
`scripts/gen-kane-test.mjs` compiles the answer key into a Kane test in
`tests/kane/`. The test types the record into the page in a real browser,
presses Process, and checks the Result region against the answer key. The
whole suite is replayed on every run with `--no-adaptive-heal`, so a regression
fails instead of the test being rewritten to fit.

The suite holds 183 tests: every record and identifier kind in the
development set (110), every held-out hand-written record (56), three of the
Synthea records (13), and four tests of the page itself. A held-out record
gets no Kane test for an identifier the engine is known to miss (three such
pairs, listed under Limits), because a test that fails by design would block
every commit.

A commit is made only when both layers pass, and the commit message says what
passed.

The deployed site has its own Kane test, `tests/kane-live/live-sample_test.md`,
which loads the sample record on the live URL and checks the Result the same
way. It is kept out of the regression suite so that a sleeping free instance
cannot block a commit:

```bash
kane-cli testmd run tests/kane-live/live-sample_test.md --agent --headless --no-adaptive-heal
```

### The loop is wired into the agent

`.claude/settings.json` registers two Claude Code hooks, so the checks run
without anyone asking for them:

- **After every edit** to the engine or the page, `scripts/hooks/post-edit.mjs`
  runs layer 1 and hands the result to the agent. A failure comes back as an
  error naming the failed tests. A pass comes back with the current scores on
  every record set, so each edit shows what it changed.
- **Before the agent may finish a turn**, `scripts/hooks/stop-verify.mjs`
  checks whether the engine, the page or the Kane tests changed since the last
  full verification. If they did, it replays every Kane test. A failing test
  blocks the agent from stopping and sends it Kane's explanation of what the
  browser showed.

One round of that loop, from this repo's history:

1. Kane ran a test generated from a record the engine had never seen and
   failed it: *"The de-identification result left the patient's name in the
   displayed medical record"* (`Margaret O'Sullivan`).
2. The agent changed the name rules. After each edit the hook reported the
   scores moving, with the medical content kept at 100% throughout.
3. One of the new rules masked `Referred for Physical Therapy` as if it were a
   person. A unit test caught it through the hook; the rule was removed rather
   than kept for the score.
4. The Kane suite was replayed, including the test that had failed.

### Records the engine was not written against

A perfect score on the development set means little: the rules were tuned on
it. Two more sets exist to measure, and their answer keys were fixed before
the engine was run on them.

| Set | Records | First run, untuned | After the loop |
|---|---|---|---|
| Development (`tests/fixtures`) | 20 | 165/165 | 165/165 |
| Hand-written held-out (`tests/heldout`) | 10 | 62/88 (70%) | 84/88 (95%) |
| Synthea, tuning half (`tests/synthea` s01-s29) | 29 | 74/266 (28%) | 266/266 |
| Synthea, untouched half (s30-s58) | 29 | 77/271 (28%) | 269/271 (99%) |

Medical content kept: 100% on every set, at every stage.

The Synthea records come from [Synthea](https://github.com/synthetichealth/synthea)
v4.0.0 with a fixed seed; `scripts/build-synthea-fixtures.mjs` builds the
fixtures and derives the answer keys from Synthea's own CSV. The untouched half
was never inspected record by record while changing the engine: only its totals
are printed. It shares one layout with the tuning half, so its 99% shows that
the rules carry over to new names, addresses and drugs in that layout, not to
new kinds of documents. The first-run column is the honest measure of that.

### Word documents

Real records arrive as documents, not as pasted text, and a document has
things a pasted note does not: a letterhead, a page footer repeating the
patient's name on every page, demographics in a table, a signature block.
`tests/documents/` holds six fictional Word documents built that way
(discharge summary, referral letter, clinic note, radiology report,
after-visit summary, nursing-home transfer form), each with an answer key
that also lists the facility, the clinicians, the service dates and order
numbers.

`src/document.rs` reads a `.docx` into text: headers, body, footnotes,
comments, footers, with each table row on one line. A layer 1 test checks that
reading loses nothing an answer key names, so an identifier cannot "pass"
because the reader dropped the part of the page it was on.

| | First run, no rule written for documents | After the loop |
|---|---|---|
| Identifiers removed | 98/132 (74%) | 132/132 |
| Medical content kept | 81/81 | 81/81 |

The first run removed none of the 10 facility names and missed names written
in table cells. These six documents are a development set: the rules were
tuned on them, and there is no held-out document set yet, so 132/132 says the
loop works, not how the next hospital's template will fare. The files are
generated by a script, not saved from Word, and file upload is covered by
layer 1 only: the Kane tests do not choose a file.

### What Kane caught

- An empty error placeholder in the page was being matched instead of the
  result area, so a "this text is absent" check passed without looking at the
  result. The page was restructured so the result is an unambiguous region.
- A step holding only "does not contain" checks was judged from a screenshot
  rather than by text comparison. Tests now pair those checks with "contains"
  checks in one step, and `scripts/kane-authored-check.mjs` rejects any
  recording whose answer-key checks are not exact.
- With phone masking switched off as a control, every replayed phone test
  failed. Switched back on, they passed.

## Limits

This is a rule-based engine, and the first-run scores above show what that
means: on documents it was not written against it removed 70% of identifiers,
and 28% when the layout was unfamiliar. Tuning brought those up, but the next
unfamiliar document type will start low again. Known misses that are left
unfixed because fixing them would mask medical text:

- names written in lower case (`its bao-tran again`, `my husband minh`)
- a name introduced only by an ordinary preposition (`an interaction for
  Leilani Kahananui`)

The research table is weaker than the de-identification. On its first run
against held-out records it got 63% of cells right and found the diagnosis
in 4 of 10: a diagnosis is found when a field or a set phrase introduces
it, and new documents word it differently. The Synthea layout
(`MEDICATIONS:` and `CONDITIONS:` sections) is not read at all. Empty
cells are the designed failure: the table should be missing a value rather
than holding a wrong one. Files can be plain text or Word (`.docx`). There is
no PDF or CSV yet, no scanned document (that needs OCR, and one misread
character lets a name through), no HL7, FHIR or DICOM, and no batch larger
than 200 files. Identifiers are replaced with tags, not with realistic
surrogates.

The hand-written records, held-out ones included, share an author with the
rules. Only the Synthea values are independent of both.

This is a hackathon project, not a tool to rely on for real patient data.

All test data is fictional: phone numbers use the reserved `555-01xx` range,
emails use `example.com`, SSNs come from ranges that are never issued, and the
town names are made up.

## Layout

```text
src/engine.rs          rules for everything except names
src/names.rs           finding and removing names
src/extract.rs         the research columns: age, sex, diagnoses, medications
src/document.rs        reads an uploaded Word document into text
src/main.rs            Axum server: POST /api/process, /api/read-file, serves the page
tests/documents/       6 Word documents laid out like real ones, with answer keys
tests/documents_test.rs layer 1 tests of the documents
scripts/gen-docx.py    builds the Word documents from tests/documents/source
tests/extraction/      answer keys for the research table
tests/extraction-heldout/ the same for the held-out records, measured only
tests/extraction_test.rs layer 1 tests of the research table
tests/kane-live/       Kane test against the deployed site
web/                   TypeScript + Vite front end
tests/fixtures/        20 development records with answer keys
tests/heldout/         10 held-out records with answer keys
tests/synthea/         58 Synthea records with answer keys
tests/fixtures_test.rs layer 1 answer-key tests and the scoreboards
tests/kane/            layer 2 Kane tests, generated from the answer keys
scripts/verify.sh      runs both layers
scripts/hooks/         the two Claude Code hooks
.claude/settings.json  registers the hooks
CLAUDE.md              the rules the coding agent works under
```

Scoreboards of removed identifiers by kind and difficulty, for all three sets:

```bash
cargo test --test fixtures_test -- --nocapture scoreboard
```
