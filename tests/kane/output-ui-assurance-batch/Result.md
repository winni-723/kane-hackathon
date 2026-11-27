---
test: ../ui-assurance-batch_test.md
status: passed
started: 2026-11-26T23:24:06.206Z
duration_s: 65
session_id: 0374c300-9e31-4458-b0fe-482e9dcdcea7
---

# The three-record sample gets one assurance report with group sizes and no identifiers — Result

## Load the sample batch from the menu and submit it ✓ passed (29.5s)
md5: 144424a92b99ee4bb01d5ca76979b7ba
Select 'Three records (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Research table heading is visible.

## The report covers the set and names no patient ✓ passed (32.4s)
md5: ab924180f60b67bed7c660e7539ccee8
Verify that the Assurance report region, taken as a whole, contains the text '11 names'
Verify that the Assurance report region, taken as a whole, contains the text 'Smallest group sharing a 10-year age band and sex: 1'
Verify that the Assurance report region, taken as a whole, contains the text 'Records with every line kept: 3 of 3'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Grace Whitfield'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Bernard Kowalczyk'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Malik'
