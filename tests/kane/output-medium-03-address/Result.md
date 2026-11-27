---
test: ../medium-03-address_test.md
status: passed
started: 2026-11-26T23:04:36.503Z
duration_s: 64
session_id: 90098a21-4cd1-47d2-ba5d-90406bab9c6e
---

# medium-03: address is removed, medical content is kept — Result

## Submit the record ✓ passed (22.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (37.1s)
md5: 82356046d0052b35ab4fcdbb65532098
Verify that the Result region contains the text 'LDL cholesterol is 172 mg/dL'.
Verify that the Result region contains the text 'HDL is 38 mg/dL'.
Verify that the Result region contains the text 'Atorvastatin 20 mg nightly'.
Verify that the Result region contains the text 'Mediterranean diet'.
Verify that the Result region contains the text '150 minutes of exercise per week'.
Verify that the Result region contains the text 'early coronary artery disease'.
Verify that the Result region does not contain the text '31 Tannery Row'.
Verify that the Result region does not contain the text 'Apt 9'.
Verify that the Result region does not contain the text 'Bexley Point'.
Verify that the Result region does not contain the text '10299'.
