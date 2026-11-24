---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
---

# The three-record sample gets one assurance report with group sizes and no identifiers

Hand-written UI test. Do not edit: this test is part of the regression suite.
The records are tests/fixtures/medium-01.json, medium-04.json and hard-04.json.

## Load the sample batch from the menu and submit it
Select 'Three records (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Research table heading is visible.

## The report covers the set and names no patient
Verify that the Assurance report region, taken as a whole, contains the text '11 names'
Verify that the Assurance report region, taken as a whole, contains the text 'Smallest group sharing a 10-year age band and sex: 1'
Verify that the Assurance report region, taken as a whole, contains the text 'Records with every line kept: 3 of 3'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Grace Whitfield'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Bernard Kowalczyk'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Malik'
