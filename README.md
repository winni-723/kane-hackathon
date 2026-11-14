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
src/main.rs            Axum server: POST /api/process, serves the page
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
