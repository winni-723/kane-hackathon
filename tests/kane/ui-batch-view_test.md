---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
---

# Records opened from the research table are de-identified, and Next moves on

Hand-written UI test. Do not edit: this test is part of the regression suite.
The records are tests/fixtures/medium-04.json and hard-04.json.

## Build the table from the three-record sample
Select 'Three records (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Research table heading is visible.

## Open the second record
Click the 'View record 2' button.
Verify that the Result region contains the text 'admitted with community-acquired pneumonia'.
Verify that the Result region does not contain the text 'Bernard Kowalczyk'.
Verify that the Result region does not contain the text '740 Sycamore Hollow Road'.

## Move on to the third record
Click the Next button.
Verify that the Result region contains the text 'Diagnosis: post-traumatic stress disorder'.
Verify that the Result region does not contain the text 'Malik'.
Verify that the Result region does not contain the text '(518) 555 0118'.

## The details say which record this is
Verify that the Record details region contains the text 'Record 3 of 3'.
