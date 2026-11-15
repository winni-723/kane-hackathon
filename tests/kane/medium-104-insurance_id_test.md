---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [insurance_id, medium]
variables:
  record:
    value: |-
      Physical Therapy Evaluation

      Referred after left total knee replacement three weeks ago. Patient, Margaret O'Sullivan, lives with her husband in a two-story home and wants to return to gardening. Knee flexion is 85 degrees, extension lacks 10 degrees. Quadriceps strength 3+/5. Timed Up and Go is 19 seconds.

      Goals: flexion to 120 degrees in 6 weeks; independent stair negotiation. Home exercise program reviewed with Margaret and with Declan, who will help with icing. Appointments should be confirmed with Declan O'Sullivan at 207.555.0171 or declan.os@example.net. Insurance requires her date of birth (March 21, 1957) and policy number MED-5540129 on each claim. Mailing address: 15 Lighthouse Cove Road, Port Elswick, ME 04099.
---

# medium-104: insurance id is removed, medical content is kept

Generated from tests/heldout/medium-104.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted insurance id is gone
Verify that the Result region contains the text 'left total knee replacement'.
Verify that the Result region contains the text 'Knee flexion is 85 degrees'.
Verify that the Result region contains the text 'Quadriceps strength 3+/5'.
Verify that the Result region contains the text 'Timed Up and Go is 19 seconds'.
Verify that the Result region contains the text 'flexion to 120 degrees in 6 weeks'.
Verify that the Result region contains the text 'independent stair negotiation'.
Verify that the Result region does not contain the text 'MED-5540129'.
