---
test: ../ui-menu-details_test.md
status: passed
started: 2026-11-19T21:31:38.731Z
duration_s: 106
session_id: e5cfbe09-72a6-438d-9e8c-737c410a6c6e
---

# A single record from the menu shows what was removed and its research columns — Result

## Load the sample from the menu and submit it ✓ passed (50.9s)
md5: 3a8dbe5bf0cdb997a4cb866e927fc9e4
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Record details heading is visible.

## The details describe the record without identifying the patient ✓ passed (39.8s)
md5: ca84078a35466a4ca09fd977c0fa3c00
Verify that the Record details region, taken as a whole, contains the text '8 identifiers removed'.
Verify that the Record details region, taken as a whole, contains the text 'atrial fibrillation'.
Verify that the Record details region, taken as a whole, contains the text 'Warfarin 5 mg daily; Metoprolol 25 mg twice daily'.
Verify that the Record details region, taken as a whole, does not contain the text 'Grace Whitfield'.
Verify that the Record details region, taken as a whole, does not contain the text '18 Larkspur Lane'.
