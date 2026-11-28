# OCR test set

One scan: `../pdf/pdf-06.pdf`, the fax referral pdf-03 printed and scanned.
It has no text layer, so the text reader refuses it; `src/ocr.rs` reads its
page picture instead.

| Path | What it is |
|---|---|
| `keys/pdf-06.json` | pdf-03's answer key, unchanged, with `"scan_of": "pdf-03"` |

The key is the same page's key, so it lists what is printed on the scan. It
is kept here rather than in `../pdf/keys/`, because every key there is read
against the text reader's output, and that reader gets nothing from a scan.
Like every answer key, it is not edited to make a test pass.

`tests/ocr_test.rs` (feature `ocr`, on by default) checks that:

- the text reader still refuses the scan
- the OCR text holds every string the key names
- every `must_keep` string survives the engine
- every planted identifier is removed

Scores: `cargo test --test ocr_test -- --nocapture scoreboard`.

## First run (2026-11-26)

These numbers are the honest starting point and are not overwritten.

| | Read | Identifiers removed | Medical content kept |
|---|---|---|---|
| OCR text as it came, no rule changed | 21/21 | 18/21 | 11/11 |
| After `ocr::tidy` | 21/21 | 21/21 | 11/11 |

The three misses were punctuation OCR changes on a form: `Dr,` for `Dr.`,
`Town. ST 12345` for `Town, ST 12345`, and a lower case `l` for the last
capital `I` of an ID. `ocr::tidy` puts back those three forms only; its unit
tests include ordinary text it must leave alone.

This is a development set of one document whose text the rules were already
tuned on. 21/21 says the OCR path works end to end, not how well OCR reads a
scan it has not seen.
