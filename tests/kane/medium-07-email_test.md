---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [email, medium]
variables:
  record:
    value: |-
      Pre-operative Assessment

      Samuel Oyelaran, 58, is scheduled for laparoscopic cholecystectomy for symptomatic cholelithiasis. He confirmed his contact details at check-in: home phone (404) 555-0135, email s.oyelaran@example.net. His wife Bisi will drive him home and can be reached on 404-555-0172.

      Past history includes hypertension controlled on Amlodipine 5 mg daily. No known drug allergies. ASA class II. Hold Amlodipine on the morning of surgery; nothing by mouth after midnight. Samuel was born on 3/8/1967. Chart number 60917734. Post-operative instructions will be mailed to 1408 Peachtree Hollow Way, Dunlow Crossing, GA 30399.
---

# medium-07: email is removed, medical content is kept

Generated from tests/fixtures/medium-07.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone
Verify that the Result region contains the text 'laparoscopic cholecystectomy'.
Verify that the Result region contains the text 'symptomatic cholelithiasis'.
Verify that the Result region contains the text 'Amlodipine 5 mg daily'.
Verify that the Result region contains the text 'No known drug allergies'.
Verify that the Result region contains the text 'ASA class II'.
Verify that the Result region contains the text 'nothing by mouth after midnight'.
Verify that the Result region does not contain the text 's.oyelaran@example.net'.
