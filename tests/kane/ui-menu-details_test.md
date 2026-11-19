---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
---

# A single record from the menu shows what was removed and its research columns

Hand-written UI test. Do not edit: this test is part of the regression suite.
The expected cells come from tests/extraction/medium-01.json.

## Load the sample from the menu and submit it
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Record details heading is visible.

## The details describe the record without identifying the patient
Verify that the Record details region, taken as a whole, contains the text '8 identifiers removed'.
Verify that the Record details region, taken as a whole, contains the text 'atrial fibrillation'.
Verify that the Record details region, taken as a whole, contains the text 'Warfarin 5 mg daily; Metoprolol 25 mg twice daily'.
Verify that the Record details region, taken as a whole, does not contain the text 'Grace Whitfield'.
Verify that the Record details region, taken as a whole, does not contain the text '18 Larkspur Lane'.
