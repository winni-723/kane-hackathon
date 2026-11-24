---
test: ../ui-assurance-record_test.md
status: passed
started: 2026-11-24T05:39:57.212Z
duration_s: 87
session_id: eabffda0-6bc1-4395-af7d-9f79b7382b00
---

# A single record from the menu gets an assurance report that holds none of its text — Result

## Load the sample from the menu and submit it ✓ passed (29.3s)
md5: 9342f8b98faf03091700819ea09b02d7
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Assurance report heading is visible.

## The report states its review status and what was replaced, and no record text ✓ passed (40.3s)
md5: 3f13edf6624edb40ae1b7b720b9047b4
Verify that the Assurance report region, taken as a whole, contains the text 'Review status: not reviewed. Release: not approved.'
Verify that the Assurance report region, taken as a whole, contains the text '3 names, 2 phone numbers'
Verify that the Assurance report region, taken as a whole, contains the text 'It is not a HIPAA compliance determination'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Grace Whitfield'
Verify that the Assurance report region, taken as a whole, does not contain the text '18 Larkspur Lane'
Verify that the Assurance report region, taken as a whole, does not contain the text 'atrial fibrillation'

## The report can be downloaded ✓ passed (12.9s)
md5: 853dc0e9e00b1ab6a69c26e867103287
Verify that the Download report (JSON) button is visible.
