---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [name, easy]
variables:
  record:
    value: |-
      Elsa Solís
      ========í=
      Race:                White
      Ethnicity:           Hispanic
      Gender:              F
      Age:                 1
      Birth Date:          2025-01-06
      Marital Status:      S
      SSN:                 999-66-6592
      Address:             491 Ondricka Neck
      City:                Haverhill
      State:               Massachusetts
      Zip:                 01832
      --------------------------------------------------------------------------------
      ALLERGIES:
      No Known Allergies
      --------------------------------------------------------------------------------
      CONDITIONS:
        2025-12-15 -            : Medication review due (situation)
        2025-06-16 - 2025-09-15 : Medication review due (situation)
        2025-04-14 - 2025-04-14 : Medication review due (situation)
        2025-01-06 - 2025-02-10 : Medication review due (situation)
---

# easy-s16: name is removed, medical content is kept

Generated from tests/synthea/easy-s16.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text 'Elsa'.
Verify that the Result region does not contain the text 'Solís'.
