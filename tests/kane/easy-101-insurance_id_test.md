---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [insurance_id, easy]
variables:
  record:
    value: |-
      REGISTRATION SHEET
      Last Name: Brennan
      First Name: Oliver
      Birthdate: 6/2/1975
      Sex: M
      Street: 512 Copperleaf Terrace
      City/State/ZIP: Hartwell Glen, OR 97099
      Cell: 503-555-0166
      E-mail: oliver.brennan@example.org
      Subscriber ID: QRT-88120457
      Next of Kin: Siobhan Brennan (spouse) 503-555-0119

      Presenting Problem: Burning epigastric pain after meals for one month.
      Assessment: Gastroesophageal reflux disease.
      Rx: Omeprazole 20 mg daily before breakfast for 8 weeks.
      Advice: Avoid late meals, elevate head of bed.
---

# easy-101: insurance id is removed, medical content is kept

Generated from tests/heldout/easy-101.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted insurance id is gone
Verify that the Result region contains the text 'Burning epigastric pain after meals'.
Verify that the Result region contains the text 'Gastroesophageal reflux disease'.
Verify that the Result region contains the text 'Omeprazole 20 mg daily before breakfast for 8 weeks'.
Verify that the Result region contains the text 'elevate head of bed'.
Verify that the Result region does not contain the text 'QRT-88120457'.
