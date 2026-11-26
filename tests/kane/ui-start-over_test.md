---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
variables:
  first:
    value: |-
      Phone: (617) 555-0142
      Diagnosis: Migraine without aura.
  second:
    value: |-
      Phone: (413) 555-0177
      Diagnosis: Seasonal allergic rhinitis.
---

# Start over on the Result goes back to an empty input

Hand-written UI test. Do not edit: this test is part of the regression suite.
If Start over kept the first record, the second Result would hold both.

## Submit the first record
Type {{first}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Start over
Click the Start over button in the Result region.
Verify that the Upload records heading is visible.

## Submit a second record
Type {{second}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## The Result holds only the second record
Verify that the Result region contains the text 'Seasonal allergic rhinitis'.
Verify that the Result region does not contain the text 'Migraine without aura'.
Verify that the Result region does not contain the text '(413) 555-0177'.
