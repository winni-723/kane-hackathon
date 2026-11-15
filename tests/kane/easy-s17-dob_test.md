---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [dob, easy]
variables:
  record:
    value: |-
      Evia Ernser
      ===========
      Race:                White
      Ethnicity:           Non-Hispanic
      Gender:              F
      Age:                 2
      Birth Date:          2024-03-17
      Marital Status:      S
      SSN:                 999-35-7133
      Address:             517 Bogan Passage
      City:                Quincy
      State:               Massachusetts
      Zip:                 02169
      --------------------------------------------------------------------------------
      ALLERGIES:
      No Known Allergies
      --------------------------------------------------------------------------------
      CONDITIONS:
        2026-08-23 - 2026-08-23 : Medication review due (situation)
        2026-02-22 - 2026-02-22 : Medication review due (situation)
        2025-08-24 - 2025-08-24 : Medication review due (situation)
        2025-05-25 - 2025-05-25 : Medication review due (situation)
        2024-11-24 - 2025-02-23 : Medication review due (situation)
        2024-03-17 - 2024-08-25 : Medication review due (situation)
---

# easy-s17: dob is removed, medical content is kept

Generated from tests/synthea/easy-s17.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '2024-03-17'.
