# Medical Record De-identifier

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

## Run it

Requirements: Rust (stable) and Node.js 18+.

```bash
npm --prefix web install && npm --prefix web run build && cargo run
```

Then open http://127.0.0.1:3000, paste a record, and press **Process**.
Sample records are in `tests/fixtures/` (the `text` field of each file).

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

Ages, states, diagnoses, doses, lab values and scores are kept. Terms that
only look like identifiers are kept too: `Parkinson's disease`,
`Montreal Cognitive Assessment`, `Blood pressure 148/92`.

The engine is rule-based Rust (`src/engine.rs`, `src/names.rs`). Nothing is
sent anywhere: the record goes to the local server and comes back.

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

A commit is made only when both layers pass, and the commit message says what
passed.

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

The 20 records and the rules were written by the same author, and the rules
were tuned against these records. A full score here means the engine handles
this set. It does not show how it performs on notes it has never seen, and
names and landmark descriptions written in unfamiliar ways will be missed.
This is a hackathon project, not a tool to rely on for real patient data.

All test data is fictional: phone numbers use the reserved `555-01xx` range,
emails use `example.com`, SSNs come from ranges that are never issued, and the
town names are made up.

## Layout

```text
src/engine.rs          rules for everything except names
src/names.rs           finding and removing names
src/main.rs            Axum server: POST /api/process, serves the page
web/                   TypeScript + Vite front end
tests/fixtures/        20 records with answer keys
tests/fixtures_test.rs layer 1 answer-key tests and the scoreboard
tests/kane/            layer 2 Kane tests, generated from the answer keys
scripts/verify.sh      runs both layers
CLAUDE.md              the rules the coding agent works under
```

Scoreboard of removed identifiers by kind and difficulty:

```bash
cargo test --test fixtures_test scoreboard -- --nocapture
```
