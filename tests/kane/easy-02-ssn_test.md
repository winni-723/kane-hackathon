---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ssn, easy]
variables:
  record:
    value: |-
      PATIENT RECORD
      Name: Marcus Delgado
      Date of Birth: 11/02/1971
      Phone: 415-555-0117
      Email: marcus.delgado@example.com
      Address: 1260 Juniper Court, Apt 4B, Alder Creek, CA 94016
      SSN: 987-65-4321

      Reason for Visit: Routine follow-up for type 2 diabetes mellitus.
      Labs: HbA1c 7.8%, fasting glucose 152 mg/dL.
      Medications: Metformin 1000 mg twice daily.
      Assessment: Glycemic control suboptimal. Add Empagliflozin 10 mg daily.
      Follow-up: Repeat HbA1c in 3 months.
---

# easy-02: ssn is removed, medical content is kept

Generated from tests/fixtures/easy-02.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone
Verify that the Result region contains the text 'type 2 diabetes mellitus'.
Verify that the Result region contains the text 'HbA1c 7.8%'.
Verify that the Result region contains the text '152 mg/dL'.
Verify that the Result region contains the text 'Metformin 1000 mg twice daily'.
Verify that the Result region contains the text 'Empagliflozin 10 mg daily'.
Verify that the Result region does not contain the text '987-65-4321'.
