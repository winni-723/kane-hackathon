---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [email, easy]
variables:
  record:
    value: |-
      PATIENT RECORD
      Name: Rosalind Achterberg
      DOB: 12/30/1963
      Phone: +1 303 555 0154
      Email: r.achterberg@example.org
      Address: 615 Quarry Hill Avenue, Unit 12, Stonehaven Ridge, CO 80299
      SSN: 000-45-6789
      MRN: 31058862

      Diagnosis: Major depressive disorder, recurrent, moderate.
      Medications: Sertraline 100 mg daily.
      Assessment: PHQ-9 score 14, improved from 19.
      Plan: Continue current dose, cognitive behavioral therapy weekly.
---

# easy-06: email is removed, medical content is kept

Generated from tests/fixtures/easy-06.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text 'r.achterberg@example.org'.
