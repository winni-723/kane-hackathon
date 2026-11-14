---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [address, easy]
variables:
  record:
    value: |-
      PATIENT RECORD
      Name: Walter Nakamura
      DOB: 01/05/1946
      Phone: 206.555.0108
      Address: 9 Thistle Ridge Drive, Corwin Bay, WA 98199
      MRN: 77120345
      Insurance ID: HX-4490217

      Chief Complaint: Resting tremor of the right hand and slowed gait.
      Diagnosis: Parkinson's disease, Hoehn and Yahr stage 2.
      Medications: Carbidopa-Levodopa 25-100 mg three times daily.
      Plan: Physical therapy referral, fall precautions.
---

# easy-04: address is removed, medical content is kept

Generated from tests/fixtures/easy-04.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone
Verify that the Result region contains the text 'Resting tremor of the right hand'.
Verify that the Result region contains the text "Parkinson's disease".
Verify that the Result region contains the text 'Hoehn and Yahr stage 2'.
Verify that the Result region contains the text 'Carbidopa-Levodopa 25-100 mg three times daily'.
Verify that the Result region contains the text 'fall precautions'.
Verify that the Result region does not contain the text '9 Thistle Ridge Drive'.
Verify that the Result region does not contain the text 'Corwin Bay'.
Verify that the Result region does not contain the text '98199'.
