---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [dob, easy]
variables:
  record:
    value: |-
      PATIENT RECORD
      Name: Eleanor Carter
      DOB: 03/14/1958
      Phone: (617) 555-0142
      Address: 48 Willow Bend Road, Westmarch, MA 02199
      MRN: 00482917

      Chief Complaint: Shortness of breath on exertion for two weeks.
      Diagnosis: Congestive heart failure, NYHA class II.
      Medications: Furosemide 40 mg daily; Lisinopril 10 mg daily.
      Plan: Low-sodium diet, daily weights, follow-up echocardiogram in 4 weeks.
---

# easy-01: dob is removed, medical content is kept

Generated from tests/fixtures/easy-01.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone
Verify that the Result region contains the text 'Shortness of breath on exertion'.
Verify that the Result region contains the text 'Congestive heart failure'.
Verify that the Result region contains the text 'Furosemide 40 mg daily'.
Verify that the Result region contains the text 'Lisinopril 10 mg daily'.
Verify that the Result region contains the text 'echocardiogram in 4 weeks'.
Verify that the Result region does not contain the text '03/14/1958'.
