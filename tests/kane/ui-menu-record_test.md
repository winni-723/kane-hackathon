---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
---

# The progress note sample from the menu comes back de-identified

Hand-written UI test. Do not edit: this test is part of the regression suite.
The record is tests/fixtures/medium-01.json, served by the page as a sample.

## Load the sample from the menu and submit it
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Result heading is visible.

## The sample comes back de-identified
Verify that the Result region contains the text 'follow-up of atrial fibrillation'.
Verify that the Result region contains the text 'Warfarin 5 mg daily'.
Verify that the Result region does not contain the text 'Grace Whitfield'.
Verify that the Result region does not contain the text '18 Larkspur Lane'.
Verify that the Result region does not contain the text '(508) 555-0114'.
