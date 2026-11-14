---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, hard]
variables:
  record:
    value: |-
      Home Health Note

      Visited Ms. Carter this afternoon for a wound check. E. Carter is recovering from a left total hip arthroplasty and is weight bearing as tolerated with a walker. Mrs. Carter's neighbor let me in; the patient lives in the yellow house across from the old Fairmont cannery, second floor.

      Incision is clean, dry and intact with no erythema. Pain is 3 out of 10 on Oxycodone 5 mg every 6 hours as needed. Continue Enoxaparin 40 mg subcutaneously daily for DVT prophylaxis. Her grandson Teddy fills her pill organizer and said to ring his cell, five five five, zero one four four, if anything changes. Homans sign negative.
---

# hard-01: phone is removed, medical content is kept

Generated from tests/fixtures/hard-01.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text 'left total hip arthroplasty'.
Verify that the Result region contains the text 'weight bearing as tolerated'.
Verify that the Result region contains the text 'Incision is clean, dry and intact'.
Verify that the Result region contains the text 'Pain is 3 out of 10'.
Verify that the Result region contains the text 'Oxycodone 5 mg every 6 hours as needed'.
Verify that the Result region contains the text 'Enoxaparin 40 mg subcutaneously daily'.
Verify that the Result region contains the text 'Homans sign negative'.
Verify that the Result region does not contain the text 'five five five, zero one four four'.
