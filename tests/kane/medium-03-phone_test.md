---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, medium]
variables:
  record:
    value: |-
      Telephone Encounter

      Spoke with Naomi Feldstein regarding her lab results. She asked that we send a copy to naomi.feldstein@example.com and confirmed that her mobile number is still 646-555-0159. Her husband Daniel was on the line and asked about dietary changes.

      LDL cholesterol is 172 mg/dL and HDL is 38 mg/dL. Started Atorvastatin 20 mg nightly. Advised a Mediterranean diet and 150 minutes of exercise per week. Naomi was born 02/11/1967 and has a family history of early coronary artery disease. Mail the lipid handout to 31 Tannery Row, Apt 9, Bexley Point, NY 10299.
---

# medium-03: phone is removed, medical content is kept

Generated from tests/fixtures/medium-03.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text 'LDL cholesterol is 172 mg/dL'.
Verify that the Result region contains the text 'HDL is 38 mg/dL'.
Verify that the Result region contains the text 'Atorvastatin 20 mg nightly'.
Verify that the Result region contains the text 'Mediterranean diet'.
Verify that the Result region contains the text '150 minutes of exercise per week'.
Verify that the Result region contains the text 'early coronary artery disease'.
Verify that the Result region does not contain the text '646-555-0159'.
