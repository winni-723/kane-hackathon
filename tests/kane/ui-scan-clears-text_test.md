---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui, upload, pdf, scan]
---

# A scan loaded after a sample leaves nothing of the sample to process

Hand-written UI test. Do not edit: this test is part of the regression suite.
The sample is tests/fixtures/medium-01.json; the scan is the page's fictional
scanned fax. A file that cannot be read empties the text box, so Process has
no earlier record to de-identify.

## Load a sample record, then the scan
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Select 'Scanned fax (PDF, no text)' in the sample file menu and click the Load sample file button.
Verify that the page contains the text 'Scanned documents cannot be read here'.

## Process without reading the scan
Click the Process button.
Verify that the Record details heading is visible.

## The earlier sample was not processed
Verify that the Record details region, taken as a whole, contains the text 'No identifiers were found in this record'.
Verify that the Record details region, taken as a whole, does not contain the text 'Warfarin 5 mg daily'.
Verify that the Record details region, taken as a whole, does not contain the text 'atrial fibrillation'.
