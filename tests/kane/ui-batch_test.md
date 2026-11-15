---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
---

# Several records become a research table with no identifiers in it

Hand-written UI test. Do not edit: this test is part of the regression suite.
The expected cells come from tests/extraction/medium-01.json, medium-04.json
and hard-04.json.

## Load the sample batch and submit it
Click the 'Load sample batch' button and then click the Process button.
Verify that the Research table heading is visible.

## The table holds the research columns and no identifiers
Verify that the Research table region contains the text 'atrial fibrillation'.
Verify that the Research table region contains the text 'community-acquired pneumonia'.
Verify that the Research table region contains the text 'post-traumatic stress disorder'.
Verify that the Research table region contains the text 'Sertraline 50 mg daily'.
Verify that the Research table region contains the text 'Record 3'.
Verify that the Research table region does not contain the text 'Grace Whitfield'.
Verify that the Research table region does not contain the text 'Bernard Kowalczyk'.
Verify that the Research table region does not contain the text 'Malik'.
