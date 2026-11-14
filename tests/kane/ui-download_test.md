---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui]
variables:
  record:
    value: |-
      Phone: (617) 555-0142
      Diagnosis: Migraine without aura.
---

# The Result offers a download and a way back to the original

Hand-written UI test. Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## The Result has its controls
Verify that the Result region contains the text 'Migraine without aura'.
Verify that the Result region does not contain the text '(617) 555-0142'.
Verify that a button labelled 'Download result' is visible in the Result region.
Verify that a button labelled 'Edit original' is visible in the Result region.
