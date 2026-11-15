---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, medium]
variables:
  record:
    value: |-
      Nursing Shift Note

      Received report on Mr. Abdullahi at 0700. Ibrahim Abdullahi is post-op day 2 from a right hemicolectomy for adenocarcinoma of the ascending colon. Pain controlled on Hydromorphone PCA; tolerating clear liquids. Ambulated 200 feet with assistance.

      His eldest, Fatima, phoned from 651-555-0104 asking about visiting hours, and his brother Yusuf Abdullahi is listed as the health care proxy (yusuf.abdullahi@example.com). Drain output 40 mL serosanguinous. Continue Cefazolin 2 g IV every 8 hours and Heparin 5000 units subcutaneously every 12 hours. Patient's birthday is January 4, 1949. Discharge planning: he lives at 1187 Sandpiper Row, North Tallis, MN.
---

# medium-102: phone is removed, medical content is kept

Generated from tests/heldout/medium-102.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text 'post-op day 2'.
Verify that the Result region contains the text 'right hemicolectomy'.
Verify that the Result region contains the text 'adenocarcinoma of the ascending colon'.
Verify that the Result region contains the text 'Hydromorphone PCA'.
Verify that the Result region contains the text 'Ambulated 200 feet'.
Verify that the Result region contains the text 'Drain output 40 mL'.
Verify that the Result region contains the text 'Cefazolin 2 g IV every 8 hours'.
Verify that the Result region contains the text 'Heparin 5000 units subcutaneously every 12 hours'.
Verify that the Result region does not contain the text '651-555-0104'.
