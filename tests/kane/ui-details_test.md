---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
---

# A single record shows what was removed and its research columns

Hand-written UI test. Do not edit: this test is part of the regression suite.
The expected cells come from tests/extraction/medium-01.json.

## Load the sample and submit it
Click the 'Load sample record' button and then click the Process button.
Verify that the Record details heading is visible.

## The details describe the record without identifying the patient
Verify that the Record details region contains the text '8 identifiers removed'.
Verify that the Record details region contains the text 'atrial fibrillation'.
Verify that the Record details region contains the text 'Warfarin 5 mg daily; Metoprolol 25 mg twice daily'.
Verify that the Record details region does not contain the text 'Grace Whitfield'.
Verify that the Record details region does not contain the text '18 Larkspur Lane'.
