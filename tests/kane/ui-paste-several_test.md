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

# Two pasted records become a research table

Hand-written UI test. Do not edit: this test is part of the regression suite.

## Paste two records
Type {{first}} into the medical record text box.
Click the '+ Add another record' button.
Type {{second}} into the Record 2 text box.
Click the Process button.
Verify that the Research table heading is visible.

## The table holds both records and no phone numbers
Verify that the Research table region, taken as a whole, contains the text 'Migraine without aura'.
Verify that the Research table region, taken as a whole, contains the text 'Seasonal allergic rhinitis'.
Verify that the Research table region, taken as a whole, contains the text 'Record 2'.
Verify that the Research table region, taken as a whole, does not contain the text '(617) 555-0142'.
Verify that the Research table region, taken as a whole, does not contain the text '(413) 555-0177'.
