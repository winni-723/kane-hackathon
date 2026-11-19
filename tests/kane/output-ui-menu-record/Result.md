---
test: ../ui-menu-record_test.md
status: passed
started: 2026-11-19T20:26:49.820Z
duration_s: 71
session_id: 729022e9-937a-4e0f-bfa7-4fe60e64ba0b
---

# The progress note sample from the menu comes back de-identified — Result

## Load the sample from the menu and submit it ✓ passed (37.4s)
md5: ce7daf0ba9c442a2d31c8a9df12d11b4
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Result heading is visible.

## The sample comes back de-identified ✓ passed (30.3s)
md5: 8516882ee251ed4ac6210617701fe31b
Verify that the Result region contains the text 'follow-up of atrial fibrillation'.
Verify that the Result region contains the text 'Warfarin 5 mg daily'.
Verify that the Result region does not contain the text 'Grace Whitfield'.
Verify that the Result region does not contain the text '18 Larkspur Lane'.
Verify that the Result region does not contain the text '(508) 555-0114'.
