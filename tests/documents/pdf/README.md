# PDF test set

Six fictional documents, made the ways real PDFs are made, because each
program lays text out differently and a reader has to cope with all of them.

| Id | Document | Made by | What makes it hard |
|---|---|---|---|
| pdf-01 | Lab report, two pages | Chrome printing a portal page | Four-column demographics table; footer with name and MRN on every page |
| pdf-02 | Discharge summary, two pages | Word, saved as PDF | Long paragraphs wrapped by Word; header and footer on every page |
| pdf-03 | Referral form sent by fax | Text placed at fixed positions | Two columns of fields on one line; fax banner; NPI and Medicare ID |
| pdf-04 | Radiology report | Chrome | Hyphenated surnames; accession number; prior-study date |
| pdf-05 | Pharmacy medication list | Word, saved as PDF | A prescriber's name wrapped inside a table cell; Rx numbers |
| pdf-06 | pdf-03, scanned | A picture of the page | No text at all: the site must say it cannot read scans, not return an empty result |

| Path | What it is |
|---|---|
| `source/<id>.json` | The content and layout |
| `<id>.pdf` | The document, built by `python scripts/gen-pdf.py` (needs Chrome, Word, PyMuPDF) |
| `keys/<id>.json` | Its answer key, same format and HIPAA Safe Harbor types as `../keys/` |

pdf-06 has no key: nothing can be read from it.

## First run (2026-11-17)

Scores: `cargo test --test pdf_test scoreboard -- --nocapture`. These numbers
are the honest starting point and are not overwritten.

| | Identifiers removed | Not read by the reader | Medical content kept |
|---|---|---|---|
| `pdf-extract` 0.12.1 as published | 81/100 | 3 | 59/62 |
| Reader fixed, no engine rule changed | 88/100 | 0 | 62/62 |

All three medical losses on the first run were the reader's, not the
engine's: `pdf-04` came out as `PET -CT`, `ef fusion`, `Adebayo-W alsh`.

After the loop (2026-11-17, four commits, one theme each): **identifiers
100/100, medical content 62/62.** Every (type, difficulty) pair in these
keys is now in `GATES` in `tests/pdf_test.rs`.

## About the keys

The keys list what is printed on the page. A string that the PDF itself splits
over two lines (a prescriber's name in pdf-05) is listed as its parts. The
keys were checked against the page text with PyMuPDF, an independent reader,
before the site's own reader existed, and are not edited to make a test pass.
