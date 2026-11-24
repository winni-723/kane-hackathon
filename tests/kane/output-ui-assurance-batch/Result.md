---
test: ../ui-assurance-batch_test.md
status: passed
started: 2026-11-24T05:39:56.976Z
duration_s: 102
session_id: f9d1be97-82d2-42a1-9f91-70b0e9a7d4eb
---

# The three-record sample gets one assurance report with group sizes and no identifiers — Result

## Load the sample batch from the menu and submit it ✓ passed (50.7s)
md5: 144424a92b99ee4bb01d5ca76979b7ba
Select 'Three records (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Research table heading is visible.

## The report covers the set and names no patient ✓ passed (47s)
md5: ab924180f60b67bed7c660e7539ccee8
Verify that the Assurance report region, taken as a whole, contains the text '11 names'
Verify that the Assurance report region, taken as a whole, contains the text 'Smallest group sharing a 10-year age band and sex: 1'
Verify that the Assurance report region, taken as a whole, contains the text 'Records with every line kept: 3 of 3'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Grace Whitfield'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Bernard Kowalczyk'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Malik'
