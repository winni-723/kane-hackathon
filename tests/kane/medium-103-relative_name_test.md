---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [relative_name, medium]
variables:
  record:
    value: |-
      Pharmacy Callback

      Pharmacist called regarding a possible interaction for Leilani Kahananui. The caller noted the patient is taking Clarithromycin 500 mg twice daily for H. pylori and was just prescribed Simvastatin 40 mg nightly. Recommended holding the statin until the antibiotic course ends in 14 days.

      Confirmed identity with DOB 9/9/1968 and the last four of her SSN. Full SSN on file: 000-77-1234. Leilani asked that we update her address to 6 Kukui Nut Way, Unit 204, Hanalei Ridge, HI 96799 and text her son Keoni at (808) 555-0155 with the new instructions. Also on Amlodipine 10 mg daily; no change needed.
---

# medium-103: relative name is removed, medical content is kept

Generated from tests/heldout/medium-103.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted relative name is gone
Verify that the Result region contains the text 'Clarithromycin 500 mg twice daily'.
Verify that the Result region contains the text 'H. pylori'.
Verify that the Result region contains the text 'Simvastatin 40 mg nightly'.
Verify that the Result region contains the text 'until the antibiotic course ends in 14 days'.
Verify that the Result region contains the text 'Amlodipine 10 mg daily'.
Verify that the Result region does not contain the text 'Keoni'.
