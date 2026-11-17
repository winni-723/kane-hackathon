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

The keys list what is printed on the page. A string that the PDF itself splits
over two lines (a prescriber's name in pdf-05) is listed as its parts. The
keys were checked against the page text with PyMuPDF, an independent reader,
before the site's own reader existed, and are not edited to make a test pass.
