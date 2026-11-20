---
test: ../live-menu-sample_test.md
status: passed
started: 2026-11-20T21:02:01.546Z
duration_s: 161
session_id: bc64de02-f8c3-46eb-be2c-4a0f5bdb4427
---

# The deployed site de-identifies the progress note sample from the menu — Result

## Load the sample from the menu and submit it ✓ passed (50s)
md5: ce7daf0ba9c442a2d31c8a9df12d11b4
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Result heading is visible.

## The sample comes back de-identified ✓ passed (40.5s)
md5: 8516882ee251ed4ac6210617701fe31b
Verify that the Result region contains the text 'follow-up of atrial fibrillation'.
Verify that the Result region contains the text 'Warfarin 5 mg daily'.
Verify that the Result region does not contain the text 'Grace Whitfield'.
Verify that the Result region does not contain the text '18 Larkspur Lane'.
Verify that the Result region does not contain the text '(508) 555-0114'.
