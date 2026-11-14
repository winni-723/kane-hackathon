---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [address, hard]
variables:
  record:
    value: |-
      Social Work Note

      Met with the patient known on the unit as "Big Jim" (James T. Holloway Jr.) and his daughter-in-law, Rosa. Mr. Holloway has end-stage COPD on 3 L/min home oxygen and was admitted for his third exacerbation this year. He uses Tiotropium 18 mcg daily and Albuterol nebulizers every 4 hours.

      J.T. says he cannot manage stairs at his place, the room above Holloway's Hardware on the corner of Finch and Lowell. Rosa offered her ground-floor home instead. Hospice referral discussed; GOLD stage 4, mMRC dyspnea grade 4. Rosa's contact: text only, 9 1 7 5 5 5 0 1 3 1. Patient DOB per wristband: 09-06-48.
---

# hard-06: address is removed, medical content is kept

Generated from tests/fixtures/hard-06.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone
Verify that the Result region contains the text 'end-stage COPD'.
Verify that the Result region contains the text '3 L/min home oxygen'.
Verify that the Result region contains the text 'Tiotropium 18 mcg daily'.
Verify that the Result region contains the text 'Albuterol nebulizers every 4 hours'.
Verify that the Result region contains the text 'Hospice referral discussed'.
Verify that the Result region contains the text 'GOLD stage 4'.
Verify that the Result region contains the text 'mMRC dyspnea grade 4'.
Verify that the Result region does not contain the text "room above Holloway's Hardware on the corner of Finch and Lowell".
