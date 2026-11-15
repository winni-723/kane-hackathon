---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [address, easy]
variables:
  record:
    value: |-
      LAB REQUISITION
      Pt: Delphine Moreau
      D.O.B.: 02-19-1990
      Acct #: 40077213
      Tel: (504) 555-0148
      Home: 88 Rue Cypress Lane, Apt 3C, Bellefontaine Bayou, LA 70099
      Soc Sec: 900-31-4477

      Tests Ordered: CBC with differential, ferritin, TSH.
      Indication: Fatigue and heavy menstrual bleeding for 3 months.
      Results: Hemoglobin 9.8 g/dL, ferritin 6 ng/mL, TSH 2.1 mIU/L.
      Impression: Iron deficiency anemia.
      Plan: Ferrous sulfate 325 mg twice daily; recheck in 6 weeks.
---

# easy-102: address is removed, medical content is kept

Generated from tests/heldout/easy-102.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone
Verify that the Result region contains the text 'CBC with differential'.
Verify that the Result region contains the text 'Fatigue and heavy menstrual bleeding for 3 months'.
Verify that the Result region contains the text 'Hemoglobin 9.8 g/dL'.
Verify that the Result region contains the text 'ferritin 6 ng/mL'.
Verify that the Result region contains the text 'Iron deficiency anemia'.
Verify that the Result region contains the text 'Ferrous sulfate 325 mg twice daily'.
Verify that the Result region does not contain the text '88 Rue Cypress Lane'.
Verify that the Result region does not contain the text 'Apt 3C'.
Verify that the Result region does not contain the text 'Bellefontaine Bayou'.
Verify that the Result region does not contain the text '70099'.
