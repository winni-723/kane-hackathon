---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
---

# The sample record button fills the text box with a record that de-identifies

Hand-written UI test. Do not edit: this test is part of the regression suite.

## Load the sample and submit it
Click the 'Load sample record' button and then click the Process button.
Verify that the Result heading is visible.

## The sample comes back de-identified
Verify that the Result region contains the text 'follow-up of atrial fibrillation'.
Verify that the Result region contains the text 'Warfarin 5 mg daily'.
Verify that the Result region does not contain the text 'Grace Whitfield'.
Verify that the Result region does not contain the text '18 Larkspur Lane'.
Verify that the Result region does not contain the text '(508) 555-0114'.
