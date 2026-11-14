---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, easy]
variables:
  record:
    value: |-
      PATIENT RECORD
      Name: Tobias Lindqvist
      DOB: 05/22/1980
      Phone: (612) 555-0129
      Work Phone: (612) 555-0181
      Address: 204 Birchwater Trail, Ellery Falls, MN 55499
      Emergency Contact: Ingrid Lindqvist (wife) - (612) 555-0102

      Chief Complaint: Right knee pain and swelling after a twisting injury.
      Imaging: MRI shows a medial meniscus tear.
      Diagnosis: Medial meniscus tear, right knee.
      Medications: Ibuprofen 600 mg every 8 hours with food.
      Plan: Orthopedic referral for arthroscopy, crutches, ice 20 minutes three times daily.
---

# easy-07: phone is removed, medical content is kept

Generated from tests/fixtures/easy-07.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text 'Right knee pain and swelling'.
Verify that the Result region contains the text 'MRI shows a medial meniscus tear'.
Verify that the Result region contains the text 'Ibuprofen 600 mg every 8 hours with food'.
Verify that the Result region contains the text 'arthroscopy'.
Verify that the Result region contains the text 'ice 20 minutes three times daily'.
Verify that the Result region does not contain the text '(612) 555-0129'.
Verify that the Result region does not contain the text '(612) 555-0181'.
Verify that the Result region does not contain the text '(612) 555-0102'.
