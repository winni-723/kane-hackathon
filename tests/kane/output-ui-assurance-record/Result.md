---
test: ../ui-assurance-record_test.md
status: passed
started: 2026-11-26T23:39:12.540Z
duration_s: 88
session_id: 5d94c799-5108-42bf-9ca6-e65da38267ca
---

# A single record from the menu gets an assurance report that holds none of its text — Result

## Load the sample from the menu and submit it ✓ passed (39.1s)
md5: 9342f8b98faf03091700819ea09b02d7
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Assurance report heading is visible.

## The report states its review status and what was replaced, and no record text ✓ passed (30.8s)
md5: 3f13edf6624edb40ae1b7b720b9047b4
Verify that the Assurance report region, taken as a whole, contains the text 'Review status: not reviewed. Release: not approved.'
Verify that the Assurance report region, taken as a whole, contains the text '3 names, 2 phone numbers'
Verify that the Assurance report region, taken as a whole, contains the text 'It is not a HIPAA compliance determination'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Grace Whitfield'
Verify that the Assurance report region, taken as a whole, does not contain the text '18 Larkspur Lane'
Verify that the Assurance report region, taken as a whole, does not contain the text 'atrial fibrillation'

## The report can be downloaded ✓ passed (13s)
md5: 853dc0e9e00b1ab6a69c26e867103287
Verify that the Download report (JSON) button is visible.
