---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
---

# A single record from the menu gets an assurance report that holds none of its text

Hand-written UI test. Do not edit: this test is part of the regression suite.
The progress note sample is tests/fixtures/medium-01.json.

## Load the sample from the menu and submit it
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Assurance report heading is visible.

## The report states its review status and what was replaced, and no record text
Verify that the Assurance report region, taken as a whole, contains the text 'Review status: not reviewed. Release: not approved.'
Verify that the Assurance report region, taken as a whole, contains the text '3 names, 2 phone numbers'
Verify that the Assurance report region, taken as a whole, contains the text 'It is not a HIPAA compliance determination'
Verify that the Assurance report region, taken as a whole, does not contain the text 'Grace Whitfield'
Verify that the Assurance report region, taken as a whole, does not contain the text '18 Larkspur Lane'
Verify that the Assurance report region, taken as a whole, does not contain the text 'atrial fibrillation'

## The report can be downloaded
Verify that the Download report (JSON) button is visible.
