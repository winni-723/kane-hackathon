---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, easy]
variables:
  record:
    value: |-
      PATIENT RECORD
      Patient: Amara Okafor
      Date of Birth: 09/18/2015
      Parent/Guardian: Chinedu Okafor
      Guardian Phone: (713) 555-0176
      Address: 3305 Pecan Grove Street, Marlow Springs, TX 77099

      Chief Complaint: Wheezing and nighttime cough for 3 days.
      Diagnosis: Mild persistent asthma, acute exacerbation.
      Medications: Albuterol inhaler 2 puffs every 4 hours as needed; Fluticasone 44 mcg twice daily.
      Vitals: SpO2 95% on room air, respiratory rate 28.
      Plan: Asthma action plan reviewed with family.
---

# easy-05: phone is removed, medical content is kept

Generated from tests/fixtures/easy-05.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text 'Wheezing and nighttime cough for 3 days'.
Verify that the Result region contains the text 'Mild persistent asthma'.
Verify that the Result region contains the text 'Albuterol inhaler 2 puffs every 4 hours as needed'.
Verify that the Result region contains the text 'Fluticasone 44 mcg twice daily'.
Verify that the Result region contains the text 'SpO2 95% on room air'.
Verify that the Result region contains the text 'respiratory rate 28'.
Verify that the Result region does not contain the text '(713) 555-0176'.
