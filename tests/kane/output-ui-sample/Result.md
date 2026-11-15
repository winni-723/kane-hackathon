---
test: ../ui-sample_test.md
status: passed
started: 2026-11-15T01:37:10.982Z
duration_s: 57
session_id: c7a403ee-f8b9-49e4-9ac3-b926df2ac234
---

# The sample record button fills the text box with a record that de-identifies — Result

## Load the sample and submit it ✓ passed (25s)
md5: 4955cfdbf6ec596ea1ff69254f1fc812
Click the 'Load sample record' button and then click the Process button.
Verify that the Result heading is visible.

## The sample comes back de-identified ✓ passed (28s)
md5: 8516882ee251ed4ac6210617701fe31b
Verify that the Result region contains the text 'follow-up of atrial fibrillation'.
Verify that the Result region contains the text 'Warfarin 5 mg daily'.
Verify that the Result region does not contain the text 'Grace Whitfield'.
Verify that the Result region does not contain the text '18 Larkspur Lane'.
Verify that the Result region does not contain the text '(508) 555-0114'.
