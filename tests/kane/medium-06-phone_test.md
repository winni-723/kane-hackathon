---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, medium]
variables:
  record:
    value: |-
      Pediatric Visit

      Mother, Farah Haddad, brings in her 4-year-old son Zayd with fever of 102.4 F and right ear pain for two days. Zayd attends preschool and two classmates were recently sick. Father Karim is traveling but asked to be updated by text at 248-555-0163.

      Exam shows a bulging, erythematous right tympanic membrane. Diagnosis: acute otitis media. Prescribed Amoxicillin 400 mg twice daily for 10 days and Acetaminophen 160 mg every 6 hours as needed. Return if fever persists beyond 48 hours. Zayd's birthday is 08/21/2021. The family lives at 56 Orchard Wren Circle, Halvorsen Mills, MI 48399.
---

# medium-06: phone is removed, medical content is kept

Generated from tests/fixtures/medium-06.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text '4-year-old son'.
Verify that the Result region contains the text 'fever of 102.4 F'.
Verify that the Result region contains the text 'right ear pain for two days'.
Verify that the Result region contains the text 'acute otitis media'.
Verify that the Result region contains the text 'Amoxicillin 400 mg twice daily for 10 days'.
Verify that the Result region contains the text 'Acetaminophen 160 mg every 6 hours as needed'.
Verify that the Result region contains the text 'beyond 48 hours'.
Verify that the Result region does not contain the text '248-555-0163'.
