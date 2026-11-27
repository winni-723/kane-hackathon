---
test: ../ui-menu-details_test.md
status: passed
started: 2026-11-26T23:27:03.558Z
duration_s: 59
session_id: e768c272-7d18-4712-9d82-87182f7d9023
---

# A single record from the menu shows what was removed and its research columns — Result

## Load the sample from the menu and submit it ✓ passed (27.5s)
md5: 3a8dbe5bf0cdb997a4cb866e927fc9e4
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Record details heading is visible.

## The details describe the record without identifying the patient ✓ passed (27.5s)
md5: ca84078a35466a4ca09fd977c0fa3c00
Verify that the Record details region, taken as a whole, contains the text '8 identifiers removed'.
Verify that the Record details region, taken as a whole, contains the text 'atrial fibrillation'.
Verify that the Record details region, taken as a whole, contains the text 'Warfarin 5 mg daily; Metoprolol 25 mg twice daily'.
Verify that the Record details region, taken as a whole, does not contain the text 'Grace Whitfield'.
Verify that the Record details region, taken as a whole, does not contain the text '18 Larkspur Lane'.
